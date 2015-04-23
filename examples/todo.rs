#[macro_use] extern crate rui;
extern crate glutin;

use std::collections::LinkedList;
use rui::prelude::*;
use rui::components::Button;

//this is the model which will be rendered

#[derive(Debug)]
pub struct Task{
	pub index: usize,
	pub done: bool,
	///the description of the Task
	pub desc: String,
}

impl Widget for Task{
	type State = ();
	type Event = ();
	fn render<C:Context>(&self, ctx: &mut C){
		ctx.text(100.,(self.index as f32)*20.+100.,&self.desc[..]);
		//ctx.add(1, Label::new(&self.desc[0..]).font_size(16.0));
	}
}

#[derive(Debug)]
pub struct TodoApp{
	///All Tasks
	pub tasks: LinkedList<Task>,
	///value for the task going to be inserted
	pub input: String,
}
impl TodoApp{
	fn new() -> TodoApp{
		TodoApp{
			tasks: LinkedList::new(),
			input: String::new(),
		}
	}
	/// append a new task to the todo list
	fn append_item(&mut self, desc: &str){
		let i = self.tasks.len();
		self.tasks.push_back(Task{
			done: false,
			desc: desc.to_string(),
			index: i,
		});
	}
}
impl Widget for TodoApp{
	type State = ();
	type Event = ();

	fn render<C:Context>(&self, ctx: &mut C){
		ctx.text(0.,0.,"test-label");
		//ctx.add(1, Label::new("test-label").font_size(100.0).font_face("Arial"));
		let d = Button{
			..ctx.default()
		};
		ctx.add(2, &d);
		let mut i = 3;
		for task in self.tasks.iter(){
			ctx.add(i, task);
			i+=1;
		}
	}
}

fn main(){
	let mut ta = TodoApp::new();
	ta.append_item("Sichtbar Machen");
	ta.append_item("Hausaufgaben erledigen");
	for i in 1..50{
		ta.append_item(&format!("{:?}. Element", i)[..])
	}

	let window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl_version((3,2))
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	App::new(window, ta).show();
}
