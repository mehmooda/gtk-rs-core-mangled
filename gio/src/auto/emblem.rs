// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EmblemOrigin;
use crate::Icon;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct Emblem(Object<ffi::GEmblem, ffi::GEmblemClass>) @implements Icon;

    match fn {
        get_type => || ffi::g_emblem_get_type(),
    }
}

impl Emblem {
    pub fn new<P: IsA<Icon>>(icon: &P) -> Emblem {
        unsafe { from_glib_full(ffi::g_emblem_new(icon.as_ref().to_glib_none().0)) }
    }

    pub fn with_origin<P: IsA<Icon>>(icon: &P, origin: EmblemOrigin) -> Emblem {
        unsafe {
            from_glib_full(ffi::g_emblem_new_with_origin(
                icon.as_ref().to_glib_none().0,
                origin.to_glib(),
            ))
        }
    }

    pub fn get_icon(&self) -> Option<Icon> {
        unsafe { from_glib_none(ffi::g_emblem_get_icon(self.to_glib_none().0)) }
    }

    pub fn get_origin(&self) -> EmblemOrigin {
        unsafe { from_glib(ffi::g_emblem_get_origin(self.to_glib_none().0)) }
    }
}

impl fmt::Display for Emblem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Emblem")
    }
}
