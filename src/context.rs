use Widget;
use ID;
use draw;
use std::any::Any;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;
use Backend;
use Transform;

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
	type Backend: Backend;

	/// add a component
	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, _: u16, _: &NW){}

	/// add with event adds a component and listen to events fired from this component
	fn awe<NW:Widget<State=NS>,NS:StateT,L:Fn(NW::Event, &mut EventHandle<NW>)>(&mut self, id: u16, w: &NW, _:L){
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

	/// direct access to the backend or the closure won't be called, this is true
	/// on the EventContext where nothing needs to be drawn
	fn draw<F:Fn(&mut Self::Backend)>(&mut self, F){}

	/// translate the context so the following Widgets beeing drawn are positioned
	/// after the new origin givent. This is relativ to the own origin.
	fn translate(&mut self, x:f32, y:f32){
		let t = Transform::translated(x,y);
		self.transform(t);
	}

	fn scale(&mut self, x:f32, y:f32){
		unimplemented!()
	}

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
	fn on_click<F:Fn(Pos, &mut EventHandle<Self::TWidget>)>(&mut self, _: F){}
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
	transform: Transform,
	e: PhantomData<W>,
}

impl<'a, D:Backend, W:Widget<State=S>,S:StateT> DrawContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> DrawContext<'a, D, W>{
		DrawContext{
			transform: Transform::normal(),
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
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, path: draw::Path<I,V>){
		self.c.be.draw_path(path);
	}

	fn text(&mut self, x:f32, y:f32, text: &str) -> f32{
		self.c.be.text(x, y, text)
	}

	fn font_face(&mut self, name: &str){
		self.c.be.font_face(name)
	}

	fn transform(&mut self, t:Transform){
		self.transform.multiply(t);
		self.c.be.transform(t);
		println!("self: {:?} {:?}",t, self.transform+t);
		println!("{:?}", self.c.be.current_transform());
	}

	fn reset(&mut self){
		self.c.be.transform(-self.transform);
		self.transform = Transform::null();
	}
	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, id: u16, w: &NW){
		{
			self.c.push(id);

			//println!("add: {:?} as {:?}", NW::name(), nid);
			let mut c:DrawContext<D, NW> = DrawContext::new(self.c);
			w.render(&mut c);
			c.reset();
		}

		self.c.pop();
	}
	fn draw<F:Fn(&mut D)>(&mut self, f: F){
		(f)(&mut self.c.be)
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
	transform: Transform,
	//TODO: optionaly also emit to parent
}

impl<'a, D:Backend, W:Widget> EventContext<'a, D, W>{
	pub fn new(c: &'a mut Common<D>) -> EventContext<'a, D, W>{
		println!("event context");

		EventContext{
			transform: Transform::normal(),
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
	type Backend = D;

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
	fn transform(&mut self, t:Transform){
		self.transform = self.transform + t;
		self.c.be.transform(t);
	}
	fn reset(&mut self){
		self.c.be.transform(self.transform);
	}
	fn add<NW:Widget<State=NS>,NS:StateT>(&mut self, id: u16, w: &NW){
		{
			self.c.push(id);

			let mut c:EventContext<D, NW> = EventContext::new(self.c);

			w.render(&mut c);
			c.reset();
		}
		self.c.pop();
	}
	//TODO: make mit correct by make EventHandle<W>
	fn awe<NW:Widget<State=NS>,NS:StateT,L:Fn(NW::Event, &mut EventHandle<NW>)>
		(&mut self, id: u16, w: &NW, f:L){
		{
			self.c.push(id);
			println!("awe: {:?}  //event context", W::name());

			let mut c:EventContext<D, NW> = EventContext{
				transform: Transform::normal(),
				c: self.c,
				p: PhantomData,
				emit: Some(&f),
			};

			w.render(&mut c);
			c.reset();
		}
		self.c.pop();
	}
	/*fn draw<F:Fn(&mut D)>(&self, f: F){
		(f)(&mut self.c.be)
	}*/
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
struct Emiter<'a, Ev, W:Widget>{
	f: &'a Fn(Ev, &mut EventHandle<W>),
}
trait EEmi<E>{
	fn emit(&self, E);
}
impl<'a, Ev, W:Widget<State=S>, S:StateT> EEmi<Ev> for Emiter<'a, Ev, W>{
	fn emit(&self, e: Ev){
		println!("emit the event");
		//let mut eh:EventHandle<W> = EventHandle::new(&self.id, &mut state);
		(self.f)(e, &mut eh)
		//TODO: implement calling correctly and create EventHandle
	}
}*/
