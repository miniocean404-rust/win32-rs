#![allow(clippy::missing_safety_doc)]

use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

unsafe fn dialog(){
    MessageBoxW(None, w!("我是消息体"), w!("消息"), MB_OK);
}

// 执行某个程序
// use windows::Win32::System::Threading::WinExec;
// use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW};
// WinExec(s!("regedit.exe"), SW_SHOW.0);