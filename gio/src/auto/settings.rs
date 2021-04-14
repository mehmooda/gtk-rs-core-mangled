// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Action;
use crate::SettingsBackend;
use crate::SettingsSchema;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Settings(Object<ffi::GSettings, ffi::GSettingsClass>);

    match fn {
        type_ => || ffi::g_settings_get_type(),
    }
}

impl Settings {
    #[doc(alias = "g_settings_new")]
    pub fn new(schema_id: &str) -> Settings {
        unsafe { from_glib_full(ffi::g_settings_new(schema_id.to_glib_none().0)) }
    }

    #[doc(alias = "g_settings_new_full")]
    pub fn new_full<P: IsA<SettingsBackend>>(
        schema: &SettingsSchema,
        backend: Option<&P>,
        path: Option<&str>,
    ) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_full(
                schema.to_glib_none().0,
                backend.map(|p| p.as_ref()).to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_new_with_backend")]
    pub fn with_backend<P: IsA<SettingsBackend>>(schema_id: &str, backend: &P) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_with_backend(
                schema_id.to_glib_none().0,
                backend.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_new_with_backend_and_path")]
    pub fn with_backend_and_path<P: IsA<SettingsBackend>>(
        schema_id: &str,
        backend: &P,
        path: &str,
    ) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_with_backend_and_path(
                schema_id.to_glib_none().0,
                backend.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_new_with_path")]
    pub fn with_path(schema_id: &str, path: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_with_path(
                schema_id.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_settings_sync")]
    pub fn sync() {
        unsafe {
            ffi::g_settings_sync();
        }
    }

    #[doc(alias = "g_settings_unbind")]
    pub fn unbind<P: IsA<glib::Object>>(object: &P, property: &str) {
        unsafe {
            ffi::g_settings_unbind(object.as_ref().to_glib_none().0, property.to_glib_none().0);
        }
    }
}

pub const NONE_SETTINGS: Option<&Settings> = None;

pub trait SettingsExt: 'static {
    #[doc(alias = "g_settings_apply")]
    fn apply(&self);

    #[doc(alias = "g_settings_bind_writable")]
    fn bind_writable<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        inverted: bool,
    );

    #[doc(alias = "g_settings_create_action")]
    fn create_action(&self, key: &str) -> Action;

    #[doc(alias = "g_settings_delay")]
    fn delay(&self);

    //#[doc(alias = "g_settings_get")]
    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "g_settings_get_boolean")]
    fn boolean(&self, key: &str) -> bool;

    #[doc(alias = "g_settings_get_child")]
    fn child(&self, name: &str) -> Settings;

    #[doc(alias = "g_settings_get_default_value")]
    fn default_value(&self, key: &str) -> Option<glib::Variant>;

    #[doc(alias = "g_settings_get_double")]
    fn double(&self, key: &str) -> f64;

    #[doc(alias = "g_settings_get_enum")]
    fn enum_(&self, key: &str) -> i32;

    #[doc(alias = "g_settings_get_flags")]
    fn flags(&self, key: &str) -> u32;

    #[doc(alias = "g_settings_get_has_unapplied")]
    fn has_unapplied(&self) -> bool;

    #[doc(alias = "g_settings_get_int")]
    fn int(&self, key: &str) -> i32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_settings_get_int64")]
    fn int64(&self, key: &str) -> i64;

    //#[doc(alias = "g_settings_get_mapped")]
    //fn mapped(&self, key: &str, mapping: /*Unimplemented*/FnMut(&glib::Variant, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "g_settings_get_string")]
    fn string(&self, key: &str) -> glib::GString;

    #[doc(alias = "g_settings_get_strv")]
    fn strv(&self, key: &str) -> Vec<glib::GString>;

    #[doc(alias = "g_settings_get_uint")]
    fn uint(&self, key: &str) -> u32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_settings_get_uint64")]
    fn uint64(&self, key: &str) -> u64;

    #[doc(alias = "g_settings_get_user_value")]
    fn user_value(&self, key: &str) -> Option<glib::Variant>;

    #[doc(alias = "g_settings_get_value")]
    fn value(&self, key: &str) -> glib::Variant;

    #[doc(alias = "g_settings_is_writable")]
    fn is_writable(&self, name: &str) -> bool;

    #[doc(alias = "g_settings_list_children")]
    fn list_children(&self) -> Vec<glib::GString>;

    #[cfg_attr(feature = "v2_46", deprecated)]
    #[doc(alias = "g_settings_list_keys")]
    fn list_keys(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_settings_reset")]
    fn reset(&self, key: &str);

    #[doc(alias = "g_settings_revert")]
    fn revert(&self);

    //#[doc(alias = "g_settings_set")]
    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    #[doc(alias = "g_settings_set_boolean")]
    fn set_boolean(&self, key: &str, value: bool) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_double")]
    fn set_double(&self, key: &str, value: f64) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_enum")]
    fn set_enum(&self, key: &str, value: i32) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_flags")]
    fn set_flags(&self, key: &str, value: u32) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_int")]
    fn set_int(&self, key: &str, value: i32) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_settings_set_int64")]
    fn set_int64(&self, key: &str, value: i64) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_string")]
    fn set_string(&self, key: &str, value: &str) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_strv")]
    fn set_strv(&self, key: &str, value: &[&str]) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_uint")]
    fn set_uint(&self, key: &str, value: u32) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_settings_set_uint64")]
    fn set_uint64(&self, key: &str, value: u64) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "g_settings_set_value")]
    fn set_value(&self, key: &str, value: &glib::Variant) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "get_property_backend")]
    fn backend(&self) -> Option<SettingsBackend>;

    #[doc(alias = "get_property_delay_apply")]
    fn is_delay_apply(&self) -> bool;

    #[doc(alias = "get_property_path")]
    fn path(&self) -> Option<glib::GString>;

    #[doc(alias = "get_property_schema_id")]
    fn schema_id(&self) -> Option<glib::GString>;

    #[doc(alias = "get_property_settings_schema")]
    fn settings_schema(&self) -> Option<SettingsSchema>;

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    fn connect_writable_change_event<F: Fn(&Self, u32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Settings>> SettingsExt for O {
    fn apply(&self) {
        unsafe {
            ffi::g_settings_apply(self.as_ref().to_glib_none().0);
        }
    }

    fn bind_writable<P: IsA<glib::Object>>(
        &self,
        key: &str,
        object: &P,
        property: &str,
        inverted: bool,
    ) {
        unsafe {
            ffi::g_settings_bind_writable(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                inverted.to_glib(),
            );
        }
    }

    fn create_action(&self, key: &str) -> Action {
        unsafe {
            from_glib_full(ffi::g_settings_create_action(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn delay(&self) {
        unsafe {
            ffi::g_settings_delay(self.as_ref().to_glib_none().0);
        }
    }

    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_settings_get() }
    //}

    fn boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_boolean(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn child(&self, name: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_get_child(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn default_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_default_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn double(&self, key: &str) -> f64 {
        unsafe { ffi::g_settings_get_double(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    fn enum_(&self, key: &str) -> i32 {
        unsafe { ffi::g_settings_get_enum(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    fn flags(&self, key: &str) -> u32 {
        unsafe { ffi::g_settings_get_flags(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    fn has_unapplied(&self) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_has_unapplied(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn int(&self, key: &str) -> i32 {
        unsafe { ffi::g_settings_get_int(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn int64(&self, key: &str) -> i64 {
        unsafe { ffi::g_settings_get_int64(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    //fn mapped(&self, key: &str, mapping: /*Unimplemented*/FnMut(&glib::Variant, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_settings_get_mapped() }
    //}

    fn string(&self, key: &str) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_settings_get_string(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn strv(&self, key: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_get_strv(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn uint(&self, key: &str) -> u32 {
        unsafe { ffi::g_settings_get_uint(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn uint64(&self, key: &str) -> u64 {
        unsafe { ffi::g_settings_get_uint64(self.as_ref().to_glib_none().0, key.to_glib_none().0) }
    }

    fn user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_user_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn value(&self, key: &str) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::g_settings_get_value(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn is_writable(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_is_writable(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn list_children(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_children(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_keys(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reset(&self, key: &str) {
        unsafe {
            ffi::g_settings_reset(self.as_ref().to_glib_none().0, key.to_glib_none().0);
        }
    }

    fn revert(&self) {
        unsafe {
            ffi::g_settings_revert(self.as_ref().to_glib_none().0);
        }
    }

    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:g_settings_set() }
    //}

    fn set_boolean(&self, key: &str, value: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_boolean(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value.to_glib()
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_double(&self, key: &str, value: f64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_double(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_enum(&self, key: &str, value: i32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_enum(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_flags(&self, key: &str, value: u32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_flags(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_int(&self, key: &str, value: i32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_int(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn set_int64(&self, key: &str, value: i64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_int64(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_string(&self, key: &str, value: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_string(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value.to_glib_none().0
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_strv(&self, key: &str, value: &[&str]) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_strv(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value.to_glib_none().0
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_uint(&self, key: &str, value: u32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_uint(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn set_uint64(&self, key: &str, value: u64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_uint64(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value
                ),
                "Can't set readonly key"
            )
        }
    }

    fn set_value(&self, key: &str, value: &glib::Variant) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_settings_set_value(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    value.to_glib_none().0
                ),
                "Can't set readonly key"
            )
        }
    }

    fn backend(&self) -> Option<SettingsBackend> {
        unsafe {
            let mut value = glib::Value::from_type(<SettingsBackend as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    fn is_delay_apply(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"delay-apply\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `delay-apply` getter")
        }
    }

    fn path(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"path\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `path` getter")
        }
    }

    fn schema_id(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"schema-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `schema-id` getter")
        }
    }

    fn settings_schema(&self) -> Option<SettingsSchema> {
        unsafe {
            let mut value = glib::Value::from_type(<SettingsSchema as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"settings-schema\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `settings-schema` getter")
        }
    }

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented keys: *.CArray TypeId { ns_id: 2, id: 5 }
    //}

    fn connect_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GSettings,
            key: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Settings::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("changed::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_writable_change_event<F: Fn(&Self, u32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn writable_change_event_trampoline<
            P,
            F: Fn(&P, u32) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GSettings,
            key: libc::c_uint,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast_ref(), key).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"writable-change-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    writable_change_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn writable_changed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GSettings,
            key: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Settings::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("writable-changed::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"writable-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    writable_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_apply_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::delay-apply\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_delay_apply_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_unapplied_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Settings>,
        {
            let f: &F = &*(f as *const F);
            f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-unapplied\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_unapplied_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Settings")
    }
}
