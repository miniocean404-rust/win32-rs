#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use win32_utils::window::hwnd::get_hwnd_for_title;
use win32_utils::window::info::get_window_pid;
use windows::core::w;
use windows::Win32::Foundation::FALSE;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

use crate::utils::read_memory;

// 植物大战僵尸中文版

pub unsafe fn ptr_scan() -> anyhow::Result<()> {
    let hwnd = get_hwnd_for_title(w!("植物大战僵尸中文版"))?;
    let pid = get_window_pid(hwnd);
    let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)?;

    let address = 0x14520FC8;
    let res = read_memory::<u32>(address as *const usize, 0);

    dbg!(res);

    // let one = build_ptr(BASE_ADDRESS, 0x768);

    // let b = (*a + 0x5578) as *const usize;
    // dbg!(b);

    // let mut data_ptr: *mut u32 = BASE_ADDRESS as *mut u32;

    // unsafe {
    //     *data_ptr = (*data_ptr + 0x5560) as u32;

    //     dbg!(data_ptr);
    // }

    Ok(())
}
