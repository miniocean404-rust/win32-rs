use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetWindowModuleFileNameW;

// 执行某个程序
// use windows::Win32::System::Threading::WinExec;
// use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW};
// WinExec(s!("regedit.exe"), SW_SHOW.0);

// 不知道是做什么的：检索与指定窗口句柄关联的模块的完整路径和文件名。
#[allow(dead_code)]
pub unsafe fn get_window_program_path(window: HWND) -> String {
    let mut path: [u16; 512] = [0; 512];
    let path_len = GetWindowModuleFileNameW(window, &mut path);
    String::from_utf16_lossy(&path[..path_len as usize])
}

