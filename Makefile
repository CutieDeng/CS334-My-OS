# 1.1 构建目标
TARGET := riscv64imac-unknown-none-elf
MODE ?= debug # 询问构建模式，默认debug. 
PROJECT_NAME := kernel
ifeq ($(MODE), release)
	CARGO_BUILD_MODE := --release
else
	CARGO_BUILD_MODE :=
endif
# 1.2 推导出编译的二进制位置，分ELF格式(用于debug)和BIN格式(用于加载到qemu)
KERNEL_ELF := target/$(TARGET)/$(MODE)/$(PROJECT_NAME)     #cargo编译生成
KERNEL_BIN := target/$(TARGET)/$(MODE)/$(PROJECT_NAME).bin #make调用objcopy生成

# 1.3 操作系统的基础执行环境：监督层二进制接口(Supervisor Binary Interface)的定义
SBI ?=qemu-rustsbi # 选用的SBI的名称，应当放置同名bin扩展名文件在bootloader文件夹下。可选参数，默认为qemu下模拟运行 rustsbi 。
BOOTLOADER := ./bootloader/$(SBI).bin # 推导出位置
KERNEL_ENTRY_PA ?= $(KERNEL_ENTRY_PA)         # 内核代码开始的物理地址，随qemu或者硬件设置可能不同。默认为该地址。

# 2. 工具
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# 3.
.PHONY: doc kernel build clean qemu run asm r c cbuild debug

build: $(KERNEL_BIN)

doc: 
	@cargo doc --document-private-items

kernel: 
	@cargo build $(CARGO_BUILD_MODE)

$(KERNEL_BIN): kernel 
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

asm: 
	@$(OBJDUMP) -d $(KERNEL_ELF) | less 

clean: 
	@cargo clean 

qemu: build 
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

run: build qemu 

r: run 

c: 
	@cargo c

cbuild: 
	@make clean 
	@make build

debugserver: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S
ds: debugserver

debug: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S & \
	riscv64-unknown-elf-gdb --symbols=$(KERNEL_ELF) --eval-command='target remote localhost:1234'



