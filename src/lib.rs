#![crate_name = "gui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(macro_rules, globs)]

extern crate libc;

extern crate cairo;
extern crate sdl2;

//use std::ptr;
//use std::io::timer;
//use std::time::duration::Duration;
use cairo::Context;

use sdl2::video::{OPENGL, WindowPos};
use sdl2::event::{poll_event, Event};
//use std::cell::RefCell;
//use sdl2::rect::{Rect};
mod sw;

pub mod components; //TODO: export gui components

//TODO: inline functions in cairo wrapper for better speed

/// Struct of a desctop app which is the basic setup
/// it includes everything nedded for a desktop applicatio
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
        let mut ctx: Context = Context::new(&mut sw::SurfaceWrapper::from_sdl(window.get_surface().unwrap())).unwrap();
        Window{
            delay: 12,
            // Create a window
            window: window,
            ctx: ctx,
        }
    }

    /// function which takes the render function to generate the content, and then listens for input events
    /// it will return, if the window has been closed.
    pub fn show(&mut self, render: |&mut CTX|){
        use sdl2::event;

        {
            let none = &Event::None;
            let mut ctx = CTX::new(self, none, true);
            render(&mut ctx);
        }
        self.update();

        'event : loop {
            let e = poll_event();
            match e {
                Event::Quit(_) => break 'event,
                Event::KeyDown(_, _, key_code, scan_code, _,_) => println!("deydown {} {}", key_code, scan_code),
                Event::KeyUp(_, _, _, _, _, _) => println!("key up"),
                Event::MouseMotion(_, _, _, _, x, y, xrel, yrel) => {
                    //TODO: get mouse state
                    //println!("mouse move: ({}|{}) ({}|{})",x,y,xrel,yrel);
                    {
                        let mut ctx = CTX::new(self, &e, true);
                        render(&mut ctx);
                    }
                    self.update();
                },
                Event:: Window(_, _, id, d1, d2) => {
                    //TODO: use this event to redraw on size changes and to sleep
                    println!("window event {} {} {}", id, d1, d2);
                },
                _ => sdl2::timer::delay(self.delay)
            }
        }
        sdl2::quit();
    }

    /// get the ciro drawing context to draw on it
    #[inline(always)]
    pub fn caro_context(&mut self) -> &mut cairo::Context{
        &mut self.ctx
    }

    /// calls the given function and provides a cairo drawing context
    #[inline(always)]
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        draw_closure(&mut self.ctx);
    }

    ///Function to update view to cairo drawing
    #[inline(always)]
    pub fn update(&mut self){
        let _ = self.window.update_surface();//surface updaten
    }

    /// close the window
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
    fn render(&self, ctx: &mut CTX) -> (f64, f64);

    /// Method which is used by the layout engine to get the size of a component
    /// by default the size will be calculated by using the render function with
    /// blocked drawing, but if speed matters this function can provide faster information
    #[inline(always)]
    fn size(&self) -> (f64, f64) {
        (0.0,0.0)
    }
    /// should this component be cached in its own frame buffer?
    /// if nothing is set it wil be painted each time it is called
    #[inline(always)]
    fn do_cach() -> bool{
        false
    }
}

#[cfg(test)]
mod test{
    use components::Button;

    #[test]
    fn show(){
        let mut w = super::Window::new("window-test",640,480);
        w.show(|ctx|{
            let b = Button{text:"test1".to_string(), width: 100.0, height: 30.0};
            let (_,height) = ctx.add(1, &b, Some(|evt| println!("click")));
            ctx.pos = (0.0, ctx.y() + height+2.0);
            let (_,height) = ctx.add(2, &b, None);
            ctx.pos = (0.0, ctx.y() + height+2.0);
            let (_,height) = ctx.add(3, &b, None);
            ctx.pos = (0.0, ctx.y() + height+2.0);
            let (_,height) = ctx.add(4, &b, None);
            ctx.pos = (0.0, ctx.y() + height+2.0);
            let (_,height) = ctx.add(5, &b, None);
        });//start event loop
        panic!("good");
    }
}

pub struct CTX<'a>{
    window: &'a mut Window,//ref to the window
    event: &'a sdl2::event::Event,
    pos: (f64,f64),
    should_draw: bool,//wether anything should be drawing
    should_handle: bool,
}

impl<'a> CTX<'a>{
    fn new(window: &'a mut Window, event:&'a sdl2::event::Event, should_draw: bool) -> CTX<'a>{
        CTX{
            window: window,
            event: event,
            pos: (0.0,0.0),
            should_draw: should_draw,
            should_handle: true,
        }
    }
}

impl<'a> CTX<'a>{
    ///this function calls a closure with the cairo drawing context as parameter
    ///so the widget can draw some lower level stuff
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        if !self.should_draw{
            return;
        }
        let (x, y) = self.pos;
        self.window.draw(|ctx|{
            ctx.translate(x, y);//TODO: replace with a better suited function becaus it is relative
            draw_closure(ctx);
        });
    }

    ///add a component as a child of the current component
    /// id: this is an unique identifier which is used to manage state and caching
    ///     it has to be unique in this component
    /// w: the widget which should be added. One widget can be added multible times
    /// event: the closure which should be called if the Widget fires an event
    ///        normaly widgets use enums which can then be matched an events can be handled
    ///        if no event should be catched this should be None
    #[inlne]
    pub fn add<Event>(&mut self,id: uint, w: &Widget<Event>, event:Option<|Event|>) -> (f64, f64){
        //println!("add");
        //TODO: create a sub context
        w.render(self)
        //TODO: event handling strategie überdenken
        //TODO: look for event
        //TODO: call function for event handling

        //TODO: ignore emits if no event handler is given
        //TODO: call render function
    }

    ///emit an event which can be handled by the parent element.
    pub fn emit<Event: Clone>(&mut self, event: Event){

    }

    #[inline(always)]
    pub fn go_to(&mut self, x: f64, y: f64){
        self.pos = (x,y);
    }

    #[inline(always)]
    pub fn x(&self) -> f64{
        let (x,_) = self.pos;
        x
    }
    #[inline(always)]
    pub fn y(&self) -> f64{
        let (_,y) = self.pos;
        y
    }

    ///with this function it is possible to mutate the state of the component,
    ///it should be avoided in render function, maybe in event handler
    pub fn get_mut_state(){}

    //TODO: add window event subsribing capabilitys
    pub fn mouseover(&mut self, size: (f64,f64), handle: |int|){
        use sdl2::event;
        if !self.should_handle{
            return;
        }
        //TODO: register instead of handling directly

        let (sx, sy) = self.pos;
        //println!("({}|{})", sx,sy);

        match self.event{
            &Event::MouseMotion(_, _, _, _, x, y, xrel, yrel) => {
                let (bx, by) = size;
                let x = x as f64;
                let y = y as f64;
                if x >= sx && y >= sy && x <= bx+sx && y <= by+sy{
                    handle(0);
                }
                //self.found = true;
            },
            _ => return,
        }
    }
}

#[test]
fn cairo_test(){

}
