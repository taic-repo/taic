#![allow(static_mut_refs)]

use lazy_init::LazyInit;
use taic_driver::{LocalQueue, Taic};

const LQ_NUM: usize = 8;
const TAIC_BASE: usize = 0x100_0000;
const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
const NUM: usize = 10_0000;

extern crate utestcases;

fn main() {
    let lq0 = TAIC.alloc_lq(1, 2).unwrap();
    lq0.register_sender(1, 0);
    let timestamp = 0x200_0000 as *mut usize;
    for _ in 0..NUM {
        unsafe {
            *timestamp = riscv::register::cycle::read();
        }
        lq0.send_intr(1, 0);
    }
    let mut u2s2u = Vec::new();
    for _ in 0..NUM {
        let start = riscv::register::cycle::read();
        unsafe { core::arch::asm!("ebreak") };
        let end = riscv::register::cycle::read();
        u2s2u.push(end - start);
    }
    println!("u2s2u: {:?}", u2s2u);
}
