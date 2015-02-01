use Window;
use cairo;
use Widget;
use Event;

pub struct Context<'a>{
    window: &'a mut Window,
    depth: u8,
}

impl<'a> Context<'a>{
    pub fn new(window: &'a mut Window) -> Context<'a>{
        println!("neuer Context");
        Context{
            window: window,
            depth: 0
        }
    }

    ///this function allows accessing the drawing context directly
    ///by default it does nothing
    pub fn draw<F>(&mut self, draw: F) where F: Fn(&mut cairo::Context) {
        draw(self.window.cairo_context())
    }

    ///Add a new component, it will return an event, if it throws one
    ///the id has to be unique in this component
    pub fn add/*<F, Ev>*/(&mut self, id: u16, widget: &Widget/*, then: Option<F>*/) /*where F: Fn(&Ev)*/ {
        println!("add as child node");
        let mut c = Context{
            window: self.window,
            depth: self.depth+1,
        };
        if c.depth > 11{
            panic!("the structure is to deep only a 12 child deep tree is allowed");
        }
        widget.render(&mut c);
    }

    ///this function registers a event listener, the closure is called
    ///on every event
    ///If variables in its scope should be used the enviroment has to be
    ///captured by using the move keyword like so: move |event| ...
    pub fn on<F>(&mut self, then: F) where F: Fn(&Event) {

    }
}
