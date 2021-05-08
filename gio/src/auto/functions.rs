// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::BusType;
use crate::Cancellable;
use crate::DBusConnection;
use crate::File;
use crate::IOErrorEnum;
use crate::IOStream;
use crate::Icon;
use crate::InputStream;
use crate::Resource;
use crate::ResourceLookupFlags;
use crate::SettingsBackend;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::pin::Pin;
use std::ptr;

#[doc(alias = "g_bus_get")]
pub fn bus_get<
    P: IsA<Cancellable>,
    Q: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
>(
    bus_type: BusType,
    cancellable: Option<&P>,
    callback: Q,
) {
    let user_data: Box_<Q> = Box_::new(callback);
    unsafe extern "C" fn bus_get_trampoline<
        Q: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut crate::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let ret = ffi::g_bus_get_finish(res, &mut error);
        let result = if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
        callback(result);
    }
    let callback = bus_get_trampoline::<Q>;
    unsafe {
        ffi::g_bus_get(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

pub fn bus_get_future(
    bus_type: BusType,
) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusConnection, glib::Error>> + 'static>> {
    Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
        let cancellable = Cancellable::new();
        bus_get(bus_type, Some(&cancellable), move |res| {
            send.resolve(res);
        });

        cancellable
    }))
}

#[doc(alias = "g_bus_get_sync")]
pub fn bus_get_sync<P: IsA<Cancellable>>(
    bus_type: BusType,
    cancellable: Option<&P>,
) -> Result<DBusConnection, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_bus_get_sync(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[doc(alias = "g_bus_own_name")]
//pub fn bus_own_name(bus_type: BusType, name: &str, flags: BusNameOwnerFlags, bus_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_lost_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_own_name() }
//}

//#[doc(alias = "g_bus_own_name_on_connection")]
//pub fn bus_own_name_on_connection(connection: &DBusConnection, name: &str, flags: BusNameOwnerFlags, name_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_lost_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_own_name_on_connection() }
//}

//#[doc(alias = "g_bus_watch_name")]
//pub fn bus_watch_name(bus_type: BusType, name: &str, flags: BusNameWatcherFlags, name_appeared_handler: Option<Box_<dyn Fn(&DBusConnection, &str, &str) + 'static>>, name_vanished_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_watch_name() }
//}

//#[doc(alias = "g_bus_watch_name_on_connection")]
//pub fn bus_watch_name_on_connection(connection: &DBusConnection, name: &str, flags: BusNameWatcherFlags, name_appeared_handler: Option<Box_<dyn Fn(&DBusConnection, &str, &str) + 'static>>, name_vanished_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_watch_name_on_connection() }
//}

#[doc(alias = "g_content_type_can_be_executable")]
pub fn content_type_can_be_executable(type_: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_can_be_executable(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_equals")]
pub fn content_type_equals(type1: &str, type2: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_equals(
            type1.to_glib_none().0,
            type2.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_from_mime_type")]
pub fn content_type_from_mime_type(mime_type: &str) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::g_content_type_from_mime_type(
            mime_type.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_get_description")]
pub fn content_type_get_description(type_: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::g_content_type_get_description(type_.to_glib_none().0)) }
}

#[doc(alias = "g_content_type_get_generic_icon_name")]
pub fn content_type_get_generic_icon_name(type_: &str) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_generic_icon_name(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_get_icon")]
pub fn content_type_get_icon(type_: &str) -> Icon {
    unsafe { from_glib_full(ffi::g_content_type_get_icon(type_.to_glib_none().0)) }
}

#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
#[doc(alias = "g_content_type_get_mime_dirs")]
pub fn content_type_get_mime_dirs() -> Vec<glib::GString> {
    unsafe { FromGlibPtrContainer::from_glib_none(ffi::g_content_type_get_mime_dirs()) }
}

#[doc(alias = "g_content_type_get_mime_type")]
pub fn content_type_get_mime_type(type_: &str) -> Option<glib::GString> {
    unsafe { from_glib_full(ffi::g_content_type_get_mime_type(type_.to_glib_none().0)) }
}

#[doc(alias = "g_content_type_get_symbolic_icon")]
pub fn content_type_get_symbolic_icon(type_: &str) -> Icon {
    unsafe {
        from_glib_full(ffi::g_content_type_get_symbolic_icon(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_guess")]
pub fn content_type_guess(filename: Option<&str>, data: &[u8]) -> (glib::GString, bool) {
    let data_size = data.len() as usize;
    unsafe {
        let mut result_uncertain = mem::MaybeUninit::uninit();
        let ret = from_glib_full(ffi::g_content_type_guess(
            filename.to_glib_none().0,
            data.to_glib_none().0,
            data_size,
            result_uncertain.as_mut_ptr(),
        ));
        let result_uncertain = result_uncertain.assume_init();
        (ret, from_glib(result_uncertain))
    }
}

#[doc(alias = "g_content_type_guess_for_tree")]
pub fn content_type_guess_for_tree<P: IsA<File>>(root: &P) -> Vec<glib::GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_content_type_guess_for_tree(
            root.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_is_a")]
pub fn content_type_is_a(type_: &str, supertype: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_a(
            type_.to_glib_none().0,
            supertype.to_glib_none().0,
        ))
    }
}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "g_content_type_is_mime_type")]
pub fn content_type_is_mime_type(type_: &str, mime_type: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_mime_type(
            type_.to_glib_none().0,
            mime_type.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_is_unknown")]
pub fn content_type_is_unknown(type_: &str) -> bool {
    unsafe { from_glib(ffi::g_content_type_is_unknown(type_.to_glib_none().0)) }
}

#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
#[doc(alias = "g_content_type_set_mime_dirs")]
pub fn content_type_set_mime_dirs(dirs: &[&str]) {
    unsafe {
        ffi::g_content_type_set_mime_dirs(dirs.to_glib_none().0);
    }
}

#[doc(alias = "g_content_types_get_registered")]
pub fn content_types_get_registered() -> Vec<glib::GString> {
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::g_content_types_get_registered()) }
}

#[doc(alias = "g_dbus_address_escape_value")]
pub fn dbus_address_escape_value(string: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::g_dbus_address_escape_value(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_address_get_for_bus_sync")]
pub fn dbus_address_get_for_bus_sync<P: IsA<Cancellable>>(
    bus_type: BusType,
    cancellable: Option<&P>,
) -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_dbus_address_get_for_bus_sync(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_dbus_address_get_stream")]
pub fn dbus_address_get_stream<
    P: IsA<Cancellable>,
    Q: FnOnce(Result<(IOStream, Option<glib::GString>), glib::Error>) + Send + 'static,
>(
    address: &str,
    cancellable: Option<&P>,
    callback: Q,
) {
    let user_data: Box_<Q> = Box_::new(callback);
    unsafe extern "C" fn dbus_address_get_stream_trampoline<
        Q: FnOnce(Result<(IOStream, Option<glib::GString>), glib::Error>) + Send + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut crate::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let mut out_guid = ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_finish(res, &mut out_guid, &mut error);
        let result = if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
        callback(result);
    }
    let callback = dbus_address_get_stream_trampoline::<Q>;
    unsafe {
        ffi::g_dbus_address_get_stream(
            address.to_glib_none().0,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

pub fn dbus_address_get_stream_future(
    address: &str,
) -> Pin<
    Box_<
        dyn std::future::Future<Output = Result<(IOStream, Option<glib::GString>), glib::Error>>
            + 'static,
    >,
> {
    let address = String::from(address);
    Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
        let cancellable = Cancellable::new();
        dbus_address_get_stream(&address, Some(&cancellable), move |res| {
            send.resolve(res);
        });

        cancellable
    }))
}

#[doc(alias = "g_dbus_address_get_stream_sync")]
pub fn dbus_address_get_stream_sync<P: IsA<Cancellable>>(
    address: &str,
    cancellable: Option<&P>,
) -> Result<(IOStream, Option<glib::GString>), glib::Error> {
    unsafe {
        let mut out_guid = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_sync(
            address.to_glib_none().0,
            &mut out_guid,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_dbus_generate_guid")]
pub fn dbus_generate_guid() -> glib::GString {
    unsafe { from_glib_full(ffi::g_dbus_generate_guid()) }
}

#[doc(alias = "g_dbus_gvalue_to_gvariant")]
pub fn dbus_gvalue_to_gvariant(gvalue: &glib::Value, type_: &glib::VariantTy) -> glib::Variant {
    unsafe {
        from_glib_full(ffi::g_dbus_gvalue_to_gvariant(
            gvalue.to_glib_none().0,
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_dbus_gvariant_to_gvalue")]
pub fn dbus_gvariant_to_gvalue(value: &glib::Variant) -> glib::Value {
    unsafe {
        let mut out_gvalue = glib::Value::uninitialized();
        ffi::g_dbus_gvariant_to_gvalue(value.to_glib_none().0, out_gvalue.to_glib_none_mut().0);
        out_gvalue
    }
}

#[doc(alias = "g_dbus_is_address")]
pub fn dbus_is_address(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_address(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_guid")]
pub fn dbus_is_guid(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_guid(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_interface_name")]
pub fn dbus_is_interface_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_interface_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_member_name")]
pub fn dbus_is_member_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_member_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_name")]
pub fn dbus_is_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_supported_address")]
pub fn dbus_is_supported_address(string: &str) -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::g_dbus_is_supported_address(string.to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_dbus_is_unique_name")]
pub fn dbus_is_unique_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_unique_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_io_error_from_errno")]
pub fn io_error_from_errno(err_no: i32) -> IOErrorEnum {
    unsafe { from_glib(ffi::g_io_error_from_errno(err_no)) }
}

//#[doc(alias = "g_io_modules_load_all_in_directory")]
//pub fn io_modules_load_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi:g_io_modules_load_all_in_directory() }
//}

//#[doc(alias = "g_io_modules_load_all_in_directory_with_scope")]
//pub fn io_modules_load_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi:g_io_modules_load_all_in_directory_with_scope() }
//}

#[doc(alias = "g_io_modules_scan_all_in_directory")]
pub fn io_modules_scan_all_in_directory<P: AsRef<std::path::Path>>(dirname: P) {
    unsafe {
        ffi::g_io_modules_scan_all_in_directory(dirname.as_ref().to_glib_none().0);
    }
}

//#[doc(alias = "g_io_modules_scan_all_in_directory_with_scope")]
//pub fn io_modules_scan_all_in_directory_with_scope<P: AsRef<std::path::Path>>(dirname: P, scope: /*Ignored*/&mut IOModuleScope) {
//    unsafe { TODO: call ffi:g_io_modules_scan_all_in_directory_with_scope() }
//}

#[doc(alias = "g_io_scheduler_cancel_all_jobs")]
pub fn io_scheduler_cancel_all_jobs() {
    unsafe {
        ffi::g_io_scheduler_cancel_all_jobs();
    }
}

//#[doc(alias = "g_io_scheduler_push_job")]
//pub fn io_scheduler_push_job<P: IsA<Cancellable>>(job_func: /*Unimplemented*/Fn(/*Ignored*/IOSchedulerJob, Option<&Cancellable>) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, io_priority: i32, cancellable: Option<&P>) {
//    unsafe { TODO: call ffi:g_io_scheduler_push_job() }
//}

#[doc(alias = "g_keyfile_settings_backend_new")]
pub fn keyfile_settings_backend_new(
    filename: &str,
    root_path: &str,
    root_group: Option<&str>,
) -> SettingsBackend {
    unsafe {
        from_glib_full(ffi::g_keyfile_settings_backend_new(
            filename.to_glib_none().0,
            root_path.to_glib_none().0,
            root_group.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_memory_settings_backend_new")]
pub fn memory_settings_backend_new() -> SettingsBackend {
    unsafe { from_glib_full(ffi::g_memory_settings_backend_new()) }
}

#[doc(alias = "g_networking_init")]
pub fn networking_init() {
    unsafe {
        ffi::g_networking_init();
    }
}

#[doc(alias = "g_null_settings_backend_new")]
pub fn null_settings_backend_new() -> SettingsBackend {
    unsafe { from_glib_full(ffi::g_null_settings_backend_new()) }
}

#[doc(alias = "g_resources_enumerate_children")]
pub fn resources_enumerate_children(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<Vec<glib::GString>, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_enumerate_children(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_get_info")]
pub fn resources_get_info(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<(usize, u32), glib::Error> {
    unsafe {
        let mut size = mem::MaybeUninit::uninit();
        let mut flags = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::g_resources_get_info(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            size.as_mut_ptr(),
            flags.as_mut_ptr(),
            &mut error,
        );
        let size = size.assume_init();
        let flags = flags.assume_init();
        if error.is_null() {
            Ok((size, flags))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_lookup_data")]
pub fn resources_lookup_data(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<glib::Bytes, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_lookup_data(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_open_stream")]
pub fn resources_open_stream(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<InputStream, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_resources_open_stream(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_register")]
pub fn resources_register(resource: &Resource) {
    unsafe {
        ffi::g_resources_register(resource.to_glib_none().0);
    }
}

#[doc(alias = "g_resources_unregister")]
pub fn resources_unregister(resource: &Resource) {
    unsafe {
        ffi::g_resources_unregister(resource.to_glib_none().0);
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[doc(alias = "g_unix_is_mount_path_system_internal")]
pub fn unix_is_mount_path_system_internal<P: AsRef<std::path::Path>>(mount_path: P) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_mount_path_system_internal(
            mount_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
#[doc(alias = "g_unix_is_system_device_path")]
pub fn unix_is_system_device_path<P: AsRef<std::path::Path>>(device_path: P) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_system_device_path(
            device_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
#[doc(alias = "g_unix_is_system_fs_type")]
pub fn unix_is_system_fs_type(fs_type: &str) -> bool {
    unsafe { from_glib(ffi::g_unix_is_system_fs_type(fs_type.to_glib_none().0)) }
}
