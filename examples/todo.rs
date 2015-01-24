extern crate gui;

use std::collections::DList;
use gui::components::*;
use gui::Widget;
use gui::Context;

//this is the model which will be rendered

#[derive(Show)]
pub struct Task{
    pub done: bool,
    ///the description of the Task
    pub desc: String,
}

#[derive(Show)]
pub struct List{
    ///All Tasks
    pub tasks: DList<Task>,
    ///value for the task going to be inserted
    pub input: String,
}

pub struct App<'a>{
    data: &'a mut List
}
impl<'a> Widget for App<'a>{
    type Event = ();
    fn render(&self, ctx: &mut Context<()>){
        //ctx.add()
    }
}

fn main(){
    //show the window
    let mut list = List{
        tasks: DList::new(),
        input: String::new(),
    };
    gui::Window::new("todo-app",480,640);
}
