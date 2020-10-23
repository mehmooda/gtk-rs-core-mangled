use glib::translate::*;
use graphene_sys;
use Size;

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_size_alloc();
            graphene_sys::graphene_size_init(alloc, width, height);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_size(src: &Size) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_size_alloc();
            graphene_sys::graphene_size_init_from_size(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
