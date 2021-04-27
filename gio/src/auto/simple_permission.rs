// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Permission;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct SimplePermission(Object<ffi::GSimplePermission>) @extends Permission;

    match fn {
        type_ => || ffi::g_simple_permission_get_type(),
    }
}

impl SimplePermission {
    #[doc(alias = "g_simple_permission_new")]
    pub fn new(allowed: bool) -> SimplePermission {
        unsafe {
            Permission::from_glib_full(ffi::g_simple_permission_new(allowed.into_glib()))
                .unsafe_cast()
        }
    }
}

impl fmt::Display for SimplePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SimplePermission")
    }
}
