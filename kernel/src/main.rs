#![no_std]
#![no_main]

use core::arch::global_asm;

use kernel::{print, println, sbi}; 

global_asm!(
    core::include_str!("entry.asm")
); 

#[no_mangle]
pub fn rust_main() -> ! {
    // println!("Start the game. "); 
    sbi::shutdown(); 
}