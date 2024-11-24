#![allow(clippy::missing_safety_doc)]

// 构建指针
pub fn build_ptr(base: usize, offset: usize) -> *const usize {
    (base + offset) as *const usize
}

pub unsafe fn read_memory<T>(base_ptr: *const usize, offset: usize) -> T
where
    T: Copy,
{
    let base_ptr_deref = *base_ptr;
    let data_ptr = (base_ptr_deref + offset) as *const T;
    *data_ptr
}
