mod x11util;
use std::iter::FromIterator;
use crate::x11util::XDisplay;
use libc;
use scopeguard::defer;
use std::ffi::{CStr, CString};
use x11::xlib::{
    XResourceManagerString, XrmDatabase, XrmDestroyDatabase, XrmGetResource, XrmGetStringDatabase,
    XrmValue,
};

/// A struct for representing the colours found in the X Resource Manager's database.
#[derive(Clone, Debug, Default)]
pub struct ColorScheme {
    /// Foreground colour: matches the "foreground" key.
    pub fg: Option<String>,
    /// Background colour: matches the "background" key.
    pub bg: Option<String>,
    /// Cursor colour: matches the "cursorColor" key.
    pub cursor: Option<String>,
    /// Colours 0 to 15: match the "color{}" keys, where {} is a number from 0 to 15 inclusive.
    pub colors: [Option<String>; 16],
}

impl ColorScheme {
    // Searches for a prefix in the Xresource database, use of xlib functions based on documentation for the C versions.
    pub fn new<'a>(prefix: &'a str) -> Option<Self> {
        let display = XDisplay::new().expect("Failed to acquire X display!");
        unsafe {
            let rms = XResourceManagerString(*display);
            if !rms.is_null() {
                let db = XrmGetStringDatabase(rms);
                if !db.is_null() {
                    defer!({
                        XrmDestroyDatabase(db);
                    });
                    return Some(ColorScheme::from_database(db, prefix));
                }
            }
        }
        None
    }

    unsafe fn from_database<'a>(db: XrmDatabase, prefix: &'a str) -> Self {
        let mut xcolors = ColorScheme::default();
        let fg_str = format!("{}.foreground", prefix);
        let bg_str = format!("{}.background", prefix);
        let cursor_str = format!("{}.cursorColor", prefix);
        let fg = get_xrm_resource(db, &fg_str).map(|s| String::from(s));
        let bg = get_xrm_resource(db, &bg_str).map(|s| String::from(s));
        let cursor = get_xrm_resource(db, &cursor_str).map(|s| String::from(s));
        let color_names = (0..16).map(|i| format!("{}.color{}", prefix, i));
        let colors = color_names
            .map(|s| get_xrm_resource(db, &s).map(|s| String::from(s)))
            .collect::<Vec<_>>();
        xcolors.fg = fg;
        xcolors.bg = bg;
        xcolors.cursor = cursor;


        for x in 0..16 {
            xcolors.colors[x] = colors[x].clone();
        }
        xcolors
    }
}

unsafe fn get_xrm_resource<'a>(db: XrmDatabase, name: &'a str) -> Option<&'a str> {
    let mut value = XrmValue {
        size: 0,
        addr: std::ptr::null_mut(),
    };

    let mut value_type: *mut libc::c_char = std::ptr::null_mut();
    let name_c_str = CString::new(name).unwrap();
    let c_str = CString::new("String").unwrap();
    if XrmGetResource(
        db,
        name_c_str.as_ptr(),
        c_str.as_ptr(),
        &mut value_type,
        &mut value,
    ) != 0
        && !value.addr.is_null()
    {
        let value_addr: &CStr = CStr::from_ptr(value.addr);
        value_addr.to_str().ok()
    } else {
        None
    }
}

fn main() {

    let xstruct = ColorScheme::new("st");
    let xstruct2 = ColorScheme::new("Xterm");

    println!("{:?}", xstruct);
    println!("{:?}", xstruct2);




}
