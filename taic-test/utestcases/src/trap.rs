use core::arch::global_asm;
use handler_table::{Handler, HandlerTable};
use lazy_init::LazyInit;
use riscv::register::{
    ucause, uepc, uie, uip,
    ustatus::{self, Ustatus},
    utval,
    utvec::{self, TrapMode},
};

extern "C" {
    fn __alltraps_u();
}

/// `Interrupt` bit in `ucause`
pub(super) const INTC_IRQ_BASE: usize = 1 << (usize::BITS - 1);

/// User software interrupt in `ucause`
#[allow(unused)]
pub(super) const U_SOFT: usize = INTC_IRQ_BASE + 0;

/// User timer interrupt in `ucause`
pub(super) const U_TIMER: usize = INTC_IRQ_BASE + 4;

/// User external interrupt in `ucause`
pub(super) const U_EXT: usize = INTC_IRQ_BASE + 8;

/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 1024;

static SOFT_HANDLER: LazyInit<Handler> = LazyInit::new();
static TIMER_HANDLER: LazyInit<Handler> = LazyInit::new();

/// The type if an IRQ handler.
pub type IrqHandler = handler_table::Handler;

static IRQ_HANDLER_TABLE: HandlerTable<MAX_IRQ_COUNT> = HandlerTable::new();

/// The soft IRQ number (User software interrupt in `ucause`).
pub const SOFT_IRQ_NUM: usize = U_SOFT;

macro_rules! with_cause {
    ($cause: expr, @TIMER => $timer_op: expr, @EXT => $ext_op: expr, @SOFT => $soft_op: expr $(,)?) => {
        match $cause {
            U_TIMER => $timer_op,
            U_EXT => $ext_op,
            U_SOFT => $soft_op,
            _ => panic!("invalid trap cause: {:#x}", $cause),
        }
    };
}

/// Enables or disables the given IRQ.
pub fn set_enable(ucause: usize, _enabled: bool) {
    if ucause == U_EXT {
        // TODO: set enable in PLIC
    }
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(ucause: usize, handler: Handler) -> bool {
    with_cause!(
        ucause,
        @TIMER => if !TIMER_HANDLER.is_init() {
            TIMER_HANDLER.init_by(handler);
            true
        } else {
            false
        },
        @EXT => register_handler_common(ucause & !INTC_IRQ_BASE, handler),
        @SOFT => if !SOFT_HANDLER.is_init() {
            SOFT_HANDLER.init_by(handler);
            true
        } else {
            false
        },
    )
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(ucause: usize) {
    with_cause!(
        ucause,
        @TIMER => {
            trace!("IRQ: timer");
            TIMER_HANDLER();
        },
        @EXT => dispatch_irq_common(0), // TODO: get IRQ number from PLIC
        @SOFT => {
            trace!("IRQ: soft");
            SOFT_HANDLER();
        },
    );
}

/// Platform-independent IRQ dispatching.
#[allow(dead_code)]
pub(crate) fn dispatch_irq_common(irq_num: usize) {
    trace!("IRQ {}", irq_num);
    if !IRQ_HANDLER_TABLE.handle(irq_num) {
        warn!("Unhandled IRQ {}", irq_num);
    }
}

/// Platform-independent IRQ handler registration.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
#[allow(dead_code)]
pub(crate) fn register_handler_common(irq_num: usize, handler: IrqHandler) -> bool {
    if irq_num < MAX_IRQ_COUNT && IRQ_HANDLER_TABLE.register_handler(irq_num, handler) {
        set_enable(irq_num, true);
        return true;
    }
    warn!("register handler for IRQ {} failed", irq_num);
    false
}

pub fn init_utrap() {
    unsafe {
        utvec::write(__alltraps_u as usize, TrapMode::Direct);
        ustatus::set_uie();
        uie::set_usoft();
    }
}

#[repr(C)]
pub struct UserTrapContext {
    pub x: [usize; 32],
    pub ustatus: Ustatus,
    pub uepc: usize,
    pub utvec: usize,
}

global_asm!(include_str!("trap.S"));

#[no_mangle]
pub fn user_trap_handler(cx: &mut UserTrapContext) -> &mut UserTrapContext {
    let ucause = ucause::read();
    let utval = utval::read();
    match ucause.cause() {
        ucause::Trap::Interrupt(ucause::Interrupt::UserSoft) => unsafe {
            dispatch_irq(ucause.bits());
            uip::clear_usoft();
        },
        _ => {
            println!(
                "Unsupported trap {:?}, utval = {:#x}, uepc = {:#x}!",
                ucause.cause(),
                utval,
                uepc::read()
            );
        }
    }
    cx
}
