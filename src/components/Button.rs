pub enum ButtonState{
    Hover,
    Out
}

pub struct Button{
    pub text: String,
    state: ButtonState,
}
impl Button{
    pub fn new() -> Button{
        Button{
            text: "".to_string()
        }
    }
}

impl Widget for Button{
    fn render(&self){
        //TODO: add event
        ctx.on_mouse_event(|evt, &mut this|{
            //TODO: check if in
            this.state = ButtonState::Hover;
            evt.rerender();
        });
    }
}
