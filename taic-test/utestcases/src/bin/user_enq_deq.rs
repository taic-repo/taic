use taic_driver::Taic;

const LQ_NUM: usize = 2;
const TAIC_BASE: usize = 0x100_0000;
const TAIC: Taic = Taic::new(TAIC_BASE, LQ_NUM);
const NUM: usize = 10_0000;

fn main() {
    println!("Start user enq & deq test...");
    let lq0 = TAIC.alloc_lq(1, 2).unwrap();
    let mut enq_cycles = Vec::new();
    let mut deq_cycles = Vec::new();
    for i in 0..NUM {
        let enq_start = riscv::register::cycle::read();
        lq0.task_enqueue(i + 0x109);
        let enq_end = riscv::register::cycle::read();
        enq_cycles.push(enq_end - enq_start);
        let deq_start = riscv::register::cycle::read();
        let _ = lq0.task_dequeue();
        let deq_end = riscv::register::cycle::read();
        deq_cycles.push(deq_end - deq_start);
    }
    println!("Enqueue cycles: {:?}", enq_cycles);
    println!("---------------------------------");
    println!("Dequeue cycles: {:?}", deq_cycles);
}
