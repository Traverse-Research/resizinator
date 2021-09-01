use crate::{Resizinator, ResizinatorError, Result};
use winapi::shared::windef::HWND;
pub(crate) struct WinResizinator {
    hwnd: HWND,
}

impl WinResizinator {
    pub(crate) fn new(window_name: &str) -> Result<Self> {
        let c_str = std::ffi::CString::new(window_name).unwrap();
        let hwnd = unsafe { winapi::um::winuser::FindWindowA(std::ptr::null(), c_str.as_ptr()) };
        if hwnd == std::ptr::null_mut() {
            Err(ResizinatorError::WindowNotFound(window_name.into()))
        } else {
            Ok(Self { hwnd })
        }
    }
}

impl Resizinator for WinResizinator {
    fn resize(&self, x: usize, y: usize, width: usize, height: usize) {
        unsafe {
            winapi::um::winuser::SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                x as _,
                y as _,
                width as _,
                height as _,
                0,
            )
        };
    }
}
