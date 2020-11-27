// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use crate::File;
use crate::FileInfo;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::glib_wrapper! {
    pub struct FileEnumerator(Object<ffi::GFileEnumerator, ffi::GFileEnumeratorClass>);

    match fn {
        get_type => || ffi::g_file_enumerator_get_type(),
    }
}

pub const NONE_FILE_ENUMERATOR: Option<&FileEnumerator> = None;

pub trait FileEnumeratorExt: 'static {
    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn get_child(&self, info: &FileInfo) -> Option<File>;

    fn get_container(&self) -> Option<File>;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn next_file<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<FileInfo>, glib::Error>;

    fn next_files_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<FileInfo>, glib::Error>) + Send + 'static,
    >(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn next_files_async_future(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<FileInfo>, glib::Error>> + 'static>>;

    fn set_pending(&self, pending: bool);
}

impl<O: IsA<FileEnumerator>> FileEnumeratorExt for O {
    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_file_enumerator_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_file_enumerator_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_enumerator_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.close_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn get_child(&self, info: &FileInfo) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_file_enumerator_get_child(
                self.as_ref().to_glib_none().0,
                info.to_glib_none().0,
            ))
        }
    }

    fn get_container(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::g_file_enumerator_get_container(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_enumerator_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_enumerator_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next_file<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<FileInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_enumerator_next_file(
                self.as_ref().to_glib_none().0,
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

    fn next_files_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<FileInfo>, glib::Error>) + Send + 'static,
    >(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn next_files_async_trampoline<
            Q: FnOnce(Result<Vec<FileInfo>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_file_enumerator_next_files_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = next_files_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_enumerator_next_files_async(
                self.as_ref().to_glib_none().0,
                num_files,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn next_files_async_future(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<FileInfo>, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.next_files_async(num_files, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn set_pending(&self, pending: bool) {
        unsafe {
            ffi::g_file_enumerator_set_pending(self.as_ref().to_glib_none().0, pending.to_glib());
        }
    }
}

impl fmt::Display for FileEnumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileEnumerator")
    }
}
