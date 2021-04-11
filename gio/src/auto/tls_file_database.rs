// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TlsDatabase;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct TlsFileDatabase(Interface<ffi::GTlsFileDatabase, ffi::GTlsFileDatabaseInterface>) @requires TlsDatabase;

    match fn {
        get_type => || ffi::g_tls_file_database_get_type(),
    }
}

impl TlsFileDatabase {
    #[doc(alias = "g_tls_file_database_new")]
    pub fn new<P: AsRef<std::path::Path>>(anchors: P) -> Result<TlsFileDatabase, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_file_database_new(anchors.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_TLS_FILE_DATABASE: Option<&TlsFileDatabase> = None;

pub trait TlsFileDatabaseExt: 'static {
    #[doc(alias = "get_property_anchors")]
    fn anchors(&self) -> Option<glib::GString>;

    #[doc(alias = "set_property_anchors")]
    fn set_anchors(&self, anchors: Option<&str>);

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsFileDatabase>> TlsFileDatabaseExt for O {
    fn anchors(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"anchors\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `anchors` getter")
        }
    }

    fn set_anchors(&self, anchors: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"anchors\0".as_ptr() as *const _,
                glib::Value::from(anchors).to_glib_none().0,
            );
        }
    }

    fn connect_property_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchors_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GTlsFileDatabase,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TlsFileDatabase>,
        {
            let f: &F = &*(f as *const F);
            f(&TlsFileDatabase::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anchors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_anchors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsFileDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsFileDatabase")
    }
}
