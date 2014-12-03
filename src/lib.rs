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

/// Struct of a desctop app which is the basic setup
/// it includes everything nedded for a desktop applicatio
pub struct Window{
    delay: uint,//delay betwen event checks default is 10
    pub window: sdl2::video::Window,
}

impl Window{
    /// create a new window with given title, width and height
    pub fn new(title: &str, width: int, height: int) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);
        Window{
            delay: 12,
            // Create a window
            window: sdl2::video::Window::new(title, WindowPos::PosCentered, WindowPos::PosCentered, width, height, OPENGL).unwrap(),
        }
    }

    /// function which takes the render function to generate the content, and then listens for input events
    /// it will return, if the window has been closed.
    pub fn show(&mut self, render: |&mut CTX|){
        use sdl2::event;

        {
            let mut ctx = CTX{
                window: self,
                pos: (0.0,0.0),
            };
            render(&mut ctx);
        }

        'event : loop {
            match poll_event() {
                Event::Quit(_) => break 'event,
                Event::KeyDown(_, _, key_code, scan_code, _,_) => println!("deydown {} {}", key_code, scan_code),
                Event::KeyUp(_, _, _, _, _, _) => println!("key up"),
                Event::MouseMotion(_, _, _, _, x, y, xrel, yrel) => {
                    //TODO: get mouse state
                    println!("mouse move: ({}|{}) ({}|{})",x,y,xrel,yrel);
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

    /// calls the given function and provides a cairo drawing context
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        let sdl_surface = self.window.get_surface().unwrap();
        let mut surface = sw::SurfaceWrapper::from_sdl(sdl_surface);//generate a wrapper around the surface
        let mut ctx = Context::new(&mut surface).unwrap();//generate the cairo context

        draw_closure(&mut ctx);

        let _ = self.window.update_surface();//surface updaten
    }

    /// close the window
    pub fn close(&mut self){
        sdl2::quit();
    }

    /// set the title of the window
    pub fn title(&mut self, val: String){
        self.window.set_title(val.as_slice());
    }

    /// get title of the window
    pub fn get_title(&self) -> String{
        self.window.get_title()
    }

    /// get the size of the window
    pub fn get_size(&self) -> (int, int){
        self.window.get_size()
    }

    //get size of the canvas to draw on
    pub fn get_draw_size(&self) -> (int, int){
        self.window.get_drawable_size()
    }
}

#[test]
fn window(){
    let mut w = Window::new("window-test",640,480);
    assert_eq!(w.get_title(), "window-test".to_string());

    //Test the sizing of the window
    let (width, height) = w.get_size();
    assert_eq!(width, 640);
    assert_eq!(height, 480);

    //Drawing logic of the Program
    let mut x = 0;
    let mut y = 0;

    w.draw(|ctx| {//drow something
        ctx.set_source_rgb(0.85, 0.85, 0.85);
        ctx.rectangle(0.0, 0.0, width as f64, height as f64);
        ctx.fill();

        ctx.set_source_rgb(0.5, 0.5, 1.0);
        ctx.set_line_width(10.0);

        ctx.move_to(x as f64, y as f64);
        ctx.line_to((width - x) as f64, (height - y) as f64);
        ctx.stroke();

        ctx.set_source_rgb(0.2, 0.7, 0.2);
        ctx.set_line_width(2.0);
        ctx.rectangle(10.0, 10.0, (width-20) as f64, (height-20) as f64);//draw box

        ctx.stroke();

        ctx.translate(50.0,50.0);
        ctx.set_source_rgb(0.5, 0.5, 0.5);
        ctx.set_line_width(3.0);
        ctx.rectangle(0.0,0.0, 100.0, 30.0);
        ctx.stroke();
    });

    w.show(|ctx|{});//start event loop
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
    fn size(&self) -> (f64, f64) {
        (0.0,0.0)
    }
    /// should this component be cached in its own frame buffer?
    /// if nothing is set it wil be painted each time it is called
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
    pos: (f64,f64),
}

impl<'a> CTX<'a>{
    ///this function calls a closure with the cairo drawing context as parameter
    ///so the widget can draw some lower level stuff
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        let (x, y) = self.pos;
        self.window.draw(|ctx|{
            ctx.translate(x, y);
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
    pub fn add<Event>(&mut self,id: uint, w: &Widget<Event>, event:Option<|Event|>) -> (f64, f64){
        println!("add");
        //TODO: create a sub context
        return w.render(self);
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
    pub fn mouseover(&mut self, pos: (f64,f64), handle: |int|){
        println!("register mousover event");
    }
}

#[test]
fn cairo_test(){

}
