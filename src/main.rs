
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm_experimental_arch)]

extern crate alloc;

use core::arch::{asm, global_asm};
use linked_list_allocator::LockedHeap;

pub mod isviewer;

global_asm!(include_str!("boot.s"));

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::isviewer::write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[no_mangle]
extern "C" fn rust_entrypoint() -> ! {
    extern "C" {
        static __bss_end: u32;
    }
    
    let heap_start = (unsafe { &__bss_end as *const u32 as u32 } & 0x1FFF_FFFF) | 0xA000_0000; // uncached and unmapped
    let heap_size = (4 * 1024 * 1024) - unsafe { __bss_end }; // Remaining unused RDRAM (increase 4 -> 8, if using Expansion Pak)
    
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size as usize);
    }
    
    main_loop()
}

fn main_loop() -> ! {
    println!("Hello World!");
    
    loop {
        unsafe { asm!("nop"); }
    }
}


#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("panic: {info}");
    
    loop {}
}