#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod lang_items;
#[macro_use]
mod console;
mod syscall;
#[allow(warnings)]
mod sbi;
#[macro_use]
mod logging;

use core::panic::PanicInfo;
use log::{info, warn, debug, error, trace};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
extern "C" fn rust_main() {
    clear_bss();
    logging::init();
    error!("hello rcore");
    warn!("hello rcore");
    info!("hello rcore");
    debug!("hello rcore");
    trace!("hello rcore");
    //panic!();
    loop{};
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
