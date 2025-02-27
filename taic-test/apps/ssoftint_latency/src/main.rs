#![no_std]
#![no_main]
#[allow(static_mut_refs)]
extern crate alloc;
extern crate axstd;
use alloc::vec::Vec;
use axstd::println;
use lazy_init::LazyInit;
use taic_driver::{LocalQueue, Taic};

const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;
const LQ_NUM: usize = 8;

const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
const NUM: usize = 10_0000;
static mut START: usize = 0;
static mut INT_LATENCY: Vec<usize> = Vec::new();
static LQ: LazyInit<LocalQueue> = LazyInit::new();

#[no_mangle]
fn main() {
    println!("Start Taic software interrupt test ...");
    let lq0 = TAIC.alloc_lq(1, 0).unwrap();
    LQ.init_by(lq0);
    LQ.whart(axhal::cpu::this_cpu_id());
    LQ.register_receiver(1, 2, 0, 0x109);

    axhal::irq::register_handler(axhal::platform::irq::SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe { INT_LATENCY.push(end - START) };
        log::trace!("software interrupt {} {}", unsafe { START }, end);
        LQ.task_dequeue();
        LQ.register_receiver(1, 2, 0, 0x109);
    });
    let lq1 = TAIC.alloc_lq(1, 2).unwrap();
    lq1.register_sender(1, 5, 0);
    lq1.register_sender(1, 0, 0);
    let mut send_softintr_latency = Vec::new();
    for _ in 0..NUM {
        let start = riscv::register::cycle::read();
        lq1.send_intr(1, 5, 0);
        let end = riscv::register::cycle::read();
        send_softintr_latency.push(end - start);
    }
    println!("Send softintr latencys: {:?}", send_softintr_latency);
    println!("---------------------------------");
    for _ in 0..NUM {
        unsafe { START = riscv::register::cycle::read() };
        lq1.send_intr(1, 0, 0);
    }
    println!("Int latencys: {:?}", unsafe { INT_LATENCY.clone() });
}
