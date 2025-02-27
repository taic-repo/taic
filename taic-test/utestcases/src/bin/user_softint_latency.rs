#![allow(static_mut_refs)]

use lazy_init::LazyInit;
use taic_driver::{LocalQueue, Taic};

const LQ_NUM: usize = 8;
const TAIC_BASE: usize = 0x100_0000;
const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
static mut START: usize = 0;
static mut INT_LATENCY: Vec<usize> = Vec::new();
static LQ: LazyInit<LocalQueue> = LazyInit::new();
const NUM: usize = 10_0000;

extern crate utestcases;
use utestcases::{init_utrap, register_handler, SOFT_IRQ_NUM};

fn main() {
    println!("Start user software interrupt test...");
    let hartid = std::env::var_os("hartid")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    init_utrap();
    let lq0 = TAIC.alloc_lq(1, 2).unwrap();
    LQ.init_by(lq0);
    LQ.whart(hartid);
    register_handler(SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe { INT_LATENCY.push(end - START) };
        // println!("user software interrupt");
        LQ.task_dequeue();
        LQ.register_receiver(1, 5, 0, 0x109);
    });
    LQ.register_receiver(1, 5, 0, 0x109);
    let lq1 = TAIC.alloc_lq(1, 5).unwrap();
    lq1.register_sender(1, 2, 0);
    let mut send_softintr_latency = Vec::new();
    for _ in 0..NUM {
        let start = riscv::register::cycle::read();
        lq1.send_intr(1, 0, 0);
        let end = riscv::register::cycle::read();
        send_softintr_latency.push(end - start);
    }
    println!("Send softintr latencys: {:?}", send_softintr_latency);
    println!("---------------------------------");
    for _ in 0..NUM {
        unsafe { START = riscv::register::cycle::read() };
        lq1.send_intr(1, 2, 0);
    }
    println!("Int latencys: {:?}", unsafe { INT_LATENCY.clone() });
}
