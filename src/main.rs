
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm_experimental_arch)]

extern crate alloc;

use core::arch::asm;
use core::ptr;
use linked_list_allocator::LockedHeap;

pub mod isviewer;

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
extern "C" fn _entrypoint() -> ! {
    extern "C" {
        static __bss_end: u32;
    }
    let bss_end = (&raw const __bss_end).addr();
    
    const IS_HEAP_CACHED: bool = true; // Change to `false` if you want the heap to bypass the CPU cache
    let heap_start = if IS_HEAP_CACHED {
        (bss_end & 0x1FFF_FFFF) | 0x8000_0000
    } else {
        (bss_end & 0x1FFF_FFFF) | 0xA000_0000
    };
    
    // The libdragon IPL3 stores the total memory size in DMEM
    let total_memory = unsafe { ptr::with_exposed_provenance::<u32>(0xA400_0000).read_volatile() } as usize;
    let heap_size = total_memory - (bss_end & 0x1FFF_FFFF); // Remaining unused RDRAM
    
    unsafe {
        const STACK_PADDING: usize = 128 * 1024;
        
        // Safety:
        // 
        // The allocator requires a pointer to where in memory the heap should start.
        // 
        // Given the above code and the linker script, `heap_start` *should* point to the next
        // available byte in RDRAM. This location *should* be outside any existing heap/stack/static
        // allocation and thus safe to create a pointer to.
        // 
        // However, the heap itself may eventually conflict with the program's stack (which grows
        // backwards from the end of RDRAM). To help avoid this, the size is shrunk by an arbitrary
        // amount. But keep in mind it's still possible for the stack to grow far enough that it
        // overlaps with used heap memory.
        // 
        // If this happens, try increasing the stack padding. 
        ALLOCATOR.lock().init(ptr::with_exposed_provenance_mut(heap_start), heap_size - STACK_PADDING);
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