#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;
extern crate axstarry;
use alloc::{format, vec};
use axlog::ax_println;
use axprocess::{wait_pid, yield_now_task, Process, PID2PC};
use linux_syscall_api::{create_link, trap::MappingFlags, FilePath};
const TAIC_VA_BASE: usize = axconfig::MMIO_REGIONS[1].0;
const TAIC_PA_BASE: usize = axconfig::MMIO_REGIONS[1].0;
const PAGE_SIZE: usize = 0x1000;

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
    // enable user software interrupt
    unsafe {
        use riscv::register::sideleg;
        sideleg::set_usoft();
    }
    for test in TESTS.iter() {
        run_test(test);
    }
}

const TESTS: &[&str] = &[
    "user_enq_deq",
    "user_extint_latency",
    "user_int_context",
    "user_softint_latency",
];

fn run_test(test: &str) {
    let hartid = axhal::cpu::this_cpu_id();
    let user_process =
        Process::init(vec![test.into()], &vec![format!("hartid={}", hartid)]).unwrap();
    let now_process_id = user_process.get_process_id() as i32;
    let pid2pc = PID2PC.lock();
    let process = pid2pc.get(&user_process.get_process_id()).unwrap();
    let memory_set = process.memory_set.lock();
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
    ax_println!("---------------------------------");
}
