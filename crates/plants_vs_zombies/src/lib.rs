pub mod start;

use win32_utils::ui::dialog;
use windows::core::w;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

// #[no_mangle] 告诉 Rust 编译器不要对函数名进行名称修饰（在编译过程中，Rust 编译器会对函数名进行一些处理来避免命名冲突等问题，但是 C/C++ 调用时需要原始的函数名），
// extern "C"表示这个函数使用 C 语言的调用约定，这样 C/C++ 代码就可以按照 C 语言的方式来调用这个函数。
// extern "system"是一种用于指定函数调用约定的语法。它主要用于在与外部系统（特别是与 C 或 C++ 等语言）进行交互时，告诉 Rust 编译器应该按照外部系统的默认调用约定来处理函数调用
#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _reserved: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || {
            pub const BASE_ADDRESS: u32 = 0x006a9ec0;
            unsafe { dialog(w!("halo"), w!("123")) };
        });
    }
    true
}
