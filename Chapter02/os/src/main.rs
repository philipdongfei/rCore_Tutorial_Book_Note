#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("\x1b[31mHello, world!\x1b[0m");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    // get sbss,ebss fun interface 
    // TODO: call sleep fun
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // TODO: sleep 5 secs
    //unsafe {  sleep(5); }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
