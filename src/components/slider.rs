use Widget;
use CTX;
use Event;

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
    ///Mouse hovers the slider
    Hover,
    ///event which is fired on value change
    Changed(f64)
}

//TODO: implement drag and drop behavior
//TODO: implement layout fill

impl Widget<SliderEvent> for Slider{
    fn render(&mut self, ctx: &mut CTX<SliderEvent>) -> (f64, f64) {
        let mut hover = false;
        ctx.mouseover((100.0, 10.0), |event, ctx|{
            println!("button event: {}", event);
            hover = true;
            match event{
                &Event::MouseMotion(_, _, _, _, _, _, _, _) =>{
                    ctx.emit(SliderEvent::Hover);//emit hover event
                },
                &Event::MouseButtonDown(_, _, _, _, x, _) => {
                    //TODO: calculate new value
                    let e = ctx.x();
                    println!("{}",e);
                    ctx.emit(SliderEvent::Changed(((x as f64) - e)/100.0 * self.max_value));
                },
                _ => {}
            }
        });
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
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
