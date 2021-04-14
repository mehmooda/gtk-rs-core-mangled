// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Action;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
use glib::StaticType;
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct PropertyAction(Object<ffi::GPropertyAction>) @implements Action;

    match fn {
        type_ => || ffi::g_property_action_get_type(),
    }
}

impl PropertyAction {
    #[doc(alias = "g_property_action_new")]
    pub fn new<P: IsA<glib::Object>>(
        name: &str,
        object: &P,
        property_name: &str,
    ) -> PropertyAction {
        unsafe {
            from_glib_full(ffi::g_property_action_new(
                name.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
    #[doc(alias = "get_property_invert_boolean")]
    pub fn inverts_boolean(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"invert-boolean\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `invert-boolean` getter")
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&PropertyAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&PropertyAction) + 'static>(
            this: *mut ffi::GPropertyAction,
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
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_parameter_type_notify<F: Fn(&PropertyAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parameter_type_trampoline<F: Fn(&PropertyAction) + 'static>(
            this: *mut ffi::GPropertyAction,
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
                b"notify::parameter-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parameter_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_state_notify<F: Fn(&PropertyAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<F: Fn(&PropertyAction) + 'static>(
            this: *mut ffi::GPropertyAction,
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
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_state_type_notify<F: Fn(&PropertyAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_type_trampoline<F: Fn(&PropertyAction) + 'static>(
            this: *mut ffi::GPropertyAction,
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
                b"notify::state-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PropertyAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PropertyAction")
    }
}
