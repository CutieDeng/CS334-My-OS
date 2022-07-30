//! 中断模块
//! 
//! # 模块支持
//! 
//! - [`context`] 执行环境上下文信息管理、处理
//! - [`timer`] 负责预约、处理时钟信号
//! - [`handler`] 实现了 interrupt handler, 集中处理各硬件异常

pub mod handler; 
pub mod context; 

#[cfg(not(feature = "time-disabled"))]
pub mod timer; 

/// 初始化中断相关的子模块
/// 
/// - `handler::init` 
/// - `time::init` 
pub fn init() {
    handler::init(); 
    #[cfg(not(feature = "time-disabled"))]
    timer::init(); 
    crate::println!("[[mod]] interrupt has been initialized"); 
}