#![no_std]
#![no_main]

extern crate alloc;
extern crate axstd;
use alloc::vec::Vec;
use axstd::println;
use taic_driver::Taic;

const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;
const LQ_NUM: usize = 2;

const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);

#[no_mangle]
fn main() {
    println!("Start Taic enq & deq test ...");
    correct_test();
    performance_test();
}

fn correct_test() {
    let lq0 = TAIC.alloc_lq(1, 2).unwrap();
    let lq1 = TAIC.alloc_lq(1, 2).unwrap();
    lq1.task_enqueue(0x0109);
    lq1.task_enqueue(0x010a);
    drop(lq1);

    let lq5 = TAIC.alloc_lq(1, 0).unwrap();
    lq5.task_enqueue(0x1999010b);
    lq5.task_enqueue(0x1999010c);
    lq5.task_dequeue();
    lq5.task_dequeue();
    lq5.task_dequeue();
    lq5.task_dequeue();
    let lq2 = TAIC.alloc_lq(1, 5).unwrap();
    lq2.task_enqueue(0x1999010c);
    drop(lq2);

    let lq3 = TAIC.alloc_lq(2, 5).unwrap();
    let lq4 = TAIC.alloc_lq(2, 5).unwrap();
    lq3.task_enqueue(0x1999010d);
    lq4.task_enqueue(0x1999010e);
    lq3.task_dequeue();
    lq3.task_dequeue();
    lq3.task_dequeue();
    lq3.task_dequeue();

    lq0.task_enqueue(0x1999010b);
    lq0.task_dequeue();
    lq0.task_dequeue();
    lq0.task_dequeue();
}

fn performance_test() {
    let lq0 = alloc_queue(1, 2);
    // let enq_cycles = Vec::new();
    for i in 0..1000 {
        let enq_start = riscv::register::cycle::read();
        lq_enq(lq0, i);
        // lq0.task_enqueue(i);
        let enq_end = riscv::register::cycle::read();
        println!("Enqueue cycle: {}", enq_end - enq_start);
    }
}

fn alloc_queue(os: usize, proc: usize) -> usize {
    let alloc_ptr = (TAIC_BASE + 0) as *mut usize;
    let idx = unsafe {
        alloc_ptr.write_volatile(os);
        alloc_ptr.write_volatile(proc);
        alloc_ptr.read_volatile()
    };
    println!("alloc idx {:#X}", idx);
    if idx == usize::MAX {
        return idx;
    }
    let gq_idx = (idx >> 32) & 0xffffffff;
    let lq_idx = idx & 0xffffffff;
    let base = TAIC_BASE + 0x1000 + (gq_idx * LQ_NUM + lq_idx) * 0x1000;
    base
}

fn free_queue(queue_base: usize) {
    let free_ptr = (TAIC_BASE + 0x8) as *mut usize;
    let queue_idx = (queue_base - TAIC_BASE - 0x1000) / 0x1000;
    let gq_idx = queue_idx / LQ_NUM;
    let lq_idx = queue_idx % LQ_NUM;
    let idx = (gq_idx << 32) | lq_idx;
    unsafe {
        free_ptr.write_volatile(idx);
    }
    println!("free idx {:#X}", queue_base);
}

fn lq_enq(queue_base: usize, data: usize) {
    let enq_ptr = (queue_base + 0x0) as *mut usize;
    unsafe {
        enq_ptr.write_volatile(data);
    }
}
