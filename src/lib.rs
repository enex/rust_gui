#![crate_name = "gui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(macro_rules, globs)]

extern crate libc;

extern crate cairo;
extern crate sdl2;
extern crate time;

//use std::ptr;
//use std::io::timer;
//use std::time::duration::Duration;
use cairo::Context;
use time::precise_time_ns;

use sdl2::video::{OPENGL, WindowPos};
use sdl2::event::{poll_event};
pub use sdl2::event::Event;//reexprot events
//use std::cell::RefCell;
//use sdl2::rect::{Rect};
mod sw;

pub mod components; //TODO: export gui components

//TODO: inline functions in cairo wrapper for better speed

/// Struct of a desctop app which is the basic setup
/// it includes everything nedded for a desktop application.
/// Sdl2 is used as the interface to the window manager and cairo for rendering.
pub struct Window{
    delay: uint,//delay betwen event checks default is 10
    pub window: sdl2::video::Window,
    ctx: Context,//cairo context for drawing
}

impl Window{
    /// create a new window with given title, width and height
    pub fn new(title: &str, width: int, height: int) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let window = sdl2::video::Window::new(title, WindowPos::PosCentered, WindowPos::PosCentered, width, height, OPENGL).unwrap();
        let ctx: Context = Context::new(&mut sw::SurfaceWrapper::from_sdl(window.get_surface().unwrap())).unwrap();
        Window{
            delay: 12,
            // Create a window
            window: window,
            ctx: ctx,
        }
    }

    /// function which takes the render function to generate the content, and then listens for input events
    /// it will return, if the window has been closed.
    pub fn show(&mut self, render: |&mut CTX<()>|){
        use sdl2::event;

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
                Event::KeyDown(_, _, key_code, scan_code, _,_) => println!("deydown {} {}", key_code, scan_code),
                Event::KeyUp(_, _, _, _, _, _) => println!("key up"),
                Event::MouseMotion(_,_,_,_,_,_,_,_) | Event::MouseButtonDown(_,_,_,_,_,_) | Event::MouseButtonUp(_,_,_,_,_,_) => {
                    //TODO: get mouse state
                    //println!("mouse move: ({}|{}) ({}|{})",x,y,xrel,yrel);
                    let start = precise_time_ns();
                    self.cairo_context().save();
                    {
                        let mut ctx = CTX::new::<()>(self, &e, true);
                        render(&mut ctx);
                    }
                    self.cairo_context().restore();

                    self.update();
                    let taken = precise_time_ns() - start;
                    println!("  => {:.3} ms", (taken as f64)/1000000.0);
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

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget<Event>{
    /// In this function only rendering to the screen and atouching event listeners is done
    /// the state of the component gets passed as a imutable reference, so this rutine is not
    /// able to change anything.
    /// It returns the (width, hight) of the area affected by the render method
    fn render(&mut self, ctx: &mut CTX<Event>) -> (f64, f64);

    /// Method which is used by the layout engine to get the size of a component
    /// by default the size will be calculated by using the render function with
    /// blocked drawing, but if speed matters this function can provide faster information
    #[inline(always)]
    fn size(&self) -> (f64, f64) {
        (0.0,0.0)
    }
}

pub struct CTX<'a, Event>{
    window: &'a mut Window,//ref to the window
    event: &'a sdl2::event::Event,
    pos: (f64,f64),
    should_draw: bool,//wether anything should be drawing
    should_handle: bool,
    events: Vec<Event>,
}

impl<'a> CTX<'a, Event>{
    pub fn new<'a, Event>(window: &'a mut Window, event:&'a sdl2::event::Event, should_draw: bool) -> CTX<'a, Event>{
        CTX{
            window: window,
            event: event,
            pos: (0.0,0.0),
            should_draw: should_draw,
            should_handle: true,
            events: vec![],
        }
    }
}

impl<'a, Event> CTX<'a, Event>{
    ///this function calls a closure with the cairo drawing context as parameter
    ///so the widget can draw some lower level stuff
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        if !self.should_draw{
            return;
        }
        draw_closure(self.window.cairo_context());
    }

    ///add a component as a child of the current component
    /// id: this is an unique identifier which is used to manage state and caching
    ///     it has to be unique in this component
    /// w: the widget which should be added. One widget can be added multible times
    /// event: the closure which should be called if the Widget fires an event
    ///        normaly widgets use enums which can then be matched an events can be handled
    ///        if no event should be catched this should be None
    #[inlne]
    pub fn add<Event>(&mut self,id: uint, w: &mut Widget<Event>, event:Option<|Event|>) -> (f64, f64){
        //println!("add");
        //TODO: create a sub context
        self.window.cairo_context().save();
        let (x, y) = self.pos;
        self.window.cairo_context().translate(x,y);//go to the right position
        let mut e;
        //TODO: ignore emits if no event handler is given
        {
            let mut ctx:CTX<Event> = CTX::new(self.window, self.event, self.should_draw);
            e = w.render(&mut ctx);
            match event{
                Some(c) => {//handle the events fired
                    for s in ctx.events.into_iter() {
                        c(s);
                    }
                },
                None => {}
            }
            //TODO: call callback for event
        }
        self.window.cairo_context().restore();
        e
    }

    ///emit an event which can be handled by the parent element.
    #[inline]
    pub fn emit(&mut self, event: Event){
        self.events.push(event);
    }

    /// set the position for the next widget.
    #[inline(always)]
    pub fn go_to(&mut self, x: f64, y: f64){
        self.pos = (x,y);
    }

    #[inline(always)]
    pub fn cairo_pos(&mut self) -> (f64, f64){
        let mut sx = 0.0;
        let mut sy = 0.0;
        self.window.cairo_context().user_to_device(&mut sx, &mut sy);
        (sx,sy)
    }

    /// get current x positiont
    #[inline(always)]
    pub fn x(&mut self) -> f64{
        let (x,_) = self.cairo_pos();
        x
    }

    /// get current y position
    #[inline(always)]
    pub fn y(&mut self) -> f64{
        let (_,y) = self.cairo_pos();
        y
    }

    ///with this function it is possible to mutate the state of the component,
    ///it should be avoided in render function, maybe in event handler
    pub fn get_mut_state(){
        unimplemented!();
    }

    pub fn get_state(){
        unimplemented!();
    }

    /// register mouse event listener
    //TODO: add window event subsribing capabilitys
    pub fn mouseover(&mut self, size: (f64,f64), handle: |&sdl2::event::Event, &mut CTX<Event>|){
        use sdl2::event;
        if !self.should_handle{
            return;
        }
        //TODO: register instead of handling directly

        let mut sx = 0.0;
        let mut sy = 0.0;
        self.window.cairo_context().user_to_device(&mut sx, &mut sy);

        match self.event{
            &Event::MouseMotion(_, _, _, _, x, y, _, _)
            | &Event::MouseButtonDown(_, _, _, _, x, y)
            | &Event::MouseButtonUp(_, _, _, _, x, y) => {
                let (bx, by) = size;
                let x = x as f64;
                let y = y as f64;
                if x >= sx && y >= sy && x <= bx+sx && y <= by+sy{
                    handle(self.event, self);
                }
                //self.found = true;
            },
            _ => return,
        }
    }
}
