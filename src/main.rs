#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustic_vault::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustic_vault::memory;
use rustic_vault::println;
use x86_64::structures::paging::{PageTable, Translate};
use x86_64::VirtAddr;

entry_point!(kernel_main);

// being called from boot loader
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello Sailor{}", "!");

    rustic_vault::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // init a mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rustic_vault::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustic_vault::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustic_vault::test_panic_handler(info)
}
