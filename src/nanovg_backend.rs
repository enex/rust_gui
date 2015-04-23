use Backend;
use primitives;
use nanovg::{self, Ctx, Font};
use Transform;
use Color;
use draw;

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
    fn load_font(&mut self, name: &str, path: &str){
        self.fonts.push(self.vg.create_font(name, path).unwrap())
    }

    fn transform(&mut self, t: Transform){
        self.vg.transform(nanovg::Transform::from_array(t.0))
    }
    fn current_transform(&self) -> Transform{
        Transform(self.vg.current_transform().into_array())
    }

    fn find_font(&self, name: &str) -> Option<Font>{
        self.vg.find_font(name)
    }
    fn font_face(&mut self, font: &str){
        self.vg.font_face(font)
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

    fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
            (&mut self, path: primitives::Path<I, V>){
        let mut vi = path.vals.as_ref().iter();
    	macro_rules! tv(($i:ident)=>(match $i.next(){Some(e)=>e,_=>return}.clone()));

        self.vg.begin_path();

        //just for demonstration
        self.vg.fill_color(Color::rgb(48, 121, 237));
        self.vg.stroke_color(Color::rgb(34, 34, 34));

    	for i in path.instr.as_ref().iter(){
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

        self.vg.stroke();
        self.vg.fill();
    }
}
