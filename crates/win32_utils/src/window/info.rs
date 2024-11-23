use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::{CloseHandle, HWND};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetWindowInfo, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    WINDOWINFO,
};

// 通过句柄获取窗口进程 ID
pub unsafe fn get_window_pid(hwnd: HWND) -> u32 {
    let mut p_id = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut p_id));
    p_id
}

// 获取窗口位置信息、可见性
pub unsafe fn get_window_info(hwnd: HWND) -> anyhow::Result<WINDOWINFO> {
    //     // IsWindowVisible 也可以用来判断窗口是否可见
    //     if info.dwStyle.contains(WS_VISIBLE) {
    //         println!("{:?}", info);
    //     }

    let mut info = WINDOWINFO {
        cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
        ..Default::default()
    };
    GetWindowInfo(hwnd, &mut info)?;

    anyhow::Ok(info)
}

pub unsafe fn get_window_title_classname(hwnd: HWND) -> String {
    let mut classname = [0u16; 512];
    let len = GetClassNameW(hwnd, &mut classname);

    OsString::from_wide(&classname[..len as usize])
        .to_string_lossy()
        .to_string()
}

// 获取窗口标题
pub unsafe fn get_window_title(hwnd: HWND) -> String {
    // 声明一个 u16 数组，存储 256 个字符
    let mut title = [0u16; 512];
    let len = GetWindowTextW(hwnd, &mut title);
    String::from_utf16_lossy(&title[..len as usize])

    // GetWindowTextW(hwnd, &mut title);
    // let title = OsString::from_wide(&title[..]);
    // println!("子窗口标题: {}", title.to_str().unwrap_or(""));
}

// 获取窗口标题长度
#[allow(dead_code)]
pub unsafe fn get_window_title_len(hwnd: HWND) -> i32 {
    GetWindowTextLengthW(hwnd)
}

// 根据句柄获取窗口的可执行文件路径
pub unsafe fn get_window_exec_path(hwnd: HWND) -> anyhow::Result<String> {
    let mut process_id = 0;

    GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    )?;

    // if process_handle.0 == 0 {
    //     return Err(anyhow::anyhow!("打开进程失败"));
    // }

    let mut buffer: [u16; 1024] = [0; 1024];
    let len = GetModuleFileNameExW(process_handle, None, &mut buffer);

    // if len == 0 {
    //     return Err(anyhow::anyhow!("无法获取模块文件名"));
    // }

    let path = String::from_utf16_lossy(&buffer[..len as usize]);

    CloseHandle(process_handle)?;

    anyhow::Ok(path)
}
