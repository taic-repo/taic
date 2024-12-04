#![no_std]
#![no_main]

extern crate axstd;
use axstd::println;
const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;

#[no_mangle]
fn main() {
    println!("Hello, world! indices test {:#X}", TAIC_BASE);
    let ptr = (TAIC_BASE + 16) as *mut usize;
    let data = unsafe { ptr.read_volatile() };
    println!("read data {:#X}", data);
}
