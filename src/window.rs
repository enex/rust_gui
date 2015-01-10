//use time::precise_time_ns;
use sdl2;
use cairo;
use ctx::CTX;

use sdl2::video::{OPENGL, WindowPos};
use sdl2::event::{poll_event};
use sdl2::event::Event;//reexprot events
use std::any::Any;
use Context;

/// Struct of a desktop app which is the basic setup
/// it includes everything nedded for a desktop application.
/// Sdl2 is used as the interface to the window manager and cairo for rendering.
pub struct Window{
    ///delay betwen event checks default is 10
    pub delay: uint,
    ///sdl2 Window
    pub window: sdl2::video::Window,
    ///id of the currently drawing component
    current_id: Vec<uint>,
    ctx: cairo::Context,//cairo context for drawing
    //state: Vec<state::Container>,//store for state
    ///id of the currently selected element
    pub focused: Vec<uint>,
}

/// Events are handled with moved closures as filters that means
/// you can get a streeem of event form the Context and filert it
/// with a function like so:
///     ctx.events(move |event| ..., |event| ...)
/// whrere the first closure filters the closures so only interesting closures are lef
/// an the second calls the closure which does actual event handling inside of the component

impl Window{
    /// create a new window with given title, width and height
    pub fn new(title: &str, width: int, height: int) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let window = sdl2::video::Window::new(title, WindowPos::PosCentered, WindowPos::PosCentered, width, height, OPENGL).unwrap();
        let surface = match window.get_surface() {
            Ok(s) => s,
            Err(e) => panic!("could not get window surface {}",e)
        };
        surface.lock();
        let ss = surface.raw();//raw sdl surface
        let cr = unsafe{
            let surface = cairo::ffi::cairo_image_surface_create_for_data(
                (*ss).pixels as *mut u8,
                cairo::ffi::CAIRO_FORMAT_RGB24,
                (*ss).w,
                (*ss).h,
                (*ss).pitch
            );
            if(surface.is_null()){
                panic!("could not create surface");
            }
            unsafe{
                cairo::Context::from_raw(surface)
            }
        }.unwrap();

        Window{
            delay: 12,
            window: window,
            ctx: cr,
            current_id: vec![],
            focused: Vec::new(),
        }
    }

    /// function which takes the render function to generate the content, and then listens for input events
    /// it will return, if the window has been closed.
    pub fn show<F>(&mut self, render: F) where F:  Fn(&mut CTX){
        //self.window.show();
        self.update();
        //initial draw:
        self.cairo_context().save();
        {
            let mut c = CTX::new(self);
            render(&mut c);
        }
        self.cairo_context().restore();
        self.update();

        'main : loop {
            'event : loop {
                match sdl2::event::poll_event() {
                    sdl2::event::Event::Quit(_) => break 'main,
                    sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                        if key == sdl2::keycode::KeyCode::Escape {
                            break 'main
                        }
                    },
                    sdl2::event::Event::None => break 'event,
                    _ => sdl2::timer::delay(self.delay)
                }
            }
        }
        sdl2::quit();
    }

    /// search the state of a component by Id
    pub fn find_state<T>(&self, id: Vec<uint>) -> Option<T>{
        //TODO: implement functionality
        unimplemented!()
    }

    /// set the state
    pub fn set_state<T:Any+Clone+Eq>(&self, id: Vec<uint>, v: T){
        unimplemented!()
    }

    /// get the ciro drawing context to draw on it
    #[stable]
    #[inline(always)]
    pub fn cairo_context(&mut self) -> &mut cairo::Context{
        &mut self.ctx
    }

    /*
    /// calls the given function and provides a cairo drawing context
    #[stable]
    #[inline(always)]
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        draw_closure(&mut self.ctx);
    }*/

    ///Function to update view to cairo drawing
    #[stable]
    #[inline(always)]
    pub fn update(&mut self){
        let _ = self.window.update_surface();//surface updaten
    }

    /// close the window
    #[stable]
    #[inline(always)]
    pub fn close(&mut self){
        sdl2::quit();
    }

    /// set the title of the window
    #[inline(always)]
    pub fn title(&mut self, val: String){
        self.window.set_title(val.as_slice());
    }

    /// get title of the window
    #[inline(always)]
    pub fn get_title(&self) -> String{
        self.window.get_title()
    }

    /// get the size of the window
    #[inline(always)]
    pub fn get_size(&self) -> (int, int){
        self.window.get_size()
    }

    //get size of the canvas to draw on
    #[inline(always)]
    pub fn get_draw_size(&self) -> (int, int){
        self.window.get_drawable_size()
    }
}
