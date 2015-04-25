use super::super::prelude::*;
use std::default::Default;
use components::Label;

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

/// A simple button component
/// 
/// ![all_widgets example](https://raw.githubusercontent.com/enex/rust_gui/master/button.png)
/// # Usage:
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
    pub width: f32,
    pub height: f32,
    pub background_color: Option<Color>,
}

impl<'a> Default for Button<'a>{
    fn default() -> Button<'a>{
        Button{
            text: "",
            width: 100.,
            height: 100.,
            background_color: None,//#4285f4
        }
    }
}

setter!(Button<'a>,
    width: f32,
    height: f32,
    text: &'a str,
    background_color: Option<Color>
);

impl<'a> Widget for Button<'a>{
    type Event = ButtonEvent;
    type State = ();

    fn render<C: Context<TWidget=Button<'a>>>(&self, c: &mut C) {
        //let hovered = c.hovered();
        println!("draw_button  {:?}", self);

        c.draw_path(Path::rect(0.,0.,self.width,self.height)
            .stroke(1., Color::rgb(180,180,180)));
        c.translate(2., 8.);
        let l = Label{
            text: self.text,
            font_size: self.height - 4.,
            ..c.default()
        };
        c.add(1, &l);
        c.on_click(|pos, h| {
            println!("Button !clicked at {:?}", pos);
            h.emit(ButtonEvent::Click);
        });

        //TODO: if focused make click on enter
        //TODO: draw label
        //TODO: optional icon
        //TODO: tooltip
    }

    fn name() -> &'static str{"Button"}
}
