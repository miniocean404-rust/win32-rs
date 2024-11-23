#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use std::mem;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE, VIRTUAL_FREE_TYPE};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, WaitForSingleObject, INFINITE, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS};
use windows::{core::w, Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId};
use win32_utils::window::hwnd::get_hwnd_for_title;
use win32_utils::window::info::get_window_pid;

pub const BASE_ADDRESS: u32 = 0x006a9ec0;

pub unsafe fn exec() -> anyhow::Result<()> {
    let top_windows = get_hwnd_for_title(w!("植物大战僵尸中文版"))?;

    // 获取进程句柄
    let pid = get_window_pid(top_windows);

    // // 获取进程 IO 句柄
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, p_id)?;
    let dll_path = w!("scan.dll");
    let dll_size = (dll_path.len() + 1) * mem::size_of::<PCWSTR>();

    // 开辟远程内存
    let remote_mem = VirtualAllocEx(handle, None, dll_size, MEM_COMMIT, PAGE_EXECUTE_READWRITE);

    // 把 dll 路径写入到远程进程中
    WriteProcessMemory(handle, remote_mem, dll_path.into(), dll_size, None)?;

    let h_thread = CreateRemoteThread(handle, None, 0, LPTHREAD_START_ROUTINE, Some(remote_mem), 0, None)?;

    // 等待远程线程退出信号, 当远程线程退出, 就调用VirtualFreeEx释放远程虚拟内存, 并调用CloseHandle关闭获取到的进程句柄与我们创建的远程线程句柄
    WaitForSingleObject(h_thread,INFINITE);
    VirtualFreeEx(handle, remote_mem, dll_size, MEM_COMMIT.into()).expect("释放远程内存失败");
    CloseHandle(h_thread).expect("关闭 h_thread 失败");
    CloseHandle(handle).expect("关闭 handle 失败");

    Ok(())
}
