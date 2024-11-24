pub mod constant;
pub mod utils;

use constant::{BASE_ADDRESS, SUNSHINE_OFFSET_ONE, SUNSHINE_OFFSET_TWO};
use utils::{build_ptr, write_memory};
use win32_utils::string;
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
            unsafe {
                let _ = change_sunshine_num();
            };
        });
    }
    true
}

unsafe fn change_sunshine_num() -> anyhow::Result<()> {
    let base_address_ptr = BASE_ADDRESS as *const usize;
    let sunshine_one_offset = build_ptr(*base_address_ptr, SUNSHINE_OFFSET_ONE);
    let sunshine_two_offset = build_ptr(*sunshine_one_offset, SUNSHINE_OFFSET_TWO);

    write_memory(sunshine_two_offset, 999);

    dialog(
        w!("阳光的值为"),
        string::wide_utf16_string(format!("{:?}", *sunshine_two_offset).as_str()),
    );

    Ok(())
}

unsafe fn base_address() {
    // let handle = get_module_base_address(w!("PlantsVsZombies.exe"))?;
}
