// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use DriveStartFlags;
use DriveStartStopType;
use Icon;
use MountOperation;
use MountUnmountFlags;
use Volume;

glib_wrapper! {
    pub struct Drive(Interface<gio_sys::GDrive>);

    match fn {
        get_type => || gio_sys::g_drive_get_type(),
    }
}

pub const NONE_DRIVE: Option<&Drive> = None;

pub trait DriveExt: 'static {
    fn can_eject(&self) -> bool;

    fn can_poll_for_media(&self) -> bool;

    fn can_start(&self) -> bool;

    fn can_start_degraded(&self) -> bool;

    fn can_stop(&self) -> bool;

    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn enumerate_identifiers(&self) -> Vec<GString>;

    fn get_icon(&self) -> Icon;

    fn get_identifier(&self, kind: &str) -> Option<GString>;

    fn get_name(&self) -> GString;

    fn get_sort_key(&self) -> Option<GString>;

    fn get_start_stop_type(&self) -> DriveStartStopType;

    fn get_symbolic_icon(&self) -> Icon;

    fn get_volumes(&self) -> Vec<Volume>;

    fn has_media(&self) -> bool;

    fn has_volumes(&self) -> bool;

    fn is_media_check_automatic(&self) -> bool;

    fn is_media_removable(&self) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn is_removable(&self) -> bool;

    fn poll_for_media<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn start<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn start_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn stop<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn stop_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Drive>> DriveExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(gio_sys::g_drive_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_poll_for_media(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_drive_can_poll_for_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_start(&self) -> bool {
        unsafe { from_glib(gio_sys::g_drive_can_start(self.as_ref().to_glib_none().0)) }
    }

    fn can_start_degraded(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_drive_can_start_degraded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_stop(&self) -> bool {
        unsafe { from_glib(gio_sys::g_drive_can_stop(self.as_ref().to_glib_none().0)) }
    }

    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn eject_with_operation_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_drive_eject_with_operation_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<R>;
        unsafe {
            gio_sys::g_drive_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.eject_with_operation(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn enumerate_identifiers(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_drive_enumerate_identifiers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_icon(&self) -> Icon {
        unsafe { from_glib_full(gio_sys::g_drive_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn get_identifier(&self, kind: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_drive_get_identifier(
                self.as_ref().to_glib_none().0,
                kind.to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> GString {
        unsafe { from_glib_full(gio_sys::g_drive_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_sort_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_drive_get_sort_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_start_stop_type(&self) -> DriveStartStopType {
        unsafe {
            from_glib(gio_sys::g_drive_get_start_stop_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(gio_sys::g_drive_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_volumes(&self) -> Vec<Volume> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_drive_get_volumes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_media(&self) -> bool {
        unsafe { from_glib(gio_sys::g_drive_has_media(self.as_ref().to_glib_none().0)) }
    }

    fn has_volumes(&self) -> bool {
        unsafe { from_glib(gio_sys::g_drive_has_volumes(self.as_ref().to_glib_none().0)) }
    }

    fn is_media_check_automatic(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_drive_is_media_check_automatic(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_media_removable(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_drive_is_media_removable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn is_removable(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_drive_is_removable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn poll_for_media<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn poll_for_media_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_drive_poll_for_media_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = poll_for_media_trampoline::<Q>;
        unsafe {
            gio_sys::g_drive_poll_for_media(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.poll_for_media(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn start<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn start_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_drive_start_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = start_trampoline::<R>;
        unsafe {
            gio_sys::g_drive_start(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn start_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.start(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn stop<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn stop_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_drive_stop_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = stop_trampoline::<R>;
        unsafe {
            gio_sys::g_drive_stop(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn stop_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.stop(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDrive,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Drive>,
        {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDrive,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Drive>,
        {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn eject_button_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDrive,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Drive>,
        {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"eject-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    eject_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_button_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDrive,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Drive>,
        {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Drive")
    }
}
