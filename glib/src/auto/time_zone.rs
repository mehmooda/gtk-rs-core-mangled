// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
use crate::TimeType;

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TimeZone(Shared<ffi::GTimeZone>);

    match fn {
        ref => |ptr| ffi::g_time_zone_ref(ptr),
        unref => |ptr| ffi::g_time_zone_unref(ptr),
        get_type => || ffi::g_time_zone_get_type(),
    }
}

impl TimeZone {
    #[cfg_attr(feature = "v2_68", deprecated)]
    #[doc(alias = "g_time_zone_new")]
    pub fn new(identifier: Option<&str>) -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new(identifier.to_glib_none().0)) }
    }

    #[doc(alias = "g_time_zone_new_local")]
    pub fn new_local() -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_local()) }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_time_zone_new_offset")]
    pub fn new_offset(seconds: i32) -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_offset(seconds)) }
    }

    #[doc(alias = "g_time_zone_new_utc")]
    pub fn new_utc() -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_utc()) }
    }

    #[doc(alias = "g_time_zone_find_interval")]
    pub fn find_interval(&self, type_: TimeType, time_: i64) -> i32 {
        unsafe { ffi::g_time_zone_find_interval(self.to_glib_none().0, type_.to_glib(), time_) }
    }

    #[doc(alias = "g_time_zone_get_abbreviation")]
    pub fn get_abbreviation(&self, interval: i32) -> crate::GString {
        unsafe {
            from_glib_none(ffi::g_time_zone_get_abbreviation(
                self.to_glib_none().0,
                interval,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_time_zone_get_identifier")]
    pub fn get_identifier(&self) -> crate::GString {
        unsafe { from_glib_none(ffi::g_time_zone_get_identifier(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_time_zone_get_offset")]
    pub fn get_offset(&self, interval: i32) -> i32 {
        unsafe { ffi::g_time_zone_get_offset(self.to_glib_none().0, interval) }
    }

    #[doc(alias = "g_time_zone_is_dst")]
    pub fn is_dst(&self, interval: i32) -> bool {
        unsafe { from_glib(ffi::g_time_zone_is_dst(self.to_glib_none().0, interval)) }
    }
}

unsafe impl Send for TimeZone {}
unsafe impl Sync for TimeZone {}
