use Widget;
use Context;
use Event;
use keyboard;
use cairo::ffi;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TextInputEvent{
    ///Event on value change, contains the new value
    Change(String)
}

///a text input element. This is a one row text edit.
pub struct TextInput<'a>{
    pub value: &'a str,
    pub placeholder: &'a str,
}

impl<'a> TextInput<'a>{
    pub fn new(value: &'a str, placeholder: &'a str) -> TextInput<'a>{
        TextInput{
            value: value,
            placeholder: placeholder,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State{
    cursor_pos: u16
}

//TODO: make cursor blink
//TODO: make cursor use glyph size
//TODO: set cursor appropriat

impl<'a> Widget for TextInput<'a>{
    fn render(&self, ctx: &mut Context) {
        let (px, py) = ctx.pos();
        let (sx, sy) = (px+100., py+23.);
        let size = self.value.len();//length of the value

        ctx.on(box move |e, h| match e{
            &Event::MouseButtonDown{x,y,..}
                if ((x as f64 > px) & (y as f64 > py) & ((x as f64) < sx) & ((y as f64) < sy)) => {
                h.focus();
                if !keyboard::is_text_input_active(){
                    keyboard::start_text_input();
                }
                h.set_state(box State{
                    cursor_pos: 0,
                });
                //TODO: stop text input when not focused any more
                //println!("input-event: {:?} {} ({}|{})",e, h.focused(), px, py);
            },
            &Event::TextInput{ref text,..} => {
                println!("input {}", text)
            },
            &Event::TextEditing{ref text,start, length, ..} => {
                println!("Edit {} start:{} length:{}", text, start, length)
            },
            &Event::KeyDown{keycode,..} => {
                use sdl2::keycode::KeyCode;
                //TODO: backspace u. Pfeiltasten und so implementieren
                //println!("KeyDown {:?}", keycode)
                let mut s:Option<&mut State> = h.mut_state();
                match s{
                    Some(ref mut e) => {
                        //println!("STATE: {:?}",e);
                        match keycode{
                            KeyCode::Right => {
                                //TODO: don't go over last item
                                if e.cursor_pos < size as u16{
                                    e.cursor_pos += 1;
                                }
                            },
                            KeyCode::Left => {
                                if e.cursor_pos > 0{
                                    e.cursor_pos -= 1;
                                }
                            },
                            KeyCode::End => {
                                //TODO: go to end of input
                                e.cursor_pos = size as u16;
                            },
                            //TODO: handle backspace and entf
                            _ => {}
                        }
                    },
                    None => {}
                }
            },
            _ => {}
        });

        let pos = {
            let state:Option<&State> = ctx.state();
            match state{
                Some(e) => e.cursor_pos,
                None => 0
            }
        };

        /*if self.value.len() > 0{
            ctx.add(1, Label::new(self.value).color((1.,1.,1.)));
        }else{
            let val = self.placeholder.clone();
            ctx.add(1, &mut Label::new(val));
        }*/

        let focused = ctx.focused();
        ctx.draw(|c|{
            if focused{
                c.set_source_rgb(3.0,3.0,6.0);
            }else{
                c.set_source_rgb(0.6, 0.6, 0.6);
            }
            c.move_to(0.0, 23.0);
            c.line_to(100.0,23.0);

            let p = 10.0 * pos as f64;
            c.move_to(p, 0.);
            c.line_to(p, 20.);

            c.stroke();

            unsafe{
                use std::ffi::CString;
                use std::ptr;

                let cr = c.cairo_ptr();

                ffi::cairo_set_source_rgb(cr, 34., 143., 8.);
                ffi::cairo_select_font_face(cr, CString::from_slice(b"Serif").as_ptr(),
                    ffi::CAIRO_FONT_SLANT_NORMAL,
                    ffi::CAIRO_FONT_WEIGHT_NORMAL);
                ffi::cairo_set_font_size(cr, 20.);

                let mut extents = ffi::cairo_text_extents_t{
                    x_bearing: 0.,
                    y_bearing: 0.,
                    width: 0.,
                    height: 0.,
                    x_advance: 0.,
                    y_advance: 0.,
                };
                let (mut x,mut y) = (0f64,0f64);
                let mut i = 0;
                for c in self.value.chars(){
                    let mut glyphs:[ffi::cairo_glyph_t; 1] = [ffi::cairo_glyph_t{
                        index: c as u64,
                        x: x*20.,
                        y: y*20.,
                    }];
                    x += 1.;
                    if x > 15.{
                        y += 1.;
                        x = 0.;
                    }

                    ffi::cairo_move_to(cr, 0.0, 20.);
                    //let c_glyphs: *mut ffi::Struct_Unnamed1 = &mut glyphs.get_unchecked(0) as *mut ffi::Struct_Unnamed1;
                    ffi::cairo_show_glyphs(cr, glyphs.as_ptr(), 1);
                    ffi::cairo_glyph_extents(cr, glyphs.as_ptr(), 1, &mut extents as *mut ffi::cairo_text_extents_t);
                    println!("x_bearing:{} y_bearing:{} width:{} height:{} x_advance:{} y_advance:{} => {}",
                        extents.x_bearing,
                        extents.y_bearing,
                        extents.width,
                        extents.height,
                        extents.x_advance,
                        extents.y_advance,
                        c as u64
                    );
                    i+=1;
                    //ffi::cairo_show_text(cr, CString::from_slice(self.value.as_bytes()).as_ptr());
                }
            }
        });
    }
}
