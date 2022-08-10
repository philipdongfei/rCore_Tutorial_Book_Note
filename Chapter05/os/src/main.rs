
//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;


#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
pub mod mm;
mod sbi;
pub mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;


use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    // get sbss,ebss fun interface 
    // TODO: call sleep fun
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8, 
            ebss as usize - sbss as usize).fill(0);
    }
}

#[no_mangle]
/// the rust entry-point of os
pub fn rust_main() -> ! {
    clear_bss();
    println!("\x1b[31m[Kernel] Hello, World!\x1b[0m");
    /////
    mm::init();
    //mm::heap_allocator::heap_test();
    mm::remap_test();
    task::add_initproc();
    println!("after initproc!");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}
