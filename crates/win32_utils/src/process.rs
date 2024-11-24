// 通过 win32 api 读取进程内存
use std::{ffi::c_void, mem};

use windows::{
    core::{s, w, PCWSTR},
    Win32::{
        Foundation::{CloseHandle, FALSE, HANDLE},
        System::{
            Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            Memory::{
                VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_DECOMMIT, PAGE_EXECUTE_READWRITE,
            },
            Threading::{
                CreateRemoteThread, OpenProcess, WaitForSingleObject, INFINITE,
                LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS,
            },
        },
    },
};

use crate::window::{hwnd::get_hwnd_for_title, info::get_window_pid};

pub unsafe fn open_process_for_title(title: PCWSTR) -> anyhow::Result<HANDLE> {
    let hwnd = get_hwnd_for_title(title)?;
    let pid = get_window_pid(hwnd);
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)?;

    Ok(handle)
}

pub unsafe fn read_process_address_mem(
    handle: HANDLE,
    address: usize,
    length: usize,
) -> anyhow::Result<Vec<u8>> {
    let size: *mut usize = &mut 0;

    let mut buffer = vec![0u8; length];
    ReadProcessMemory(
        handle,
        address as *const c_void,
        buffer.as_mut_ptr() as *mut c_void,
        buffer.len(),
        Some(size),
    )?;

    Ok(buffer)
}

// 通过窗口标题注入 dll
pub unsafe fn inject_dll(title: PCWSTR, dll_path: PCWSTR) -> anyhow::Result<()> {
    let hwnd = get_hwnd_for_title(title)?;

    // 获取进程句柄
    let pid = get_window_pid(hwnd);

    // 获取进程 IO 句柄
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)?;

    // 相对于要寻找的可执行文件 .exe 程序的路径，同目录下就是根目录
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
