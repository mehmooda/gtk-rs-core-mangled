// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct ActionGroup(Interface<ffi::GActionGroup>);

    match fn {
        get_type => || ffi::g_action_group_get_type(),
    }
}

pub const NONE_ACTION_GROUP: Option<&ActionGroup> = None;

pub trait ActionGroupExt: 'static {
    fn action_added(&self, action_name: &str);

    fn action_enabled_changed(&self, action_name: &str, enabled: bool);

    fn action_removed(&self, action_name: &str);

    fn action_state_changed(&self, action_name: &str, state: &glib::Variant);

    fn activate_action(&self, action_name: &str, parameter: Option<&glib::Variant>);

    fn change_action_state(&self, action_name: &str, value: &glib::Variant);

    fn get_action_enabled(&self, action_name: &str) -> bool;

    fn get_action_parameter_type(&self, action_name: &str) -> Option<glib::VariantType>;

    fn get_action_state(&self, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_hint(&self, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_type(&self, action_name: &str) -> Option<glib::VariantType>;

    fn has_action(&self, action_name: &str) -> bool;

    fn list_actions(&self) -> Vec<glib::GString>;

    fn connect_action_added<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_action_enabled_changed<F: Fn(&Self, &str, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_action_removed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_action_state_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ActionGroup>> ActionGroupExt for O {
    fn action_added(&self, action_name: &str) {
        unsafe {
            ffi::g_action_group_action_added(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    fn action_enabled_changed(&self, action_name: &str, enabled: bool) {
        unsafe {
            ffi::g_action_group_action_enabled_changed(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn action_removed(&self, action_name: &str) {
        unsafe {
            ffi::g_action_group_action_removed(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    fn action_state_changed(&self, action_name: &str, state: &glib::Variant) {
        unsafe {
            ffi::g_action_group_action_state_changed(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                state.to_glib_none().0,
            );
        }
    }

    fn activate_action(&self, action_name: &str, parameter: Option<&glib::Variant>) {
        unsafe {
            ffi::g_action_group_activate_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                parameter.to_glib_none().0,
            );
        }
    }

    fn change_action_state(&self, action_name: &str, value: &glib::Variant) {
        unsafe {
            ffi::g_action_group_change_action_state(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn get_action_enabled(&self, action_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_action_group_get_action_enabled(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn get_action_parameter_type(&self, action_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_group_get_action_parameter_type(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn get_action_state(&self, action_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_group_get_action_state(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn get_action_state_hint(&self, action_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_group_get_action_state_hint(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn get_action_state_type(&self, action_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_group_get_action_state_type(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn has_action(&self, action_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_action_group_has_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn list_actions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_action_group_list_actions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_action_added<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn action_added_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GActionGroup,
            action_name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(action_name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    action_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_action_enabled_changed<F: Fn(&Self, &str, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn action_enabled_changed_trampoline<P, F: Fn(&P, &str, bool) + 'static>(
            this: *mut ffi::GActionGroup,
            action_name: *mut libc::c_char,
            enabled: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(action_name),
                from_glib(enabled),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-enabled-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    action_enabled_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_action_removed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn action_removed_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GActionGroup,
            action_name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(action_name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    action_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_action_state_changed<F: Fn(&Self, &str, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn action_state_changed_trampoline<
            P,
            F: Fn(&P, &str, &glib::Variant) + 'static,
        >(
            this: *mut ffi::GActionGroup,
            action_name: *mut libc::c_char,
            value: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(action_name),
                &from_glib_borrow(value),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    action_state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionGroup")
    }
}
