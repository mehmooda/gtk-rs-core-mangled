// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::MenuItem;
use crate::MenuModel;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct Menu(Object<ffi::GMenu>) @extends MenuModel;

    match fn {
        get_type => || ffi::g_menu_get_type(),
    }
}

impl Menu {
    #[doc(alias = "g_menu_new")]
    pub fn new() -> Menu {
        unsafe { from_glib_full(ffi::g_menu_new()) }
    }

    #[doc(alias = "g_menu_append")]
    pub fn append(&self, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_append(
                self.to_glib_none().0,
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_append_item")]
    pub fn append_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_append_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_append_section")]
    pub fn append_section<P: IsA<MenuModel>>(&self, label: Option<&str>, section: &P) {
        unsafe {
            ffi::g_menu_append_section(
                self.to_glib_none().0,
                label.to_glib_none().0,
                section.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_append_submenu")]
    pub fn append_submenu<P: IsA<MenuModel>>(&self, label: Option<&str>, submenu: &P) {
        unsafe {
            ffi::g_menu_append_submenu(
                self.to_glib_none().0,
                label.to_glib_none().0,
                submenu.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_freeze")]
    pub fn freeze(&self) {
        unsafe {
            ffi::g_menu_freeze(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_insert")]
    pub fn insert(&self, position: i32, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_insert(
                self.to_glib_none().0,
                position,
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_insert_item")]
    pub fn insert_item(&self, position: i32, item: &MenuItem) {
        unsafe {
            ffi::g_menu_insert_item(self.to_glib_none().0, position, item.to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_insert_section")]
    pub fn insert_section<P: IsA<MenuModel>>(
        &self,
        position: i32,
        label: Option<&str>,
        section: &P,
    ) {
        unsafe {
            ffi::g_menu_insert_section(
                self.to_glib_none().0,
                position,
                label.to_glib_none().0,
                section.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_insert_submenu")]
    pub fn insert_submenu<P: IsA<MenuModel>>(
        &self,
        position: i32,
        label: Option<&str>,
        submenu: &P,
    ) {
        unsafe {
            ffi::g_menu_insert_submenu(
                self.to_glib_none().0,
                position,
                label.to_glib_none().0,
                submenu.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_prepend")]
    pub fn prepend(&self, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_prepend(
                self.to_glib_none().0,
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_prepend_item")]
    pub fn prepend_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_prepend_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "g_menu_prepend_section")]
    pub fn prepend_section<P: IsA<MenuModel>>(&self, label: Option<&str>, section: &P) {
        unsafe {
            ffi::g_menu_prepend_section(
                self.to_glib_none().0,
                label.to_glib_none().0,
                section.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_prepend_submenu")]
    pub fn prepend_submenu<P: IsA<MenuModel>>(&self, label: Option<&str>, submenu: &P) {
        unsafe {
            ffi::g_menu_prepend_submenu(
                self.to_glib_none().0,
                label.to_glib_none().0,
                submenu.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_menu_remove")]
    pub fn remove(&self, position: i32) {
        unsafe {
            ffi::g_menu_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "g_menu_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_menu_remove_all(self.to_glib_none().0);
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Menu")
    }
}
