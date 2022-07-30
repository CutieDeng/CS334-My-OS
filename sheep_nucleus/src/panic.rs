//! panic 库，实现 panic 和 abort 的基本功能。

/// 打印 panic 信息并 [`crate::shutdown`]. 
///
/// ### '#[panic_handler]' 属性
/// 声明此函数是 panic 的回调。
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::eprintln!("panic: {}:{}: '{:?}'", info.location().unwrap().file(), info.location().unwrap().line(), info.message().unwrap());
    crate::sbi::shutdown();
}

/// abort 函数
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}