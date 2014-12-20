use Widget;
use CTX;
use components::Label;
use Event;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum TextInputEvent{
    ///Event on value change, contains the new value
    Change(String)
}

///a text input element. This is a one row text edit.
pub struct TextInput{
    pub value: String,
    pub placeholder: String,
}

impl TextInput{
    pub fn new(value: String, placeholder: String) -> TextInput{
        TextInput{
            value: value,
            placeholder: placeholder,
        }
    }
}

impl Widget<TextInputEvent> for TextInput{
    fn render(&mut self, ctx: &mut CTX<TextInputEvent>) -> (f64, f64) {
        //TODO: render the input element

        ctx.mouseover((100.0, 20.0), |event, ctx|{
            //println!("input event: {}", event);
            match event{
                &Event::MouseButtonDown(_, _, _, _, _, _) => {
                    ctx.focus();//focus this element
                    ctx.set_state(0u32);//set point of cursor
                },
                _ => {}
            }
        });

        //handle key inputs
        ctx.keyevent(|event, ctx|{
            match event{
                &Event::KeyDown(_, _, key_code, scan_code, _, s) => {
                    println!("key event: {}(_,_, {}, {}, _, {})", event, key_code, scan_code, s);
                    //TODO: parse keyboard input
                    ctx.emit(TextInputEvent::Change("test".to_string()))
                },
                _ => {}
            }
        });

        if self.value.len() > 0{
            let val = self.value.clone();
            ctx.add(1, &mut Label::new(val), None);
        }else{
            let val = self.placeholder.clone();
            ctx.add(1, &mut Label::new(val), None);
        }
        let focused = ctx.is_focused();
        ctx.draw(|c|{
            if focused{
                c.set_source_rgb(3.0,3.0,6.0);
            }else{
                c.set_source_rgb(0.0, 0.0, 0.0);
            }
            c.move_to(0.0, 23.0);
            c.line_to(100.0,23.0);
            c.stroke();
        });
        (0.0,0.0)
    }
}
