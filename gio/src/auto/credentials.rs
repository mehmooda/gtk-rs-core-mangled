// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use std::ptr;

glib::glib_wrapper! {
    pub struct Credentials(Object<ffi::GCredentials, ffi::GCredentialsClass>);

    match fn {
        get_type => || ffi::g_credentials_get_type(),
    }
}

impl Credentials {
    pub fn new() -> Credentials {
        unsafe { from_glib_full(ffi::g_credentials_new()) }
    }

    //pub fn get_native(&self, native_type: CredentialsType) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_credentials_get_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub fn get_unix_pid(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_credentials_get_unix_pid(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub fn get_unix_user(&self) -> Result<u32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_credentials_get_unix_user(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn is_same_user(&self, other_credentials: &Credentials) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_credentials_is_same_user(
                self.to_glib_none().0,
                other_credentials.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //pub fn set_native(&self, native_type: CredentialsType, native: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi:g_credentials_set_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub fn set_unix_user(&self, uid: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_credentials_set_unix_user(self.to_glib_none().0, uid, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_credentials_to_string(self.to_glib_none().0)) }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Credentials {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
