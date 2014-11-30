//#![allow(dead_code)]
#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;

extern crate cairo;
extern crate sdl2;

//use std::ptr;
//use std::io::timer;
//use std::time::duration::Duration;
use cairo::Context;

use sdl2::video::{PosCentered, OPENGL};
use sdl2::event::{Quit, poll_event};
//use sdl2::rect::{Rect};
mod sw;

//pub mod components; //TODO: export gui components

/// Struct of a desctop app which is the basic setup
/// it includes everything nedded for a desktop applicatio
pub struct Window{
    delay: uint,//delay betwen event checks default is 10
    pub window: sdl2::video::Window,
}

impl Window{
    //create a new app with a window
    pub fn new(title: &str, width: int, height: int) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);
        Window{
            delay: 12,
            // Create a window
            window: sdl2::video::Window::new(title, PosCentered, PosCentered, width, height, OPENGL).unwrap(),
        }
    }

    pub fn start(&mut self){
        use sdl2::event;
        'event : loop {
            match poll_event() {
                Quit(_) => break 'event,
                event::KeyDown(_, _, KeyCode, ScanCode, _,_) => println!("deydown {} {}", KeyCode, ScanCode),
                event::KeyUp(_, _, _, _, _, _) => println!("key up"),
                event::MouseMotion(_, _, _, _, _, _, _, _) => println!("mouse move"),
                event:: Window(_, _, id, d1, d2) => {
                    //TODO: use this event to redraw on size changes and to sleep
                    println!("window event {} {} {}", id, d1, d2);
                },
                _ => sdl2::timer::delay(self.delay)
            }
        }
        sdl2::quit();
    }

    //calls the given function and provides a cairo drawing context
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        let sdl_surface = self.window.get_surface().unwrap();
        let mut surface = sw::SurfaceWrapper::from_sdl(sdl_surface);//generate a wrapper around the surface
        let mut ctx = Context::new(&mut surface).unwrap();//generate the cairo context

        draw_closure(&mut ctx);

        let _ = self.window.update_surface();//surface updaten
    }

    pub fn render(&mut self, render_closure: |&mut CTX|){
        let mut ctx = CTX{
            window: self,
            pos: (0.0,0.0),
        };
        render_closure(&mut ctx);
    }

    pub fn close(&mut self){
        sdl2::quit();
    }

    //set the title of the window
    pub fn title(&mut self, val: String){
        self.window.set_title(val.as_slice());
    }

    pub fn get_title(&self) -> String{
        self.window.get_title()
    }

    pub fn get_size(&self) -> (int, int){
        self.window.get_size()
    }

    pub fn get_draw_size(&self) -> (int, int){
        self.window.get_drawable_size()
    }
}

/*
//automatically close the window on drop
#[unsafe_destructor]
impl Drop for Window {
    fn drop(&mut self) {
        self.close();
    }
}*/

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

    w.start();//start event loop
}

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget<Event>{
    /// In this function only rendering to the screen and atouching event listeners is done
    /// the state of the component gets passed as a imutable reference, so this rutine is not
    /// able to change anything.
    fn render(&self, ctx: &mut CTX);

    /// should this component be cached in its own frame buffer?
    /// if nothing is set it wil be painted each time it is called
    fn do_cach() -> bool{
        false
    }
}

#[cfg(test)]
mod test{
    use super::Widget;
    struct Button{
        text: String,
        width: f64,
        height: f64,
    }
    enum ButtonEvent{
        Click
    }
    impl Widget<ButtonEvent> for Button{
        fn render(&self, ctx: &mut super::CTX) {
            ctx.draw(|c|{
                c.set_source_rgb(0.35, 0.35, 0.85);
                c.rectangle(0.0, 0.0, self.width, self.height);
                c.fill();
            });
            //ctx.add(1,Rect::new(0,0,100,100),None);
            println!("render");
        }
        //fn get_state(&self) -> (){ () }
    }

    #[test]
    fn show(){
        let mut w = super::Window::new("window-test",640,480);
        w.render(|ctx|{
            let b = Button{text:"test1".to_string(), width: 100.0, height: 30.0};
            ctx.add(1, &b, Some(|evt| println!("click")));
            ctx.pos = (0.0,32.0);
            ctx.add(2, &b, None);
            ctx.pos = (0.0,64.0);
            ctx.add(3, &b, None);
            ctx.pos = (0.0,96.0);
            ctx.add(4, &b, None);
            ctx.pos = (0.0,128.0);
            ctx.add(5, &b, None);
        });
        w.start();//start event loop
        panic!("good");
    }
}

pub struct CTX<'a>{
    window: &'a mut Window,//ref to the window
    pos: (f64,f64),
}

impl<'a> CTX<'a>{
    //this function calls a closure with the cairo drawing context as parameter
    //so the widget can draw some lower level stuff
    pub fn draw(&mut self, draw_closure: |&mut cairo::Context|){
        let (x, y) = self.pos;
        self.window.draw(|ctx|{
            ctx.translate(x, y);
            draw_closure(ctx);
        });
    }

    //add a component as a child of the current component
    // id: this is an unique identifier which is used to manage state and caching
    //     it has to be unique in this component
    // w: the widget which should be added. One widget can be added multible times
    // event: the closure which should be called if the Widget fires an event
    //        normaly widgets use enums which can then be matched an events can be handled
    //        if no event should be catched this should be None
    pub fn add<Event>(&mut self,id: uint, w: &Widget<Event>, event:Option<|Event|>){
        println!("add");
        w.render(self);
        //TODO: event handling strategie überdenken
        //TODO: look for event
        //TODO: call function for event handling

        //TODO: ignore emits if no event handler is given
        //TODO: call render function
    }

    //emit an event which can be handled by the parent element.
    pub fn emit<Event>(&mut self, event: Event){

    }

    //with this function it is possible to mutate the state of the component,
    //it should be avoided in render function, maybe in event handler
    pub fn get_mut_state(){}

    //TODO: add window event subsribing capabilitys
    pub fn on_mouse_over(){}
    pub fn on_mouse_move(){}
    pub fn on_mouse_down(){}
    pub fn on_mouse_up(){}
}

#[test]
fn cairo_test(){

}
