use Widget;
use ID;
use draw;
use std::any::Any;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;
use backend::Backend;
use Transform;
use draw::AsPath;
use glutin::{Event, ElementState, VirtualKeyCode};

/// representing a position, can be returned as value form on_click for example
#[derive(Clone, Debug, Default, Copy)]
pub struct Pos{
	pub x: f32,
	pub y: f32,
}

/// Type of the state, just to simplify Type declaration
pub trait StateT: Any + Default{}
impl<A:Any + Default> StateT for A{}

/// this trait object is passed to a event hanling function to handle the event
pub trait EventHandle<W:Widget> where W::State: Any+Default{
	/// emit a event of the specified event type that will then be propagated to the
	/// Widget which added this Widget
	fn emit(&mut self, e: W::Event);

	fn mut_state<'a>(&mut self) -> &'a mut W::State {
		unimplemented!()
	}
	/// shortcut for mut_state
	fn ms<'a>(&mut self) -> &'a mut W::State{
		self.mut_state()
	}
}

/// context which the programmer uses to add widgets or to draw someting or to acces
/// the state.
pub trait Context{
	/// Target widget the widget this context belongs to
	type TWidget: Widget;
	type Backend: Backend;

	/// add a component
	fn add<NW:Widget>(&mut self, _: u16, _: &NW) where NW::State: StateT{}

	/// add with event adds a component and listen to events fired from this component
	fn awe<NW:Widget,L:Fn(&NW::Event, &mut EventHandle<Self::TWidget>)>(&mut self, id: u16, w: &NW, _:L) where NW::State: StateT{
		self.add(id, w);
	}

	/// draw a path
	fn draw_path<P:AsPath>(&mut self, _: P){}

	/// returns true if the element is currently focused, if not it returns false
	fn focused(&self) -> bool;

	/// whether the element is hovered at the moment
	fn hovered(&self) -> bool;

	/// the id of the current component
	fn id(&self) -> ID;

	/// direct access to the backend or the closure won't be called, this is true
	/// on the EventContext where nothing needs to be drawn
	fn draw<F:Fn(&mut Self::Backend)>(&mut self, F){}

	/// translate the context so the following Widgets beeing drawn are positioned
	/// after the new origin givent. This is relativ to the own origin.
	fn translate(&mut self, x:f32, y:f32){}

	/// apply a transformation to the context
	fn transform(&mut self, Transform);
	/// resets all transformations done in this context.
	fn reset(&mut self);

	/// returns the default for every widget. This is also the way how
	/// theming is implemented. If the style sais this should have some
	/// specific parameters, then these are returned as a default.
	/// but theming is not jet implemented
	fn default<D:Default>(&self) -> D{
		Default::default()
	}

	/// directly draw some text to a given position
	fn text(&mut self, _:f32, _:f32, _:&str) -> f32{0.}
	/// change the font face to the given one
	fn font_face(&mut self, _:&str){}

	/// # event listeners

	/// register event listener for click event
	fn on_click<F:Fn(Pos, &mut EventHandle<Self::TWidget>)>
		(&mut self, _:f32, _:f32, _:f32, _:f32, _: F){}
	
	/// register event listener for key event
	fn on_key<F:Fn(ElementState, Option<VirtualKeyCode>, &mut EventHandle<Self::TWidget>)>
		(&mut self, _: F){}
	
	/// register event listener for key char event
	fn on_char<F:Fn(char, &mut EventHandle<Self::TWidget>)>
		(&mut self, _: F){}
	
	/// whether the context realy draws, this can be used to optimize drawing
	fn is_drawing(&self) -> bool{ false }
}

#[derive(Debug, Clone)]
pub enum EventFilter{
	/// a mouse event in the area specified
	Mouse(f32, f32, f32, f32)
}

/// data shared betwen contexts D:Backend the backend
pub struct Common<D:Backend>{
	/// the whole application state
	pub state: AppState,
	/// ide of the current element
	pub id: ID,
	/// deph of the current element
	pub depth: u8,
	/// backend of the application
	pub be: D,
	/// the global Transformation
	pub transform: Transform,
	/// event listeners
	pub listeners: Vec<(ID,EventFilter)>,
	/// the event thrown
	pub event: Event,
	/// the widgets affected this will be generated based on listeners
	pub affected: Vec<ID>,
	/// widgets that should be redrawn
	pub redraw: Vec<ID>,
	/// the current mouse positon
	pub mouse_pos: (f32, f32),
}

impl<D:Backend> Common<D>{
	fn push(&mut self, id: u16){
		self.id.0[self.depth as usize] = id;
		self.depth += 1;
		if self.depth > 12{
			panic!("id is to big, to much nesting used.");
		}
	}
	fn pop(&mut self){
		self.depth -= 1;
		self.id.0[self.depth as usize] = 0;
	}
}

/// context used to draw everything on screen
/// this will be passed to a component if this component should be drawn
//TODO: simplify
pub struct DrawContext<'a, D:Backend, W:Widget> where D:'a{
	c: &'a mut Common<D>,
	transform: Transform,
	e: PhantomData<W>,
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT> DrawContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> DrawContext<'a, D, W>{
		DrawContext{
			transform: c.be.get_transform(),
			c:c,
			e: PhantomData,
		}
	}
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT>Context for DrawContext<'a, D, W>{
	type TWidget = W;
	type Backend = D;

	fn id(&self) -> ID{
		self.c.id
	}
	fn focused(&self) -> bool{
		self.c.state.focused == self.id()
	}
	fn hovered(&self) -> bool{
		self.c.state.hovered == self.id()
	}
	fn draw_path<P:AsPath>(&mut self, path: P){
		self.c.be.draw_path(path);
	}

	fn text(&mut self, x:f32, y:f32, text: &str) -> f32{
		self.c.be.text(x, y, text)
	}

	fn font_face(&mut self, name: &str){
		self.c.be.font_face(name)
	}

	fn transform(&mut self, t:Transform){
		self.c.be.set_transform(t*self.transform);
	}
	fn translate(&mut self, x: f32, y: f32){
		self.c.be.set_transform(self.transform.translate(x,y));
		//println!("{:?}", self.c.be.current_transform());
	}

	fn reset(&mut self){
		self.c.be.set_transform(self.transform);
	}
	fn add<NW:Widget>(&mut self, id: u16, w: &NW) where NW::State: StateT{
		{
			self.c.push(id);

			//println!("add: {:?} as {:?}", NW::name(), nid);
			let mut c:DrawContext<D, NW> = DrawContext::new(self.c);
			let d = Default::default();
			w.render(&mut c, &d);
			c.reset();
		}
		self.c.pop();
	}
	fn draw<F:Fn(&mut D)>(&mut self, f: F){
		(f)(&mut self.c.be)
	}

	fn on_click<F:Fn(Pos, &mut EventHandle<Self::TWidget>)>(&mut self,x:f32,y:f32,w:f32,h:f32, _: F){
		let p = self.transform.point(x,y);
		//TODO: remove unused event listeners
		self.c.listeners.push((self.c.id, EventFilter::Mouse(p.0,p.1,w,h)));
		//println!("register event listener {:?} {:?} {:?}", self.c.id,((x,y), (w,h)), p);
	}
	fn is_drawing(&self) -> bool{ true }
}


//TODO: group constant data together so that only one pointer is needed per context
//TODO: implement emit with a struct (used as trait object) with closure and context associated

/// context used to handle events. This will be passed to every Widget on the way to
/// the event receiver
pub struct EventContext<'a, D:Backend, W:Widget> where D:'a{
	c: &'a mut Common<D>,
	pub  emited: Vec<W::Event>,
	transform: Transform,
}

impl<'a, D:Backend, W:Widget> EventContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> EventContext<'a, D, W>{
		EventContext{
			transform: c.be.get_transform(),
			c: c,
			emited: Vec::new(),
		}
	}
	/// wether the state of the component has changed, if it returns true the component
	/// has to be redrawn
	pub fn state_changed(&self) -> bool{
		true
	}
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT>Context for EventContext<'a, D, W>{
	type TWidget = W;
	type Backend = D;

	fn on_click<F:Fn(Pos, &mut EventHandle<W>)>(&mut self,x1:f32,y1:f32,x2:f32,y2:f32, f: F){
		//use EventContext itsselve as EventHandle
		let mut t = self.c.be.get_transform();
		assert_eq!(t, self.transform);
		let p1 = t.point(x1, y1);
		let p2 = t.point(x2, y2);

		let p = Pos{
			x:self.c.mouse_pos.0,
			y:self.c.mouse_pos.1
		};
		if !(p1.0 <= p.x && p1.1 <= p.y && p2.0 >= p.x && p2.1 >= p.y){
			//println!("not in box {:?}", (p, p1, p2, t));
			return;
		}
		//println!("is in the box {:?}", (p, p1, p2));
		let s:&mut EventHandle<W> = self;
		(f)(p, s);
	}
	fn on_key<F:Fn(ElementState, Option<VirtualKeyCode>, &mut EventHandle<Self::TWidget>)>
		(&mut self, f: F){
		match self.c.event{
			Event::KeyboardInput(es, _ , vkc) => {
				let s:&mut EventHandle<W> = self;
				(f)(es, vkc, s);
			},
			_ => ()
		}
	}
	
	fn on_char<F:Fn(char, &mut EventHandle<Self::TWidget>)>
		(&mut self, f: F){
		match self.c.event{
			Event::ReceivedCharacter(c) => {
				let s:&mut EventHandle<W> = self;
				(f)(c, s);
			},
			_ => ()
		}
	}

	fn id(&self) -> ID{
		self.c.id
	}
	fn focused(&self) -> bool{
		self.c.state.focused == self.id()
	}
	fn hovered(&self) -> bool{
		self.c.state.hovered == self.id()
	}
	fn transform(&mut self, t:Transform){
		unimplemented!()
	}
	fn translate(&mut self, x: f32, y: f32){
		self.c.be.set_transform(self.transform.translate(x,y));
	}
	fn reset(&mut self){
		self.c.be.set_transform(self.transform);
	}
	fn add<NW:Widget>(&mut self, id: u16, w: &NW) where NW::State: StateT{
		{
			self.c.push(id);

			let mut c:EventContext<D, NW> = EventContext::new(self.c);
			let d = Default::default();
			w.render(&mut c, &d);
			c.reset();
		}
		self.c.pop();
	}
	fn awe<NW:Widget,L:Fn(&NW::Event, &mut EventHandle<W>)>
		(&mut self, id: u16, w: &NW, f:L) where NW::State: StateT{
		let emited = {
			self.c.push(id);
			//println!("awe: {:?}  //event context", W::name());

			let mut c:EventContext<D, NW> = EventContext::new(self.c);
			let d = Default::default();
			w.render(&mut c, &d);
			c.reset();
			c.emited
		};
		self.c.pop();

		for e in emited.iter(){// call the event handler for each event
			(f)(e, self as &mut EventHandle<W>);
		}
	}
}

impl<'a, D:Backend, W:Widget<State=S,Event=E>,S:StateT, E>EventHandle<W> for EventContext<'a, D, W>{
	fn emit(&mut self, e: E){
		self.emited.push(e);
	}
}
