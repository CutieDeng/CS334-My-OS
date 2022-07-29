# 基于 RISC-V 架构与Rust语言的16KiB页面操作系统

# ——操作系统 Project 报告



## 项目简介

### 项目协作

12011404 叶璨铭。

12012029 邓值仁。

---

### 为什么选择用Rust语言做Project？

本次 Project 我们没有使用Lab中的C语言代码，而是根据rCore的指导重新构建出一个操作系统。选择Rust有以下几个好处：

1. 生产力。

   1. 可拔插的语言特性。

      比起C语言和C++写操作系统没有任何标准库，Rust语言在设计时便考虑了裸机环境、嵌入式设备下没有标准库的问题。使得我们在编写操作系统的时候，仍然可以使用core核心库。

      此外，只要在具体平台上编写相应的语言特性，便可以逐步恢复标准库的功能。

   2. 优质的包管理器。

      比如，

      相比用C语言做这次Project

   3. 出色的文档

      比如，

      相比用C语言做这次Project

   4. 清晰的错误提示

      比如，

      相比用C语言做这次Project

2. 可靠性。Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。[^7]

   举个例子，

3. 高性能。

   Rust 和C++语言一样，提供了“零开销”的抽象。我们用Rust写操作系统，不仅不会比C语言慢，还可能因为更加清晰的类型系统和所有权机制使得导致变慢的操作在编译时就消除，从而获得更好的内存利用率。

---



## 应用程序与基本执行环境

### 16KiB下的QEMU

### Rust 执行环境



## 16KiB下内存布局与地址空间的设计

### 

### 实验：cargo bench 测试16KiB页面的性能



## 实现多道程序、分时多任务与进程。





# 参考文献

[^1]: R. H. Arpaci-Dusseau and A. C. Arpaci-Dusseau, *Operating systems: Three easy pieces*, 1.00. Arpaci-Dusseau Books, 2018.
[^2]: “rCore-Tutorial-Book-v3 3.6.0-alpha.1 文档.” http://rcore-os.cn/rCore-Tutorial-Book-v3/index.html (accessed Jul. 27, 2022).
[^3]: “rCore-Tutorial V3 实验指导书” http://rcore-os.cn/rCore-Tutorial-deploy/ (accessed Jul. 21, 2022).
[^4]: “Rust 语言圣经 - Rust语言圣经(Rust Course).” https://course.rs/about-book.html (accessed Jul. 21, 2022).
[^5]: “uCore-Tutorial-Guide-2022S 0.1 文档.” https://learningos.github.io/uCore-Tutorial-Guide-2022S/index.html (accessed Jul. 27, 2022).
[^6]: "RISCV 中文手册." https://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf (accessed Jul. 21, 2022).
[^7]: Rust 程序设计语言.” https://www.rust-lang.org/zh-CN/ (accessed Jul. 29, 2022).

