#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use crate::windows::hwnd::get_hwnd_for_title;
use windows::{core::w, Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId};
use windows::Win32::Foundation::FALSE;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::LibraryLoader::LoadLibraryW;
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

pub const BASE_ADDRESS: u32 = 0x006a9ec0;

pub unsafe fn exec() -> anyhow::Result<()> {
    let top_windows = get_hwnd_for_title(w!("植物大战僵尸中文版"))?;

    // 获取进程句柄
    let mut p_id = 0;
    GetWindowThreadProcessId(top_windows, Some(&mut p_id));

    // 获取进程 IO 句柄
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, p_id)?;

    let dll_path = w!("scan.dll");

    let remote_mem = VirtualAllocEx(handle, None, dll_path.len() + 1, MEM_COMMIT, PAGE_EXECUTE_READWRITE);

    // 把 dll 路径写入到远程进程中
    WriteProcessMemory(handle, remote_mem, "scan.dll", dll_path.len(), None)?;

    // LoadLibraryW(w!(""));
    Ok(())
}
