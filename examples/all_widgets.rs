extern crate gui;

use gui::components::{Button, Slider, Label};
//TODO: insert widgets here

fn main(){
    gui::Window::new("test",640,480).show(|ctx|{
        ctx.add(1, &Button::new("test-Button",270.,50.));
        ctx.goto(0.,100.);
        ctx.add(2, &Button::new("test-Button2",290.,50.));
        ctx.goto(0.,150.);
        ctx.add(3, &Slider::new(10., 100., 1.));
        ctx.goto(0.,20.);
        ctx.add(4, Label::new("Test-Label").color((0.3,0.5,0.6)));
        ctx.goto(0.,20.);
        ctx.add(4, Label::new("Test-Label").color((0.7,0.5,0.6)));
    });
}
