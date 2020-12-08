// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::GlyphItem;
use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GlyphItemIter(Boxed<ffi::PangoGlyphItemIter>);

    match fn {
        copy => |ptr| ffi::pango_glyph_item_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_glyph_item_iter_free(ptr),
        get_type => || ffi::pango_glyph_item_iter_get_type(),
    }
}

impl GlyphItemIter {
    #[doc(alias = "pango_glyph_item_iter_init_end")]
    pub fn init_end(&mut self, glyph_item: &mut GlyphItem, text: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_init_end(
                self.to_glib_none_mut().0,
                glyph_item.to_glib_none_mut().0,
                text.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_glyph_item_iter_init_start")]
    pub fn init_start(&mut self, glyph_item: &mut GlyphItem, text: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_init_start(
                self.to_glib_none_mut().0,
                glyph_item.to_glib_none_mut().0,
                text.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_glyph_item_iter_next_cluster")]
    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_next_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "pango_glyph_item_iter_prev_cluster")]
    pub fn prev_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_prev_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }
}
