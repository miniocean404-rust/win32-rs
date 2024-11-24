use windows::core::PCWSTR;

pub fn wide_utf16_string(text: &str) -> PCWSTR {
    let class_name = text
        .encode_utf16()
        // 方法在 Rust 中用于将单个元素追加到迭代器的末尾，这种方法通常用于处理需要以空终止符结尾的字符串，特别是在与 C API 或 Windows API 交互时，这些 API 需要字符串以空终止符结尾。这里的 0 作为 UTF-16 字符串的空终止符，表示字符串的结束
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    PCWSTR(class_name.as_ptr())
}
