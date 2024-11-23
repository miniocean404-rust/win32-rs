#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowInfo, MessageBoxW, MB_OK, WINDOWINFO};

pub unsafe fn dialog(title: PCWSTR, body: PCWSTR) {
    MessageBoxW(None, body, title, MB_OK);
}

