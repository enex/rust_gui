use Widget;
use CTX;

///Layout for placing everything in a row
pub struct Row<'a>{
    pub spacing: f64,
    pub render_childs: |&mut CTX<()>|:'a,
}

impl<'a> Widget<()> for Row<'a>{
    fn render(&mut self, ctx: &mut CTX<()>) -> (f64, f64) {
        //TODO: render the child nodes
        (self.render_childs)(ctx);
        (0.0,0.0)
    }
}

impl<'a> Row<'a>{
    #[inline(always)]
    pub fn childs(&mut self, c:|&mut CTX<()>|:'a) {
        self.render_childs = c;
    }
}

///Layout for placing into a column
pub struct Col{
    pub spacing: f64,
}
