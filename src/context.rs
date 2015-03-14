use Widget;
use Event;
use ID;
use std::any::Any;
use std::collections::HashMap;
use glutin;
use nanovg;
use Ctx;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;

pub struct Context<'a, Event, State>{
	depth: u8,
	id: ID,
	//x: f32,
	//y: f32,
	ctx: &'a mut Ctx,
	state: &'a AppState,

	e: PhantomData<Event>,
	s: PhantomData<State>,
}

impl<'a, Event, State> Context<'a, Event, State>{
	pub fn new(ctx: &'a mut Ctx, state: &'a AppState) -> Context<'a, Event, State>{
		Context{
			depth: 0,
			id: [0; 12],
			//x: 0.,
			//y: 0.,
			ctx: ctx,
			state: state,
			e: PhantomData,
			s: PhantomData
		}
	}

	/// this function allows accessing the drawing context directly
	/// by default it does nothing
	pub fn draw<F>(&mut self, draw: F) where F: Fn(&mut Ctx) {
		(draw)(self.ctx)
	}


	/// Add a new component.
	/// The id has to be unique in this component
	#[inline(always)]
	pub fn add<W,E,S>(&mut self, id: u16, widget: &W) where W:Widget<Event=E,State=S>{
		let mut nid = self.id;
		nid[self.depth as usize] = id;

		let mut c:Context<E,S> = Context{
			id: nid,
			depth: self.depth+1,
			ctx: self.ctx,
			state: self.state,
			e: PhantomData,
			s: PhantomData
		};
		if c.depth > 11{
			panic!("the structure is to deep only a 12 child deep tree is allowed");
		}
		widget.render(&mut c);
	}

	/*/// add element at a given position
	#[inline(always)]
	pub fn add_at<E,S>(&mut self, id: u16, widget: &Widget<Event=E,State=S>, x: f32, y: f32) {
		println!("add_at x:{} y:{}",x,y);
		self.add(id, widget)
	}*/

	/// the id of the current component
	#[stable]
	#[inline(always)]
	pub fn id(&self) -> ID{
		self.id
	}


	/// this function registers a event listener, the closure is called
	/// on every event it should decide if the event is relevant and then it should
	/// return true otherwhise it should do nothing
	/// If variables in its scope should be used the enviroment has to be
	/// captured by using the move keyword like so: move |event| ...
	pub fn on<F>(&mut self, filter: Box<F>)
			where F: Fn(&glutin::Event, &mut EventHandle<Event, State>)+'static {
		let s = self.id();
		//TODO: check if not already exsisting
		unimplemented!();
		//self.window.register_event_listener(s, filter)
	}

	/// returns true if the element is currently focused, if not it returns false
	pub fn focused(&self) -> bool{
		self.state.focused == self.id
	}

	/// wether the element is hovered at the moment
	pub fn hover(&self) -> bool{
		self.state.hovered == self.id
	}


	/// get the state of the current component, if it has not been set by
	/// an event it has the default value.
	pub fn state(&self) -> &State{
		use state::UncheckedAnyRefExt;
		unimplemented!();
		/*self.state.state.get(&self.id).map(|any|{
			unsafe{ (*any).downcast_ref_unchecked::<State>()}
		})*/
	}

	/// get the current position
	pub fn pos(&mut self) -> (f64, f64){
		let x = 0.;
		let y = 0.;

		(x, y)
	}
	pub fn pos_x(&mut self) -> f64{let (x,_) = self.pos(); x}
	pub fn pos_y(&mut self) -> f64{let (_,y) = self.pos(); y}
}


pub struct EventHandle<'a, Event, State>{
	id: &'a ID,
	state: &'a mut AppState,
	e: PhantomData<Event>,
	s: PhantomData<State>,
}

impl<'a, Event, State:'a> EventHandle<'a, Event, State> where State: Default + 'static{
	pub fn new(id: &'a ID, state: &'a mut AppState) -> EventHandle<'a, Event, State>{
		EventHandle{
			id: id,
			state: state,
			e: PhantomData,
			s: PhantomData
		}
	}

	//TODO: make shure the type of state is correct and use the information given

	/// get the state of the component immutable
	pub fn state(&'a mut self) -> &'a State {
		self.state.get(&self.id)
	}
	/// get mutable reference to state of the widget
	pub fn mut_state(&'a mut self) -> &'a mut State {
		self.state.get_mut(&self.id)
	}

	/// emit an event for parent widgets
	pub fn emit(&mut self, e: Event){
		unimplemented!();
	}

	/// set the widget as focused, so keyboard events can be catched by this
	/// component or parent components
	pub fn focus(&mut self){
		self.state.focused = *self.id
	}

	/// wether the current component is focused or not
	pub fn focused(&self) -> bool{
		self.state.focused == *self.id
	}

	/// set the cursor to one of the system cursors.
	pub fn set_cursor(&mut self, cursor: glutin::MouseCursor){
		unimplemented!();
		//TODO: make this work somehow
	}
}
