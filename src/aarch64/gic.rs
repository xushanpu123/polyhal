use aarch64_cpu::registers::{Readable, DAIF};
use arm_gic::gic_v2::{GicCpuInterface, GicDistributor};
use arm_gic::{translate_irq, InterruptType};
use irq_safety::MutexIrqSafe;
use tock_registers::interfaces::ReadWriteable;

use crate::addr::PhysAddr;
use crate::irq::{IRQVector, IRQ};

/// The maximum number of IRQs.
#[allow(dead_code)]
pub const MAX_IRQ_COUNT: usize = 1024;

/// The timer IRQ number.
pub const TIMER_IRQ_NUM: usize = translate_irq(14, InterruptType::PPI).unwrap();

/// The UART IRQ number.
#[allow(dead_code)]
pub const UART_IRQ_NUM: usize = translate_irq(1, InterruptType::SPI).unwrap();

const GICD_BASE: PhysAddr = PhysAddr::new(0x0800_0000);
const GICC_BASE: PhysAddr = PhysAddr::new(0x0801_0000);

static GICD: MutexIrqSafe<GicDistributor> =
    MutexIrqSafe::new(GicDistributor::new(GICD_BASE.get_mut_ptr()));

// per-CPU, no lock
static GICC: GicCpuInterface = GicCpuInterface::new(GICC_BASE.get_mut_ptr());

/// Initializes GICD, GICC on the primary CPU.
pub(crate) fn init() {
    info!("Initialize GICv2...");
    GICD.lock().init();
    GICC.init();
}

/// Implmente the irq vector methods
impl IRQVector {
    /// Get the irq number in this vector
    #[inline]
    pub const fn irq_num(&self) -> usize {
        self.0 & 0x3ff
    }

    /// Acknowledge the irq
    pub fn ack(&self) {
        GICC.eoi(self.0 as u32);
    }
}

/// Get the irq Vector that was
#[inline]
pub fn get_irq() -> IRQVector {
    IRQVector(GICC.iar() as _)
}

/// Implement IRQ operations for the IRQ interface.
impl IRQ {
    /// Enable irq for the given IRQ number.
    #[inline]
    pub fn irq_enable(irq_num: usize) {
        GICD.lock().set_enable(irq_num, true);
        // GICD.lock().configure_interrupt(irq_num, arm_gic::TriggerMode::Level);
    }

    /// Disable irq for the given IRQ number.
    #[inline]
    pub fn irq_disable(irq_num: usize) {
        GICD.lock().set_enable(irq_num, false);
    }

    /// Enable interrupt.
    #[inline]
    pub fn int_enable() {
        unsafe { core::arch::asm!("msr daifclr, #2") };
        // DAIF.modify(DAIF::I::Unmasked);
    }

    /// Disable interrupt.
    #[inline]
    pub fn int_disable() {
        unsafe { core::arch::asm!("msr daifset, #2") };
        // DAIF.modify(DAIF::I::Masked);
    }

    /// Check if the interrupt was enabled.
    #[inline]
    pub fn int_enabled() -> bool {
        DAIF.read(DAIF::I) == 0
    }
}
