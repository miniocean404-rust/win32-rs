use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

// 获取前台应用窗口句柄
pub unsafe fn get_foreground_window() -> HWND {
    GetForegroundWindow()
}

// 获取窗口标题
pub unsafe fn get_window_title(window: HWND) -> String {
    // 声明一个 u16 数组，存储 256 个字符
    let mut title = [0u16; 512];
    let len = GetWindowTextW(window, &mut title);
    String::from_utf16_lossy(&title[..len as usize])

    // GetWindowTextW(hwnd, &mut title);
    // let title = OsString::from_wide(&title[..]);
    // println!("子窗口标题: {}", title.to_str().unwrap_or(""));
}
