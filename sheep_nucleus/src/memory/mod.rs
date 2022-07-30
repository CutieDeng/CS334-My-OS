mod config; 
mod heap; 
mod address; 
mod frame; 
mod range; 

#[cfg(feature = "cutie-custom-mapping")] 
mod mapping_self; 
#[cfg(feature = "cutie-custom-mapping")] 
use mapping_self as mapping; 

#[cfg(not(feature = "cutie-custom-mapping"))] 
mod mapping_std; 
#[cfg(not(feature = "cutie-custom-mapping"))]
use mapping_std as mapping; 

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

pub type MemoryResult<T> = Result<T, &'static str>;