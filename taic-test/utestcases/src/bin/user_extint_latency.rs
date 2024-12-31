#![allow(static_mut_refs)]

use lazy_init::LazyInit;
use taic_driver::{LocalQueue, Taic};

const LQ_NUM: usize = 2;
const TAIC_BASE: usize = 0x100_0000;
const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
static mut START: usize = 0;
static mut INT_LATENCY: Vec<usize> = Vec::new();
static LQ: LazyInit<LocalQueue> = LazyInit::new();
const NUM: usize = 10_0000;

extern crate utestcases;
use utestcases::{init_utrap, register_handler, SOFT_IRQ_NUM};

fn main() {
    println!("Start user external interrupt test...");
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
