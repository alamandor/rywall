use std::{
    ops::{Deref, DerefMut},
    ptr
};
use x11::{
    xlib::{Display, XCloseDisplay, XOpenDisplay},
};

#[derive(Copy, Clone, Debug)]
pub struct XDisplayError;

pub struct XDisplay(*mut Display);
impl XDisplay {
    pub fn new() -> Result<Self, XDisplayError> {
        unsafe {
            let ptr = XOpenDisplay(ptr::null());
            if ptr.is_null() {
                Err(XDisplayError)
            } else {
                Ok(XDisplay(ptr))
            }
        }
    }
}
impl Deref for XDisplay {
    type Target = *mut Display;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for XDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.0);
        }
    }
}
