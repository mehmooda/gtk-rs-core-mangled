// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gobject_sys;
use graphene_sys;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Box;
use Plane;
use Point3D;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use RayIntersectionKind;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Sphere;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Triangle;
use Vec3;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Ray(Boxed<graphene_sys::graphene_ray_t>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(graphene_sys::graphene_ray_get_type(), ptr as *mut _) as *mut graphene_sys::graphene_ray_t,
        free => |ptr| gobject_sys::g_boxed_free(graphene_sys::graphene_ray_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || graphene_sys::graphene_ray_get_type(),
    }
}

impl Ray {
    fn equal(&self, b: &Ray) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_ray_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    pub fn get_closest_point_to_point(&self, p: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            graphene_sys::graphene_ray_get_closest_point_to_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn get_direction(&self) -> Vec3 {
        unsafe {
            let mut direction = Vec3::uninitialized();
            graphene_sys::graphene_ray_get_direction(
                self.to_glib_none().0,
                direction.to_glib_none_mut().0,
            );
            direction
        }
    }

    pub fn get_distance_to_plane(&self, p: &Plane) -> f32 {
        unsafe {
            graphene_sys::graphene_ray_get_distance_to_plane(
                self.to_glib_none().0,
                p.to_glib_none().0,
            )
        }
    }

    pub fn get_distance_to_point(&self, p: &Point3D) -> f32 {
        unsafe {
            graphene_sys::graphene_ray_get_distance_to_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
            )
        }
    }

    pub fn get_origin(&self) -> Point3D {
        unsafe {
            let mut origin = Point3D::uninitialized();
            graphene_sys::graphene_ray_get_origin(
                self.to_glib_none().0,
                origin.to_glib_none_mut().0,
            );
            origin
        }
    }

    pub fn get_position_at(&self, t: f32) -> Point3D {
        unsafe {
            let mut position = Point3D::uninitialized();
            graphene_sys::graphene_ray_get_position_at(
                self.to_glib_none().0,
                t,
                position.to_glib_none_mut().0,
            );
            position
        }
    }

    pub fn init(&mut self, origin: Option<&Point3D>, direction: Option<&Vec3>) {
        unsafe {
            graphene_sys::graphene_ray_init(
                self.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
        }
    }

    pub fn init_from_ray(&mut self, src: &Ray) {
        unsafe {
            graphene_sys::graphene_ray_init_from_ray(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    pub fn init_from_vec3(&mut self, origin: Option<&Vec3>, direction: Option<&Vec3>) {
        unsafe {
            graphene_sys::graphene_ray_init_from_vec3(
                self.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersect_box(&self, b: &Box) -> (RayIntersectionKind, f32) {
        unsafe {
            let mut t_out = mem::MaybeUninit::uninit();
            let ret = from_glib(graphene_sys::graphene_ray_intersect_box(
                self.to_glib_none().0,
                b.to_glib_none().0,
                t_out.as_mut_ptr(),
            ));
            let t_out = t_out.assume_init();
            (ret, t_out)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersect_sphere(&self, s: &Sphere) -> (RayIntersectionKind, f32) {
        unsafe {
            let mut t_out = mem::MaybeUninit::uninit();
            let ret = from_glib(graphene_sys::graphene_ray_intersect_sphere(
                self.to_glib_none().0,
                s.to_glib_none().0,
                t_out.as_mut_ptr(),
            ));
            let t_out = t_out.assume_init();
            (ret, t_out)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersect_triangle(&self, t: &Triangle) -> (RayIntersectionKind, f32) {
        unsafe {
            let mut t_out = mem::MaybeUninit::uninit();
            let ret = from_glib(graphene_sys::graphene_ray_intersect_triangle(
                self.to_glib_none().0,
                t.to_glib_none().0,
                t_out.as_mut_ptr(),
            ));
            let t_out = t_out.assume_init();
            (ret, t_out)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersects_box(&self, b: &Box) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_ray_intersects_box(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersects_sphere(&self, s: &Sphere) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_ray_intersects_sphere(
                self.to_glib_none().0,
                s.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn intersects_triangle(&self, t: &Triangle) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_ray_intersects_triangle(
                self.to_glib_none().0,
                t.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for Ray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Ray {}
