pub mod scan;

use crate::scan::run;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _reserved: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe {
            std::thread::spawn(move || {
                let _ = run();
            });
        }
    }
    true
}
