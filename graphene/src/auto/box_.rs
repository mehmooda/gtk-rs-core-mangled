// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Point3D;
use crate::Sphere;
use crate::Vec3;
use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Box(Boxed<ffi::graphene_box_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_box_get_type(), ptr as *mut _) as *mut ffi::graphene_box_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_box_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::graphene_box_get_type(),
    }
}

impl Box {
    #[doc(alias = "graphene_box_contains_box")]
    pub fn contains_box(&self, b: &Box) -> bool {
        unsafe {
            from_glib(ffi::graphene_box_contains_box(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_box_contains_point")]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        unsafe {
            from_glib(ffi::graphene_box_contains_point(
                self.to_glib_none().0,
                point.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_box_equal")]
    fn equal(&self, b: &Box) -> bool {
        unsafe {
            from_glib(ffi::graphene_box_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_box_expand")]
    pub fn expand(&self, point: &Point3D) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand(
                self.to_glib_none().0,
                point.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_expand_scalar")]
    pub fn expand_scalar(&self, scalar: f32) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand_scalar(
                self.to_glib_none().0,
                scalar,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_expand_vec3")]
    pub fn expand_vec3(&self, vec: &Vec3) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand_vec3(
                self.to_glib_none().0,
                vec.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_get_bounding_sphere")]
    pub fn get_bounding_sphere(&self) -> Sphere {
        unsafe {
            let mut sphere = Sphere::uninitialized();
            ffi::graphene_box_get_bounding_sphere(
                self.to_glib_none().0,
                sphere.to_glib_none_mut().0,
            );
            sphere
        }
    }

    #[doc(alias = "graphene_box_get_center")]
    pub fn get_center(&self) -> Point3D {
        unsafe {
            let mut center = Point3D::uninitialized();
            ffi::graphene_box_get_center(self.to_glib_none().0, center.to_glib_none_mut().0);
            center
        }
    }

    #[doc(alias = "graphene_box_get_depth")]
    pub fn get_depth(&self) -> f32 {
        unsafe { ffi::graphene_box_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_get_height")]
    pub fn get_height(&self) -> f32 {
        unsafe { ffi::graphene_box_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_get_max")]
    pub fn get_max(&self) -> Point3D {
        unsafe {
            let mut max = Point3D::uninitialized();
            ffi::graphene_box_get_max(self.to_glib_none().0, max.to_glib_none_mut().0);
            max
        }
    }

    #[doc(alias = "graphene_box_get_min")]
    pub fn get_min(&self) -> Point3D {
        unsafe {
            let mut min = Point3D::uninitialized();
            ffi::graphene_box_get_min(self.to_glib_none().0, min.to_glib_none_mut().0);
            min
        }
    }

    #[doc(alias = "graphene_box_get_size")]
    pub fn get_size(&self) -> Vec3 {
        unsafe {
            let mut size = Vec3::uninitialized();
            ffi::graphene_box_get_size(self.to_glib_none().0, size.to_glib_none_mut().0);
            size
        }
    }

    //#[doc(alias = "graphene_box_get_vertices")]
    //pub fn get_vertices(&self, vertices: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 0 }; 8) {
    //    unsafe { TODO: call ffi:graphene_box_get_vertices() }
    //}

    #[doc(alias = "graphene_box_get_width")]
    pub fn get_width(&self) -> f32 {
        unsafe { ffi::graphene_box_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_init")]
    pub fn init(&mut self, min: Option<&Point3D>, max: Option<&Point3D>) {
        unsafe {
            ffi::graphene_box_init(
                self.to_glib_none_mut().0,
                min.to_glib_none().0,
                max.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_box_init_from_box")]
    pub fn init_from_box(&mut self, src: &Box) {
        unsafe {
            ffi::graphene_box_init_from_box(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_box_init_from_vec3")]
    pub fn init_from_vec3(&mut self, min: Option<&Vec3>, max: Option<&Vec3>) {
        unsafe {
            ffi::graphene_box_init_from_vec3(
                self.to_glib_none_mut().0,
                min.to_glib_none().0,
                max.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_box_intersection")]
    pub fn intersection(&self, b: &Box) -> Option<Box> {
        unsafe {
            let mut res = Box::uninitialized();
            let ret = from_glib(ffi::graphene_box_intersection(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            ));
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_box_union")]
    pub fn union(&self, b: &Box) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_union(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_empty")]
    pub fn empty() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_empty()) }
    }

    #[doc(alias = "graphene_box_infinite")]
    pub fn infinite() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_infinite()) }
    }

    #[doc(alias = "graphene_box_minus_one")]
    pub fn minus_one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_minus_one()) }
    }

    #[doc(alias = "graphene_box_one")]
    pub fn one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_one()) }
    }

    #[doc(alias = "graphene_box_one_minus_one")]
    pub fn one_minus_one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_one_minus_one()) }
    }

    #[doc(alias = "graphene_box_zero")]
    pub fn zero() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_zero()) }
    }
}

impl PartialEq for Box {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Box {}
