// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use crate::FileInfo;
use crate::ZlibCompressorFormat;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct ZlibCompressor(Object<ffi::GZlibCompressor, ffi::GZlibCompressorClass>) @implements Converter;

    match fn {
        get_type => || ffi::g_zlib_compressor_get_type(),
    }
}

impl ZlibCompressor {
    pub fn new(format: ZlibCompressorFormat, level: i32) -> ZlibCompressor {
        unsafe { from_glib_full(ffi::g_zlib_compressor_new(format.to_glib(), level)) }
    }
}

pub const NONE_ZLIB_COMPRESSOR: Option<&ZlibCompressor> = None;

pub trait ZlibCompressorExt: 'static {
    fn get_file_info(&self) -> Option<FileInfo>;

    fn set_file_info(&self, file_info: Option<&FileInfo>);

    fn get_property_format(&self) -> ZlibCompressorFormat;

    fn get_property_level(&self) -> i32;

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ZlibCompressor>> ZlibCompressorExt for O {
    fn get_file_info(&self) -> Option<FileInfo> {
        unsafe {
            from_glib_none(ffi::g_zlib_compressor_get_file_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_file_info(&self, file_info: Option<&FileInfo>) {
        unsafe {
            ffi::g_zlib_compressor_set_file_info(
                self.as_ref().to_glib_none().0,
                file_info.to_glib_none().0,
            );
        }
    }

    fn get_property_format(&self) -> ZlibCompressorFormat {
        unsafe {
            let mut value =
                glib::Value::from_type(<ZlibCompressorFormat as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `format` getter")
                .unwrap()
        }
    }

    fn get_property_level(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"level\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `level` getter")
                .unwrap()
        }
    }

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_info_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GZlibCompressor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ZlibCompressor>,
        {
            let f: &F = &*(f as *const F);
            f(&ZlibCompressor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_info_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ZlibCompressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ZlibCompressor")
    }
}
