use Widget;
use CTX;

pub struct Row{
    pub spacing: f64,
}

impl Widget<()> for Row{
    fn render(&self, ctx: &mut CTX) -> (f64, f64) {
        //TODO: render the child nodes
        (0.0,0.0)
    }
}
impl Row{
    fn childs(&self, c:|&mut CTX|) {
        //TODO: draw child nodes
    }
}

pub struct Column{
    pub spacing: f64,
}
