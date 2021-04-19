// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::InetAddress;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
use crate::ResolverNameLookupFlags;
use crate::ResolverRecordType;
use crate::SrvTarget;
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
    pub struct Resolver(Object<ffi::GResolver, ffi::GResolverClass>);

    match fn {
        type_ => || ffi::g_resolver_get_type(),
    }
}

impl Resolver {
    //#[doc(alias = "g_resolver_free_addresses")]
    //pub fn free_addresses(addresses: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi:g_resolver_free_addresses() }
    //}

    //#[doc(alias = "g_resolver_free_targets")]
    //pub fn free_targets(targets: /*Unimplemented*/&[&Fundamental: Pointer]) {
    //    unsafe { TODO: call ffi:g_resolver_free_targets() }
    //}

    #[doc(alias = "g_resolver_get_default")]
    pub fn default() -> Resolver {
        unsafe { from_glib_full(ffi::g_resolver_get_default()) }
    }
}

pub const NONE_RESOLVER: Option<&Resolver> = None;

pub trait ResolverExt: 'static {
    #[doc(alias = "g_resolver_lookup_by_address")]
    fn lookup_by_address<P: IsA<InetAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "g_resolver_lookup_by_address_async")]
    fn lookup_by_address_async<
        P: IsA<InetAddress>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<glib::GString, glib::Error>) + Send + 'static,
    >(
        &self,
        address: &P,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn lookup_by_address_async_future<P: IsA<InetAddress> + Clone + 'static>(
        &self,
        address: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>>;

    #[doc(alias = "g_resolver_lookup_by_name")]
    fn lookup_by_name<P: IsA<Cancellable>>(
        &self,
        hostname: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<InetAddress>, glib::Error>;

    #[doc(alias = "g_resolver_lookup_by_name_async")]
    fn lookup_by_name_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
    >(
        &self,
        hostname: &str,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn lookup_by_name_async_future(
        &self,
        hostname: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_resolver_lookup_by_name_with_flags")]
    fn lookup_by_name_with_flags<P: IsA<Cancellable>>(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&P>,
    ) -> Result<Vec<InetAddress>, glib::Error>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_resolver_lookup_by_name_with_flags_async")]
    fn lookup_by_name_with_flags_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
    >(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn lookup_by_name_with_flags_async_future(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>;

    #[doc(alias = "g_resolver_lookup_records")]
    fn lookup_records<P: IsA<Cancellable>>(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::Variant>, glib::Error>;

    #[doc(alias = "g_resolver_lookup_records_async")]
    fn lookup_records_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::Variant>, glib::Error>) + Send + 'static,
    >(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn lookup_records_async_future(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::Variant>, glib::Error>> + 'static>,
    >;

    #[doc(alias = "g_resolver_lookup_service")]
    fn lookup_service<P: IsA<Cancellable>>(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<SrvTarget>, glib::Error>;

    #[doc(alias = "g_resolver_lookup_service_async")]
    fn lookup_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<SrvTarget>, glib::Error>) + Send + 'static,
    >(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn lookup_service_async_future(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<SrvTarget>, glib::Error>> + 'static>>;

    #[doc(alias = "g_resolver_set_default")]
    fn set_default(&self);

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Resolver>> ResolverExt for O {
    fn lookup_by_address<P: IsA<InetAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_address(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
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

    fn lookup_by_address_async<
        P: IsA<InetAddress>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<glib::GString, glib::Error>) + Send + 'static,
    >(
        &self,
        address: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn lookup_by_address_async_trampoline<
            R: FnOnce(Result<glib::GString, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_by_address_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_address_async_trampoline::<R>;
        unsafe {
            ffi::g_resolver_lookup_by_address_async(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_by_address_async_future<P: IsA<InetAddress> + Clone + 'static>(
        &self,
        address: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>>
    {
        let address = address.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_by_address_async(&address, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn lookup_by_name<P: IsA<Cancellable>>(
        &self,
        hostname: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<InetAddress>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
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

    fn lookup_by_name_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
    >(
        &self,
        hostname: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lookup_by_name_async_trampoline<
            Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_by_name_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_name_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_by_name_async(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_by_name_async_future(
        &self,
        hostname: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>
    {
        let hostname = String::from(hostname);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_by_name_async(&hostname, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn lookup_by_name_with_flags<P: IsA<Cancellable>>(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&P>,
    ) -> Result<Vec<InetAddress>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name_with_flags(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                flags.to_glib(),
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

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn lookup_by_name_with_flags_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
    >(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lookup_by_name_with_flags_async_trampoline<
            Q: FnOnce(Result<Vec<InetAddress>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name_with_flags_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_name_with_flags_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_by_name_with_flags_async(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn lookup_by_name_with_flags_async_future(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>
    {
        let hostname = String::from(hostname);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_by_name_with_flags_async(&hostname, flags, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn lookup_records<P: IsA<Cancellable>>(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::Variant>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_records(
                self.as_ref().to_glib_none().0,
                rrname.to_glib_none().0,
                record_type.to_glib(),
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

    fn lookup_records_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::Variant>, glib::Error>) + Send + 'static,
    >(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lookup_records_async_trampoline<
            Q: FnOnce(Result<Vec<glib::Variant>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_records_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_records_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_records_async(
                self.as_ref().to_glib_none().0,
                rrname.to_glib_none().0,
                record_type.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_records_async_future(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::Variant>, glib::Error>> + 'static>,
    > {
        let rrname = String::from(rrname);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_records_async(&rrname, record_type, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn lookup_service<P: IsA<Cancellable>>(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&P>,
    ) -> Result<Vec<SrvTarget>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resolver_lookup_service(
                self.as_ref().to_glib_none().0,
                service.to_glib_none().0,
                protocol.to_glib_none().0,
                domain.to_glib_none().0,
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

    fn lookup_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<SrvTarget>, glib::Error>) + Send + 'static,
    >(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn lookup_service_async_trampoline<
            Q: FnOnce(Result<Vec<SrvTarget>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_service_async(
                self.as_ref().to_glib_none().0,
                service.to_glib_none().0,
                protocol.to_glib_none().0,
                domain.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_service_async_future(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<SrvTarget>, glib::Error>> + 'static>>
    {
        let service = String::from(service);
        let protocol = String::from(protocol);
        let domain = String::from(domain);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.lookup_service_async(
                &service,
                &protocol,
                &domain,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_resolver_set_default(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reload_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GResolver,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Resolver>,
        {
            let f: &F = &*(f as *const F);
            f(&Resolver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reload\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    reload_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Resolver")
    }
}
