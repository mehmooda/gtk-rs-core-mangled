// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use crate::FilterInputStream;
use crate::InputStream;
use crate::PollableInputStream;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct ConverterInputStream(Object<ffi::GConverterInputStream, ffi::GConverterInputStreamClass>) @extends FilterInputStream, InputStream, @implements PollableInputStream;

    match fn {
        get_type => || ffi::g_converter_input_stream_get_type(),
    }
}

impl ConverterInputStream {
    pub fn new<P: IsA<InputStream>, Q: IsA<Converter>>(
        base_stream: &P,
        converter: &Q,
    ) -> ConverterInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_converter_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ConverterInputStreamBuilder {
    converter: Option<Converter>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl ConverterInputStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ConverterInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref converter) = self.converter {
            properties.push(("converter", converter));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        let ret = glib::Object::new(ConverterInputStream::static_type(), &properties)
            .expect("object new")
            .downcast::<ConverterInputStream>()
            .expect("downcast");
        ret
    }

    pub fn converter<P: IsA<Converter>>(mut self, converter: &P) -> Self {
        self.converter = Some(converter.clone().upcast());
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

pub const NONE_CONVERTER_INPUT_STREAM: Option<&ConverterInputStream> = None;

pub trait ConverterInputStreamExt: 'static {
    fn get_converter(&self) -> Option<Converter>;
}

impl<O: IsA<ConverterInputStream>> ConverterInputStreamExt for O {
    fn get_converter(&self) -> Option<Converter> {
        unsafe {
            from_glib_none(ffi::g_converter_input_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ConverterInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConverterInputStream")
    }
}
