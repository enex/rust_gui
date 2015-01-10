use Window;
use Context;
use cairo;

pub struct CTX<'a>{
    window: &'a mut Window
}

impl<'a> CTX<'a>{
    pub fn new(window: &'a mut Window) -> CTX<'a>{
        CTX{
            window: window
        }
    }
    pub fn draw<F>(&mut self, draw: F) where F: Fn(&mut cairo::Context) {
        draw(self.window.cairo_context())
    }
}

impl<'a> Context<()> for CTX<'a>{
    fn draw<F>(&mut self, draw: F) where F: Fn(&mut cairo::Context) {
        draw(self.window.cairo_context())
    }
}
