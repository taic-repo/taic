#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;
extern crate axstarry;
use core::ptr::addr_of;

use alloc::vec::Vec;
use alloc::{format, vec};
use axhal::mem;
use axlog::ax_println;
use axprocess::{wait_pid, yield_now_task, Process, PID2PC};
use linux_syscall_api::{create_link, trap::MappingFlags, FilePath};
const TAIC_VA_BASE: usize = axconfig::MMIO_REGIONS[1].0;
const TAIC_PA_BASE: usize = axconfig::MMIO_REGIONS[1].0;
const PAGE_SIZE: usize = 0x1000;

use lazy_init::LazyInit;
use taic_driver::{LocalQueue, Taic};

const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;
const LQ_NUM: usize = 8;

const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);

static mut INT_LATENCY: Vec<usize> = Vec::new();
static LQ: LazyInit<LocalQueue> = LazyInit::new();

#[repr(C, align(4096))]
struct Timestamp {
    pub value: usize,
}

static mut START: Timestamp = Timestamp { value: 0 };

#[no_mangle]
fn main() {
    create_link(
        &FilePath::new("/lib/libc.so").unwrap(),
        &FilePath::new("/libc.so").unwrap(),
    );
    create_link(
        &FilePath::new("/lib/ld-musl-riscv64.so.1").unwrap(),
        &FilePath::new("/libc.so").unwrap(),
    );
    create_link(
        &FilePath::new("/lib/libgcc_s.so.1").unwrap(),
        &FilePath::new("/libgcc_s.so.1").unwrap(),
    );
    // setup receiver in supervisor mode
    let lq0 = TAIC.alloc_lq(1, 0).unwrap();
    LQ.init_by(lq0);
    LQ.whart(axhal::cpu::this_cpu_id());
    LQ.register_receiver(1, 2, 0, 0x109);

    axhal::irq::register_handler(axhal::platform::irq::SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe { INT_LATENCY.push(end - START.value) };
        log::trace!(
            "supervisor software interrupt from user {} {}",
            unsafe { START.value },
            end
        );
        LQ.task_dequeue();
        LQ.register_receiver(1, 2, 0, 0x109);
    });
    run_test("usint");
}

fn run_test(test: &str) {
    let hartid = axhal::cpu::this_cpu_id();
    let user_process =
        Process::init(vec![test.into()], &vec![format!("hartid={}", hartid)]).unwrap();
    let now_process_id = user_process.get_process_id() as i32;
    let pid2pc = PID2PC.lock();
    let process = pid2pc.get(&user_process.get_process_id()).unwrap();
    let memory_set = process.memory_set.lock();
    let start_paddr = axhal::mem::virt_to_phys((addr_of!(START) as usize).into());
    memory_set
        .lock()
        .map_page_without_alloc(
            0x200_0000.into(),
            start_paddr,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )
        .unwrap();
    for i in 0..(axconfig::MMIO_REGIONS[1].1 / PAGE_SIZE) {
        memory_set
            .lock()
            .map_page_without_alloc(
                (TAIC_VA_BASE + i * PAGE_SIZE).into(),
                (TAIC_PA_BASE + i * PAGE_SIZE).into(),
                MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
            )
            .unwrap();
    }
    drop(memory_set);
    drop(pid2pc);
    let mut exit_code = 0;
    loop {
        if unsafe { wait_pid(now_process_id, &mut exit_code as *mut i32) }.is_ok() {
            break;
        }
        yield_now_task();
    }
    linux_syscall_api::recycle_user_process();
    ax_println!("Int latencys: {:?}", unsafe { INT_LATENCY.clone() });
}
