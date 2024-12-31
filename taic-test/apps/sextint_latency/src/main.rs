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
const LQ_NUM: usize = 2;

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

    axhal::irq::register_handler(axhal::platform::irq::SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe { INT_LATENCY.push(end - START) };
        log::trace!("software interrupt {} {}", unsafe { START }, end);
        LQ.task_dequeue();
    });

    let mut send_extintr_latency = Vec::new();
    for _ in 0..NUM {
        let start = riscv::register::cycle::read();
        TAIC.sim_extint(0);
        let end = riscv::register::cycle::read();
        send_extintr_latency.push(end - start);
    }
    println!("Send extintr latencys: {:?}", send_extintr_latency);
    println!("---------------------------------");
    for _ in 0..NUM {
        LQ.register_extintr(0, 0x109);
        unsafe { START = riscv::register::cycle::read() };
        TAIC.sim_extint(0);
    }
    println!("Int latencys: {:?}", unsafe { INT_LATENCY.clone() });
}
