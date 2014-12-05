use Widget;
use CTX;

pub enum ButtonEvent{
    Click,//click event on mouse down
    Hover,//if mose move and mouse over button
    Leave
}

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
    fn render(&self, ctx: &mut CTX) -> (f64, f64) {
        let mut hover = false;
        ctx.mouseover((self.width, self.height), |event|{
            println!("button event: {}", event);
            hover = true;
        });
        ctx.draw(|c|{
            if hover{
                c.set_source_rgb(0.55, 0.55, 0.95);
            }else{
                c.set_source_rgb(0.27, 0.36, 0.93);
            }
            c.rectangle(0.0, 0.0, self.width, self.height);
            c.fill();
        });
        //ctx.add(1,Rect::new(0,0,100,100),None);
        //println!("render");
        (self.width, self.height)
    }
    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}
