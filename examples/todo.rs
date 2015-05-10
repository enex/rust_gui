#[macro_use] extern crate rui;
extern crate glutin;

use rui::prelude::*;
use rui::components::*;

// this is the model which will be rendered

#[derive(Debug, Clone)]
pub struct Task{
	/// a index to uniquely identify this Task
	pub index: usize,
	/// whether the task is alerady done
	pub done: bool,
	///the description of the Task
	pub desc: String,
}

impl Widget for Task{
	type State = ();
	type Event = TodoEvent;

	fn render<C:Context<TWidget=Task>>(&self, c: &mut C, _:&()){
		let y = 10.;
		c.draw_path(path!(M:0,y-10.; L:500,(y-10.)).stroke(1., Color::rgb(120,120,120)));

		c.translate(48., y);
		Label::new(&self.desc[..]).font_size(17.).draw(c, 1);

		c.translate(8., y);
		if self.done{
			components::Icon{
				icon: fa::check_circle_o,
				..c.default()
			}
		} else {
			components::Icon{
				icon: fa::circle_o,
				..c.default()
			}
		}.draw(c, 2);
		let d = c.default();
		c.translate(8., 0.);
		c.awe(3, &components::Button{
			width: 30.,
			height: 30.,
			..d
		}, |e, h| match e{
			&ButtonEvent::Click => h.emit(if self.done{
					TodoEvent::UndoneTask(self.index) 
				}else{
					TodoEvent::DoneTask(self.index) 
				}),
			_ => ()
		});

		c.translate(468., y);
		components::Icon{
			icon: fa::remove,
			..c.default()
		}.draw(c, 4);
		
		let d = c.default();
		c.translate(468., 0.);
		c.awe(5, &components::Button{
			width: 30.,
			height: 30.,
			..d
		}, |e, h| match e{
			&ButtonEvent::Click => h.emit(TodoEvent::RemoveTask(self.index)),
			_ => ()
		});
	}
}

#[derive(Clone, Debug)]
pub enum TodoEvent{
	SwitchTab(Tab),
	
	/// add a new task at the end
	AddTask(String),
	/// remove task by id
	RemoveTask(usize),
	/// update task description to the given
	UpdateTask(usize, String),
	/// mark task as done
	DoneTask(usize),
	/// mark task as not done again
	UndoneTask(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Tab{
    All,
	Active,
	Completed,
}
impl Default for Tab{
	fn default() -> Tab{
		Tab::All
	}
}
#[derive(Debug, Clone, Default)]
pub struct TodoState{
	tab: Tab,
	tasks: Vec<Task>,
	input: String,
	next_id: usize,
}
impl TodoState{
	fn new() -> TodoState{
		TodoState{
			tab: Default::default(),
			tasks: Vec::new(),
			input: String::new(),
			next_id: 0,
		}
	}
	/// generate an id for the next item
	fn next_id(&mut self) -> usize(){
		let id = self.next_id;
		self.next_id += 1;
		id
	}
	/// append a new task to the todo list
	fn append_item(&mut self, desc: &str){
		let id = self.next_id();
		self.tasks.push(Task{
			done: false,
			desc: desc.to_string(),
			index: id,
		});
	}
}
impl rui::UIState<TodoEvent> for TodoState{
	fn handle(&mut self, e: TodoEvent){
		use self::TodoEvent::*;
		println!("event: {:?}", e);
		match e{
			SwitchTab(e) => self.tab = e,
			AddTask(d) => self.append_item(&d[..]),
			RemoveTask(id) => self.tasks.retain(|e| e.index != id ),
			DoneTask(id) => for mut task in self.tasks.iter_mut(){
				if task.index == id{
					task.done = true;
				}
			},
			UndoneTask(id) => for mut task in self.tasks.iter_mut(){
				if task.index == id{
					task.done = false;
				}
			},
			_ => ()
		}
	}
}

pub struct TodoApp;
impl Widget for TodoApp{
	type State = TodoState;
	type Event = TodoEvent;

	fn render<C:Context<TWidget = TodoApp>>(&self, c: &mut C, s: &TodoState){
		c.draw_path(Path::rect(0.,0., 500.,500.)//clear background
			.fill(Color::rgb(0,0,0)));
			
		c.translate(48., 14.);
		Label::new("What needs to be done?")
			.font_size(22.)
			.color(Color::rgb(150,150,150)).draw(c, 1);

		let mut i = 20;
		c.translate(0., 40.);

		for (pos, task) in s.tasks.iter().filter(|t| match s.tab{
			Tab::All => true,
			Tab::Completed => t.done,
			Tab::Active => !t.done,
		}).enumerate(){
			c.translate(0.,(pos as f32)*30. + 40.);
			c.awe(i, task, |e,h| h.emit(e.clone()) );//propagate further
			i += 1;
		}

		c.reset();
		c.draw_path(Path::rect(0.,460., 500.,500.)
			.stroke(2., Color::rgb(160,160,160))
			.fill(Color::rgb(0,0,0)));//just a line for separation

		let pd = Button{ //prototype button for the following buttons
			height: 24.,
			width: 36.,
			..c.default()
		};

		fn bgc(e: Tab,v: Tab) -> Option<Color>{
			if e == v{
				Some(Color::rgb(40,40,40))
			}else{
				None
			}
		}

		c.translate(148., 470.);
		c.awe(2, &Button{
			text: "All",
			background_color: bgc(Tab::All, s.tab),
			..pd.clone()
		}, |e, h| h.emit(TodoEvent::SwitchTab(Tab::All)) );

		c.translate(190., 470.);
		c.awe(3, &Button{
			width: 60.,
			background_color: bgc(Tab::Active, s.tab),
			text: "Active", ..pd.clone()
		}, |e, h| h.emit(TodoEvent::SwitchTab(Tab::Active)) );

		c.translate(256., 470.);
		c.awe(4, &Button{
			text: "Completed",
			width: 90.,
			background_color: bgc(Tab::Completed, s.tab),
			..pd.clone()
		}, |e, h| match e{
			&ButtonEvent::Click => h.emit(TodoEvent::SwitchTab(Tab::Completed)),
			_ => ()
		});

		c.translate(10., 478.);
		Label::new(&format!("{} items", s.tasks.len())[..])
			.font_size(14.).draw(c, 5);
		//TODO: render buttons for view
		//TODO: render clear if task is ready
		//TODO: render remaining tasks
	}
}

fn main(){
	let mut state = TodoState::new();
	state.append_item("contribute");
	state.append_item("start experimenting");
	state.append_item("share the library");
	for i in 1..15{
		state.append_item(&format!("{:?}. Element", i)[..])
	}
	state.tasks[1].done = true;
	state.tasks[5].done = true;
	state.tasks[6].done = true;
	state.tasks[7].done = true;
	state.tasks[12].done = true;
	state.tasks[16].done = true;

	let window = glutin::WindowBuilder::new()
		.with_title("todo example".to_string())
		.with_dimensions(500, 500)
		.with_gl(glutin::GlRequest::GlThenGles {
	        opengl_version: (3, 2),
	        opengles_version: (3, 2),
	    },)
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	let mut app = App::new(window, TodoApp)
	
	app.load_font("sans", "res/Roboto-Regular.ttf").unwrap();
	app.load_font("font-awesome", "res/fontawesome-webfont.ttf").unwrap();
	app.load_font("sans-bold", "res/Roboto-Bold.ttf").unwrap();
	
	app.show(&mut state);
}
