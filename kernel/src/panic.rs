//! panic 库，实现 panic 和 abort 的基本功能。


/// 打印 panic 信息并 [`shutdown`]. 
/// 
/// ### '#[panic_handler]' 属性
/// 声明此函数是 panic 的回调。
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // quote: 
    // `\x1b[??m` 是控制终端字符输出格式的指令，在支持的平台上可以改变文字颜色等等，这里使用红色
    // ref: https://misc.flogisoft.com/bash/tip_colors_and_formatting

    crate::println!("{}panic: {}:{}: '{:?}'{}", "\x1b[1;31m", 
        info.location().unwrap().file(), 
        info.location().unwrap().line(), 
        info.message().unwrap(), "\x1b[0m"); 
    crate::sbi::shutdown(); 
}

#[no_mangle] 
extern "C" fn abort() -> ! {
    panic! ("abort()") 
}