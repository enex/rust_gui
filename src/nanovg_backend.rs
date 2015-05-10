use Backend;
use primitives;
use nanovg::{self, Ctx, Font};
use Transform;
use Color;
use draw;
use draw::AsPath;

pub struct NanovgBackend{
    vg: Ctx,
    pub fonts: Vec<Font>,
}

impl NanovgBackend{
    pub fn new(vg: Ctx) -> NanovgBackend{
        NanovgBackend{
            vg: vg,
            fonts: Vec::new()
        }
    }
}

impl Backend for NanovgBackend{
    fn load_font(&mut self, name: &str, path: &str) -> Result<(), ()>{
        self.fonts.push(match self.vg.create_font(name, path){
			Some(o) => o,
			None => return Err(())
        });
        Ok(())
    }

    fn transform(&mut self, t: Transform){
        self.vg.transform(nanovg::Transform::from_array(t.0))
    }
    fn current_transform(&self) -> Transform{
        Transform(self.vg.current_transform().into_array())
    }
    fn reset_transform(&mut self){
        self.vg.reset_transform();
    }

    fn find_font(&self, name: &str) -> Option<Font>{
        self.vg.find_font(name)
    }
    fn font_face(&mut self, font: &str){
        self.vg.font_face(font)
    }
    fn font_size(&mut self, size: f32){
        self.vg.font_size(size)
    }
    fn font_color(&mut self, color: Color){
        self.vg.fill_color(color);
    }

    fn text(&self, x: f32, y: f32, text: &str) -> f32{
        self.vg.text(x,y,text)
    }

    fn begin(&mut self, width: i32, height: i32){
        self.vg.begin_frame(width, height, 1.);
    }
    fn end(&mut self){
        self.vg.end_frame();
    }

    fn draw_path<P:AsPath>(&mut self, path: P){
    	macro_rules! tv(($i:ident)=>(match $i.next(){Some(e)=>e,_=>return}.clone()));

        self.vg.begin_path();

        //fill and stroke rules
        if let Some(s) = path.get_stroke(){
            self.vg.stroke_width(s.0);
            self.vg.stroke_color(s.1);
        }
        if let Some(f) = path.get_fill(){
            self.vg.fill_color(f);
        }

        let mut vi = path.values().iter();
    	for i in path.instructions().iter(){
    		use draw::PathInstr::*;

    		match i{
    			&Move => self.vg.move_to(tv!(vi), tv!(vi)),
    			&Line => self.vg.line_to(tv!(vi), tv!(vi)),
    			&QuadCurve => self.vg.quad_to(tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
    			&BeizierCurve => self.vg.bezier_to(tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
    			&Arc => self.vg.arc_to(tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
    			&ClosePath => self.vg.close_path(),
    		}
    	}

        if let Some(_) = path.get_stroke(){
            self.vg.stroke();
        }
        if let Some(_) = path.get_fill(){
            self.vg.fill();
        }
    }
}
