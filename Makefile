SHELL :=/bin/bash
c:
	@cargo c

# 0. 是否对make的流程打日志
MAKE_SAY ?= true
echo := echo
ifneq ($(MAKE_SAY), true)
	echo:= ":"
endif
# 1.1 构建目标
TARGET := riscv64imac-unknown-none-elf
# 询问构建模式，默认为这里的值。dev, release, test, bench
MODE ?= dev
CARGO_BUILD_MODE := --profile=$(MODE)
ifeq ($(MODE), bench)
	RUSTC_MODE := release
else
	ifeq ($(MODE), release)
		RUSTC_MODE := release
	endif
	RUSTC_MODE := debug
endif
#PROJECT_NAME := kernel
PROJECT_NAME := sheep_nucleus

# 1.2 推导出编译的二进制位置，分ELF格式(用于debug)和BIN格式(用于加载到qemu)
#cargo编译生成
KERNEL_ELF := target/$(TARGET)/$(RUSTC_MODE)/$(PROJECT_NAME)
#make调用objcopy生成
KERNEL_BIN := target/$(TARGET)/$(RUSTC_MODE)/$(PROJECT_NAME).bin
# 1.3.1 选择qemu版本
QEMU_VERSION ?=7.0-4Ki
QEMU_BUILD := $(shell pwd)/qemu-bin/qemu-$(QEMU_VERSION)
set_qemu: $(QEMU_BUILD)
	@echo "正在设置Qemu为指定版本($(QEMU_VERSION))。"
	@source ./set_qemu.bash
	qemu-as-$(QEMU_VERSION)
	@echo "设置完成。"
# 1.3.2 操作系统的基础执行环境：监督层二进制接口(Supervisor Binary Interface)的定义
# 选用的SBI的名称，应当放置同名bin扩展名文件在bootloader文件夹下。可选参数，默认为qemu下模拟运行 rustsbi 。
SBI ?=rustsbi-qemu
# 推导出位置
# BOOTLOADER ?= ./bootloader/$(SBI).bin
BOOTLOADER ?= default
# 内核代码开始的物理地址，随qemu或者硬件设置可能不同。默认为该地址。
KERNEL_ENTRY_PA ?= 0x80200000

# 2. 工具
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# 3.
.PHONY: doc kernel build clean qemu run asm r c cbuild debug check

build: $(KERNEL_BIN)

doc: 
	@cargo doc --document-private-items

kernel: 
	@cargo build $(CARGO_BUILD_MODE)

$(KERNEL_BIN): kernel
	@$(echo) "内核构建成功。正在从elf格式导出bin格式。"
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

asm: 
	@$(OBJDUMP) -d $(KERNEL_ELF) | less 

clean: 
	@cargo clean
	@$(echo) "已经清理cargo项目。"

qemu: build
	@$(echo) "正在启动qemu模拟器。"
	@qemu-system-riscv64 \
		-machine virt \
		-bios $(BOOTLOADER) \
		-nographic \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

run: build qemu 

r: run 

cbuild: 
	@make clean 
	@make build

debugserver: build
	@$(echo) "启动调试器。"
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S
ds: debugserver

debug: build
	@$(echo) "启动调试器。"
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S & \
	riscv64-unknown-elf-gdb --symbols=$(KERNEL_ELF) --eval-command='target remote localhost:1234'

check: 
	@cargo check