use super::super::prelude::*;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
pub enum ButtonEvent{
	/// click event on mouse down
    Click,
	/// double click event on mouse down
    DoubleClick,
	/// if mose move and mouse over button
    Hover,
	/// mouse goes away from button
    Leave
}

/// Usage:
///
/// ```rust
/// use rui::components::Button;
/// use std::default::Default;
///
/// ctx.add(Button{
/// 	text: "Button",
/// 	..Default::default()
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Button<'a>{
    pub text: &'a str,
    pub width: f64,
    pub height: f64,
    pub background_color: (f64, f64, f64),
}

impl<'a> Default for Button<'a>{
    fn default() -> Button<'a>{
        Button{
            text: "",
            width: 100.,
            height: 100.,
            background_color: (0.5,0.5,0.5),//#4285f4
        }
    }
}

setter!(Button<'a>,
    width: f64,
    height: f64,
    text: &'a str,
    background_color: (f64, f64, f64)
);

impl<'a> Widget for Button<'a>{
    type Event = ButtonEvent;
    type State = ();

    fn render<C:Context>(&self, c: &mut C) {
        let hovered = c.hovered();
        println!("draw_button  {:?}", self);

        c.draw_path(Path::rect(0.,0.,100.,100.));

        /*ctx.on(box move |e,h|match e{
            &Event::MouseButtonDown{x,y,..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy)) => {
                h.focus();
                println!("Button-event: {:?} {} ({}|{})",e, h.focused(), px, py);
            },
            &Event::MouseMotion{x, y, ..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy)) => {
                h.set_cursor(SystemCursor::Crosshair);
                //println!("set to hand");
            },
            _ => {}
        });*/
        /*
        c.on_hover(|e|{

        });
        c.on_click(|e|{

        });
        c.on_key_down(|e|{

        });
        */

        //TODO: if focused make click on enter

        /*
        ctx.draw(move |c|{
            if hover{
                c.set_source_rgb(0.55, 0.55, 0.95);
            }else{
                c.set_source_rgb(0.27, 0.36, 0.93);
            }
            c.rectangle(0.0, 0.0, self.width, self.height);
            c.fill();
            c.stroke();
        });*/
        //ctx.go_to(0.0,0.0);
        //ctx.goto(3.0, 0.0);
        //let l = Label::new(self.text.clone()).font_size(self.height - 4.0);
        //let (height, width) = ctx.measure(l);
        //ctx.add(1, &l);
    }

    fn name() -> &'static str{"Button"}
}
