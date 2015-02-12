use Widget;
use Context;
use components::Label;
use Event;
use SystemCursor;

#[derive(Debug, Copy)]
pub enum ButtonEvent{
    Click,//click event on mouse down
    Hover,//if mose move and mouse over button
    Leave
}

#[derive(Debug)]
pub struct Button<'a>{
    pub text: &'a str,
    pub width: f64,
    pub height: f64,
    pub background_color: (f64, f64, f64),
}

impl<'a> Button<'a>{
    pub fn new(text: &'a str, width: f64, height: f64) -> Button<'a>{
        Button{
            text: text,
            width: width,
            height: height,
            background_color: (0.5,0.5,0.5),//#4285f4
        }
    }
}

#[macro_export]
macro_rules! widget{
    ( $name:ident ( $( $atr_name:ident : $attr_type:ty),* ){
        $( $prop_name:ident : $prop_type:ty = $prop_val:expr ),*
    }) => (
        pub struct $name{
            $( $prop_name : $prop_type ,)*
        }
        impl $name{
            pub fn new( $( $atr_name : $attr_type),* ) -> $name{
                $name{
                    $( $prop_name : $prop_val ),*
                }
            }
            //TODO: implement setter
        }
    )
}

widget!(Box(width: f64, height: f64){
    width: f64 = width,
    height: f64 = height,
    color: (f32, f32, f32) = (0.0,0.0,0.0)
});

macro_rules! setter{
    (
        $e:ty,
        $( $prop:ident : $prop_type:ty ),+
    ) => (
        impl<'a> $e{
            $(
                pub fn $prop(&'a mut self, $prop: $prop_type) -> &'a mut $e{
                    self.$prop = $prop;
                    self
                }
            )+
        }
    )
}

setter!(Button<'a>,
    width: f64,
    height: f64,
    text: &'a str,
    background_color: (f64,f64,f64)
);

impl<'a> Widget for Button<'a>{
    fn render(&self, ctx: &mut Context) {
        let hover = ctx.focused();

        //some values for the event listener
        let (px, py) = ctx.pos();
        let (sx, sy) = (px+self.width, py+self.height);

        ctx.on(box move |e,h|match e{
            &Event::MouseButtonDown{x,y,..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy)) => {
                h.focus();
                println!("Button-event: {:?} {} ({}|{})",e, h.focused(), px, py);
            },
            &Event::MouseMotion{x, y, ..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy)) => {
                h.set_cursor(SystemCursor::Crosshair);
                //println!("set to hand");
            },
            _ => {}
        });

        //TODO: if focused make click on enter
        ctx.draw(move |c|{
            if hover{
                c.set_source_rgb(0.55, 0.55, 0.95);
            }else{
                c.set_source_rgb(0.27, 0.36, 0.93);
            }
            c.rectangle(0.0, 0.0, self.width, self.height);
            c.fill();
            c.stroke();
        });
        //ctx.go_to(0.0,0.0);
        ctx.goto(3.0, 0.0);
        ctx.add(1, Label::new(self.text.clone()).font_size(self.height - 4.0))
    }
    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}
