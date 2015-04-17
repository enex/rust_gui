use Backend;
use primitives;
use draw;
use cairo;

/// backend to use Cairo to draw the GUI
pub struct CairoBackend{
    ctx: cairo::Cairo,
}

impl CairoBackend{
    fn new(surface: &mut cairo::surface::Surface) -> CairoBackend{
        CairoBackend{
            ctx: cairo::Cairo::create(surface)
        }
    }
}

impl Backend for CairoBackend{
    fn load_font(&mut self, path: &str){
        unimplemented!()
    }
    fn draw_path(&mut self, path: primitives::Path){
        macro_rules! take(($i:ident)=>{match $i.next(){Some(e)=>e.clone(),None=>return}});
        let mut vi = path.vals.iter();
        for i in path.instr.iter(){
            use draw::PathInstr::*;

            //TODO: take care of fill and stroke style

            match i{
                &Move => self.ctx.move_to(take!(vi), take!(vi)),
                &Line => self.ctx.line_to(take!(vi), take!(vi)),
                &QuadCurve => (),
                &BeizierCurve => (),
                &Arc => (),
                &ClosePath => self.ctx.close_path()
            };
            println!("draw path");
        }
    }
}
