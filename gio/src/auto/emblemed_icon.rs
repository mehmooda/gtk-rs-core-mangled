// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Emblem;
use crate::Icon;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::glib_wrapper! {
    pub struct EmblemedIcon(Object<ffi::GEmblemedIcon, ffi::GEmblemedIconClass>) @implements Icon;

    match fn {
        get_type => || ffi::g_emblemed_icon_get_type(),
    }
}

impl EmblemedIcon {
    #[doc(alias = "g_emblemed_icon_new")]
    pub fn new<P: IsA<Icon>>(icon: &P, emblem: Option<&Emblem>) -> EmblemedIcon {
        unsafe {
            from_glib_full(ffi::g_emblemed_icon_new(
                icon.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_EMBLEMED_ICON: Option<&EmblemedIcon> = None;

pub trait EmblemedIconExt: 'static {
    #[doc(alias = "g_emblemed_icon_add_emblem")]
    fn add_emblem(&self, emblem: &Emblem);

    #[doc(alias = "g_emblemed_icon_clear_emblems")]
    fn clear_emblems(&self);

    #[doc(alias = "g_emblemed_icon_get_emblems")]
    fn get_emblems(&self) -> Vec<Emblem>;

    #[doc(alias = "g_emblemed_icon_get_icon")]
    fn get_icon(&self) -> Option<Icon>;

    fn get_property_gicon(&self) -> Option<Icon>;
}

impl<O: IsA<EmblemedIcon>> EmblemedIconExt for O {
    fn add_emblem(&self, emblem: &Emblem) {
        unsafe {
            ffi::g_emblemed_icon_add_emblem(
                self.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            );
        }
    }

    fn clear_emblems(&self) {
        unsafe {
            ffi::g_emblemed_icon_clear_emblems(self.as_ref().to_glib_none().0);
        }
    }

    fn get_emblems(&self) -> Vec<Emblem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_emblemed_icon_get_emblems(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_none(ffi::g_emblemed_icon_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_gicon(&self) -> Option<Icon> {
        unsafe {
            let mut value = glib::Value::from_type(<Icon as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"gicon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gicon` getter")
        }
    }
}

impl fmt::Display for EmblemedIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EmblemedIcon")
    }
}
