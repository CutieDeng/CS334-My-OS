#![no_std]
#![feature(panic_info_message, never_type, alloc_error_handler)]

mod sbi; 
pub mod console; 
mod panic; 

pub use sbi::shutdown; 
pub use sbi::console_putchar; 

pub use memory::FrameTracker; 

pub mod interrupt;
pub mod memory; 

pub use debug::*; 

#[deprecated]
pub mod debug {

    #[deprecated]
    pub fn output_val(mut a: usize) {
        while a != 0 {
            super::console_putchar('0' as usize + a % 10); 
            a /= 10; 
        } 
        super::console_putchar(b'\n' as usize); 
    }

    #[deprecated]
    pub fn output_val_0x(mut a: usize) {
        let mut cached = [0usize; 16]; 
        let mut pointer = 0usize; 
        while a != 0 {
            let k = a % 16; 
            cached[pointer] = k; 
            pointer += 1; 
            a /= 16; 
        } 
        if pointer == 0 {
            super::console_putchar('0' as usize); 
        } else {
            while pointer > 0 {
                pointer -= 1; 
                if cached[pointer] >= 10 {
                    super::console_putchar('A' as usize + cached[pointer] - 10); 
                } else {
                    super::console_putchar('0' as usize + cached[pointer]); 
                }
            }
        }
        super::console_putchar(b'\n' as usize); 
    }

    #[deprecated] 
    pub fn output_pte(t: usize) {
        assert_eq!( t % crate::memory::PAGE_SIZE, 0 ); 
        let t = t as *const u64; 
        for o in 0..super::memory::PAGE_SIZE / 8 {
            let val = unsafe { *t.offset(o.try_into().unwrap()) }; 
            if val & 0x1 == 0 {
                // Invalid page, just skip it 
            } else {
                let ppn = ( val << 10 ) >> 20; 
                let can_executable = (val & 0x8) != 0; 
                let can_read = (val & 0x2) != 0; 
                let can_write = (val & 0x4) != 0; 
                let is_dirty = (val & 0x80) != 0; 
                let is_accessible = (val & 0x40) != 0; 
                let is_global = (val & 0x20) != 0; 
                let user_mode = (val & 0x10) != 0; 
                crate::println!("页表项 0x{:x}：PhysicalPageNumber: 0x{:x}, executable: {}, readable: {}, writable: {}, globally: {}, \
                    accessible: {}, dirty?: {}, user-fetch: {}", 
                    o, ppn, can_executable, can_read, can_write, is_global, is_accessible, is_dirty, user_mode); 
            }
        }
    }

    #[deprecated]
    pub fn output_val_with_hint(hint: &str, t: usize) {
        for ele in hint.as_bytes() {
            super::console_putchar(*ele as usize); 
        }
        output_val_0x(t); 
    }

}