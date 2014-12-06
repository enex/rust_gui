use Widget;
use CTX;
use std::f32;
use std::str;
//use std::from_str::FromStr;

pub struct Icon<'a>{
    path: &'a str,//a path like in svg
    color: (f64, f64, f64),
}

impl<'a> Icon<'a>{
    pub fn new(path: &'a str, color: (f64,f64,f64)) -> Icon<'a>{
        Icon{
            path: path,
            color: color,
        }
    }
}

struct PathParser<'a>{
    chars: str::Chars<'a>,
    last: Option<char>,//last character to unread
    unread: bool,
}

impl<'a> PathParser<'a>{
    fn new(c: str::Chars<'a>) -> PathParser<'a>{
        PathParser{
            chars: c,
            last: None,
            unread: false,
        }
    }
    #[inline]
    fn next(&mut self) -> Option<char>{
        if self.unread{
            self.unread = false;
            self.last
        }else{
            self.last = self.chars.next();
            self.last
        }
    }
    #[inline]
    fn unread(&mut self){
        self.unread = true;
    }

    #[inline]
    fn skip_space(&mut self){
        loop{
            match self.next(){
                None => break,
                Some(c) => if !((c == ' ') | (c == ',')){
                    break;
                }
            }
        }
        self.unread();
    }

    #[inline]
    fn num(&mut self) -> Option<f64>{//closure to read from stream
        let mut b = String::new();
        self.skip_space();
        let mut cc = match self.next(){
            Some(e) => e,
            None => return None
        };
        if cc == '-'{//handle minus
            b.push(cc);
            cc = match self.next(){
                Some(e) => e,
                None => return None
            };
        }
        let mut nf = true;
        while(cc.is_digit(10)|((cc == '.')&&nf)){
            //println!("    {}", cc);
            if cc == '.'{
                nf = false;
            }
            b.push(cc);
            cc = match self.next(){
                Some(e) => e,
                None => return None
            };
        }
        self.unread();
        //println!("    {}", cc);
        //println!("num {}", b);
        from_str::<f64>(b.as_slice())
    }
}

impl<'a> Widget<()> for Icon<'a>{
    ///this function will render a svg path, if the path is not correct it will panic
    fn render(&self, ctx: &mut CTX) -> (f64, f64) {
        ctx.draw(|c|{
            //Parsing and drawing logic
            //TODO: add this
            let (r,g,b) = self.color;
            c.set_source_rgb(r, g, b);
            let mut chars = PathParser::new(self.path.chars());

            loop{
                //TODO: make multible comands work
                let mut cc = match chars.next(){
                    Some(e) => e,
                    None => break
                };
                match cc{
                    'M' => {//move to
                        let x = chars.num().unwrap();
                        let y = chars.num().unwrap();
                        c.move_to(x,y);
                        //println!("move to ({} {})", x, y);
                        //TODO: parse move to
                    },
                    'm' => {//move to
                        let x = chars.num().unwrap();
                        let y = chars.num().unwrap();
                        c.rel_move_to(x,y);
                    },
                    'L' => {//line to
                        loop{//loop over the numbers
                            let x = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            let y = chars.num().unwrap();
                            c.line_to(x,y);
                            //println!("line to ({} {})",x,y);
                        }
                    },
                    'l' => {//line to
                        //println!("line to relative");
                        loop{//loop over the numbers
                            let x = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            let y = chars.num().unwrap();
                            c.rel_line_to(x,y);
                            //println!("line to ({} {})",x,y);
                        }
                    },
                    'V' => {//vertical line
                        //println!("vertival line");
                        c.line_to(0.0,chars.num().unwrap());
                    },
                    'v' => {//vertical line
                        //println!("vertival line relative");
                        c.rel_line_to(0.0,chars.num().unwrap());
                    },
                    'H' => {//horizontal line
                        c.line_to(chars.num().unwrap(),0.0);
                        //println!("horizontal line");
                    },
                    'h' => {//horizontal line
                        c.rel_line_to(chars.num().unwrap(),0.0);
                        //println!("horizontal line relative");
                    },
                    'Z' | 'z' => {//close path
                        //println!("close path");
                        c.close_path();
                    },
                    'c' => {//relative curve
                        loop{//loop over the numbers
                            let x1 = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            //println!("curve to");
                            //let x1 = chars.num().unwrap();
                            let y1 = chars.num().unwrap();
                            let x2 = chars.num().unwrap();
                            let y2 = chars.num().unwrap();
                            let x3 = chars.num().unwrap();
                            let y3 = chars.num().unwrap();

                            c.rel_curve_to(x1,y1,x2,y2,x3,y3);
                        }
                    },
                    'C' => {//relative curve
                        loop{//loop over the numbers
                            let x1 = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            //println!("curve to");
                            let y1 = chars.num().unwrap();
                            let x2 = chars.num().unwrap();
                            let y2 = chars.num().unwrap();
                            let x3 = chars.num().unwrap();
                            let y3 = chars.num().unwrap();

                            c.curve_to(x1,y1,x2,y2,x3,y3);
                        }
                    },
                    's' => {
                        //TODO: make it work properly
                        loop{
                            let x2 = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            let y2 = chars.num().unwrap();
                            let x  = chars.num().unwrap();
                            let y  = chars.num().unwrap();
                            c.rel_curve_to(x2,y2,x2,y2,x,y);
                            println!("s");
                        }
                    },
                    'S' => {
                        loop{
                            let x2 = match chars.num(){
                                Some(s) => s,
                                None => break
                            };
                            let y2 = chars.num().unwrap();
                            let x  = chars.num().unwrap();
                            let y  = chars.num().unwrap();
                            c.curve_to(x2,y2,x2,y2,x,y);
                            println!("S");
                        }
                    },
                    //TODO: implement the following functions
                    'T' => {//quadratische bizar kurven
                        println!("T");
                    },
                    't' => {//quadratische bizar kurven
                        println!("t");
                    },
                    'A' => {//Bogenkurven
                        println!("A");
                    },
                    'a' => {//Bogenkurven
                        println!("a");
                    },
                    'q' => {//quadratic curve
                        println!("q");
                    },
                    'Q' => {
                        println!("Q");
                    },
                    _ => {
                        //println!("something else {}", cc);
                    }
                }
                //println!("ch: {}", cc);
            }
            c.stroke();
        });
        (0.0, 0.0)
    }
}
