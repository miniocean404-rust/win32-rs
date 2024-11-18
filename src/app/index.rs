#![allow(clippy::missing_safety_doc)]

use windows::core::{s, w};
use windows::Win32::System::LibraryLoader::{ GetModuleHandleW};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW};

pub unsafe fn run() -> anyhow::Result<()> {
    let top_window = FindWindowW(None, w!("微信"))?;
    let module_base_addr = GetModuleHandleW(w!("clash-win64.exe")).map(|h| h.0 as u32)?;
    println!("模块基址: {:#X}", module_base_addr);

    // s! 带尾空结束符的UTF-8字符串。
    // 检索其类名和窗口名称与指定字符串匹配的顶级窗口的句柄。 此函数不搜索子窗口。 此函数不执行区分大小写的搜索。
    // let top_window = FindWindowA(None, s!("微信"))?;
    // let mut window_info = WINDOWINFO::default();
    // GetWindowInfo(top_window, &mut window_info)?;
    //
    // println!("窗口信息: {:?}", window_info);
    // println!("窗口标题: {:?}", get_window_title(top_window));

    Ok(())
}

