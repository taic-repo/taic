#![no_std]
#![no_main]

extern crate alloc;
extern crate axstd;
use alloc::vec::Vec;
use axstd::println;
use taic_driver::Taic;

const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;
const LQ_NUM: usize = 8;

const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
const NUM: usize = 10_0000;
#[no_mangle]
fn main() {
    println!("Start Taic enq & deq test ...");
    let lq0 = TAIC.alloc_lq(1, 2).unwrap();
    let mut enq_cycles = Vec::new();
    let mut deq_cycles = Vec::new();
    for i in 0..NUM {
        let enq_start = riscv::register::cycle::read();
        lq0.task_enqueue(i);
        let enq_end = riscv::register::cycle::read();
        enq_cycles.push(enq_end - enq_start);
    }
    for _i in 0..NUM {
        let deq_start = riscv::register::cycle::read();
        let _ = lq0.task_dequeue();
        let deq_end = riscv::register::cycle::read();
        deq_cycles.push(deq_end - deq_start);
    }
    println!("Enq cycles: {:?}", enq_cycles);
    println!("---------------------------------");
    println!("Deq cycles: {:?}", deq_cycles);
}
