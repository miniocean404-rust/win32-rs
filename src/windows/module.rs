#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use windows::{
    core::{w, PCWSTR},
    Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleW},
};

// GetModuleHandle 获取一个特定的应用程序或动态链接库的模块句柄，且这个模块必须已经被加载到调用者的进程空间中(就是在 exe 中注入 这个程序的 dll，然后它被执行了)。
// 例如：user32.dll, 或者 WeChat.exe 通过 ProcessHacker.exe 注入了 dll,就可以调用这个方法调用 WeChat.exe 的模块句柄，获取地址。
pub unsafe fn a(id: PCWSTR) -> windows::core::Result<HMODULE> {
    GetModuleHandleW(id)
}
