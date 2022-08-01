# sheep-os 

sheep-os 的核心设计主要来自于 **rCore**, 基本实现了操作系统内核资源管理的相关内容。

它的实现内容主要分为以下几个部分：

- 硬件中断处理
- 内存抽象
- 进程调度

当然，除此之外，其内部还实现了基本的 *panic*, *console*, *sbi* 接口。（ console 相关的接口中，向控制台接受 input 作为一种保留的抽象，当前版本的代码中还尚没有对此细节进行补全。）

该内核完全由 **rust** 语言编写，通过 `core::arch::asm!` macro 进行相关的汇编调用。[^0] 

[^0]: 值得注意的是，由于 rcore_tutorial 的代码使用的是 *rustc-nightly-2020-06-27* 的编译器，其代码中使用的是更早的 `llvm_asm!` 宏语法，所以需要对其涉及到的代码做相关的转换，以保证程序能够正确运行。

下面先讨论一下 rust 语言提供的额外支持。

## Rust with nostd

首先，需要声明的一点是，本次项目由于写的是一个 os-kernel, 我们没有办法链接 rust 的标准库，这也意味着 `std` 模块下的所有内容都无法被获取和调用。

### Core

但幸运的是， rust 语言除了 `std` 模块外，还预留了一个特殊的库：`core`, 以供 nostd 的场景进行便捷地开发和使用。
通过 sh 的命令 `rustup doc --core` 可以打开本地化的 rust 文档以阅读相关的模块支持。

而在本次项目中，核心库里常用的 `Iter` 抽象，`Option`/`Result` 类型在代码中也常常被直接使用。

### Third-party Lib Suport

在本次 os-kernel 中，出于便利起见，我们直接使用了若干的第三方库代码。

在 *Cargo.toml* 文件中可以阅读到相关的依赖声明，包括 riscv 平台的支持库、buddy-system-allocator 的经典实现, 自旋锁实现 spin, 日志 log, 位域处理库 bitflags, bit-field 等。

rust 的包管理工具提供了更好的库管理支持以便于我们能在指定 target, feature 的情形下依旧能够使用所需的内容。
