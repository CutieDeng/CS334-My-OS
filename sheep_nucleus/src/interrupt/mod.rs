//! 中断模块

mod handler; 
mod context; 

#[cfg(feature = "time_enabled")]
mod timer; 

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`] 
/// - [`time::init`] 
pub fn init() {
    handler::init(); 
    #[cfg(feature = "time_enabled")]
    timer::init(); 
    crate::println!("[[mod]] interrupt has been initialized"); 
}