// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AppInfoMonitor(Object<gio_sys::GAppInfoMonitor, AppInfoMonitorClass>);

    match fn {
        get_type => || gio_sys::g_app_info_monitor_get_type(),
    }
}

impl AppInfoMonitor {
    pub fn get() -> AppInfoMonitor {
        unsafe { from_glib_full(gio_sys::g_app_info_monitor_get()) }
    }

    pub fn connect_changed<F: Fn(&AppInfoMonitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&AppInfoMonitor) + 'static>(
            this: *mut gio_sys::GAppInfoMonitor,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AppInfoMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppInfoMonitor")
    }
}
