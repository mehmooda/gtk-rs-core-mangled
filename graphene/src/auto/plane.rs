// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gobject_sys;
use graphene_sys;
use Point3D;
use Vec3;
use Vec4;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Plane(Boxed<graphene_sys::graphene_plane_t>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(graphene_sys::graphene_plane_get_type(), ptr as *mut _) as *mut graphene_sys::graphene_plane_t,
        free => |ptr| gobject_sys::g_boxed_free(graphene_sys::graphene_plane_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || graphene_sys::graphene_plane_get_type(),
    }
}

impl Plane {
    pub fn distance(&self, point: &Point3D) -> f32 {
        unsafe {
            graphene_sys::graphene_plane_distance(self.to_glib_none().0, point.to_glib_none().0)
        }
    }

    fn equal(&self, b: &Plane) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_plane_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    pub fn get_constant(&self) -> f32 {
        unsafe { graphene_sys::graphene_plane_get_constant(self.to_glib_none().0) }
    }

    pub fn get_normal(&self) -> Vec3 {
        unsafe {
            let mut normal = Vec3::uninitialized();
            graphene_sys::graphene_plane_get_normal(
                self.to_glib_none().0,
                normal.to_glib_none_mut().0,
            );
            normal
        }
    }

    pub fn init(&mut self, normal: Option<&Vec3>, constant: f32) {
        unsafe {
            graphene_sys::graphene_plane_init(
                self.to_glib_none_mut().0,
                normal.to_glib_none().0,
                constant,
            );
        }
    }

    pub fn init_from_plane(&mut self, src: &Plane) {
        unsafe {
            graphene_sys::graphene_plane_init_from_plane(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    pub fn init_from_point(&mut self, normal: &Vec3, point: &Point3D) {
        unsafe {
            graphene_sys::graphene_plane_init_from_point(
                self.to_glib_none_mut().0,
                normal.to_glib_none().0,
                point.to_glib_none().0,
            );
        }
    }

    pub fn init_from_points(&mut self, a: &Point3D, b: &Point3D, c: &Point3D) {
        unsafe {
            graphene_sys::graphene_plane_init_from_points(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
        }
    }

    pub fn init_from_vec4(&mut self, src: &Vec4) {
        unsafe {
            graphene_sys::graphene_plane_init_from_vec4(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    pub fn negate(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            graphene_sys::graphene_plane_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize(&self) -> Plane {
        unsafe {
            let mut res = Plane::uninitialized();
            graphene_sys::graphene_plane_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
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
