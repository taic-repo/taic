#![allow(static_mut_refs)]

static mut START: usize = 0;
static mut CONTEXT_OVERHEAD: Vec<usize> = Vec::new();
const NUM: usize = 10_0000;

extern crate utestcases;
use utestcases::{init_utrap, register_handler, SOFT_IRQ_NUM};

fn main() {
    println!("Start user interrupt context test...");

    init_utrap();
    register_handler(SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe {
            CONTEXT_OVERHEAD.push(end - START);
        };
    });
    for _ in 0..NUM {
        unsafe {
            START = riscv::register::cycle::read();
            riscv::register::uip::set_usoft();
        }
    }
    println!("User interrupt context overhead: {:?}", unsafe {
        CONTEXT_OVERHEAD.clone()
    });
}
