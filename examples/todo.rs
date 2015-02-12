extern crate gui;

use std::collections::DList;
use gui::components::*;
use gui::{Widget, Context, App};

//this is the model which will be rendered

#[derive(Debug)]
pub struct Task{
    pub done: bool,
    ///the description of the Task
    pub desc: String,
}

impl Widget for Task{
    fn render(&self, ctx: &mut Context){
        println!("render Task");
        ctx.add(1, Label::new(&self.desc[0..]).font_size(16.0));
    }
}

#[derive(Debug)]
pub struct TodoApp{
    ///All Tasks
    pub tasks: DList<Task>,
    ///value for the task going to be inserted
    pub input: String,
}
impl TodoApp{
    fn new() -> TodoApp{
        TodoApp{
            tasks: DList::new(),
            input: String::new(),
        }
    }
    /// append a new task to the todo list
    fn append_item(&mut self, desc: &str){
        self.tasks.push_back(Task{
            done: false,
            desc: desc.to_string(),
        });
    }
}
impl App for TodoApp{
    fn render(&mut self, ctx: &mut Context){
        ctx.draw(|cr|{//make background gray
            cr.set_source_rgb(0.18, 0.18, 0.18);
            cr.paint();
        });
        ctx.add(1, Label::new("test-label").font_size(100.0).font_face("Arial"));
        ctx.add(2, &Button::new("test-button",100.0,20.0));
        let mut i = 3;
        for task in self.tasks.iter(){
            ctx.add(i, task);
            i+=1;
        }
        println!("do the Work of drawing.");
    }
}

fn main(){
    let mut ta = TodoApp::new();
    ta.append_item("Sichtbar Machen");
    ta.append_item("Hausaufgaben erledigen");
    gui::Window::new("todo-app", 480, 640).app(&mut ta);
}
