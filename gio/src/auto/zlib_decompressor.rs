// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use crate::FileInfo;
use crate::ZlibCompressorFormat;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct ZlibDecompressor(Object<ffi::GZlibDecompressor, ffi::GZlibDecompressorClass>) @implements Converter;

    match fn {
        get_type => || ffi::g_zlib_decompressor_get_type(),
    }
}

impl ZlibDecompressor {
    pub fn new(format: ZlibCompressorFormat) -> ZlibDecompressor {
        unsafe { from_glib_full(ffi::g_zlib_decompressor_new(format.to_glib())) }
    }

    pub fn get_file_info(&self) -> Option<FileInfo> {
        unsafe {
            from_glib_none(ffi::g_zlib_decompressor_get_file_info(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_format(&self) -> ZlibCompressorFormat {
        unsafe {
            let mut value =
                glib::Value::from_type(<ZlibCompressorFormat as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `format` getter")
                .unwrap()
        }
    }

    pub fn connect_property_file_info_notify<F: Fn(&ZlibDecompressor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_info_trampoline<F: Fn(&ZlibDecompressor) + 'static>(
            this: *mut ffi::GZlibDecompressor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_info_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ZlibDecompressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ZlibDecompressor")
    }
}
