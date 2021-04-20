// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Font;
use crate::Fontset;
use crate::Language;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct FontsetSimple(Object<ffi::PangoFontsetSimple, ffi::PangoFontsetSimpleClass>) @extends Fontset;

    match fn {
        type_ => || ffi::pango_fontset_simple_get_type(),
    }
}

impl FontsetSimple {
    #[doc(alias = "pango_fontset_simple_new")]
    pub fn new(language: &mut Language) -> FontsetSimple {
        unsafe { from_glib_full(ffi::pango_fontset_simple_new(language.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_fontset_simple_append")]
    pub fn append<P: IsA<Font>>(&self, font: &P) {
        unsafe {
            ffi::pango_fontset_simple_append(self.to_glib_none().0, font.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "pango_fontset_simple_size")]
    pub fn size(&self) -> i32 {
        unsafe { ffi::pango_fontset_simple_size(self.to_glib_none().0) }
    }
}

impl fmt::Display for FontsetSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontsetSimple")
    }
}
