#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

pub unsafe fn dialog(title: PCWSTR, body: PCWSTR) {
    MessageBoxW(None, body, title, MB_OK);
}
