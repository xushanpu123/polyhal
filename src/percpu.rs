/// !TIPS: x86_64 will
use core::{alloc::Layout, mem::size_of, ptr::copy_nonoverlapping};

use alloc::alloc::alloc;

use crate::PAGE_SIZE;

static BOOT_PERCPU_DATA_AREA: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

/// This is a empty seat for percpu section.
/// Force the linker to create the percpu section.
#[link_section = "percpu"]
#[used(linker)]
static _PERCPU_SEAT: [usize; 0] = [0; 0];

#[cfg(target_arch = "x86_64")]
const PERCPU_RESERVED: usize = size_of::<crate::currrent_arch::PerCPUReserved>();
#[cfg(not(target_arch = "x86_64"))]
const PERCPU_RESERVED: usize = 0;

/// Returns the base address of the per-CPU data area on the given CPU.
///
/// if `cpu_id` is 0, it returns the base address of all per-CPU data areas.
pub fn percpu_area_init(cpu_id: usize) -> usize {
    // Get initial per-CPU data area
    extern "Rust" {
        fn __start_percpu();
        fn __stop_percpu();
    }
    let start = __start_percpu as usize;
    let size = __stop_percpu as usize - start + PERCPU_RESERVED;

    // Get the base address of the per-CPU data area
    // If cpu_id is boot,core then use BOOT_PERCPU_DATA_AREA.
    // else alloc area.
    let dst = if cpu_id == 0 {
        BOOT_PERCPU_DATA_AREA.as_ptr() as usize as *mut u8
    } else {
        let layout =
            Layout::from_size_align(size, size_of::<usize>()).expect("percpu area align failed");
        unsafe { alloc(layout) }
    };

    // Init the area with original data.
    unsafe {
        copy_nonoverlapping(start as *const u8, dst.add(PERCPU_RESERVED), size);
    }

    dst as usize
}

/// Read the architecture-specific thread pointer register on the current CPU.
pub fn get_local_thread_pointer() -> usize {
    let tp;
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                tp = x86::msr::rdmsr(x86::msr::IA32_GS_BASE) as usize
            } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
                core::arch::asm!("mv {}, gp", out(reg) tp)
            } else if #[cfg(target_arch = "aarch64")] {
                core::arch::asm!("mrs {}, TPIDR_EL1", out(reg) tp)
            } else if #[cfg(target_arch = "loongarch64")] {
                core::arch::asm!("move {}, $r21", out(reg) tp)
            }
        }
    }
    tp
}

/// Set the architecture-specific thread pointer register to the per-CPU data
/// area base on the current CPU.
///
/// `cpu_id` indicates which per-CPU data area to use.
pub fn set_local_thread_pointer(cpu_id: usize) {
    let tp = percpu_area_init(cpu_id);
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                x86::msr::wrmsr(x86::msr::IA32_GS_BASE, tp as u64);
                // Write cpu_local pointer to the first usize of the per-CPU data area
                // Write the valid address to the second usize of the per-CPU data area
                let percpu_reserved = crate::currrent_arch::PerCPUReserved::mut_from_ptr(tp as _);
                percpu_reserved.self_ptr = tp;
                percpu_reserved.valid_ptr = tp + PERCPU_RESERVED;
            } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
                core::arch::asm!("mv gp, {}", in(reg) tp)
            } else if #[cfg(target_arch = "aarch64")] {
                core::arch::asm!("msr TPIDR_EL1, {}", in(reg) tp)
            } else if #[cfg(target_arch = "loongarch64")] {
                core::arch::asm!("move $r21, {}", in(reg) tp)
            }
        }
    }
}
