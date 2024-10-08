use fdt::Fdt;
use riscv::register::{sie, sstatus};

use crate::components::common::{CPU_ID, CPU_NUM, DTB_PTR};
use crate::components::consts::VIRT_ADDR_START;
use crate::components::debug_console::{display_info, println};
use crate::components::instruction::Instruction;
use crate::components::pagetable::{PTEFlags, PTE};
use crate::PageTable;

use super::PageAlignment;

/// TODO: Map the whole memory in the available memory region.
#[link_section = ".data"]
pub(crate) static mut PAGE_TABLE: PageAlignment = {
    let mut arr: [PTE; PageTable::PTE_NUM_IN_PAGE] = [PTE(0); PageTable::PTE_NUM_IN_PAGE];
    // 初始化页表信息
    // 0x00000000_80000000 -> 0x80000000 (1G)
    // 高半核
    // 0xffffffc0_00000000 -> 0x00000000 (1G)
    // 0xffffffc0_80000000 -> 0x80000000 (1G)

    // arr[0] = PTE::from_addr(0x0000_0000, PTEFlags::VRWX);
    // arr[1] = PTE::from_addr(0x4000_0000, PTEFlags::VRWX);
    arr[2] = PTE::from_addr(0x8000_0000, PTEFlags::ADVRWX);
    arr[0x100] = PTE::from_addr(0x0000_0000, PTEFlags::ADGVRWX);
    arr[0x101] = PTE::from_addr(0x4000_0000, PTEFlags::ADGVRWX);
    arr[0x102] = PTE::from_addr(0x8000_0000, PTEFlags::ADGVRWX);
    arr[0x103] = PTE::from_addr(0xc000_0000, PTEFlags::ADGVRWX);
    arr[0x104] = PTE::from_addr(0x1_0000_0000, PTEFlags::ADGVRWX);
    arr[0x105] = PTE::from_addr(0x1_4000_0000, PTEFlags::ADGVRWX);
    arr[0x106] = PTE::from_addr(0x8000_0000, PTEFlags::ADVRWX);
    PageAlignment(arr)
};

/// 汇编入口函数
///
/// 分配栈 初始化页表信息 并调到rust入口函数
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // Chcek boot core
        "
            beqz    a0, 2f
        ",
        // Ensure that boot core is 0
        "1:
            // li      a7, 0x48534D
            // li      a6, 0
            // li      a0, 0
            // mv      a2, a1
            // la      a1, _start
            // ecall
            // li      a7, 0x48534D
            // li      a6, 1   // 0: START, 1: STOP, 2: STATUS
            // li      a0, 0
            // mv      a2, a1
            // la      a1, _start
            // ecall
            // wfi
            // la      ra, 1b
            // ret
        ",
        // 1. 设置栈信息
        // sp = bootstack + (hartid + 1) * 0x10000
        "2:
            la      sp, {boot_stack}
            li      t0, {stack_size}
            add     sp, sp, t0              // set boot stack

            li      s0, {virt_addr_start}   // add virtual address
            or      sp, sp, s0
        ",
        // 2. 开启分页模式
        // satp = (8 << 60) | PPN(page_table)
        "
            la      t0, {page_table}
            srli    t0, t0, 12
            li      t1, 8 << 60
            or      t0, t0, t1
            csrw    satp, t0
            sfence.vma
        ",
        // 3. 跳到 rust_main 函数，绝对路径
        "
            la      a2, {entry}
            or      a2, a2, s0
            jalr    a2                      // call rust_main
        ",
        stack_size = const crate::components::boot::STACK_SIZE,
        boot_stack = sym crate::components::boot::BOOT_STACK,
        page_table = sym PAGE_TABLE,
        entry = sym rust_main,
        virt_addr_start = const VIRT_ADDR_START,
        options(noreturn),
    )
}

/// 汇编函数入口
///
/// 初始化也表信息 并调到 rust_secondary_main 入口函数
#[naked]
#[no_mangle]
pub(crate) unsafe extern "C" fn secondary_start() -> ! {
    core::arch::asm!(
        // 1. 设置栈信息
        // sp = bootstack + (hartid + 1) * 0x10000
        "
            mv      s6, a0
            mv      sp, a1

            li      s0, {virt_addr_start}   // add virtual address
            or      sp, sp, s0
        ",
        // 2. 开启分页模式
        // satp = (8 << 60) | PPN(page_table)
        "
            la      t0, {page_table}
            srli    t0, t0, 12
            li      t1, 8 << 60
            or      t0, t0, t1
            csrw    satp, t0
            sfence.vma
        ", 
        // 3. 跳到 secondary_entry
        "
            la      a2, {entry}
            or      a2, a2, s0
            mv      a0, s6
            jalr    a2                      // call rust_main
        ",
        page_table = sym PAGE_TABLE,
        entry = sym rust_secondary_main,
        virt_addr_start = const VIRT_ADDR_START,
        options(noreturn)
    );
}

pub(crate) fn rust_main(hartid: usize, device_tree: usize) {
    crate::clear_bss();
    #[cfg(feature = "logger")]
    crate::components::debug_console::DebugConsole::log_init();
    // Init allocator
    crate::components::percpu::set_local_thread_pointer(hartid);
    CPU_ID.write_current(hartid);
    #[cfg(feature = "trap")]
    crate::components::trap::init();

    unsafe {
        // Enable SUM for access user memory directly.
        // TODO: Call set_sum() for riscv version up than 1.0, Close when below 1.0
        sstatus::set_sum();
        // Open float point support.
        sstatus::set_fs(sstatus::FS::Dirty);
        sie::set_sext();
        sie::set_ssoft();
    }

    CPU_NUM.init_by(match unsafe { Fdt::from_ptr(device_tree as *const u8) } {
        Ok(fdt) => fdt.cpus().count(),
        Err(_) => 1,
    });

    DTB_PTR.init_by(device_tree);

    display_info!();
    println!(include_str!("../../banner.txt"));
    display_info!("Platform Name", "riscv64");
    if let Ok(fdt) = unsafe { Fdt::from_ptr(device_tree as *const u8) } {
        display_info!("Platform HART Count", "{}", fdt.cpus().count());
        fdt.memory().regions().for_each(|x| {
            display_info!(
                "Platform Memory Region",
                "{:#p} - {:#018x}",
                x.starting_address,
                x.starting_address as usize + x.size.unwrap()
            );
        });
        // TODO: Map available memory regions in the current region.
    }
    display_info!("Platform Virt Mem Offset", "{:#x}", VIRT_ADDR_START);
    display_info!();
    display_info!("Boot HART ID", "{}", hartid);
    display_info!();

    unsafe { crate::components::boot::_main_for_arch(hartid) };
    Instruction::shutdown();
}

pub(crate) extern "C" fn rust_secondary_main(hartid: usize) {
    crate::components::percpu::set_local_thread_pointer(hartid);
    CPU_ID.write_current(hartid);

    // Initialize the trap.
    #[cfg(feature = "trap")]
    crate::components::trap::init();

    // TODO: Get the hart_id and device_tree for the specified device.
    // let (hartid, _device_tree) = boards::init_device(hartid, 0);

    unsafe {
        // Enable SUM for access user memory directly.
        // TODO: Call set_sum() for riscv version up than 1.0, Close when below 1.0
        sstatus::set_sum();
        // Open float point support.        
        sstatus::set_fs(sstatus::FS::Dirty);
        sie::set_sext();
        sie::set_ssoft();
    }

    info!("secondary hart {} started", hartid);
    unsafe { crate::components::boot::_main_for_arch(hartid) };
    Instruction::shutdown();
}

pub fn boot_page_table() -> PageTable {
    PageTable(crate::addr::PhysAddr(unsafe {
        PAGE_TABLE.0.as_ptr() as usize & !VIRT_ADDR_START
    }))
}
