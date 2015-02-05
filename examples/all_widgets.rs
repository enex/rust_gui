extern crate gui;

use gui::components::{Button};
//TODO: insert widgets here

fn main(){
    gui::Window::new("test",640,480).show(|ctx|{
        ctx.add(1, &Button::new("test-Button",270.,50.));
    });
}
