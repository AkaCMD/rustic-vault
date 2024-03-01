#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustic_vault::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustic_vault::memory::{self, BootInfoFrameAllocator};
use rustic_vault::println;
use x86_64::structures::paging::{Page, PageTable, Translate};
use x86_64::VirtAddr;

entry_point!(kernel_main);

// being called from boot loader
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello Sailor{}", "!");

    rustic_vault::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

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
