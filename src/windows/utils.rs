use eframe::Frame;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GWL_STYLE, GetWindowLongW, HWND_TOPMOST, MB_ICONERROR, MB_OK, MessageBoxW,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowLongW, SetWindowPos,
    WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_MAXIMIZEBOX,
    WS_MINIMIZEBOX,
};
use windows::core::PCWSTR;

fn from_frame_to_hwnd(frame: &Frame) -> Option<HWND> {
    if let Ok(handle) = frame.window_handle() {
        let raw = handle.as_raw();
        if let RawWindowHandle::Win32(win32) = raw {
            return Some(HWND(win32.hwnd.get() as *mut _));
        }
    }
    None
}

pub(crate) fn make_window_click_through(frame: &Frame) {
    if let Some(hwnd) = from_frame_to_hwnd(frame) {
        unsafe {
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
            );
        }
    }
}

pub(crate) fn initialize_tool_window(frame: &Frame) {
    if let Some(hwnd) = from_frame_to_hwnd(frame) {
        unsafe {
            let style = GetWindowLongW(hwnd, GWL_STYLE);
            SetWindowLongW(
                hwnd,
                GWL_STYLE,
                style & !(WS_MINIMIZEBOX | WS_MAXIMIZEBOX).0 as i32,
            );
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_TOOLWINDOW.0 as i32 | WS_EX_NOACTIVATE.0 as i32,
            );
            let _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE,
            );
        }
    }
}

pub(crate) fn initialize_window(frame: &Frame) {
    if let Some(hwnd) = from_frame_to_hwnd(frame) {
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
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
