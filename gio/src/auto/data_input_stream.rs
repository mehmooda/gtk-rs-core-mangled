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
use glib::ToValue;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use BufferedInputStream;
use Cancellable;
use DataStreamByteOrder;
use DataStreamNewlineType;
use FilterInputStream;
use InputStream;
use Seekable;

glib_wrapper! {
    pub struct DataInputStream(Object<gio_sys::GDataInputStream, gio_sys::GDataInputStreamClass>) @extends BufferedInputStream, FilterInputStream, InputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_data_input_stream_get_type(),
    }
}

impl DataInputStream {
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> DataInputStream {
        unsafe {
            from_glib_full(gio_sys::g_data_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct DataInputStreamBuilder {
    byte_order: Option<DataStreamByteOrder>,
    newline_type: Option<DataStreamNewlineType>,
    buffer_size: Option<u32>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl DataInputStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DataInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref byte_order) = self.byte_order {
            properties.push(("byte-order", byte_order));
        }
        if let Some(ref newline_type) = self.newline_type {
            properties.push(("newline-type", newline_type));
        }
        if let Some(ref buffer_size) = self.buffer_size {
            properties.push(("buffer-size", buffer_size));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        let ret = glib::Object::new(DataInputStream::static_type(), &properties)
            .expect("object new")
            .downcast::<DataInputStream>()
            .expect("downcast");
        ret
    }

    pub fn byte_order(mut self, byte_order: DataStreamByteOrder) -> Self {
        self.byte_order = Some(byte_order);
        self
    }

    pub fn newline_type(mut self, newline_type: DataStreamNewlineType) -> Self {
        self.newline_type = Some(newline_type);
        self
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn base_stream<P: IsA<InputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_DATA_INPUT_STREAM: Option<&DataInputStream> = None;

pub trait DataInputStreamExt: 'static {
    fn get_byte_order(&self) -> DataStreamByteOrder;

    fn get_newline_type(&self) -> DataStreamNewlineType;

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u8, glib::Error>;

    fn read_int16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i16, glib::Error>;

    fn read_int32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, glib::Error>;

    fn read_int64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i64, glib::Error>;

    //fn read_line_finish_utf8(&self, result: /*Ignored*/&AsyncResult) -> Result<(Option<GString>, usize), glib::Error>;

    fn read_uint16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>)
        -> Result<u16, glib::Error>;

    fn read_uint32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>)
        -> Result<u32, glib::Error>;

    fn read_uint64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>)
        -> Result<u64, glib::Error>;

    fn set_byte_order(&self, order: DataStreamByteOrder);

    fn set_newline_type(&self, type_: DataStreamNewlineType);

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<DataInputStream>> DataInputStreamExt for O {
    fn get_byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(gio_sys::g_data_input_stream_get_byte_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_newline_type(&self) -> DataStreamNewlineType {
        unsafe {
            from_glib(gio_sys::g_data_input_stream_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u8, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_int16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_int32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_int64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_int64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn read_line_finish_utf8(&self, result: /*Ignored*/&AsyncResult) -> Result<(Option<GString>, usize), glib::Error> {
    //    unsafe { TODO: call gio_sys:g_data_input_stream_read_line_finish_utf8() }
    //}

    fn read_uint16<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<u16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_uint16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_uint32<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<u32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_uint32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_uint64<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<u64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_data_input_stream_read_uint64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            gio_sys::g_data_input_stream_set_byte_order(
                self.as_ref().to_glib_none().0,
                order.to_glib(),
            );
        }
    }

    fn set_newline_type(&self, type_: DataStreamNewlineType) {
        unsafe {
            gio_sys::g_data_input_stream_set_newline_type(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            );
        }
    }

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_byte_order_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDataInputStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DataInputStream>,
        {
            let f: &F = &*(f as *const F);
            f(&DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::byte-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_byte_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDataInputStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DataInputStream>,
        {
            let f: &F = &*(f as *const F);
            f(&DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DataInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataInputStream")
    }
}
