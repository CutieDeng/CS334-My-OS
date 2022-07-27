mod config; 
mod heap; 
mod address; 
mod frame; 
mod range; 

pub use config::*; 
pub use address::*; 
pub use range::Range; 

pub use frame::*; 

pub fn init() {
    heap::init(); 
    use crate::println; 
    println!("[[mod]] memory has been initialized. "); 
    // println!("The sheep_nucleus end address is {}. ", *KERNEL_END_ADDRESS);
}
