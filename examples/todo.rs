#[macro_use] extern crate rui;
extern crate glutin;

use std::collections::LinkedList;
use rui::prelude::*;
use rui::components::*;

// this is the model which will be rendered

#[derive(Debug)]
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

	fn render<C:Context>(&self, c: &mut C){
		let y = (self.index as f32)*30. + 10.;
		c.draw_path(path!(M:0,y-10.; L:500,(y-10.)).stroke(1., Color::rgb(120,120,120)));

		c.font_face("sans");
		c.text(48., y + 10., &self.desc[..]);

		let i = if self.done{
			components::Icon{
				icon: fa::check_circle_o,
				..c.default()
			}
		} else {
			components::Icon{
				icon: fa::circle_o,
				..c.default()
			}
		};
		c.translate(8., y);
		c.add(1, &i);

		let i = components::Icon{
			icon: fa::remove,
			..c.default()
		};
		c.translate(468., y);
		c.add(2, &i);
		//ctx.add(1, Label::new(&self.desc[0..]).font_size(16.0));
	}
}

#[derive(Debug)]
pub struct TodoApp{
	///All Tasks
	pub tasks: LinkedList<Task>,
	next_id: usize,
	///value for the task going to be inserted
	pub input: String,
}
impl TodoApp{
	fn new() -> TodoApp{
		TodoApp{
			tasks: LinkedList::new(),
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
		self.tasks.push_back(Task{
			done: false,
			desc: desc.to_string(),
			index: id,
		});
	}
}
impl Widget for TodoApp{
	type State = ();
	type Event = ();

	fn render<C:Context>(&self, c: &mut C){
		c.add(1, &Label::new("What needs to be done?"));
		//c.add(1, Label::new("test-label").font_size(100.0).font_face("Arial"));
		let mut i = 20;
		c.translate(0., 40.);
		for task in self.tasks.iter(){
			c.add(i, task);
			i += 1;
		}
		c.reset();
		c.draw_path(Path::rect(0.,460., 500.,500.)
			.stroke(2., Color::rgb(160,160,160))
			.fill(Color::rgb(0,0,0)));//just a line for separation
		let mut d = Button{
			text: "All",
			height: 24.,
			width: 32.,
			..c.default()
		};
		c.translate(100., 470.);
		c.add(2, &d);
		d.text = "Active";
		c.add(3, &d);
		c.translate(160., 470.);
		d.text = "Completed";
		c.translate(280., 470.);
		c.add(4, &d);
		//TODO: render buttons for view
		//TODO: render clear if task is ready
		//TODO: render remaining tasks
	}
}

fn main(){
	let mut ta = TodoApp::new();
	ta.append_item("contribute");
	ta.append_item("start experimenting");
	ta.append_item("share the library");
	for i in 1..15{
		ta.append_item(&format!("{:?}. Element", i)[..])
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

	App::new(window, ta).show();
}
