//! 内存地址相关信息描述
//! 
//! 提供了便利的内存地址之间的相互转换信息
//! 特别地，通过 sv47 宏调整 bit 的相关设置信息

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)] 
pub struct PhysicalAddress(pub usize); 

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)] 
pub struct PhysicalPageNumber(pub usize); 

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)] 
pub struct VirtualAddress(pub usize); 

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)] 
pub struct VirtualPageNumber(pub usize); 

impl VirtualPageNumber {
    pub fn levels(self) -> [usize; 3] {
        if cfg!(feature = "sv47") {
            [
                self.0.get_bits(22..33), 
                self.0.get_bits(11..22), 
                self.0.get_bits(..11), 
            ]
        } else {
            [
                self.0.get_bits(18..27), 
                self.0.get_bits(9..18), 
                self.0.get_bits(..9), 
            ]
        }
    }
}

use bit_field::BitField;

use super::config::PAGE_SIZE;

// quote from RCore. 
// 实现各类型之前的快速转换，PhysicalAddress -> PhysicalPageNumber & PhysicalPageNumber -> PhysicalAddress 
macro_rules! implement_address_to_page_number {
    // 这里面的类型转换实现 [`From`] trait，会自动实现相反的 [`Into`] trait
    ($address_type: ty, $page_number_type: ty) => {
        impl From<$page_number_type> for $address_type {
            /// 从页号转换为地址
            fn from(page_number: $page_number_type) -> Self {
                Self(page_number.0 * PAGE_SIZE)
            }
        }
        impl From<$address_type> for $page_number_type {
            /// 从地址转换为页号，直接进行移位操作
            ///
            /// 不允许转换没有对齐的地址，这种情况应当使用 `floor()` 和 `ceil()`
            fn from(address: $address_type) -> Self {
                assert!(address.0 % PAGE_SIZE == 0);
                Self(address.0 / PAGE_SIZE)
            }
        }
        impl $page_number_type {
            /// 将地址转换为页号，向下取整
            pub const fn floor(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE)
            }
            /// 将地址转换为页号，向上取整
            pub const fn ceil(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
            }
        }
    };
}
implement_address_to_page_number! {PhysicalAddress, PhysicalPageNumber}
implement_address_to_page_number! {VirtualAddress, VirtualPageNumber}

/// 为各种仅包含一个 usize 的类型实现运算操作
macro_rules! implement_usize_operations {
    ($type_name: ty) => {
        /// `+`
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }
        /// `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }
        /// `-`
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }
        /// `-`
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }
        /// `-=`
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }
        /// 和 usize 相互转换
        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        /// 和 usize 相互转换
        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
        impl $type_name {
            /// 是否有效（0 为无效）
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }
        /// {} 输出
        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}
implement_usize_operations! {PhysicalAddress}
implement_usize_operations! {PhysicalPageNumber}
implement_usize_operations! {VirtualAddress}
implement_usize_operations! {VirtualPageNumber}

impl PhysicalAddress {
    /// 从物理地址经过线性映射取得 &mut 引用
    pub fn deref_kernel<T>(self) -> &'static mut T {
        println!(""); 
        VirtualAddress::from(self).deref()
    }
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}
impl VirtualPageNumber {
    /// 从虚拟地址取得页面
    pub fn deref<'a> (self) -> &'a mut [u8; PAGE_SIZE] {
        VirtualAddress::from(self).deref()
    }
}
impl PhysicalPageNumber {
    /// 从物理地址经过线性映射取得页面
    pub fn deref_kernel<'a> (self) -> &'a mut [u8; PAGE_SIZE] {
        PhysicalAddress::from(self).deref_kernel()
    }
}

/// 从指针转换为虚拟地址
impl<T> From<*const T> for VirtualAddress {
    fn from(pointer: *const T) -> Self {
        Self(pointer as usize)
    }
}
/// 从指针转换为虚拟地址
impl<T> From<*mut T> for VirtualAddress {
    fn from(pointer: *mut T) -> Self {
        Self(pointer as usize)
    }
}

use super::KERNEL_MAP_OFFSET; 

/// 虚实页号之间的线性映射
impl From<PhysicalPageNumber> for VirtualPageNumber {
    fn from(ppn: PhysicalPageNumber) -> Self {
        Self(ppn.0 + KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}
/// 虚实页号之间的线性映射
impl From<VirtualPageNumber> for PhysicalPageNumber {
    fn from(vpn: VirtualPageNumber) -> Self {
        Self(vpn.0 - KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}
/// 虚实地址之间的线性映射
impl From<PhysicalAddress> for VirtualAddress {
    fn from(pa: PhysicalAddress) -> Self {
        Self(pa.0 + KERNEL_MAP_OFFSET)
    }
}
/// 虚实地址之间的线性映射
impl From<VirtualAddress> for PhysicalAddress {
    fn from(va: VirtualAddress) -> Self {
        Self(va.0 - KERNEL_MAP_OFFSET)
    }
}

impl VirtualAddress {
    /// 从虚拟地址取得某类型的 &mut 引用
    pub fn deref<T>(self) -> &'static mut T {
        unsafe { &mut *(self.0 as *mut T) }
    }
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}