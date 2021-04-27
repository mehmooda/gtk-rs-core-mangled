// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Pixbuf;
use crate::PixbufAnimation;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct PixbufSimpleAnim(Object<ffi::GdkPixbufSimpleAnim, ffi::GdkPixbufSimpleAnimClass>) @extends PixbufAnimation;

    match fn {
        type_ => || ffi::gdk_pixbuf_simple_anim_get_type(),
    }
}

impl PixbufSimpleAnim {
    #[doc(alias = "gdk_pixbuf_simple_anim_new")]
    pub fn new(width: i32, height: i32, rate: f32) -> PixbufSimpleAnim {
        unsafe { from_glib_full(ffi::gdk_pixbuf_simple_anim_new(width, height, rate)) }
    }

    #[doc(alias = "gdk_pixbuf_simple_anim_add_frame")]
    pub fn add_frame(&self, pixbuf: &Pixbuf) {
        unsafe {
            ffi::gdk_pixbuf_simple_anim_add_frame(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_pixbuf_simple_anim_get_loop")]
    pub fn is_loop(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_simple_anim_get_loop(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_simple_anim_set_loop")]
    pub fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gdk_pixbuf_simple_anim_set_loop(self.to_glib_none().0, loop_.into_glib());
        }
    }

    pub fn connect_property_loop_notify<F: Fn(&PixbufSimpleAnim) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_loop_trampoline<F: Fn(&PixbufSimpleAnim) + 'static>(
            this: *mut ffi::GdkPixbufSimpleAnim,
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
                b"notify::loop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PixbufSimpleAnim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PixbufSimpleAnim")
    }
}
