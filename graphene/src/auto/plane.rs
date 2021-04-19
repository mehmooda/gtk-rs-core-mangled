// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use crate::Matrix;
use crate::Point3D;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Plane(Boxed<ffi::graphene_plane_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_plane_get_type(), ptr as *mut _) as *mut ffi::graphene_plane_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_plane_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        type_ => || ffi::graphene_plane_get_type(),
    }
}

impl Plane {
    #[doc(alias = "graphene_plane_distance")]
    pub fn distance(&self, point: &Point3D) -> f32 {
        unsafe { ffi::graphene_plane_distance(self.to_glib_none().0, point.to_glib_none().0) }
    }

    #[doc(alias = "graphene_plane_equal")]
    fn equal(&self, b: &Plane) -> bool {
        unsafe {
            from_glib(ffi::graphene_plane_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "graphene_plane_get_constant")]
    pub fn constant(&self) -> f32 {
        unsafe { ffi::graphene_plane_get_constant(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_plane_get_normal")]
    pub fn normal(&self) -> Vec3 {
        unsafe {
            let mut normal = Vec3::uninitialized();
            ffi::graphene_plane_get_normal(self.to_glib_none().0, normal.to_glib_none_mut().0);
            normal
        }
    }

    #[doc(alias = "graphene_plane_init")]
    pub fn init(&mut self, normal: Option<&Vec3>, constant: f32) {
        unsafe {
            ffi::graphene_plane_init(self.to_glib_none_mut().0, normal.to_glib_none().0, constant);
        }
    }

    #[doc(alias = "graphene_plane_init_from_plane")]
    pub fn init_from_plane(&mut self, src: &Plane) {
        unsafe {
            ffi::graphene_plane_init_from_plane(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_plane_init_from_point")]
    pub fn init_from_point(&mut self, normal: &Vec3, point: &Point3D) {
        unsafe {
            ffi::graphene_plane_init_from_point(
                self.to_glib_none_mut().0,
                normal.to_glib_none().0,
                point.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_plane_init_from_points")]
    pub fn init_from_points(&mut self, a: &Point3D, b: &Point3D, c: &Point3D) {
        unsafe {
            ffi::graphene_plane_init_from_points(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_plane_init_from_vec4")]
    pub fn init_from_vec4(&mut self, src: &Vec4) {
        unsafe {
            ffi::graphene_plane_init_from_vec4(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_plane_negate")]
    pub fn negate(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_plane_normalize")]
    pub fn normalize(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "graphene_plane_transform")]
    pub fn transform(&self, matrix: &Matrix, normal_matrix: Option<&Matrix>) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            ffi::graphene_plane_transform(
                self.to_glib_none().0,
                matrix.to_glib_none().0,
                normal_matrix.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }
}

impl PartialEq for Plane {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Plane {}
