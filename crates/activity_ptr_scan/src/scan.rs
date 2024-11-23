#![allow(clippy::missing_safety_doc)]

use windows::core::w;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use win32_utils::window::hwnd::get_hwnd_for_title;

pub unsafe fn run() -> anyhow::Result<()> {
    let _ = get_hwnd_for_title(w!("微信"))?;

    let handle = GetModuleHandleW(w!("WeChat.exe"));

    match handle {
        Ok(handle) => {
            let module_base_addr = handle.0 as u32;
            println!("模块基址: {:#X}", module_base_addr);
        }
        Err(_) => {
            // 根据 WIN32_ERROR 的值，在VS的 工具 -> 错误查找 ，输入你的错误号查找对应的原因。
            let error = GetLastError();
            let _ = dbg!(error);
        }
    }

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
