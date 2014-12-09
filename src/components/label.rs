use Widget;
use CTX;
use cairo::ffi;

pub struct Label{
    pub text: String,
    pub font_size: f64,
}

impl Label{
    pub fn new(text: String) -> Label{
        Label{
            text: text,
            font_size: 20.0,
        }
    }
}

impl Widget<()> for Label{
    fn render(&mut self, ctx: &mut CTX<()>) -> (f64, f64) {
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
            unsafe{
                let cr = c.cairo_ptr();
                ffi::cairo_set_source_rgb(cr, 0.0, 0.0, 0.0);
                ffi::cairo_select_font_face(cr, "Sans".to_c_str().as_ptr(), ffi::CAIRO_FONT_SLANT_NORMAL, ffi::CAIRO_FONT_WEIGHT_NORMAL);
                ffi::cairo_set_font_size(cr, self.font_size);

                ffi::cairo_move_to(cr, 0.0, self.font_size);
                ffi::cairo_show_text(cr, self.text.to_c_str().as_ptr());
            }
        });
        (0.0,0.0)
    }
    fn size(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}
