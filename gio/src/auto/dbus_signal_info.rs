// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DBusSignalInfo(Shared<ffi::GDBusSignalInfo>);

    match fn {
        ref => |ptr| ffi::g_dbus_signal_info_ref(ptr),
        unref => |ptr| ffi::g_dbus_signal_info_unref(ptr),
        get_type => || ffi::g_dbus_signal_info_get_type(),
    }
}
