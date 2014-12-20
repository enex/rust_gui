use Widget;
use CTX;
use components::Label;
use Event;

#[deriving(Show, Copy)]
pub enum ButtonEvent{
    Click,//click event on mouse down
    Hover,//if mose move and mouse over button
    Leave
}

#[deriving(Clone, Show)]
pub struct Button{
    pub text: String,
    pub width: f64,
    pub height: f64,
}

impl Button{
    pub fn new(text: String, width: f64, height: f64) -> Button{
        Button{
            text: text,
            width: width,
            height: height,
        }
    }
}

impl Widget<ButtonEvent> for Button{
    fn render(&mut self, ctx: &mut CTX<ButtonEvent>) -> (f64, f64) {
        let mut hover = false;
        ctx.mouseover((self.width, self.height), |event, ctx|{
            //println!("button event: {}", event);
            hover = true;
            match event{
                &Event::MouseMotion(_, _, _, _, _, _, _, _) =>{
                    ctx.emit(ButtonEvent::Hover);//emit hover event
                },
                &Event::MouseButtonDown(_, _, _, _, _, _) => {
                    ctx.emit(ButtonEvent::Click);
                },
                _ => {}
            }
        });
        ctx.draw(|c|{
            if hover{
                c.set_source_rgb(0.55, 0.55, 0.95);
            }else{
                c.set_source_rgb(0.27, 0.36, 0.93);
            }
            c.rectangle(0.0, 0.0, self.width, self.height);
            c.fill();
            c.stroke();
        });
        ctx.go_to(0.0,0.0);
        let mut l = Label::new(self.text.clone());
        l.font_size = self.height - 4.0;
        ctx.go_to(3.0, 0.0);
        ctx.add(1, &mut l, None);
        (self.width, self.height)
    }
    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}
