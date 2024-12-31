#![no_std]
#![no_main]
#[allow(static_mut_refs)]
extern crate alloc;
extern crate axstd;
use alloc::vec::Vec;
use axstd::println;

const NUM: usize = 10_0000;
static mut START: usize = 0;
static mut CONTEXT_OVERHEAD: Vec<usize> = Vec::new();

#[no_mangle]
fn main() {
    println!("Start supervisor software interrupt context test ...");
    axhal::irq::register_handler(axhal::platform::irq::SOFT_IRQ_NUM, || {
        let end = riscv::register::cycle::read();
        unsafe {
            CONTEXT_OVERHEAD.push(end - START);
            riscv::register::sip::clear_ssoft();
        }
        log::trace!("software interrupt {} {}", unsafe { START }, end);
    });
    for _ in 0..NUM {
        unsafe {
            START = riscv::register::cycle::read();
            riscv::register::sip::set_ssoft();
        }
    }
    println!("Supervisor interrupt context overhead: {:?}", unsafe {
        CONTEXT_OVERHEAD.clone()
    });
}
