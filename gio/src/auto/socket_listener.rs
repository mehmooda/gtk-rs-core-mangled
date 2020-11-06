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
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use Socket;
use SocketAddress;
use SocketConnection;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use SocketListenerEvent;
use SocketProtocol;
use SocketType;

glib_wrapper! {
    pub struct SocketListener(Object<gio_sys::GSocketListener, gio_sys::GSocketListenerClass>);

    match fn {
        get_type => || gio_sys::g_socket_listener_get_type(),
    }
}

impl SocketListener {
    pub fn new() -> SocketListener {
        unsafe { from_glib_full(gio_sys::g_socket_listener_new()) }
    }
}

impl Default for SocketListener {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET_LISTENER: Option<&SocketListener> = None;

pub trait SocketListenerExt: 'static {
    fn accept<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(SocketConnection, Option<glib::Object>), glib::Error>;

    fn accept_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn accept_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    >;

    fn accept_socket<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(Socket, Option<glib::Object>), glib::Error>;

    fn accept_socket_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn accept_socket_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    >;

    fn add_address<P: IsA<SocketAddress>, Q: IsA<glib::Object>>(
        &self,
        address: &P,
        type_: SocketType,
        protocol: SocketProtocol,
        source_object: Option<&Q>,
    ) -> Result<SocketAddress, glib::Error>;

    fn add_any_inet_port<P: IsA<glib::Object>>(
        &self,
        source_object: Option<&P>,
    ) -> Result<u16, glib::Error>;

    fn add_inet_port<P: IsA<glib::Object>>(
        &self,
        port: u16,
        source_object: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn add_socket<P: IsA<Socket>, Q: IsA<glib::Object>>(
        &self,
        socket: &P,
        source_object: Option<&Q>,
    ) -> Result<(), glib::Error>;

    fn close(&self);

    fn set_backlog(&self, listen_backlog: i32);

    fn get_property_listen_backlog(&self) -> i32;

    fn set_property_listen_backlog(&self, listen_backlog: i32);

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<SocketListener>> SocketListenerExt for O {
    fn accept<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(SocketConnection, Option<glib::Object>), glib::Error> {
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_accept(
                self.as_ref().to_glib_none().0,
                &mut source_object,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn accept_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn accept_async_trampoline<
            Q: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_accept_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_listener_accept_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    > {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.accept_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn accept_socket<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(Socket, Option<glib::Object>), glib::Error> {
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_accept_socket(
                self.as_ref().to_glib_none().0,
                &mut source_object,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn accept_socket_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn accept_socket_async_trampoline<
            Q: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_accept_socket_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_socket_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_listener_accept_socket_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_socket_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    > {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.accept_socket_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn add_address<P: IsA<SocketAddress>, Q: IsA<glib::Object>>(
        &self,
        address: &P,
        type_: SocketType,
        protocol: SocketProtocol,
        source_object: Option<&Q>,
    ) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut effective_address = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_listener_add_address(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                type_.to_glib(),
                protocol.to_glib(),
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut effective_address,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(effective_address))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_any_inet_port<P: IsA<glib::Object>>(
        &self,
        source_object: Option<&P>,
    ) -> Result<u16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_listener_add_any_inet_port(
                self.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_inet_port<P: IsA<glib::Object>>(
        &self,
        port: u16,
        source_object: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_listener_add_inet_port(
                self.as_ref().to_glib_none().0,
                port,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_socket<P: IsA<Socket>, Q: IsA<glib::Object>>(
        &self,
        socket: &P,
        source_object: Option<&Q>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_listener_add_socket(
                self.as_ref().to_glib_none().0,
                socket.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close(&self) {
        unsafe {
            gio_sys::g_socket_listener_close(self.as_ref().to_glib_none().0);
        }
    }

    fn set_backlog(&self, listen_backlog: i32) {
        unsafe {
            gio_sys::g_socket_listener_set_backlog(self.as_ref().to_glib_none().0, listen_backlog);
        }
    }

    fn get_property_listen_backlog(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"listen-backlog\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `listen-backlog` getter")
                .unwrap()
        }
    }

    fn set_property_listen_backlog(&self, listen_backlog: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"listen-backlog\0".as_ptr() as *const _,
                Value::from(&listen_backlog).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P,
            F: Fn(&P, SocketListenerEvent, &Socket) + 'static,
        >(
            this: *mut gio_sys::GSocketListener,
            event: gio_sys::GSocketListenerEvent,
            socket: *mut gio_sys::GSocket,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketListener>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SocketListener::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(event),
                &from_glib_borrow(socket),
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

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_backlog_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketListener,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketListener>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketListener::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::listen-backlog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_listen_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SocketListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketListener")
    }
}
