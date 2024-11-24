pub fn is_64_bit() -> bool {
    cfg!(target_pointer_width = "64")
}

pub fn is_32_bit() -> bool {
    cfg!(target_pointer_width = "32")
}
