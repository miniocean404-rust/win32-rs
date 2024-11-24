#![allow(clippy::missing_safety_doc)]

use std::ptr;

// 以指针的值为基础构建新指针
pub fn build_ptr(base: usize, offset: usize) -> *const usize {
    (base + offset) as *const usize
}

// 可以直接获取内存值
pub unsafe fn read_memory<T>(base_ptr: *const usize, offset: usize) -> T
where
    T: Copy,
{
    let base_ptr_deref = *base_ptr;
    let data_ptr = (base_ptr_deref + offset) as *const T;
    *data_ptr
}

pub unsafe fn write_memory(base_ptr: *const usize, value: usize) {
    ptr::write(base_ptr as *mut usize, value);
}
