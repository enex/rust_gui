//use time::precise_time_ns;
use sdl2;
use cairo;

use sdl2::video::{OPENGL, WindowPos};
use sdl2::event::{poll_event};
pub use sdl2::event::Event;//reexprot events
use std::any::Any;
use std::collections::HashMap;
use Context;
use App;
use ID;
use context::EventHandle;
use id::NULL_ID;

/// Struct of a desktop app which is the basic setup
/// it includes everything nedded for a desktop application.
/// Sdl2 is used as the interface to the window manager and cairo for rendering.
pub struct Window{
    ///delay betwen event checks default is 10
    pub delay: u32,
    ///sdl2 Window
    pub window: sdl2::video::Window,
    ///id of the currently drawing component
    ctx: cairo::Context,//cairo context for drawing
    pub state: HashMap<ID, Box<Any + 'static>>,
    event_listener: HashMap<ID, Box<Fn(&Event, &mut EventHandle)+'static>>,
    event: Event,
    //id of the currently selected element
    pub focused: ID,
}

/// Events are handled with moved closures as filters that means
/// you can get a streeem of event form the Context and filert it
/// with a function like so:
///     ctx.events(move |event| ..., |event| ...)
/// whrere the first closure filters the closures so only interesting closures are lef
/// an the second calls the closure which does actual event handling inside of the component

impl Window{
    /// create a new window with given title, width and height
    pub fn new(title: &str, width: i32, height: i32) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let window = sdl2::video::Window::new(title, WindowPos::PosCentered, WindowPos::PosCentered, width, height, OPENGL).unwrap();
        let surface = match window.get_surface() {
            Ok(s) => s,
            Err(e) => panic!("could not get window surface {}",e)
        };
        surface.lock();
        let ss = unsafe{surface.raw()};//raw sdl surface
        let cr = unsafe{
            let surface = cairo::ffi::cairo_image_surface_create_for_data(
                (*ss).pixels as *mut u8,
                cairo::ffi::CAIRO_FORMAT_RGB24,
                (*ss).w,
                (*ss).h,
                (*ss).pitch
            );
            if surface.is_null(){
                panic!("could not create surface");
            }
            cairo::Context::from_raw(surface)
        }.unwrap();

        Window{
            delay: 20,//12,
            window: window,
            ctx: cr,
            state: HashMap::new(),
            event_listener: HashMap::new(),
            event: Event::None,
            focused: NULL_ID,
        }
    }

    //TODO: combine show and app so that content is not duplicated
    //TODO: implement event propagation
    //TODO: only redraw if neccessary by tracking changes
    //TODO: only redraw changes and use caching

    /// function which takes the render function to generate the content, and then
    /// listens for input events it will return, if the window has been closed.
    pub fn show<F>(&mut self, render: F) where F:  Fn(&mut Context){
        //self.window.show();
        self.update();

        macro_rules! draw{//macro to draw
            ($e:expr) => ({
                self.cairo_context().save();
                self.cairo_context().paint();
                {
                    let mut c = $e;
                    render(&mut c);
                }
                self.cairo_context().restore();
                self.update();
            });
            () => ({
                draw!(Context::new(self));
            });
        }

        //does event handling and redraw
        macro_rules! handle_event{
            () => ({
                for (id, ref f) in self.event_listener.iter(){
                        let mut e = EventHandle::new(id, &mut self.state, &mut self.focused);
                        (*f)(&self.event, &mut e);
                }
                draw!();
            })
        }

        draw!();

        'main : loop {
            'event : loop {
                self.event = sdl2::event::wait_event().unwrap();
                match self.event {
                    Event::Quit{..} => break 'main,
                    Event::KeyDown{
                        timestamp: _,
                        keycode: key,
                        ..
                    } if key == sdl2::keycode::KeyCode::Escape => {
                        break 'main
                    },
                    Event::None => break 'event,
                    _ => handle_event!()//sdl2::time::delay(self.delay)
                }
            }
        }
        sdl2::quit();
    }

    /// function whic should be used to initialize an app. As the first parameter
    /// A Value implementing App should be given. The app will instatnly run
    /// after calling this function
    pub fn app<A>(&mut self, app: &mut A) where A:App{
        println!("lunch the app");
        self.update();
        //initial draw:
        self.cairo_context().save();
        {
            let mut c = Context::new(self);
            app.render(&mut c);
        }
        self.cairo_context().restore();
        self.update();

        'main : loop {
            'event : loop {
                match sdl2::event::poll_event() {
                    Event::Quit{..} => break 'main,
                    Event::KeyDown{
                        timestamp: _,
                        keycode: key,
                        ..
                    } => {
                        if key == sdl2::keycode::KeyCode::Escape {
                            break 'main
                        }
                    },
                    Event::None => break 'event,
                    _ => sdl2::timer::delay(self.delay)
                }
            }
        }
        sdl2::quit();
    }

    /// search the state of a component by Id
    pub fn find_state<T>(&self, id: ID) -> Option<&T> where T:Any+'static{
        use state::UncheckedAnyRefExt;
        self.state.get(&id).map(|any|{
            unsafe{ (*any).downcast_ref_unchecked::<T>()}
        })
    }

    /// set the state
    pub fn set_state<T>(&mut self, id: ID, v: Box<T>) where T:Any+Clone+Eq+'static{
        self.state.insert(id, v as Box<Any + 'static>);
    }

    pub fn register_event_listener<F>(&mut self, id: ID, listener: Box<F>) where F: Fn(&Event, &mut EventHandle)+'static{
        self.event_listener.insert(id, listener);
    }

    /// get the ciro drawing context to draw on it
    #[stable]
    #[inline(always)]
    pub fn cairo_context(&mut self) -> &mut cairo::Context{
        &mut self.ctx
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
        self.window.set_title(&val[0..]);
    }

    /// get title of the window
    #[inline(always)]
    pub fn get_title(&self) -> String{
        self.window.get_title()
    }

    /// get the size of the window
    #[inline(always)]
    pub fn get_size(&self) -> (i32, i32){
        self.window.get_size()
    }

    //get size of the canvas to draw on
    #[inline(always)]
    pub fn get_draw_size(&self) -> (i32, i32){
        self.window.get_drawable_size()
    }
}
