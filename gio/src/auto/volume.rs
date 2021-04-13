// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::Drive;
use crate::File;
use crate::Icon;
use crate::Mount;
use crate::MountMountFlags;
use crate::MountOperation;
use crate::MountUnmountFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct Volume(Interface<ffi::GVolume, ffi::GVolumeIface>);

    match fn {
        get_type => || ffi::g_volume_get_type(),
    }
}

pub const NONE_VOLUME: Option<&Volume> = None;

pub trait VolumeExt: 'static {
    #[doc(alias = "g_volume_can_eject")]
    fn can_eject(&self) -> bool;

    #[doc(alias = "g_volume_can_mount")]
    fn can_mount(&self) -> bool;

    #[doc(alias = "g_volume_eject_with_operation")]
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

    #[doc(alias = "g_volume_enumerate_identifiers")]
    fn enumerate_identifiers(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_volume_get_activation_root")]
    fn activation_root(&self) -> Option<File>;

    #[doc(alias = "g_volume_get_drive")]
    fn drive(&self) -> Option<Drive>;

    #[doc(alias = "g_volume_get_icon")]
    fn icon(&self) -> Icon;

    #[doc(alias = "g_volume_get_identifier")]
    fn get_identifier(&self, kind: &str) -> Option<glib::GString>;

    #[doc(alias = "g_volume_get_mount")]
    fn get_mount(&self) -> Option<Mount>;

    #[doc(alias = "g_volume_get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_volume_get_sort_key")]
    fn sort_key(&self) -> Option<glib::GString>;

    #[doc(alias = "g_volume_get_symbolic_icon")]
    fn symbolic_icon(&self) -> Icon;

    #[doc(alias = "g_volume_get_uuid")]
    fn uuid(&self) -> Option<glib::GString>;

    #[doc(alias = "g_volume_mount")]
    fn mount<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn mount_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_volume_should_automount")]
    fn should_automount(&self) -> bool;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Volume>> VolumeExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_volume_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_mount(&self) -> bool {
        unsafe { from_glib(ffi::g_volume_can_mount(self.as_ref().to_glib_none().0)) }
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
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_eject_with_operation_finish(
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
            ffi::g_volume_eject_with_operation(
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

    fn enumerate_identifiers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_enumerate_identifiers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn activation_root(&self) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_volume_get_activation_root(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn drive(&self) -> Option<Drive> {
        unsafe { from_glib_full(ffi::g_volume_get_drive(self.as_ref().to_glib_none().0)) }
    }

    fn icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_volume_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn get_identifier(&self, kind: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_volume_get_identifier(
                self.as_ref().to_glib_none().0,
                kind.to_glib_none().0,
            ))
        }
    }

    fn get_mount(&self) -> Option<Mount> {
        unsafe { from_glib_full(ffi::g_volume_get_mount(self.as_ref().to_glib_none().0)) }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_volume_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_volume_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    fn symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_volume_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uuid(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::g_volume_get_uuid(self.as_ref().to_glib_none().0)) }
    }

    fn mount<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn mount_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_mount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = mount_trampoline::<R>;
        unsafe {
            ffi::g_volume_mount(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn mount_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.mount(
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

    fn should_automount(&self) -> bool {
        unsafe {
            from_glib(ffi::g_volume_should_automount(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Volume>,
        {
            let f: &F = &*(f as *const F);
            f(&Volume::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Volume>,
        {
            let f: &F = &*(f as *const F);
            f(&Volume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Volume")
    }
}
