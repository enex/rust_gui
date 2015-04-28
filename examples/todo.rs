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
	type Event = ();

	fn render<C:Context>(&self, c: &mut C, _:&()){
		let y = (self.index as f32)*30. + 10.;
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

		c.translate(468., y);
		components::Icon{
			icon: fa::remove,
			..c.default()
		}.draw(c, 3);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Tab{
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
pub struct AppState{
	tab: Tab,
	tasks: Vec<Task>,
	input: String,
	next_id: usize,
}
impl AppState{
	fn new() -> AppState{
		AppState{
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

pub struct TodoApp;
impl Widget for TodoApp{
	type State = AppState;
	type Event = ();

	fn render<C:Context>(&self, c: &mut C, s: &AppState){
		c.translate(48., 14.);
		c.add(1, Label::new("What needs to be done?")
			.font_size(22.)
			.color(Color::rgb(150,150,150)));

		let mut i = 20;
		c.translate(0., 40.);

		for task in s.tasks.iter(){
			c.add(i, task);
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

		//TODO: highlight current tab
		c.translate(148., 470.);
		c.awe(2, &Button{
			text: "All",
			background_color: bgc(Tab::All, s.tab),
			..pd.clone()
		}, |e, _| println!("All {:?}", e));

		c.translate(190., 470.);
		c.awe(3, &Button{
			width: 60.,
			background_color: bgc(Tab::Active, s.tab),
			text: "Active", ..pd.clone()
		}, |e, _| println!("Active {:?}", e));

		c.translate(256., 470.);
		c.awe(4, &Button{
			text: "Completed",
			width: 90.,
			background_color: bgc(Tab::Completed, s.tab),
			..pd.clone()
		}, |e, _| println!("Completed {:?}", e));

		c.translate(10., 478.);
		c.add(5, Label::new(&format!("{} items", s.tasks.len())[..])
			.font_size(14.));
		//TODO: render buttons for view
		//TODO: render clear if task is ready
		//TODO: render remaining tasks
	}
}

fn main(){
	let mut state = AppState::new();
	state.append_item("contribute");
	state.append_item("start experimenting");
	state.append_item("share the library");
	for i in 1..15{
		state.append_item(&format!("{:?}. Element", i)[..])
	}

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

	App::new(window, TodoApp).show(&state);
}
