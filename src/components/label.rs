use Widget;
use Context;
use cairo::ffi;

pub struct Label<'a>{
    pub text: &'a str,
    pub font_size: f64,
    //TODO: add other text styling options
    pub font_face: &'a str,
    pub color: (f64, f64, f64),
}

impl<'a> Label<'a>{
    pub fn new(text: &'a str) -> Label<'a>{
        Label{
            text: text,
            font_size: 20.0,
            font_face: "Sans",
            color: (0.,0.,0.),
        }
    }
}

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

///implement setter for the propertys
setter!(Label<'a>,
    text: &'a str,
    font_size: f64,
    font_face: &'a str,
    color: (f64,f64,f64)
);

pub enum LabelEvent{
    Hover
}
impl<'a> Widget for Label<'a>{
    //type Event = LabelEvent;

    fn render(&self, ctx: &mut Context) {
        ctx.draw(|c|{ //right now the cairo wrapper does not offer an abstract way for text rendering
            unsafe{
                use std::ffi::CString;

                let cr = c.cairo_ptr();
                let (r,g,b) = self.color;
                ffi::cairo_set_source_rgb(cr, r, g, b);
                ffi::cairo_select_font_face(cr, CString::from_slice(self.font_face.as_bytes()).as_ptr(), ffi::CAIRO_FONT_SLANT_NORMAL, ffi::CAIRO_FONT_WEIGHT_NORMAL);
                ffi::cairo_set_font_size(cr, self.font_size);

                ffi::cairo_move_to(cr, 0.0, self.font_size);
                ffi::cairo_show_text(cr, CString::from_slice(self.text.as_bytes()).as_ptr());
            }
        });
    }
}
