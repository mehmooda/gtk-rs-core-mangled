// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Alignment;
use crate::AttrList;
use crate::Context;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use crate::Direction;
use crate::EllipsizeMode;
use crate::FontDescription;
use crate::LayoutIter;
use crate::LayoutLine;
use crate::Rectangle;
use crate::TabArray;
use crate::WrapMode;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::glib_wrapper! {
    pub struct Layout(Object<ffi::PangoLayout, ffi::PangoLayoutClass>);

    match fn {
        get_type => || ffi::pango_layout_get_type(),
    }
}

impl Layout {
    #[doc(alias = "pango_layout_new")]
    pub fn new(context: &Context) -> Layout {
        unsafe { from_glib_full(ffi::pango_layout_new(context.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_context_changed")]
    pub fn context_changed(&self) {
        unsafe {
            ffi::pango_layout_context_changed(self.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_copy")]
    pub fn copy(&self) -> Option<Layout> {
        unsafe { from_glib_full(ffi::pango_layout_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_alignment")]
    pub fn get_alignment(&self) -> Alignment {
        unsafe { from_glib(ffi::pango_layout_get_alignment(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_attributes")]
    pub fn get_attributes(&self) -> Option<AttrList> {
        unsafe { from_glib_none(ffi::pango_layout_get_attributes(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_auto_dir")]
    pub fn get_auto_dir(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_get_auto_dir(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_baseline")]
    pub fn get_baseline(&self) -> i32 {
        unsafe { ffi::pango_layout_get_baseline(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_character_count")]
    pub fn get_character_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_character_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_context")]
    pub fn get_context(&self) -> Option<Context> {
        unsafe { from_glib_none(ffi::pango_layout_get_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_cursor_pos")]
    pub fn get_cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            ffi::pango_layout_get_cursor_pos(
                self.to_glib_none().0,
                index_,
                strong_pos.to_glib_none_mut().0,
                weak_pos.to_glib_none_mut().0,
            );
            (strong_pos, weak_pos)
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_layout_get_direction")]
    pub fn get_direction(&self, index: i32) -> Direction {
        unsafe {
            from_glib(ffi::pango_layout_get_direction(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_ellipsize")]
    pub fn get_ellipsize(&self) -> EllipsizeMode {
        unsafe { from_glib(ffi::pango_layout_get_ellipsize(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_extents")]
    pub fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_get_font_description")]
    pub fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_font_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_height")]
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::pango_layout_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_indent")]
    pub fn get_indent(&self) -> i32 {
        unsafe { ffi::pango_layout_get_indent(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_iter")]
    pub fn get_iter(&self) -> Option<LayoutIter> {
        unsafe { from_glib_full(ffi::pango_layout_get_iter(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_justify")]
    pub fn get_justify(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_get_justify(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_line")]
    pub fn get_line(&self, line: i32) -> Option<LayoutLine> {
        unsafe { from_glib_none(ffi::pango_layout_get_line(self.to_glib_none().0, line)) }
    }

    #[doc(alias = "pango_layout_get_line_count")]
    pub fn get_line_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_line_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_line_readonly")]
    pub fn get_line_readonly(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line_readonly(
                self.to_glib_none().0,
                line,
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_layout_get_line_spacing")]
    pub fn get_line_spacing(&self) -> f32 {
        unsafe { ffi::pango_layout_get_line_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_lines")]
    pub fn get_lines(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines(self.to_glib_none().0))
        }
    }

    #[doc(alias = "pango_layout_get_lines_readonly")]
    pub fn get_lines_readonly(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines_readonly(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "pango_layout_get_log_attrs")]
    //pub fn get_log_attrs(&self, attrs: /*Ignored*/Vec<LogAttr>) -> i32 {
    //    unsafe { TODO: call ffi:pango_layout_get_log_attrs() }
    //}

    //#[doc(alias = "pango_layout_get_log_attrs_readonly")]
    //pub fn get_log_attrs_readonly(&self) -> /*Ignored*/Vec<LogAttr> {
    //    unsafe { TODO: call ffi:pango_layout_get_log_attrs_readonly() }
    //}

    #[doc(alias = "pango_layout_get_pixel_extents")]
    pub fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_pixel_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_get_pixel_size")]
    pub fn get_pixel_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_get_pixel_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    #[doc(alias = "pango_layout_get_serial")]
    pub fn get_serial(&self) -> u32 {
        unsafe { ffi::pango_layout_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_single_paragraph_mode")]
    pub fn get_single_paragraph_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_single_paragraph_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_get_size")]
    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    #[doc(alias = "pango_layout_get_spacing")]
    pub fn get_spacing(&self) -> i32 {
        unsafe { ffi::pango_layout_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_tabs")]
    pub fn get_tabs(&self) -> Option<TabArray> {
        unsafe { from_glib_full(ffi::pango_layout_get_tabs(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_text")]
    pub fn get_text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::pango_layout_get_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_get_unknown_glyphs_count")]
    pub fn get_unknown_glyphs_count(&self) -> i32 {
        unsafe { ffi::pango_layout_get_unknown_glyphs_count(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_width")]
    pub fn get_width(&self) -> i32 {
        unsafe { ffi::pango_layout_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_layout_get_wrap")]
    pub fn get_wrap(&self) -> WrapMode {
        unsafe { from_glib(ffi::pango_layout_get_wrap(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_index_to_line_x")]
    pub fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32) {
        unsafe {
            let mut line = mem::MaybeUninit::uninit();
            let mut x_pos = mem::MaybeUninit::uninit();
            ffi::pango_layout_index_to_line_x(
                self.to_glib_none().0,
                index_,
                trailing.to_glib(),
                line.as_mut_ptr(),
                x_pos.as_mut_ptr(),
            );
            let line = line.assume_init();
            let x_pos = x_pos.assume_init();
            (line, x_pos)
        }
    }

    #[doc(alias = "pango_layout_index_to_pos")]
    pub fn index_to_pos(&self, index_: i32) -> Rectangle {
        unsafe {
            let mut pos = Rectangle::uninitialized();
            ffi::pango_layout_index_to_pos(self.to_glib_none().0, index_, pos.to_glib_none_mut().0);
            pos
        }
    }

    #[doc(alias = "pango_layout_is_ellipsized")]
    pub fn is_ellipsized(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_is_ellipsized(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_is_wrapped")]
    pub fn is_wrapped(&self) -> bool {
        unsafe { from_glib(ffi::pango_layout_is_wrapped(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_layout_move_cursor_visually")]
    pub fn move_cursor_visually(
        &self,
        strong: bool,
        old_index: i32,
        old_trailing: i32,
        direction: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut new_index = mem::MaybeUninit::uninit();
            let mut new_trailing = mem::MaybeUninit::uninit();
            ffi::pango_layout_move_cursor_visually(
                self.to_glib_none().0,
                strong.to_glib(),
                old_index,
                old_trailing,
                direction,
                new_index.as_mut_ptr(),
                new_trailing.as_mut_ptr(),
            );
            let new_index = new_index.assume_init();
            let new_trailing = new_trailing.assume_init();
            (new_index, new_trailing)
        }
    }

    #[doc(alias = "pango_layout_set_alignment")]
    pub fn set_alignment(&self, alignment: Alignment) {
        unsafe {
            ffi::pango_layout_set_alignment(self.to_glib_none().0, alignment.to_glib());
        }
    }

    #[doc(alias = "pango_layout_set_attributes")]
    pub fn set_attributes(&self, attrs: Option<&AttrList>) {
        unsafe {
            ffi::pango_layout_set_attributes(self.to_glib_none().0, attrs.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_set_auto_dir")]
    pub fn set_auto_dir(&self, auto_dir: bool) {
        unsafe {
            ffi::pango_layout_set_auto_dir(self.to_glib_none().0, auto_dir.to_glib());
        }
    }

    #[doc(alias = "pango_layout_set_ellipsize")]
    pub fn set_ellipsize(&self, ellipsize: EllipsizeMode) {
        unsafe {
            ffi::pango_layout_set_ellipsize(self.to_glib_none().0, ellipsize.to_glib());
        }
    }

    #[doc(alias = "pango_layout_set_font_description")]
    pub fn set_font_description(&self, desc: Option<&FontDescription>) {
        unsafe {
            ffi::pango_layout_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_layout_set_height")]
    pub fn set_height(&self, height: i32) {
        unsafe {
            ffi::pango_layout_set_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "pango_layout_set_indent")]
    pub fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::pango_layout_set_indent(self.to_glib_none().0, indent);
        }
    }

    #[doc(alias = "pango_layout_set_justify")]
    pub fn set_justify(&self, justify: bool) {
        unsafe {
            ffi::pango_layout_set_justify(self.to_glib_none().0, justify.to_glib());
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_layout_set_line_spacing")]
    pub fn set_line_spacing(&self, factor: f32) {
        unsafe {
            ffi::pango_layout_set_line_spacing(self.to_glib_none().0, factor);
        }
    }

    #[doc(alias = "pango_layout_set_markup")]
    pub fn set_markup(&self, markup: &str) {
        let length = markup.len() as i32;
        unsafe {
            ffi::pango_layout_set_markup(self.to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    #[doc(alias = "pango_layout_set_markup_with_accel")]
    pub fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char {
        let length = markup.len() as i32;
        unsafe {
            let mut accel_char = mem::MaybeUninit::uninit();
            ffi::pango_layout_set_markup_with_accel(
                self.to_glib_none().0,
                markup.to_glib_none().0,
                length,
                accel_marker.to_glib(),
                accel_char.as_mut_ptr(),
            );
            let accel_char = accel_char.assume_init();
            std::convert::TryFrom::try_from(accel_char)
                .expect("conversion from an invalid Unicode value attempted")
        }
    }

    #[doc(alias = "pango_layout_set_single_paragraph_mode")]
    pub fn set_single_paragraph_mode(&self, setting: bool) {
        unsafe {
            ffi::pango_layout_set_single_paragraph_mode(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[doc(alias = "pango_layout_set_spacing")]
    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::pango_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "pango_layout_set_tabs")]
    pub fn set_tabs(&self, tabs: Option<&TabArray>) {
        unsafe {
            ffi::pango_layout_set_tabs(self.to_glib_none().0, mut_override(tabs.to_glib_none().0));
        }
    }

    #[doc(alias = "pango_layout_set_text")]
    pub fn set_text(&self, text: &str) {
        let length = text.len() as i32;
        unsafe {
            ffi::pango_layout_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    #[doc(alias = "pango_layout_set_width")]
    pub fn set_width(&self, width: i32) {
        unsafe {
            ffi::pango_layout_set_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "pango_layout_set_wrap")]
    pub fn set_wrap(&self, wrap: WrapMode) {
        unsafe {
            ffi::pango_layout_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    #[doc(alias = "pango_layout_xy_to_index")]
    pub fn xy_to_index(&self, x: i32, y: i32) -> (bool, i32, i32) {
        unsafe {
            let mut index_ = mem::MaybeUninit::uninit();
            let mut trailing = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::pango_layout_xy_to_index(
                self.to_glib_none().0,
                x,
                y,
                index_.as_mut_ptr(),
                trailing.as_mut_ptr(),
            ));
            let index_ = index_.assume_init();
            let trailing = trailing.assume_init();
            (ret, index_, trailing)
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Layout")
    }
}
