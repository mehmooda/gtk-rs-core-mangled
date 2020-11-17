// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use pango_sys;
use std::mem;
use AttrType;
use Attribute;

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttrIterator(Boxed<pango_sys::PangoAttrIterator>);

    match fn {
        copy => |ptr| pango_sys::pango_attr_iterator_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_attr_iterator_destroy(ptr),
        get_type => || pango_sys::pango_attr_iterator_get_type(),
    }
}

#[cfg(not(any(feature = "v1_44", feature = "dox")))]
glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttrIterator(Boxed<pango_sys::PangoAttrIterator>);

    match fn {
        copy => |ptr| pango_sys::pango_attr_iterator_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_attr_iterator_destroy(ptr),
    }
}

impl AttrIterator {
    pub fn get(&mut self, type_: AttrType) -> Option<Attribute> {
        unsafe {
            from_glib_none(pango_sys::pango_attr_iterator_get(
                self.to_glib_none_mut().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn get_attrs(&mut self) -> Vec<Attribute> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(pango_sys::pango_attr_iterator_get_attrs(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn next(&mut self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_attr_iterator_next(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn range(&mut self) -> (i32, i32) {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            pango_sys::pango_attr_iterator_range(
                self.to_glib_none_mut().0,
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            );
            let start = start.assume_init();
            let end = end.assume_init();
            (start, end)
        }
    }
}
