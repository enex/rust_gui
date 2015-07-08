use std::collections::HashMap;
use std::any::Any;

#[derive(Clone, Debug)]
struct Info;

#[derive(Debug)]
pub struct Window{
    state: Node,
    info: Info
}

pub struct Context<'a>{
    node: Option<&'a mut Node>,
    need_node: bool,
    info: &'a mut Info,
    id: u32
}

impl Window{
    /// create a new window
    pub fn new() -> Window{
        Window{
            state: Node{
                children: HashMap::new(),
                state: None,
                events: NO_EVENT
            },
            info: Info
        }
    }
    pub fn context<'a>(&'a mut self) -> Context<'a>{
        Context{
            node: Some(&mut self.state),
            info: &mut self.info,
            need_node: false,
            id: 0
        }
    }
}

impl<'a> Context<'a>{
    /// add a new component to the Context with a given id to save the state
    pub fn add<W:Widget>(&mut self, id:u32, w: &W){
        let mut c = Context{
            id: id,
            info: &mut self.info,
            need_node: false,
            node: None //TODO: fatch the correct node
        };
        w.render(&mut c);
        if c.need_node{
            //TODO: create the node if this component defines a node
            //      or return with need node to get a node
            self.need_node = true;
            return;
        }
        //TODO: add the widget
    }
    /// function returning the state and the defaults for a Component
    pub fn get<W:Widget+Any>(&self, id:u32) -> W{
        self.default()
    }
    /// set the falues of a component, you should use get most of the time
    /// because if there is a state it will not be saved this way
    pub fn default<W:Widget+Any>(&self) -> W{
        //TODO: take the one already saved
        W::init()
    }
    /// this makes it possible to nest components inside
    /// it should be used when a component has children
    pub fn child<F:Fn(&mut Context)>(&mut self, id: u32, f: F){
        unimplemented!()
    }
    #[cfg(test)]
    pub fn update(&mut self){
        panic!("you are only allowed to call update if events are beeing used");
    }
    /// update the stored state if you changed the state, you are only allowed
    /// to call this function on event handling. Doing it on the normal render
    /// can result in infinite loops and will thus panic in test builds
    #[cfg(not(test))]
    pub fn update(&mut self){
        unimplemented!()
    }
}

const NO_EVENT: u8 = 0;
const MOUSE_HOVER: u8       = 0b00000001;
const MOUSE_BUTTON_DOWN: u8 = 0b00000010;
const MOUSE_BUTTON_UP: u8   = 0b00000100;
const KEY_DOWN: u8          = 0b00001000;
const KEY_UP: u8            = 0b00010000;
const DROP: u8              = 0b00100000;

/// a struct storing all the information necessarry to save the state
#[derive(Debug)]
struct Node{
    children: HashMap<usize, Node>,
    state: Option<Box<Any>>,
    events: u8,
    //TODO: add something like a hash to identify changes
    //TODO: add cached data info
}

pub trait Widget: Clone{
    /// copy the state of the widget
    fn copy_state(&mut self, _: &Self){}
    /// the function called on every widget to render everything
    fn render(&self, &mut Context);
    /// the initial value of the state and properties as well as style
    fn init() -> Self;

    fn draw(&self, c: &mut Context, id: u32){
        c.add(id, self)
    }
}

#[cfg(test)]
mod test{
    use super::{Widget, Window, Context};

    /// the state of a button
    #[derive(Clone,Debug)]
    enum ButtonState{
        Hovered,
        Pressed,
        Leaved
    }

    #[derive(Clone, Debug)]
    struct Button<'a>{
        pub text: &'a str,
        pub state: ButtonState,
    }

    impl<'a> Widget for Button<'a>{
        fn copy_state(&mut self, o: &Button<'a>){
            self.state = o.state.clone()
        }
        fn render(&self,c: &mut Context){
            println!("render")
        }
        fn init() -> Button<'a>{
            Button{
                text: "", state: ButtonState::Leaved
            }
        }
    }

    #[test]
    fn test_widget_init() {
        let b = Button{
            text: "Hallo Welt",
            ..Widget::init()
        };
        println!("{:?}",b);
    }

    #[test]
    fn test_simple_context() {
        let mut w = Window::new();
        let mut c = w.context();

        Button{
            text: "test-Button",
            ..c.get(1)
        }.draw(&mut c, 1);
    }

    #[test]
    fn test_update_on_none_event_context(){
        let mut w = Window::new();
        let mut c = w.context();
        c.update();
    }
}
