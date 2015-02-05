use Window;
use cairo;
use Widget;
use Event;
use ID;
use id::NullID;

pub struct Context<'a>{
    window: &'a mut Window,
    depth: u8,
    id: ID,
    event: bool,
}

impl<'a> Context<'a>{
    pub fn new(window: &'a mut Window) -> Context<'a>{
        println!("neuer Context");
        Context{
            window: window,
            depth: 0,
            id: NullID,
            event: false,
        }
    }

    pub fn new_event(window: &'a mut Window) -> Context<'a>{
        Context{
            window: window,
            depth: 0,
            id: NullID,
            event: true,
        }
    }

    /// this function allows accessing the drawing context directly
    /// by default it does nothing
    pub fn draw<F>(&mut self, draw: F) where F: Fn(&mut cairo::Context) {
        if !self.event{
            draw(self.window.cairo_context())
        }
    }

    /// Add a new component.
    /// The id has to be unique in this component
    pub fn add/*<F, Ev>*/(&mut self, id: u16, widget: &Widget/*, then: Option<F>*/) /*where F: Fn(&Ev)*/ {
        println!("add as child node");
        let mut nid = self.id;
        nid[self.depth as usize] = id;
        let mut c = Context{
            window: self.window,
            depth: self.depth+1,
            id: nid,
            event: self.event,
        };
        if c.depth > 11{
            panic!("the structure is to deep only a 12 child deep tree is allowed");
        }
        widget.render(&mut c);
    }

    /// Add a Component with given mutable state
    /// This is usefull if managing the whole application from the root component is
    /// Too complicated and splitting it to components is perefered.
    pub fn add_with_state(&mut self, id: u16, widget: &Widget){

    }

    /// the id of the current component
    pub fn id(&self) -> ID{
        NullID
    }

    /// this function registers a event listener, the closure is called
    /// on every event it should decide if the event is relevant and then it should
    /// return true otherwhise it should do nothing
    /// If variables in its scope should be used the enviroment has to be
    /// captured by using the move keyword like so: move |event| ...
    pub fn on<F>(&mut self, filter: Box<F>) where F: Fn(&Event)+'static {
        let s = self.id();
        self.window.register_event_listener(s, filter)
    }
}
