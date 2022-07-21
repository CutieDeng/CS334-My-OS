#![no_std]
#![no_main]

pub extern "C" fn _start() -> ! {
    loop {} 
}

#[panic_handler] 
fn _h(_: &core::panic::PanicInfo) -> ! {
    loop {} 
}