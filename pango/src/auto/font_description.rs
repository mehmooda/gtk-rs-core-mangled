// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use pango_sys;
use std::fmt;
use std::hash;
use FontMask;
use Gravity;
use Stretch;
use Style;
use Variant;
use Weight;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)]
    pub struct FontDescription(Boxed<pango_sys::PangoFontDescription>);

    match fn {
        copy => |ptr| pango_sys::pango_font_description_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_font_description_free(ptr),
        get_type => || pango_sys::pango_font_description_get_type(),
    }
}

impl FontDescription {
    pub fn new() -> FontDescription {
        unsafe { from_glib_full(pango_sys::pango_font_description_new()) }
    }

    pub fn better_match(
        &self,
        old_match: Option<&FontDescription>,
        new_match: &FontDescription,
    ) -> bool {
        unsafe {
            from_glib(pango_sys::pango_font_description_better_match(
                self.to_glib_none().0,
                old_match.to_glib_none().0,
                new_match.to_glib_none().0,
            ))
        }
    }

    fn equal(&self, desc2: &FontDescription) -> bool {
        unsafe {
            from_glib(pango_sys::pango_font_description_equal(
                self.to_glib_none().0,
                desc2.to_glib_none().0,
            ))
        }
    }

    pub fn get_family(&self) -> Option<GString> {
        unsafe {
            from_glib_none(pango_sys::pango_font_description_get_family(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_gravity(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_set_fields(&self) -> FontMask {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_set_fields(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_size(&self) -> i32 {
        unsafe { pango_sys::pango_font_description_get_size(self.to_glib_none().0) }
    }

    pub fn get_size_is_absolute(&self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_size_is_absolute(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_stretch(&self) -> Stretch {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_stretch(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_style(&self) -> Style {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_style(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_variant(&self) -> Variant {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_variant(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    pub fn get_variations(&self) -> Option<GString> {
        unsafe {
            from_glib_none(pango_sys::pango_font_description_get_variations(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_weight(&self) -> Weight {
        unsafe {
            from_glib(pango_sys::pango_font_description_get_weight(
                self.to_glib_none().0,
            ))
        }
    }

    fn hash(&self) -> u32 {
        unsafe { pango_sys::pango_font_description_hash(self.to_glib_none().0) }
    }

    pub fn merge(&mut self, desc_to_merge: Option<&FontDescription>, replace_existing: bool) {
        unsafe {
            pango_sys::pango_font_description_merge(
                self.to_glib_none_mut().0,
                desc_to_merge.to_glib_none().0,
                replace_existing.to_glib(),
            );
        }
    }

    pub fn set_absolute_size(&mut self, size: f64) {
        unsafe {
            pango_sys::pango_font_description_set_absolute_size(self.to_glib_none_mut().0, size);
        }
    }

    pub fn set_family(&mut self, family: &str) {
        unsafe {
            pango_sys::pango_font_description_set_family(
                self.to_glib_none_mut().0,
                family.to_glib_none().0,
            );
        }
    }

    pub fn set_gravity(&mut self, gravity: Gravity) {
        unsafe {
            pango_sys::pango_font_description_set_gravity(
                self.to_glib_none_mut().0,
                gravity.to_glib(),
            );
        }
    }

    pub fn set_size(&mut self, size: i32) {
        unsafe {
            pango_sys::pango_font_description_set_size(self.to_glib_none_mut().0, size);
        }
    }

    pub fn set_stretch(&mut self, stretch: Stretch) {
        unsafe {
            pango_sys::pango_font_description_set_stretch(
                self.to_glib_none_mut().0,
                stretch.to_glib(),
            );
        }
    }

    pub fn set_style(&mut self, style: Style) {
        unsafe {
            pango_sys::pango_font_description_set_style(self.to_glib_none_mut().0, style.to_glib());
        }
    }

    pub fn set_variant(&mut self, variant: Variant) {
        unsafe {
            pango_sys::pango_font_description_set_variant(
                self.to_glib_none_mut().0,
                variant.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    pub fn set_variations(&mut self, variations: &str) {
        unsafe {
            pango_sys::pango_font_description_set_variations(
                self.to_glib_none_mut().0,
                variations.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    pub fn set_variations_static(&mut self, variations: &str) {
        unsafe {
            pango_sys::pango_font_description_set_variations_static(
                self.to_glib_none_mut().0,
                variations.to_glib_none().0,
            );
        }
    }

    pub fn set_weight(&mut self, weight: Weight) {
        unsafe {
            pango_sys::pango_font_description_set_weight(
                self.to_glib_none_mut().0,
                weight.to_glib(),
            );
        }
    }

    pub fn to_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_full(pango_sys::pango_font_description_to_filename(
                self.to_glib_none().0,
            ))
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(pango_sys::pango_font_description_to_string(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn unset_fields(&mut self, to_unset: FontMask) {
        unsafe {
            pango_sys::pango_font_description_unset_fields(
                self.to_glib_none_mut().0,
                to_unset.to_glib(),
            );
        }
    }

    pub fn from_string(str: &str) -> FontDescription {
        unsafe {
            from_glib_full(pango_sys::pango_font_description_from_string(
                str.to_glib_none().0,
            ))
        }
    }
}

impl Default for FontDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for FontDescription {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for FontDescription {}

impl fmt::Display for FontDescription {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl hash::Hash for FontDescription {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.hash(), state)
    }
}
