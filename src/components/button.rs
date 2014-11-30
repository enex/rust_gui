use gui::Widget;

pub enum ButtonEvent{
    Click,
    Hover,
    Leave
}

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
        //ctx.emit(ButtonEvent::Click)
    }
}
