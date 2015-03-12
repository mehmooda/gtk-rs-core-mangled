//! Translation between GLib/GLib-based FFI types and their Rust counterparts
//!
//! This module allows library bindings authors to decouple type translation
//! logic and use unified idioms at FFI boundaries. It also implements
//! translation of GLib core data types.
//!
//! `FromGlib`, `from_glib` and `ToGlib` translate simple types like `bool`.
//!
//! ```ignore
//!     pub fn set_accept_focus(&self, accept_focus: bool) {
//!         unsafe { ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
//!     }
//!
//!     pub fn get_accept_focus(&self) -> bool {
//!         unsafe { from_glib(ffi::gdk_window_get_accept_focus(self.pointer)) }
//!     }
//! ```
//!
//! `FromGlibPtr`, `FromGlibPtrNotNull` and `ToGlibPtr` work on `gpointer`s
//! and support different ways of managing ownership.
//!
//! ```ignore
//!     fn get_title(&self) -> Option<String> {
//!         unsafe {
//!             let title = ffi::gtk_window_get_title(self.pointer);
//!             FromGlibPtr::borrow(title)
//!         }
//!     }
//! ```
//!
//! Letting the foreign library borrow pointers from the Rust side often
//! requires having a temporary variable of an intermediate type (e.g. `CString`).
//! A `Stash` contains the temporary storage and a pointer into it that
//! is valid for the lifetime of the `Stash`. As the lifetime of the `Stash` returned
//! from `borrow_to_glib` is at least the enclosing statement, you can avoid explicitly
//! binding the stash in most cases and just take the pointer out of it:
//!
//! ```ignore
//!     pub fn set_icon_name(&self, name: &str) {
//!         unsafe {
//!             ffi::gdk_window_set_icon_name(self.pointer, name.borrow_to_glib().0)
//!         }
//!     }
//! ```

use std::iter::IntoIterator;
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use libc::{c_void, c_char};
use ffi;

/// Helper type that stores temporary values used for translation
///
/// `P` is the foreign type pointer and the first element of the tuple.
///
/// `T` is the Rust type that is translated.
///
/// The second element of the tuple is the temporary storage defined
/// by the implementation of `ToGlibPtr<P> for T`
///
/// Say you want to pass a `*mut C_GdkWindowAttr` to a foreign function. The `Stash`
/// will own a `C_GdkWindowAttr` and a `CString` that `C_GdkWindowAttr::title` points into.
///
/// ```ignore
/// impl <'a> ToGlibPtr<'a, *mut ffi::C_GdkWindowAttr> for WindowAttr {
///     type Storage = (Box<ffi::C_GdkWindowAttr>, Stash<'a, *const c_char, Option<String>>);
///
///     fn borrow_to_glib(&'a self) -> Stash<*mut ffi::C_GdkWindowAttr, WindowAttr> {
///         let title = self.title.borrow_to_glib();
///
///         let mut attrs = Box::new(ffi::C_GdkWindowAttr {
///             title: title.0,
///             // ....
///         });
///
///         Stash(&mut *attrs, (attrs, title))
///     }
/// }
/// ```
pub struct Stash<'a, P: Copy, T: ?Sized + ToGlibPtr<'a, P>> (pub P, pub <T as ToGlibPtr<'a, P>>::Storage);

/// Translate a simple type
pub trait ToGlib {
    type GlibType;

    fn to_glib(&self) -> Self::GlibType;
}

impl ToGlib for bool {
    type GlibType = ffi::Gboolean;

    fn to_glib(&self) -> ffi::Gboolean {
        if *self { ffi::GTRUE } else { ffi::GFALSE }
    }
}

/// Translate to a pointer
pub trait ToGlibPtr<'a, P: Copy> {
    type Storage;

    /// Let the foreign library borrow the pointer
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn borrow_to_glib(&'a self) -> Stash<P, Self>;
}

impl <'a, S: Str> ToGlibPtr<'a, *const c_char> for S {
    type Storage = CString;

    fn borrow_to_glib(&'a self) -> Stash<*const c_char, S> {
        let tmp = CString::new(self.as_slice()).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }
}

impl <'a> ToGlibPtr<'a, *mut c_char> for str {
    type Storage = CString;

    fn borrow_to_glib(&'a self) -> Stash<*mut c_char, str> {
        let tmp = CString::new(self.as_slice()).unwrap();
        Stash(tmp.as_ptr() as *mut _, tmp)
    }
}

impl <'a> ToGlibPtr<'a, *mut c_char> for String {
    type Storage = CString;

    fn borrow_to_glib(&'a self) -> Stash<*mut c_char, String> {
        let tmp = CString::new(self.as_slice()).unwrap();
        Stash(tmp.as_ptr() as *mut _, tmp)
    }
}

impl <'a, S: Str> ToGlibPtr<'a, *const c_char> for Option<S> {
    type Storage = Option<CString>;

    fn borrow_to_glib(&'a self) -> Stash<*const c_char, Option<S>> {
        let tmp = match self {
            &Some(ref s) => Some(CString::new(s.as_slice()).unwrap()),
            &None => None,
        };
        let ptr = tmp.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        Stash(ptr, tmp)
    }
}

impl <'a, S: Str> ToGlibPtr<'a, *mut c_char> for Option<S> {
    type Storage = Option<CString>;

    fn borrow_to_glib(&'a self) -> Stash<*mut c_char, Option<S>> {
        let tmp = match self {
            &Some(ref s) => Some(CString::new(s.as_slice()).unwrap()),
            &None => None,
        };
        let ptr = tmp.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        Stash(ptr as *mut _, tmp)
    }
}

impl <'a, P: Copy, T, I: ?Sized> ToGlibPtr<'a, *mut P> for I
where T: ToGlibPtr<'a, P>, &'a I: IntoIterator<Item = &'a T> {
    type Storage = PtrArray<'a, P, T>;

    fn borrow_to_glib(&'a self) -> Stash<*mut P, I> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| v.borrow_to_glib()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.0).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        Stash(ptr_vec.as_mut_ptr(), PtrArray(ptr_vec, tmp_vec))
    }
}

/// Temporary storage for passing a `NULL` terminated array of pointers
pub struct PtrArray<'a, P: Copy, T: ToGlibPtr<'a, P>> (Vec<P>, Vec<Stash<'a, P, T>>);

impl <'a, P: Copy, T: ToGlibPtr<'a, P>> PtrArray<'a, P, T> {
    /// Returns the length of the array not counting the `NULL` terminator
    pub fn len(&self) -> usize {
        self.1.len()
    }
}

/// Translate a simple type
pub trait FromGlib: Sized {
    type GlibType: Sized;

    fn from_glib(val: Self::GlibType) -> Self;
}

/// Translate a simple type
pub fn from_glib<T: FromGlib>(val: <T as FromGlib>::GlibType) -> T {
    FromGlib::from_glib(val)
}

impl FromGlib for bool {
    type GlibType = ffi::Gboolean;

    fn from_glib(val: ffi::Gboolean) -> bool {
        !(val == ffi::GFALSE)
    }
}

/// Translate from a pointer type that can be `NULL`
pub trait FromGlibPtr: Sized {
    type GlibType: PtrExt + Copy;

    /// Borrow the reference
    unsafe fn borrow(ptr: Self::GlibType) -> Self;

    /// Take ownership of the reference
    unsafe fn take(ptr: Self::GlibType) -> Self;

    /// Take ownership of the floating reference
    unsafe fn sink(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }
}

/// Translate from a pointer type guaranteed to never be `NULL`
pub trait FromGlibPtrNotNull: Sized {
    type GlibType: PtrExt + Copy;

    /// Borrow the reference
    unsafe fn borrow(ptr: Self::GlibType) -> Self;

    /// Take ownership of the reference
    unsafe fn take(ptr: Self::GlibType) -> Self;

    /// Take ownership of the floating reference
    unsafe fn sink(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }
}

impl FromGlibPtr for Option<String> {
    type GlibType = *const c_char;

    unsafe fn borrow(ptr: *const c_char) -> Option<String> {
        if ptr.is_null() { None }
        else { Some(FromGlibPtrNotNull::borrow(ptr)) }
    }

    unsafe fn take(ptr: *const c_char) -> Option<String> {
        if ptr.is_null() { None }
        else { Some(FromGlibPtrNotNull::take(ptr)) }
    }
}

impl FromGlibPtrNotNull for String {
    type GlibType = *const c_char;

    unsafe fn borrow(ptr: *const c_char) -> Self {
        debug_assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }

    unsafe fn take(ptr: *const c_char) -> Self {
        let res = FromGlibPtrNotNull::borrow(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

/// Translate from a container of pointers
pub trait FromGlibPtrContainer<GT: PtrExt + Copy>: Sized {
    /// Borrow the references
    unsafe fn borrow(ptr: GT) -> Self;

    /// Borrow the references with an advised expected number
    unsafe fn borrow_num(ptr: GT, _num: usize) -> Self;

    /// Take ownership of the container but borrow its contents
    unsafe fn take_outer(ptr: GT) -> Self;

    /// Take ownership of the container but borrow its contents with an advised expected number
    unsafe fn take_outer_num(ptr: GT, _num: usize) -> Self;

    /// Take ownership of the references
    unsafe fn take(ptr: GT) -> Self;

    /// Take ownership of the references with an advised expected number
    unsafe fn take_num(ptr: GT, _num: usize) -> Self;
}

unsafe fn c_array_len<P: PtrExt + Copy>(mut ptr: *const P) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let mut len = 0;
    while !(*ptr).is_null() {
        len += 1;
        ptr = ptr.offset(1);
    }
    len
}

impl <T: FromGlibPtrNotNull>
FromGlibPtrContainer<*const <T as FromGlibPtrNotNull>::GlibType>
for Vec<T> {
    unsafe fn borrow(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                         num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(FromGlibPtrNotNull::borrow(*ptr));
            ptr = ptr.offset(1);
        }
        res
    }

    unsafe fn take_outer(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                             num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn take(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::take_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                       num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(FromGlibPtrNotNull::take(*ptr));
            ptr = ptr.offset(1);
        }
        ffi::g_free(ptr as *mut _);
        res
    }
}

unsafe fn slist_len(mut ptr: *mut ffi::C_GSList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <T: FromGlibPtrNotNull> FromGlibPtrContainer<*mut ffi::C_GSList> for Vec<T> {
    unsafe fn borrow(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::borrow(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn take_outer(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_slist_free(ptr as *mut _);
        }
        res
    }

    unsafe fn take(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::take(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            ffi::g_slist_free(orig_ptr as *mut _);
        }
        res
    }
}

unsafe fn list_len(mut ptr: *mut ffi::C_GList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <T: FromGlibPtrNotNull> FromGlibPtrContainer<*mut ffi::C_GList> for Vec<T> {
    unsafe fn borrow(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::borrow(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn take_outer(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_list_free(ptr as *mut _);
        }
        res
    }

    unsafe fn take(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::take(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            ffi::g_list_free(orig_ptr as *mut _);
        }
        res
    }
}
