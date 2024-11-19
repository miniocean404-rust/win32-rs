#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

pub unsafe fn dialog(title: PCWSTR, body: PCWSTR) {
    MessageBoxW(None, body, title, MB_OK);
}

// 执行某个程序
// use windows::Win32::System::Threading::WinExec;
// use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW};
// WinExec(s!("regedit.exe"), SW_SHOW.0);
