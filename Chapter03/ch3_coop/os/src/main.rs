
//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;


#[cfg(feature = "board_qemu")]
#[path = "boards/qemu.rs"]
mod board;


#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
pub mod trap;


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
pub fn rust_main() -> ! {
    clear_bss();
    println!("\x1b[31m[Kernel] Hello World!\x1b[0m");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
