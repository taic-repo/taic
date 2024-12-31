#[macro_use]
extern crate log;

mod trap;
pub use trap::{init_utrap, register_handler, SOFT_IRQ_NUM};
