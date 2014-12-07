use Widget;
use CTX;

pub struct Slider{
    pub max_value: f64,
    pub step_size: f64,
    pub value: f64,
}

impl Slider{
    pub fn new(value: f64, max: f64, step: f64) -> Slider{
        Slider{
            max_value: max,
            step_size: step,
            value: value,
        }
    }
}

#[deriving(Clone, Show)]
pub enum SliderEvent{
    Changed(f64)//event which is fired on value change
}

impl Widget<SliderEvent> for Slider{
    fn render(&mut self, ctx: &mut CTX<SliderEvent>) -> (f64, f64) {
        let mut hover = false;
        ctx.mouseover((100.0, 10.0), |event, ctx|{
            println!("button event: {}", event);
            hover = true;
        });
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
            /*let mut x = 0.0;
            let mut y = 0.0;
            c.user_to_device(&mut x, &mut y);
            println!("user to device: ({} {})", x, y);
            c.user_to_device_distance(&mut x, &mut y);
            println!("user_to_device_distance: ({} {})", x, y);
            c.device_to_user(&mut x, &mut y);
            println!("device_to_user: ({} {})", x, y);
            c.device_to_user_distance(&mut x, &mut y);
            println!("device_to_user_distance: ({} {})", x, y);*/

            c.set_source_rgb(0.6, 0.6, 0.6);
            c.move_to(0.0,5.0);
            c.line_to(100.0,5.0);
            c.stroke();
            c.set_source_rgb(0.35,0.4,0.8);
            c.arc((self.value / self.max_value) * 100.0, 5.0, 5.0, 0.0, 360.0);
            if hover{
                c.fill();
            }
            c.stroke();
        });
        (0.0,0.0)
    }
}
