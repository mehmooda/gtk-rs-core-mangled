// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
use crate::value::FromValue;
use crate::value::FromValueOptional;
use crate::value::SetValue;
use crate::StaticType;
use crate::Type;
use bitflags::bitflags;
use std::fmt;

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
bitflags! {
    pub struct FileSetContentsFlags: u32 {
        const NONE = 0;
        const CONSISTENT = 1;
        const DURABLE = 2;
        const ONLY_EXISTING = 4;
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl fmt::Display for FileSetContentsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl ToGlib for FileSetContentsFlags {
    type GlibType = ffi::GFileSetContentsFlags;

    fn to_glib(&self) -> ffi::GFileSetContentsFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GFileSetContentsFlags> for FileSetContentsFlags {
    fn from_glib(value: ffi::GFileSetContentsFlags) -> FileSetContentsFlags {
        FileSetContentsFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct FileTest: u32 {
        const IS_REGULAR = 1;
        const IS_SYMLINK = 2;
        const IS_DIR = 4;
        const IS_EXECUTABLE = 8;
        const EXISTS = 16;
    }
}

impl fmt::Display for FileTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for FileTest {
    type GlibType = ffi::GFileTest;

    fn to_glib(&self) -> ffi::GFileTest {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileTest> for FileTest {
    fn from_glib(value: ffi::GFileTest) -> FileTest {
        FileTest::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct FormatSizeFlags: u32 {
        const DEFAULT = 0;
        const LONG_FORMAT = 1;
        const IEC_UNITS = 2;
        const BITS = 4;
    }
}

impl fmt::Display for FormatSizeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for FormatSizeFlags {
    type GlibType = ffi::GFormatSizeFlags;

    fn to_glib(&self) -> ffi::GFormatSizeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFormatSizeFlags> for FormatSizeFlags {
    fn from_glib(value: ffi::GFormatSizeFlags) -> FormatSizeFlags {
        FormatSizeFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct IOCondition: u32 {
        const IN = 1;
        const OUT = 4;
        const PRI = 2;
        const ERR = 8;
        const HUP = 16;
        const NVAL = 32;
    }
}

impl fmt::Display for IOCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for IOCondition {
    type GlibType = ffi::GIOCondition;

    fn to_glib(&self) -> ffi::GIOCondition {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIOCondition> for IOCondition {
    fn from_glib(value: ffi::GIOCondition) -> IOCondition {
        IOCondition::from_bits_truncate(value)
    }
}

impl StaticType for IOCondition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_io_condition_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IOCondition {
    unsafe fn from_value_optional(value: &crate::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IOCondition {
    unsafe fn from_value(value: &crate::Value) -> Self {
        from_glib(crate::gobject_ffi::g_value_get_flags(
            value.to_glib_none().0,
        ))
    }
}

impl SetValue for IOCondition {
    unsafe fn set_value(value: &mut crate::Value, this: &Self) {
        crate::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct KeyFileFlags: u32 {
        const NONE = 0;
        const KEEP_COMMENTS = 1;
        const KEEP_TRANSLATIONS = 2;
    }
}

impl fmt::Display for KeyFileFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    fn to_glib(&self) -> ffi::GKeyFileFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    fn from_glib(value: ffi::GKeyFileFlags) -> KeyFileFlags {
        KeyFileFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct LogLevelFlags: u32 {
        const FLAG_RECURSION = 1;
        const FLAG_FATAL = 2;
        const LEVEL_ERROR = 4;
        const LEVEL_CRITICAL = 8;
        const LEVEL_WARNING = 16;
        const LEVEL_MESSAGE = 32;
        const LEVEL_INFO = 64;
        const LEVEL_DEBUG = 128;
        const LEVEL_MASK = 4294967292;
    }
}

impl fmt::Display for LogLevelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for LogLevelFlags {
    type GlibType = ffi::GLogLevelFlags;

    fn to_glib(&self) -> ffi::GLogLevelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GLogLevelFlags> for LogLevelFlags {
    fn from_glib(value: ffi::GLogLevelFlags) -> LogLevelFlags {
        LogLevelFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct OptionFlags: u32 {
        const NONE = 0;
        const HIDDEN = 1;
        const IN_MAIN = 2;
        const REVERSE = 4;
        const NO_ARG = 8;
        const FILENAME = 16;
        const OPTIONAL_ARG = 32;
        const NOALIAS = 64;
    }
}

impl fmt::Display for OptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for OptionFlags {
    type GlibType = ffi::GOptionFlags;

    fn to_glib(&self) -> ffi::GOptionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GOptionFlags> for OptionFlags {
    fn from_glib(value: ffi::GOptionFlags) -> OptionFlags {
        OptionFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SpawnFlags: u32 {
        const DEFAULT = 0;
        const LEAVE_DESCRIPTORS_OPEN = 1;
        const DO_NOT_REAP_CHILD = 2;
        const SEARCH_PATH = 4;
        const STDOUT_TO_DEV_NULL = 8;
        const STDERR_TO_DEV_NULL = 16;
        const CHILD_INHERITS_STDIN = 32;
        const FILE_AND_ARGV_ZERO = 64;
        const SEARCH_PATH_FROM_ENVP = 128;
        const CLOEXEC_PIPES = 256;
    }
}

impl fmt::Display for SpawnFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for SpawnFlags {
    type GlibType = ffi::GSpawnFlags;

    fn to_glib(&self) -> ffi::GSpawnFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSpawnFlags> for SpawnFlags {
    fn from_glib(value: ffi::GSpawnFlags) -> SpawnFlags {
        SpawnFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
bitflags! {
    pub struct UriFlags: u32 {
        const NONE = 0;
        const PARSE_RELAXED = 1;
        const HAS_PASSWORD = 2;
        const HAS_AUTH_PARAMS = 4;
        const ENCODED = 8;
        const NON_DNS = 16;
        const ENCODED_QUERY = 32;
        const ENCODED_PATH = 64;
        const ENCODED_FRAGMENT = 128;
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl ToGlib for UriFlags {
    type GlibType = ffi::GUriFlags;

    fn to_glib(&self) -> ffi::GUriFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriFlags> for UriFlags {
    fn from_glib(value: ffi::GUriFlags) -> UriFlags {
        UriFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
bitflags! {
    pub struct UriHideFlags: u32 {
        const NONE = 0;
        const USERINFO = 1;
        const PASSWORD = 2;
        const AUTH_PARAMS = 4;
        const QUERY = 8;
        const FRAGMENT = 16;
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriHideFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl ToGlib for UriHideFlags {
    type GlibType = ffi::GUriHideFlags;

    fn to_glib(&self) -> ffi::GUriHideFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriHideFlags> for UriHideFlags {
    fn from_glib(value: ffi::GUriHideFlags) -> UriHideFlags {
        UriHideFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
bitflags! {
    pub struct UriParamsFlags: u32 {
        const NONE = 0;
        const CASE_INSENSITIVE = 1;
        const WWW_FORM = 2;
        const PARSE_RELAXED = 4;
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriParamsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl ToGlib for UriParamsFlags {
    type GlibType = ffi::GUriParamsFlags;

    fn to_glib(&self) -> ffi::GUriParamsFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriParamsFlags> for UriParamsFlags {
    fn from_glib(value: ffi::GUriParamsFlags) -> UriParamsFlags {
        UriParamsFlags::from_bits_truncate(value)
    }
}
