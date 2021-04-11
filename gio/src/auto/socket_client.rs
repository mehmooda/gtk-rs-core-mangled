// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::IOStream;
use crate::ProxyResolver;
use crate::SocketAddress;
use crate::SocketClientEvent;
use crate::SocketConnectable;
use crate::SocketConnection;
use crate::SocketFamily;
use crate::SocketProtocol;
use crate::SocketType;
use crate::TlsCertificateFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct SocketClient(Object<ffi::GSocketClient, ffi::GSocketClientClass>);

    match fn {
        get_type => || ffi::g_socket_client_get_type(),
    }
}

impl SocketClient {
    #[doc(alias = "g_socket_client_new")]
    pub fn new() -> SocketClient {
        unsafe { from_glib_full(ffi::g_socket_client_new()) }
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET_CLIENT: Option<&SocketClient> = None;

pub trait SocketClientExt: 'static {
    #[doc(alias = "g_socket_client_add_application_proxy")]
    fn add_application_proxy(&self, protocol: &str);

    #[doc(alias = "g_socket_client_connect")]
    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<SocketConnection, glib::Error>;

    #[doc(alias = "g_socket_client_connect_async")]
    fn connect_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>;

    #[doc(alias = "g_socket_client_connect_to_host")]
    fn connect_to_host<P: IsA<Cancellable>>(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error>;

    #[doc(alias = "g_socket_client_connect_to_host_async")]
    fn connect_to_host_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn connect_to_host_async_future(
        &self,
        host_and_port: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>;

    #[doc(alias = "g_socket_client_connect_to_service")]
    fn connect_to_service<P: IsA<Cancellable>>(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error>;

    #[doc(alias = "g_socket_client_connect_to_service_async")]
    fn connect_to_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn connect_to_service_async_future(
        &self,
        domain: &str,
        service: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>;

    #[doc(alias = "g_socket_client_connect_to_uri")]
    fn connect_to_uri<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error>;

    #[doc(alias = "g_socket_client_connect_to_uri_async")]
    fn connect_to_uri_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn connect_to_uri_async_future(
        &self,
        uri: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>;

    #[doc(alias = "g_socket_client_get_enable_proxy")]
    fn enables_proxy(&self) -> bool;

    #[doc(alias = "g_socket_client_get_family")]
    fn family(&self) -> SocketFamily;

    #[doc(alias = "g_socket_client_get_local_address")]
    fn local_address(&self) -> Option<SocketAddress>;

    #[doc(alias = "g_socket_client_get_protocol")]
    fn protocol(&self) -> SocketProtocol;

    #[doc(alias = "g_socket_client_get_proxy_resolver")]
    fn proxy_resolver(&self) -> ProxyResolver;

    #[doc(alias = "g_socket_client_get_socket_type")]
    fn socket_type(&self) -> SocketType;

    #[doc(alias = "g_socket_client_get_timeout")]
    fn timeout(&self) -> u32;

    #[doc(alias = "g_socket_client_get_tls")]
    fn is_tls(&self) -> bool;

    #[doc(alias = "g_socket_client_get_tls_validation_flags")]
    fn tls_validation_flags(&self) -> TlsCertificateFlags;

    #[doc(alias = "g_socket_client_set_enable_proxy")]
    fn set_enable_proxy(&self, enable: bool);

    #[doc(alias = "g_socket_client_set_family")]
    fn set_family(&self, family: SocketFamily);

    #[doc(alias = "g_socket_client_set_local_address")]
    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>);

    #[doc(alias = "g_socket_client_set_protocol")]
    fn set_protocol(&self, protocol: SocketProtocol);

    #[doc(alias = "g_socket_client_set_proxy_resolver")]
    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>);

    #[doc(alias = "g_socket_client_set_socket_type")]
    fn set_socket_type(&self, type_: SocketType);

    #[doc(alias = "g_socket_client_set_timeout")]
    fn set_timeout(&self, timeout: u32);

    #[doc(alias = "g_socket_client_set_tls")]
    fn set_tls(&self, tls: bool);

    #[doc(alias = "g_socket_client_set_tls_validation_flags")]
    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags);

    #[doc(alias = "get_property_type")]
    fn type_(&self) -> SocketType;

    #[doc(alias = "set_property_type")]
    fn set_type(&self, type_: SocketType);

    fn connect_event<
        F: Fn(&Self, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketClient>> SocketClientExt for O {
    fn add_application_proxy(&self, protocol: &str) {
        unsafe {
            ffi::g_socket_client_add_application_proxy(
                self.as_ref().to_glib_none().0,
                protocol.to_glib_none().0,
            );
        }
    }

    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
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

    fn connect_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn connect_async_trampoline<
            R: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            ffi::g_socket_client_connect_async(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let connectable = connectable.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.connect_async(&connectable, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn connect_to_host<P: IsA<Cancellable>>(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
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

    fn connect_to_host_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn connect_to_host_async_trampoline<
            Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_host_async(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_host_async_future(
        &self,
        host_and_port: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let host_and_port = String::from(host_and_port);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.connect_to_host_async(
                &host_and_port,
                default_port,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn connect_to_service<P: IsA<Cancellable>>(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
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

    fn connect_to_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn connect_to_service_async_trampoline<
            Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_service_async(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_service_async_future(
        &self,
        domain: &str,
        service: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let domain = String::from(domain);
        let service = String::from(service);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.connect_to_service_async(&domain, &service, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn connect_to_uri<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
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

    fn connect_to_uri_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn connect_to_uri_async_trampoline<
            Q: FnOnce(Result<SocketConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_uri_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_uri_async_future(
        &self,
        uri: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let uri = String::from(uri);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.connect_to_uri_async(&uri, default_port, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn enables_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_enable_proxy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_client_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn local_address(&self) -> Option<SocketAddress> {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_local_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(ffi::g_socket_client_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn proxy_resolver(&self) -> ProxyResolver {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_proxy_resolver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_client_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timeout(&self) -> u32 {
        unsafe { ffi::g_socket_client_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn is_tls(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_client_get_tls(self.as_ref().to_glib_none().0)) }
    }

    fn tls_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls_validation_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_enable_proxy(&self, enable: bool) {
        unsafe {
            ffi::g_socket_client_set_enable_proxy(self.as_ref().to_glib_none().0, enable.to_glib());
        }
    }

    fn set_family(&self, family: SocketFamily) {
        unsafe {
            ffi::g_socket_client_set_family(self.as_ref().to_glib_none().0, family.to_glib());
        }
    }

    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>) {
        unsafe {
            ffi::g_socket_client_set_local_address(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_protocol(&self, protocol: SocketProtocol) {
        unsafe {
            ffi::g_socket_client_set_protocol(self.as_ref().to_glib_none().0, protocol.to_glib());
        }
    }

    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>) {
        unsafe {
            ffi::g_socket_client_set_proxy_resolver(
                self.as_ref().to_glib_none().0,
                proxy_resolver.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_socket_type(&self, type_: SocketType) {
        unsafe {
            ffi::g_socket_client_set_socket_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_client_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_tls(&self, tls: bool) {
        unsafe {
            ffi::g_socket_client_set_tls(self.as_ref().to_glib_none().0, tls.to_glib());
        }
    }

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_socket_client_set_tls_validation_flags(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    fn type_(&self) -> SocketType {
        unsafe {
            let mut value = glib::Value::from_type(<SocketType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
                .unwrap()
        }
    }

    fn set_type(&self, type_: SocketType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"type\0".as_ptr() as *const _,
                glib::Value::from(&type_).to_glib_none().0,
            );
        }
    }

    fn connect_event<
        F: Fn(&Self, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P,
            F: Fn(&P, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            event: ffi::GSocketClientEvent,
            connectable: *mut ffi::GSocketConnectable,
            connection: *mut ffi::GIOStream,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SocketClient::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(event),
                &from_glib_borrow(connectable),
                Option::<IOStream>::from_glib_borrow(connection)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_proxy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_proxy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocol_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocol\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_protocol_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_resolver_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-resolver\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_resolver_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tls_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_validation_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls-validation-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tls_validation_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketClient")
    }
}
