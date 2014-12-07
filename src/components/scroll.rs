use Widget;
use CTX;

pub struct ScrollArea{
    ///Wehter the scollbars should be visible
    pub vertical: bool,
    pub horizonal: bool,
}

impl ScrollArea{
    pub fn new() -> ScrollArea{
        ScrollArea{
            vertical: false,
            horizonal: false,
        }
    }
}

impl Widget<()> for ScrollArea{
    fn render(&mut self, ctx: &mut CTX<()>) -> (f64, f64) {
        println!("render some scrollbars and an scrollable area");
        (0.0,0.0)
    }
}
