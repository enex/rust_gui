use Widget;
use CTX;
use Event;

#[deriving(Clone, Show)]
pub enum CheckboxEvent{
    Change(bool),//click event on mouse down
}

///a checkbox which can either be checked or not
#[deriving(Clone, Show)]
pub struct Checkbox{
    pub value: bool,
    pub width: f64,
    pub height: f64,
}

impl Checkbox{
    pub fn new(value: bool, width: f64, height: f64) -> Checkbox{
        Checkbox{
            value: value,
            width: width,
            height: height,
        }
    }
}

impl Widget<CheckboxEvent> for Checkbox{
    fn render(&mut self, ctx: &mut CTX<CheckboxEvent>) -> (f64, f64) {
        ctx.mouseover((self.width, self.height), |event, ctx|{
            match event{
                &Event::MouseButtonDown(_, _, _, _, _, _) => {
                    ctx.emit(CheckboxEvent::Change(!self.value));
                },
                _ => {}
            }
        });
        ctx.draw(|c|{
            c.set_source_rgb(0.55, 0.55, 0.55);
            c.rectangle(0.0, 0.0, self.width, self.height);
            if self.value{
                c.fill();
            }
            c.stroke();
        });
        (self.width, self.height)
    }
    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}
