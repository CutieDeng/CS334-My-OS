//! 内存处理模块

pub mod config; 
pub use config::*; 

pub mod heap; 

pub mod address; 
pub use address::*; 

mod frame; 

/// 用于描述一个区间
/// 
/// 往往和 [`address`] 中的基本区间一同使用，描述一段连续的物理内存地址、物理页、虚拟内存地址、虚拟内存页等..
pub mod range; 
pub use range::Range; 

#[cfg(feature = "cutie-custom-mapping")] 
mod mapping_self; 
#[cfg(feature = "cutie-custom-mapping")] 
use mapping_self as mapping; 

#[cfg(not(feature = "cutie-custom-mapping"))] 
mod mapping_std; 
#[cfg(not(feature = "cutie-custom-mapping"))]
use mapping_std as mapping; 


pub use frame::*; 

pub fn init() {
    heap::init(); 
    println!("[[mod]] memory has been initialized. "); 
    println!("The sheep_nucleus end address is {}. ", *KERNEL_END_ADDRESS);
}

pub type MemoryResult<T> = Result<T, &'static str>;