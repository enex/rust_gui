use Widget;
use Context;
use Event;

#[derive(Copy)]
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

#[derive(Clone, Show, Copy)]
pub enum SliderEvent{
    ///Mouse hovers the slider
    Hover,
    ///event which is fired on value change
    Changed(f64)
}

//TODO: implement drag and drop behavior
//TODO: implement layout fill

impl Widget for Slider{
    fn render(&self, ctx: &mut Context) {
        let mut focused = ctx.focused();
        let (px, py) = ctx.pos();
        let (sx, sy) = (px+100.0, py+10.0);
        ctx.on(box move |e, h| match e{
            &Event::MouseMotion{..} =>{
                //ctx.emit(SliderEvent::Hover);//emit focused event
            },
            &Event::MouseButtonDown{x,y, ..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy))=> {
                //TODO: calculate new value
                //let e = ctx.x();
                //println!("{}",e);
                //ctx.emit(SliderEvent::Changed(((x as f64) - e)/100.0 * self.max_value));
                h.focus();//set this to the currently focused element
            },
            _ => {}
        });
        /*ctx.mouseover((100.0, 10.0), |event, ctx|{
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
        });*/
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
            c.set_source_rgb(0.6, 0.6, 0.6);
            c.move_to(0.0,5.0);
            c.line_to(100.0,5.0);
            c.stroke();
            c.set_source_rgb(0.35,0.4,0.8);
            c.arc((self.value / self.max_value) * 100.0, 5.0, 5.0, 0.0, 360.0);
            if focused{
                c.fill();
            }
            c.stroke();
        });
    }
}
