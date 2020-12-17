// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ActionGroup;
use crate::ActionMap;
use crate::ApplicationCommandLine;
use crate::ApplicationFlags;
use crate::Cancellable;
use crate::DBusConnection;
use crate::File;
use crate::Notification;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Application(Object<ffi::GApplication, ffi::GApplicationClass>) @implements ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_application_get_type(),
    }
}

impl Application {
    #[doc(alias = "g_application_new")]
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Application {
        unsafe {
            from_glib_full(ffi::g_application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    #[doc(alias = "g_application_get_default")]
    pub fn get_default() -> Option<Application> {
        unsafe { from_glib_none(ffi::g_application_get_default()) }
    }

    #[doc(alias = "g_application_id_is_valid")]
    pub fn id_is_valid(application_id: &str) -> bool {
        unsafe {
            from_glib(ffi::g_application_id_is_valid(
                application_id.to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct ApplicationBuilder {
    action_group: Option<ActionGroup>,
    application_id: Option<String>,
    flags: Option<ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        let ret = glib::Object::new::<Application>(&properties).expect("object new");
        ret
    }

    pub fn action_group<P: IsA<ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

pub trait ApplicationExt: 'static {
    #[doc(alias = "g_application_activate")]
    fn activate(&self);

    #[doc(alias = "g_application_add_main_option")]
    fn add_main_option(
        &self,
        long_name: &str,
        short_name: glib::Char,
        flags: glib::OptionFlags,
        arg: glib::OptionArg,
        description: &str,
        arg_description: Option<&str>,
    );

    //#[doc(alias = "g_application_add_main_option_entries")]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]);

    //#[doc(alias = "g_application_add_option_group")]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_application_bind_busy_property")]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    #[doc(alias = "g_application_get_application_id")]
    fn get_application_id(&self) -> Option<glib::GString>;

    #[doc(alias = "g_application_get_dbus_connection")]
    fn get_dbus_connection(&self) -> Option<DBusConnection>;

    #[doc(alias = "g_application_get_dbus_object_path")]
    fn get_dbus_object_path(&self) -> Option<glib::GString>;

    #[doc(alias = "g_application_get_flags")]
    fn get_flags(&self) -> ApplicationFlags;

    #[doc(alias = "g_application_get_inactivity_timeout")]
    fn get_inactivity_timeout(&self) -> u32;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_application_get_is_busy")]
    fn get_is_busy(&self) -> bool;

    #[doc(alias = "g_application_get_is_registered")]
    fn get_is_registered(&self) -> bool;

    #[doc(alias = "g_application_get_is_remote")]
    fn get_is_remote(&self) -> bool;

    #[doc(alias = "g_application_get_resource_base_path")]
    fn get_resource_base_path(&self) -> Option<glib::GString>;

    #[doc(alias = "g_application_hold")]
    fn hold(&self);

    #[doc(alias = "g_application_mark_busy")]
    fn mark_busy(&self);

    #[doc(alias = "g_application_open")]
    fn open(&self, files: &[File], hint: &str);

    #[doc(alias = "g_application_quit")]
    fn quit(&self);

    #[doc(alias = "g_application_register")]
    fn register<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    #[doc(alias = "g_application_release")]
    fn release(&self);

    #[doc(alias = "g_application_send_notification")]
    fn send_notification(&self, id: Option<&str>, notification: &Notification);

    #[doc(alias = "g_application_set_application_id")]
    fn set_application_id(&self, application_id: Option<&str>);

    #[doc(alias = "g_application_set_default")]
    fn set_default(&self);

    #[doc(alias = "g_application_set_flags")]
    fn set_flags(&self, flags: ApplicationFlags);

    #[doc(alias = "g_application_set_inactivity_timeout")]
    fn set_inactivity_timeout(&self, inactivity_timeout: u32);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_application_set_option_context_description")]
    fn set_option_context_description(&self, description: Option<&str>);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_application_set_option_context_parameter_string")]
    fn set_option_context_parameter_string(&self, parameter_string: Option<&str>);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_application_set_option_context_summary")]
    fn set_option_context_summary(&self, summary: Option<&str>);

    #[doc(alias = "g_application_set_resource_base_path")]
    fn set_resource_base_path(&self, resource_path: Option<&str>);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_application_unbind_busy_property")]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    #[doc(alias = "g_application_unmark_busy")]
    fn unmark_busy(&self);

    #[doc(alias = "g_application_withdraw_notification")]
    fn withdraw_notification(&self, id: &str);

    fn set_property_action_group<P: IsA<ActionGroup>>(&self, action_group: Option<&P>);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_handle_local_options<F: Fn(&Self, &glib::VariantDict) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn connect_name_lost<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Application>> ApplicationExt for O {
    fn activate(&self) {
        unsafe {
            ffi::g_application_activate(self.as_ref().to_glib_none().0);
        }
    }

    fn add_main_option(
        &self,
        long_name: &str,
        short_name: glib::Char,
        flags: glib::OptionFlags,
        arg: glib::OptionArg,
        description: &str,
        arg_description: Option<&str>,
    ) {
        unsafe {
            ffi::g_application_add_main_option(
                self.as_ref().to_glib_none().0,
                long_name.to_glib_none().0,
                short_name.to_glib(),
                flags.to_glib(),
                arg.to_glib(),
                description.to_glib_none().0,
                arg_description.to_glib_none().0,
            );
        }
    }

    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]) {
    //    unsafe { TODO: call ffi:g_application_add_main_option_entries() }
    //}

    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup) {
    //    unsafe { TODO: call ffi:g_application_add_option_group() }
    //}

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_bind_busy_property(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            );
        }
    }

    fn get_application_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_application_get_application_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dbus_connection(&self) -> Option<DBusConnection> {
        unsafe {
            from_glib_none(ffi::g_application_get_dbus_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dbus_object_path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_application_get_dbus_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_flags(&self) -> ApplicationFlags {
        unsafe { from_glib(ffi::g_application_get_flags(self.as_ref().to_glib_none().0)) }
    }

    fn get_inactivity_timeout(&self) -> u32 {
        unsafe { ffi::g_application_get_inactivity_timeout(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn get_is_busy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_busy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_registered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_registered(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_remote(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_resource_base_path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_application_get_resource_base_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hold(&self) {
        unsafe {
            ffi::g_application_hold(self.as_ref().to_glib_none().0);
        }
    }

    fn mark_busy(&self) {
        unsafe {
            ffi::g_application_mark_busy(self.as_ref().to_glib_none().0);
        }
    }

    fn open(&self, files: &[File], hint: &str) {
        let n_files = files.len() as i32;
        unsafe {
            ffi::g_application_open(
                self.as_ref().to_glib_none().0,
                files.to_glib_none().0,
                n_files,
                hint.to_glib_none().0,
            );
        }
    }

    fn quit(&self) {
        unsafe {
            ffi::g_application_quit(self.as_ref().to_glib_none().0);
        }
    }

    fn register<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_application_register(
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

    fn release(&self) {
        unsafe {
            ffi::g_application_release(self.as_ref().to_glib_none().0);
        }
    }

    fn send_notification(&self, id: Option<&str>, notification: &Notification) {
        unsafe {
            ffi::g_application_send_notification(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                notification.to_glib_none().0,
            );
        }
    }

    fn set_application_id(&self, application_id: Option<&str>) {
        unsafe {
            ffi::g_application_set_application_id(
                self.as_ref().to_glib_none().0,
                application_id.to_glib_none().0,
            );
        }
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_application_set_default(self.as_ref().to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: ApplicationFlags) {
        unsafe {
            ffi::g_application_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn set_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            ffi::g_application_set_inactivity_timeout(
                self.as_ref().to_glib_none().0,
                inactivity_timeout,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_option_context_description(&self, description: Option<&str>) {
        unsafe {
            ffi::g_application_set_option_context_description(
                self.as_ref().to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_option_context_parameter_string(&self, parameter_string: Option<&str>) {
        unsafe {
            ffi::g_application_set_option_context_parameter_string(
                self.as_ref().to_glib_none().0,
                parameter_string.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_option_context_summary(&self, summary: Option<&str>) {
        unsafe {
            ffi::g_application_set_option_context_summary(
                self.as_ref().to_glib_none().0,
                summary.to_glib_none().0,
            );
        }
    }

    fn set_resource_base_path(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::g_application_set_resource_base_path(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_unbind_busy_property(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            );
        }
    }

    fn unmark_busy(&self) {
        unsafe {
            ffi::g_application_unmark_busy(self.as_ref().to_glib_none().0);
        }
    }

    fn withdraw_notification(&self, id: &str) {
        unsafe {
            ffi::g_application_withdraw_notification(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            );
        }
    }

    fn set_property_action_group<P: IsA<ActionGroup>>(&self, action_group: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"action-group\0".as_ptr() as *const _,
                glib::Value::from(action_group).to_glib_none().0,
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn command_line_trampoline<
            P,
            F: Fn(&P, &ApplicationCommandLine) -> i32 + 'static,
        >(
            this: *mut ffi::GApplication,
            command_line: *mut ffi::GApplicationCommandLine,
            f: glib::ffi::gpointer,
        ) -> libc::c_int
        where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(command_line),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"command-line\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    command_line_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_handle_local_options<F: Fn(&Self, &glib::VariantDict) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn handle_local_options_trampoline<
            P,
            F: Fn(&P, &glib::VariantDict) -> i32 + 'static,
        >(
            this: *mut ffi::GApplication,
            options: *mut glib::ffi::GVariantDict,
            f: glib::ffi::gpointer,
        ) -> libc::c_int
        where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(options),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"handle-local-options\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    handle_local_options_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn connect_name_lost<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn name_lost_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut ffi::GApplication,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"name-lost\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    name_lost_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn shutdown_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"shutdown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    shutdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn startup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"startup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    startup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_group_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action-group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_group_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_application_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::application-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_application_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inactivity_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inactivity-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inactivity_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_busy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-busy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_busy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_registered_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-registered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_registered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_remote_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-remote\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_remote_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_base_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resource-base-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resource_base_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Application")
    }
}
