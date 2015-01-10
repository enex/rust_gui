extern crate gui;

use gui::components::*;

fn main(){
    gui::Window::new("test",640,480).show(|ctx| ctx.draw(|cr|{
        cr.set_source_rgb(1.0, 1.0, 1.0);
        //cr.paint();
        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.set_line_width(5.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(1.0, 1.0);
        cr.move_to(1.0, 0.0);
        cr.line_to(0.0, 1.0);
        cr.set_line_width(0.2);
        cr.stroke();

        cr.rectangle(0.0, 0.0, 0.5, 0.5);
        cr.set_source_rgba(1.0, 0.0, 0.0, 0.80);
        cr.fill();

        cr.rectangle(0.0, 0.5, 0.5, 0.5);
        cr.set_source_rgba(0.0, 1.0, 0.0, 0.60);
        cr.fill();

        cr.rectangle(0.5, 0.0, 0.5, 0.5);
        cr.set_source_rgba(0.0, 0.0, 1.0, 0.40);
        cr.fill();
    }));
}
