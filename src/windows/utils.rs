use eframe::Frame;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetSystemMetrics, GetWindowLongW, HWND_TOPMOST, MB_ICONERROR, MB_OK, MessageBoxW,
    SM_CXSCREEN, SM_CYSCREEN, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetWindowLongW, SetWindowPos,
    WS_EX_LAYERED, WS_EX_TRANSPARENT,
};
use windows::core::PCWSTR;

fn make_window_click_through(hwnd: HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
        );
    }
}

pub(crate) fn initialize_windows(frame: &Frame) {
    if let Ok(handle) = frame.window_handle() {
        unsafe {
            let raw = handle.as_raw();
            if let RawWindowHandle::Win32(win32) = raw {
                let hwnd = HWND(win32.hwnd.get() as *mut _);
                make_window_click_through(hwnd);
                let _ = SetWindowPos(
                    hwnd,
                    Some(HWND_TOPMOST),
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
            }
        }
    }
}

pub fn show_error(msg: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(Some(0)).collect();

    unsafe {
        MessageBoxW(
            None,
            PCWSTR(wide.as_ptr()),
            PCWSTR(wide.as_ptr()),
            MB_OK | MB_ICONERROR,
        );
    }
}

pub fn get_screen_size() -> (usize, usize) {
    let (width, height) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
    (width as usize, height as usize)
}
