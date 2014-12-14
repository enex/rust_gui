use cairo;
use sdl2;


//implementation of a wrapper around the sdl surface
pub struct SurfaceWrapper{
    ptr: *mut cairo::ffi::cairo_surface_t,
}
impl SurfaceWrapper{
    pub fn from_sdl(sdl_surface: sdl2::surface::Surface) -> SurfaceWrapper{
        sdl_surface.unlock();
        unsafe{
            let ss = sdl_surface.raw();
            let surface = cairo::ffi::cairo_image_surface_create_for_data(
                (*ss).pixels as *mut u8,
                cairo::ffi::CAIRO_FORMAT_RGB24,
                (*ss).w,
                (*ss).h,
                (*ss).pitch
            );
            SurfaceWrapper{ptr: surface}
        }
    }
}
impl cairo::Surface for SurfaceWrapper{
    unsafe fn surface_ptr(&mut self) -> *mut cairo::ffi::cairo_surface_t{
        self.ptr
    }
    fn width(&self) -> i32 {
        unsafe {
            cairo::ffi::cairo_image_surface_get_width(self.ptr)
        }
    }

    fn height(&self) -> i32 {
        unsafe {
            cairo::ffi::cairo_image_surface_get_height(self.ptr)
        }
    }
}
impl Clone for SurfaceWrapper{
    fn clone(&self) -> SurfaceWrapper {
        unsafe {
            cairo::ffi::cairo_surface_reference(self.ptr);
            SurfaceWrapper { ptr : self.ptr}
        }
    }
}
impl Drop for SurfaceWrapper{
    fn drop(&mut self) {
        unsafe {
            cairo::ffi::cairo_surface_destroy(self.ptr);
        }
    }
}
