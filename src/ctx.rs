use sdl2::event::Event;//reexprot events
use sdl2;
use std::any::Any;
use Widget;
use Window;
use cairo;

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
    #[inline]
    pub fn add<Event>(&mut self,id: uint, w: &mut Widget<Event>, event:Option<|Event|>) -> (f64, f64){
        //println!("add");
        //TODO: create a sub context
        self.window.current_id.push(id);//keep track of the id

        //do the rendering
        let (x, y) = self.pos;
        self.window.cairo_context().save();
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

        self.window.current_id.pop();
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

    ///actual absolute position relativ to the top left corner of the window
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

    ///set current object as focused. So the keyboard events will be sent to this element.
    #[inline(always)]
    pub fn focus(&mut self){
        self.window.focused = self.window.current_id.clone();
    }

    ///wether the element is focused
    #[inline(always)]
    pub fn is_focused(&mut self) -> bool{
        self.window.focused == self.window.current_id
    }

    ///Register an event listener for key events.
    #[inline(always)]
    pub fn keyevent(&mut self, handle: |&sdl2::event::Event, &mut CTX<Event>|){
        if self.window.focused == self.window.current_id{
            handle(self.event, self);
        }
    }

    ///with this function it is possible to mutate the state of the component,
    ///it should be avoided in render function, maybe in event handler
    pub fn get_mut_state<T:Any+Clone+Eq>() -> Option<Box<T>>{
        unimplemented!();
    }

    pub fn set_state<T:Any+Clone+Eq>(&mut self, state: T) {
        let ci = self.window.current_id.clone();
        self.window.set_state(ci, state)
    }

    pub fn get_state<T>(&self) -> Option<T>{
        let ci = self.window.current_id.clone();
        self.window.find_state(ci)
    }

    /// register mouse event listener
    //TODO: add window event subsribing capabilitys
    pub fn mouseover(&mut self, size: (f64,f64), handle: |&sdl2::event::Event, &mut CTX<Event>|){
        if !self.should_handle{
            return;
        }
        //TODO: register instead of handling directly
        //TODO: only call callback if in handling mode

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
