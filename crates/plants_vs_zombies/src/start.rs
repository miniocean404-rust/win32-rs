#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use std::ffi::c_void;
use std::mem;
use win32_utils::window::hwnd::get_hwnd_for_title;
use win32_utils::window::info::get_window_pid;
use windows::core::PCWSTR;
use windows::core::{s, w};
use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::Memory::{
    VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_DECOMMIT, PAGE_EXECUTE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, OpenProcess, WaitForSingleObject, INFINITE, LPTHREAD_START_ROUTINE,
    PROCESS_ALL_ACCESS,
};

pub unsafe fn inject() -> anyhow::Result<()> {
    let hwnd = get_hwnd_for_title(w!("植物大战僵尸中文版"))?;

    // 获取进程句柄
    let pid = get_window_pid(hwnd);

    // 获取进程 IO 句柄
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)?;

    // 相对于要寻找的可执行文件 .exe 程序的路径，同目录下就是根目录
    let dll_path = w!("game_dll.dll");
    let dll_size = (dll_path.len() + 1) * size_of::<PCWSTR>();

    // 开辟远程内存
    let alloc_mem = VirtualAllocEx(handle, None, dll_size, MEM_COMMIT, PAGE_EXECUTE_READWRITE);

    // 把 dll 路径写入到远程进程中
    WriteProcessMemory(
        handle,
        alloc_mem,
        dll_path.as_ptr() as *const c_void,
        dll_size,
        None,
    )?;

    // 让程序通过 LoadLibraryW 加载 dll
    let h_module = LoadLibraryW(w!("kernel32.dll"))?;
    let fn_address = GetProcAddress(h_module, s!("LoadLibraryW"));
    #[allow(clippy::missing_transmute_annotations)]
    let lp_start_address: LPTHREAD_START_ROUTINE = Some(mem::transmute(fn_address));
    let h_thread = CreateRemoteThread(handle, None, 0, lp_start_address, Some(alloc_mem), 0, None)?;

    // 等待远程线程退出信号, 当远程线程退出, 就调用 VirtualFreeEx 释放远程虚拟内存, 并调用 CloseHandle 关闭获取到的进程句柄与我们创建的远程线程句柄
    WaitForSingleObject(h_thread, INFINITE);
    VirtualFreeEx(handle, alloc_mem, dll_size, MEM_DECOMMIT)?;
    CloseHandle(h_thread)?;
    CloseHandle(handle)?;

    Ok(())
}

pub unsafe fn read() -> anyhow::Result<()> {
    let hwnd = get_hwnd_for_title(w!("植物大战僵尸中文版"))?;

    // 获取进程句柄
    let pid = get_window_pid(hwnd);

    // 获取进程 IO 句柄
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)?;

    let base = 0x006a9ec0;
    let mut ptr: *mut c_void = std::ptr::null_mut();
    ptr = mem::transmute(base as usize);

    let content_ptr: *mut c_void = std::ptr::null_mut();

    ReadProcessMemory(handle, ptr, content_ptr, 4096 * 10, None)?;

    dbg!(content_ptr);

    Ok(())
}
