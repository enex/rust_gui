use Widget;
use CTX;
use cairo::ffi;

pub struct Label{
    pub text: String,
}

impl Label{
    pub fn new(text: String) -> Label{
        Label{
            text: text,
        }
    }
}

impl Widget<()> for Label{
    fn render(&self, ctx: &mut CTX) -> (f64, f64) {
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
            unsafe{
                let cr = c.cairo_ptr();
                ffi::cairo_set_source_rgb(cr, 1.0, 1.0, 0.0);
                ffi::cairo_select_font_face(cr, "Sans".to_c_str().as_ptr(), ffi::CAIRO_FONT_SLANT_NORMAL, ffi::CAIRO_FONT_WEIGHT_NORMAL);
                ffi::cairo_set_font_size(cr, 40.0);

                ffi::cairo_move_to(cr, 10.0, 50.0);
                ffi::cairo_show_text(cr, self.text.to_c_str().as_ptr());
            }
        });
        (0.0,0.0)
    }
    fn size(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}
