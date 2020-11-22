// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use glib::{Cast, Error};

use crate::Cancellable;
use crate::IOStream;

use std::mem;
use std::ptr;

use once_cell::sync::Lazy;

pub trait IOStreamImpl: ObjectImpl + IOStreamImplExt + Send {
    fn get_input_stream(&self, stream: &Self::Type) -> crate::InputStream {
        self.parent_get_input_stream(stream)
    }

    fn get_output_stream(&self, stream: &Self::Type) -> crate::OutputStream {
        self.parent_get_output_stream(stream)
    }

    fn close(&self, stream: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(stream, cancellable)
    }
}

pub trait IOStreamImplExt: ObjectSubclass {
    fn parent_get_input_stream(&self, stream: &Self::Type) -> crate::InputStream;

    fn parent_get_output_stream(&self, stream: &Self::Type) -> crate::OutputStream;

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

impl<T: IOStreamImpl> IOStreamImplExt for T {
    fn parent_get_input_stream(&self, stream: &Self::Type) -> crate::InputStream {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let f = (*parent_class)
                .get_input_stream
                .expect("No parent class implementation for \"get_input_stream\"");
            from_glib_none(f(stream.unsafe_cast_ref::<IOStream>().to_glib_none().0))
        }
    }

    fn parent_get_output_stream(&self, stream: &Self::Type) -> crate::OutputStream {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let f = (*parent_class)
                .get_output_stream
                .expect("No parent class implementation for \"get_output_stream\"");
            from_glib_none(f(stream.unsafe_cast_ref::<IOStream>().to_glib_none().0))
        }
    }

    fn parent_close(
        &self,
        stream: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GIOStreamClass;
            let mut err = ptr::null_mut();
            if let Some(f) = (*parent_class).close_fn {
                if from_glib(f(
                    stream.unsafe_cast_ref::<IOStream>().to_glib_none().0,
                    cancellable.to_glib_none().0,
                    &mut err,
                )) {
                    Ok(())
                } else {
                    Err(from_glib_full(err))
                }
            } else {
                Ok(())
            }
        }
    }
}

unsafe impl<T: IOStreamImpl> IsSubclassable<T> for IOStream {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.get_input_stream = Some(stream_get_input_stream::<T>);
        klass.get_output_stream = Some(stream_get_output_stream::<T>);
        klass.close_fn = Some(stream_close::<T>);
    }
}

static OUTPUT_STREAM_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-rs-subclass-output-stream"));
static INPUT_STREAM_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-rs-subclass-input-stream"));

unsafe extern "C" fn stream_get_input_stream<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
) -> *mut ffi::GInputStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

    let ret = imp.get_input_stream(wrap.unsafe_cast_ref());

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    let old_ptr =
        glib::gobject_ffi::g_object_get_qdata(ptr as *mut _, INPUT_STREAM_QUARK.to_glib());
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *mut _,
            ret.as_ptr(),
            "Did not return same input stream again"
        );
    }

    unsafe extern "C" fn unref(ptr: glib::ffi::gpointer) {
        glib::gobject_ffi::g_object_unref(ptr as *mut _);
    }
    glib::gobject_ffi::g_object_set_qdata_full(
        ptr as *mut _,
        INPUT_STREAM_QUARK.to_glib(),
        glib::gobject_ffi::g_object_ref(ret.as_ptr() as *mut _) as *mut _,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn stream_get_output_stream<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
) -> *mut ffi::GOutputStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

    let ret = imp.get_output_stream(wrap.unsafe_cast_ref());

    // Ensure that a) the stream stays alive as long as the IO stream instance and
    // b) that the same stream is returned every time. This is a requirement by the
    // IO stream API.
    let old_ptr =
        glib::gobject_ffi::g_object_get_qdata(ptr as *mut _, OUTPUT_STREAM_QUARK.to_glib());
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *mut _,
            ret.as_ptr(),
            "Did not return same output stream again"
        );
    }

    unsafe extern "C" fn unref(ptr: glib::ffi::gpointer) {
        glib::gobject_ffi::g_object_unref(ptr as *mut _);
    }
    glib::gobject_ffi::g_object_set_qdata_full(
        ptr as *mut _,
        OUTPUT_STREAM_QUARK.to_glib(),
        glib::gobject_ffi::g_object_ref(ret.as_ptr() as *mut _) as *mut _,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn stream_close<T: IOStreamImpl>(
    ptr: *mut ffi::GIOStream,
    cancellable: *mut ffi::GCancellable,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<IOStream> = from_glib_borrow(ptr);

    match imp.close(
        wrap.unsafe_cast_ref(),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(e) => {
            let mut e = mem::ManuallyDrop::new(e);
            *err = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}
