use crate::memory::config::KERNEL_END_ADDRESS;

mod config; 
mod heap; 
mod address; 
pub mod frame; 
mod range; 

pub use config::*; 
pub use address::*; 
pub use range::Range; 

pub fn init() {
    heap::init(); 
    use crate::println; 
    println!("[[mod]] memory has been initialized. "); 
    println!("The kernel end address is {}. ", *KERNEL_END_ADDRESS); 
}
