// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::glib_wrapper! {
    pub struct ProxyResolver(Interface<ffi::GProxyResolver>);

    match fn {
        get_type => || ffi::g_proxy_resolver_get_type(),
    }
}

impl ProxyResolver {
    #[doc(alias = "g_proxy_resolver_get_default")]
    pub fn get_default() -> Option<ProxyResolver> {
        unsafe { from_glib_none(ffi::g_proxy_resolver_get_default()) }
    }
}

pub const NONE_PROXY_RESOLVER: Option<&ProxyResolver> = None;

pub trait ProxyResolverExt: 'static {
    #[doc(alias = "g_proxy_resolver_is_supported")]
    fn is_supported(&self) -> bool;

    #[doc(alias = "g_proxy_resolver_lookup")]
    fn lookup<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::GString>, glib::Error>;

    #[doc(alias = "g_proxy_resolver_lookup_async")]
    fn lookup_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn lookup_async_future(
        &self,
        uri: &str,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    >;
}

impl<O: IsA<ProxyResolver>> ProxyResolverExt for O {
    fn is_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::g_proxy_resolver_is_supported(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lookup<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_resolver_lookup(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lookup_async_trampoline<
            Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_proxy_resolver_lookup_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_async_trampoline::<Q>;
        unsafe {
            ffi::g_proxy_resolver_lookup_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_async_future(
        &self,
        uri: &str,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    > {
        let uri = String::from(uri);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_async(&uri, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for ProxyResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProxyResolver")
    }
}
