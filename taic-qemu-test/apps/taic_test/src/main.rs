#![no_std]
#![no_main]

extern crate axstd;
use axstd::println;
use core::arch::asm;
const TAIC_BASE: usize = axconfig::PHYS_VIRT_OFFSET + axconfig::MMIO_REGIONS[1].0;

#[no_mangle]
fn main() {
    println!("Hello, world! taic controller test {:#X}", TAIC_BASE);
    int_init();
    let lq0 = alloc_queue(1, 2);
    let lq1 = alloc_queue(1, 2);
    lq_enq(lq1, 0x0109);
    lq_enq(lq1, 0x010a);
    free_queue(lq1);
    let lq5 = alloc_queue(1, 0);
    lq_enq(lq5, 0x1999010b);
    lq_enq(lq5, 0x1999010c);
    lq_deq(lq5);
    lq_deq(lq5);
    lq_deq(lq5);
    lq_deq(lq5);

    let lq2 = alloc_queue(1, 5);
    lq_enq(lq2, 0x1999010c);
    free_queue(lq2);

    lq_enq(lq2, 0x1999010d);
    lq_enq(lq2, 0x1999010e);
    lq_deq(lq2);
    lq_deq(lq2);
    lq_deq(lq2);

    let lq3 = alloc_queue(2, 5);
    let lq4 = alloc_queue(2, 5);
    lq_enq(lq3, 0x1999010d);
    lq_enq(lq4, 0x1999010e);
    lq_deq(lq3);
    lq_deq(lq3);
    lq_deq(lq3);
    lq_deq(lq3);

    lq_deq(lq1);
    lq_enq(lq0, 0x1999010b);
    lq_deq(lq0);
    lq_deq(lq0);
    lq_deq(lq0);

    register_ext_handler(lq0, 0, 0x23456);
    register_ext_handler(lq0, 1, 0x1223456);
    sim_ext_intr(1);
    sim_ext_intr(0);
    sim_ext_intr(0);
    lq_deq(lq0);
    lq_deq(lq0);
    lq_enq(lq0, 0x1999010f);
    lq_deq(lq0);

    queue_register_sender(lq0, 2, 7);
    queue_register_receiver(lq3, 1, 2, 0x123457);
    lq_deq(lq3);
    queue_send_intr(lq0, 2, 7);
    let _lq6 = alloc_queue(3, 7);
    lq_deq(lq3);
    lq_deq(lq3);

    queue_register_sender(lq0, 2, 5);
    write_hartid(lq3, 0);
    queue_send_intr(lq0, 2, 5);
    // read uip
    let mut uip: usize;
    unsafe {
        asm!("csrr {}, 0x044", out(reg) uip);
    }
    println!("uip: {:x}", uip);
    lq_enq(lq3, 0x1999010f);
    lq_deq(lq3);
    unsafe {
        asm!("csrr {}, 0x044", out(reg) uip);
    }
    println!("uip: {:x}", uip);
    lq_deq(lq3);
    lq_deq(lq3);

    write_hartid(lq0, 0);
    register_ext_handler(lq0, 0, 0x23457);
    sim_ext_intr(0);
    // read uip
    unsafe {
        asm!("csrr {}, 0x044", out(reg) uip);
    }
    println!("uip: {:X}\n", uip);
    lq_deq(lq0);
    unsafe {
        asm!("csrr {}, 0x044", out(reg) uip);
    }
    println!("uip: {:X}\n", uip);

    write_hartid(lq5, 0);
    register_ext_handler(lq5, 0, 0x23457);
    sim_ext_intr(0);
    // read sip
    let mut sip: usize;
    unsafe {
        asm!("csrr {}, sip", out(reg) sip);
    }
    println!("sip: {:X}\n", sip);
    lq_deq(lq5);
    unsafe {
        asm!("csrr {}, sip", out(reg) sip);
    }
    println!("sip: {:X}\n", sip);
}

fn int_init() {
    // enable user software interrupt delegation
    let sideleg = 0x1;
    unsafe {
        asm!("csrw 0x103, {}", in(reg) sideleg);
    }
    println!("enable user software interrupt delegation");

    // clear uie
    let mut uie = 0;
    unsafe {
        asm!("csrw 0x004, {}", in(reg) uie);
    }
    println!("clear uie");

    // clear uip
    let uip = 0;
    unsafe {
        asm!("csrw 0x044, {}", in(reg) uip);
    }
    println!("clear uip");

    // set utvec
    let utvec = 0x80002000 as u64;
    unsafe {
        asm!("csrw 0x005, {}", in(reg) utvec);
    }
    println!("set utvec {:#X}", utvec);

    // ustatus enable user software interrupt
    let mut ustatus: usize;
    unsafe {
        asm!("csrr {}, 0x000", out(reg) ustatus);
    }
    ustatus |= 0x1;
    unsafe {
        asm!("csrw 0x000, {}", in(reg) ustatus);
    }
    unsafe {
        asm!("csrr {}, 0x000", out(reg) ustatus);
    }
    println!("enable ustatus: {:#X}", ustatus);

    // enable user software interrupt
    unsafe {
        asm!("csrr {}, 0x004", out(reg) uie);
    }
    uie |= 0x1;
    unsafe {
        asm!("csrw 0x004, {}", in(reg) uie);
    }
    unsafe {
        asm!("csrr {}, 0x004", out(reg) uie);
    }
    println!("enable uie: {}", uie);

    // // set usip
    // let usip = 0x1;
    // unsafe {
    //     asm!("csrw 0x044, {}", in(reg) usip);
    // }

    // // try return to user
    // unsafe {
    //     let mut sstatus = 0u64;
    //     asm!("csrr {}, 0x100", out(reg) sstatus);
    //     sstatus &= !0x100;
    //     sstatus |= 0x1;
    //     asm!("csrw 0x100, {}", in(reg) sstatus);
    //     let sepc = 0x80002000u64 + VIRT_ADDR_OFFSET;
    //     asm!("csrw 0x141, {}", in(reg) sepc);
    //     asm!("sret");
    // }

    println!("--------------------");
}

const LQ_NUM: usize = 2;

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
    println!("enq data {:#X}", data);
}

fn lq_deq(queue_base: usize) -> isize {
    let deq_ptr = (queue_base + 0x8) as *mut usize;
    let data = unsafe { deq_ptr.read_volatile() };
    println!("deq data {:#X}", data);
    data as isize
}

fn register_ext_handler(queue_base: usize, irq: usize, handler: usize) {
    let reg_ext_ptr = (queue_base + 0x40 + 0x8 * irq) as *mut usize;
    unsafe {
        reg_ext_ptr.write_volatile(handler);
    }
    println!(
        "{:x} register ext {:x}, handler {:x}",
        queue_base, irq, handler
    );
}

fn sim_ext_intr(ext_idx: usize) {
    let base = (TAIC_BASE + 0x10 + ext_idx * 0x8) as *mut usize;
    unsafe {
        base.write_volatile(1);
    }
    println!("sim ext intr {:x}", ext_idx);
}

fn queue_register_sender(queue_base: usize, receiver_os: usize, receiver_proc: usize) {
    let reg_send_ptr = (queue_base + 0x18) as *mut usize;
    unsafe {
        reg_send_ptr.write_volatile(receiver_os);
        reg_send_ptr.write_volatile(receiver_proc);
    }
    println!(
        "{:x} register sender receiver_os {:X} receiver_proc {:x}",
        queue_base, receiver_os, receiver_proc
    );
}

#[allow(unused)]
fn queue_cancel_sender(queue_base: usize, receiver_os: usize, receiver_proc: usize) {
    let cancel_send_ptr = (queue_base + 0x20) as *mut usize;
    unsafe {
        cancel_send_ptr.write_volatile(receiver_os);
        cancel_send_ptr.write_volatile(receiver_proc);
    }
    println!(
        "{:x} cancel sender receiver_os {:X} receiver_proc {:x}",
        queue_base, receiver_os, receiver_proc
    );
}

fn queue_register_receiver(
    queue_base: usize,
    sender_os: usize,
    sender_proc: usize,
    handler: usize,
) {
    let reg_recv_ptr = (queue_base + 0x28) as *mut usize;
    unsafe {
        reg_recv_ptr.write_volatile(sender_os);
        reg_recv_ptr.write_volatile(sender_proc);
        reg_recv_ptr.write_volatile(handler);
    }
    println!(
        "{:x} register receiver sender_os {:x} sender_proc {:x} handler {:x}",
        queue_base, sender_os, sender_proc, handler
    );
}

fn queue_send_intr(queue_base: usize, receiver_os: usize, receiver_proc: usize) {
    let sendintr_ptr = (queue_base + 0x30) as *mut usize;
    unsafe {
        sendintr_ptr.write_volatile(receiver_os);
        sendintr_ptr.write_volatile(receiver_proc);
    }
    println!(
        "{:x} send_intr receiver_os {:x} receiver_proc {:x}",
        queue_base, receiver_os, receiver_proc
    );
}

fn write_hartid(queue_base: usize, hartid: usize) {
    let write_ptr = (queue_base + 0x38) as *mut usize;
    unsafe {
        write_ptr.write_volatile(hartid);
    }
    println!("write hartid {:x}", hartid);
}
