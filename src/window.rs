use cairo::Context;
//use time::precise_time_ns;
use sdl2;
use cairo;
use CTX;

use sdl2::video::{OPENGL, WindowPos};
use sdl2::event::{poll_event};
use sdl2::event::Event;//reexprot events
use std::any::Any;

use state;

/// Struct of a desctop app which is the basic setup
/// it includes everything nedded for a desktop application.
/// Sdl2 is used as the interface to the window manager and cairo for rendering.
pub struct Window{
    delay: uint,//delay betwen event checks default is 10
    pub window: sdl2::video::Window,
    ctx: Context,//cairo context for drawing
    pub current_id: Vec<uint>,//id of the currently drawing component
    state: Vec<state::Container>,//store for state
    ///id of the currently selected element
    pub focused: Vec<uint>,
}

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
        let ctx =  match unsafe{
            let surface = cairo::ffi::cairo_image_surface_create_for_data(
                (*ss).pixels as *mut u8,
                cairo::ffi::CAIRO_FORMAT_RGB24,
                (*ss).w,
                (*ss).h,
                (*ss).pitch
            );
            Context::from_raw(surface)
        }{
            Ok(e) => e,
            Err(e) => panic!("not able to create cairo-context {}", e)
        };
        Window{
            delay: 12,
            window: window,
            ctx: ctx,
            current_id: vec![],
            state: Vec::new(),
            focused: Vec::new(),
        }
    }

    /// function which takes the render function to generate the content, and then listens for input events
    /// it will return, if the window has been closed.
    pub fn show(&mut self, render: |&mut CTX<()>|){
        self.cairo_context().save();
        {
            let none = &Event::None;
            let mut ctx = CTX::new::<()>(self, none, true);
            render(&mut ctx);
        }
        self.cairo_context().restore();
        self.update();

        'event : loop {
            let e = poll_event();
            match e {
                Event::Quit(_) => break 'event,
                Event::KeyDown(_, _, _, _, _,_) |
                Event::KeyUp(_, _, _, _, _, _) |
                Event::MouseMotion(_,_,_,_,_,_,_,_) | Event::MouseButtonDown(_,_,_,_,_,_) | Event::MouseButtonUp(_,_,_,_,_,_) => {
                    //TODO: get mouse state
                    //println!("mouse move: ({}|{}) ({}|{})",x,y,xrel,yrel);
                    //let start = precise_time_ns();
                    self.cairo_context().save();
                    {
                        let mut ctx = CTX::new::<()>(self, &e, true);
                        render(&mut ctx);
                    }
                    self.cairo_context().restore();

                    self.update();
                    //let taken = precise_time_ns() - start;
                    //println!("  => {:.3} ms", (taken as f64)/1000000.0);
                },
                Event::Window(_, _, id, d1, d2) => {
                    //TODO: use this event to redraw on size changes and to sleep
                    println!("window event {} {} {}", id, d1, d2);
                },
                _ => sdl2::timer::delay(self.delay)
            }
        }
        sdl2::quit();
    }

    /// search the state of a component by Id
    pub fn find_state<T>(&self, id: Vec<uint>) -> Option<T>{
        //TODO: implement functionality
        None
    }

    /// set the state
    pub fn set_state<T:Any+Clone+Eq>(&self, id: Vec<uint>, v: T){
        //TODO: make this work
        let list = &self.state;
        for s in id.into_iter(){
            println!("{}", s);
            for j in range(0,list.len()) {
                if list[j].id == s{
                    println!("gefunden")
                }
            }
            println!("neu hinzufügen");
        }
    }

    /// get the ciro drawing context to draw on it
    #[stable]
    #[inline(always)]
    pub fn cairo_context(&mut self) -> &mut cairo::Context{
        &mut self.ctx
    }

    /// calls the given function and provides a cairo drawing context
    #[stable]
    #[inline(always)]
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        draw_closure(&mut self.ctx);
    }

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
