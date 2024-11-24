use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_YESNOCANCEL};

pub unsafe fn dialog(title: PCWSTR, body: PCWSTR) {
    MessageBoxW(None, body, title, MB_YESNOCANCEL);
}
