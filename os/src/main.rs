#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod lang_items;
mod console;

#[allow(deprecated)]
mod sbi;

use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
extern "C" fn rust_main() {
    clear_bss();
    println!("hello world");
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
