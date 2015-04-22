use Widget;
use ID;
use glutin;
use draw;
use std::any::Any;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;
use Backend;

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
pub trait EventHandle<W:Widget>{
	/// emit a event of the specified event type that will then be propagated to the
	/// Widget which added this Widget
	fn emit(&mut self, e: W::Event);
}

/// context which the programmer uses to add widgets or to draw someting or to acces
/// the state.
pub trait Context{
	/// Target widget the widget this context belongs to
	type TWidget: Widget;

	/// add a component
	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, id: u16, _: NW){}

	/// add with event adds a component and listen to events fired from this component
	fn awe<NW:Widget<State=NS>,NS:StateT,L:Fn(NW::Event, &mut EventHandle<NW>)>(&mut self, id: u16, w: NW, _:L){
		self.add(id, w);
	}

	/// draw a path
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, _: draw::Path<I,V>){}

	/// get the current state
	//fn state(&mut self) -> &Self::TWidget::State{unimplemented!()}

	/// returns true if the element is currently focused, if not it returns false
	fn focused(&self) -> bool;

	/// whether the element is hovered at the moment
	fn hovered(&self) -> bool;

	/// the id of the current component
	fn id(&self) -> ID;

	/// returns the default for every widget. This is also the way how
	/// theming is implemented. If the style sais this should have some
	/// specific parameters, then these are returned as a default.
	/// but theming is not jet implemented
	fn default<D:Default>(&self) -> D{
		Default::default()
	}

	/// # event listeners

	/// register event listener for click event
	fn on_click<F:Fn(Pos, &mut EventHandle<Self::TWidget>)>(&mut self, f: F){}
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
}

impl<D:Backend> Common<D>{
	fn push(&mut self, id: u16){
		self.id[self.depth as usize] = id;
		self.depth += 1;
		if self.depth > 12{
			panic!("id is to big, to much nesting used.");
		}
	}
	fn pop(&mut self){
		self.depth -= 1;
		self.id[self.depth as usize] = 0;
	}
}

/// context used to draw everything on screen
/// this will be passed to a component if this component should be drawn
//TODO: simplify
pub struct DrawContext<'a, D:Backend, W:Widget> where D:'a{
	c: &'a mut Common<D>,
	e: PhantomData<W>,
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT> DrawContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> DrawContext<'a, D, W>{
		DrawContext{
			c:c,
			e: PhantomData,
		}
	}
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT>Context for DrawContext<'a, D, W>{
	type TWidget = W;

	fn id(&self) -> ID{
		self.c.id
	}
	fn focused(&self) -> bool{
		self.c.state.focused == self.id()
	}
	fn hovered(&self) -> bool{
		self.c.state.hovered == self.id()
	}
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, path: draw::Path<I,V>){
		self.c.be.draw_path(path);
	}

	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, id: u16, w: NW){
		{
			self.c.push(id);

			//println!("add: {:?} as {:?}", NW::name(), nid);
			let mut c:DrawContext<D, NW> = DrawContext{
				c: self.c,
				e: PhantomData,
			};

			w.render(&mut c);
		}

		self.c.pop();
	}
}

//TODO: group constant data together so that only one pointer is needed per context
//TODO: implement emit with a struct (used as trait object) with closure and context associated

/// context used to handle events. This will be passed to every Widget on the way to
/// the event receiver
pub struct EventContext<'a, D:Backend, W:Widget> where D:'a{
	c: &'a mut Common<D>,
	p: PhantomData<W>,
	emit: Option<&'a Fn(W::Event, &mut EventHandle<W>)>,
	//TODO: optionaly also emit to parent
}

impl<'a, D:Backend, W:Widget> EventContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> EventContext<'a, D, W>{
		println!("event context");

		EventContext{
			c: c,
			p: PhantomData,
			emit: None,
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

	fn on_click<F:Fn(Pos, &mut EventHandle<W>)>(&mut self, f: F){
		//use EventContext itsselve as EventHandle
		let s:&mut EventHandle<W> = self;
		(f)(Pos{x:10.,y:10.}, s);
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

	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, id: u16, w: NW){
		{
			self.c.push(id);

			let mut c:EventContext<D, NW> = EventContext{
				c: self.c,
				p: PhantomData,
				emit: None,
				//emit: None,
			};

			w.render(&mut c);
		}
		self.c.pop();
	}
	//TODO: amke mit correct by make EventHandle<W>
	fn awe<NW:Widget<State=NS>,NS:StateT,L:Fn(NW::Event, &mut EventHandle<NW>)>
		(&mut self, id: u16, w: NW, f:L){
		{
			self.c.push(id);
			println!("awe: {:?}  //event context", W::name());

			let mut c:EventContext<D, NW> = EventContext{
				c: self.c,
				p: PhantomData,
				emit: Some(&f),
			};

			w.render(&mut c);
		}
		self.c.pop();
	}
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT>EventHandle<W> for EventContext<'a, D, W>{
	fn emit(&mut self, e: W::Event){
		match self.emit{
			Some(f) => (f)(e, self),
			None => ()
		}
	}
}

/*
/// hack to use a trait object for error handling
struct Emiter<'a,'b, Ev, W:Widget>{
	f: &'a Fn(Ev, &mut EventHandle<'b, W>)
}
trait EEmi<E>{
	fn emit<'b>(&'b self, E, &'b mut AppState);
}
impl<'a,'b, Ev, W:Widget<State=S>, S:StateT> EEmi<Ev> for Emiter<'a,'b, Ev, W>{
	fn emit<'c>(&'c self, e: Ev, state: &'c mut AppState){
		println!("emit the event");
		//let mut eh:EventHandle<W> = EventHandle::new(&self.id, &mut state);
		//(self.f)(e, &mut eh)
		//TODO: implement calling correctly and create EventHandle
	}
}*/

/*
/// this struct is used to handle events. Every registered event handler will
/// get a instance of this struct. This way it can for example propagate
pub struct EventHandle<'a, W:Widget>{
	id: &'a ID,
	state: &'a mut AppState,
	/// function of the parent, called when emit gets called
	emit: Option<&'a EEmi<W::Event>>,
	//event_ctx: &mut EventContext<D, W>,
}

impl<'a, W:Widget<State=State,Event=E>, State:'a + Any + Default, E> EventHandle<'a, W>{
	pub fn new(id: &'a ID, state: &'a mut AppState) -> EventHandle<'a, W>{
		EventHandle{
			id: id,
			state: state,
			emit: None,
		}
	}

	//TODO: make sure the type of state is correct and use the information given
	/*
	/// get the state of the component immutable
	pub fn state(&'a mut self) -> &'a State {
		self.state.get(&self.id)
	}

	/// get mutable reference to state of the widget this marks the widget as
	/// dirty and it will be rerendered.
	pub fn mut_state(&'a mut self) -> &'a mut State {
		self.state.get_mut(&self.id)
	}
	*/
	/// emit an event for parent widgets this event can then be catched by the
	/// parent and can be used to set the state of the widget or to propagate further
	pub fn emit(&mut self, e: E){
		match self.emit{
			Some(ev) => {
				ev.emit(e, self.state)
			},
			_ => ()
		}
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
*/
