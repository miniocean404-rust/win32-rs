use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::WS_VISIBLE;

use crate::unknown::get_window_program_path;
use crate::window::info::{get_window_info, get_window_title, get_window_title_classname};

// HWND 是一种数据类型，表示窗口句柄（Handle to a Window）。
// EnumWindows(Some(enum_windows), LPARAM(0)).unwrap(); // EnumWindows 是一个 Windows API 函数，用于枚举所有顶级窗口。
#[allow(dead_code)]
pub unsafe extern "system" fn enum_windows(window: HWND, _: LPARAM) -> BOOL {
    let info = get_window_info(window).unwrap();
    let title = get_window_title(window);
    let path = get_window_program_path(window);

    if !title.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
        println!("窗口名称：{:?} ", title);
        println!("窗口程序路径：{:?} ", path);
        println!(" ")
        // println!(
        //     "位置：({}, {}) {:?}",
        //     info.rcWindow.left, info.rcWindow.top, info
        // );
    }
    true.into()
}

// 获取子窗口句柄
// EnumChildWindows(explore_window, Some(enum_child_proc), LPARAM(0));
#[allow(dead_code)]
pub unsafe extern "system" fn enum_child_proc(window: HWND, _: LPARAM) -> BOOL {
    let classname = get_window_title_classname(window);

    if classname == "ShellTabWindowClass" {
        let title = get_window_title(window);
        // 在这里处理每个子窗口的类名和标题
        println!("子窗口类名: {}", classname);
        println!("子窗口标题: {}", title);
        println!(" ",);
    }

    true.into()
}
