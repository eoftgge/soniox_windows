use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub(crate) struct HandleWrapper(pub(crate) HANDLE);

impl Drop for HandleWrapper {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.0); }
    }
}