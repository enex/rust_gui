use Widget;
use CTX;
use std::str;
//use std::from_str::FromStr;

///a Widget used for monochrom vektor Icons.
pub struct Icon<'a>{
    path: &'a str,//a path like in svg
    pub fill: Option<(f64,f64,f64)>,
    color: (f64, f64, f64),
}

impl<'a> Icon<'a>{
    ///create a new icon form a given svg-path
    pub fn new(path: &'a str, color: (f64,f64,f64)) -> Icon<'a>{
        Icon{
            path: path,
            color: color,
            fill: None,
        }
    }
}

//internal struct to parse a svg-path
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
        while cc.is_digit(10)|((cc == '.')&&nf){
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
    fn render(&mut self, ctx: &mut CTX<()>) -> (f64, f64) {
        ctx.draw(|c|{
            //Parsing and drawing logic
            //TODO: add this
            let (r,g,b) = self.color;
            let mut before: Option<(f64,f64)> = None;//for s path
            c.set_source_rgb(r, g, b);
            let mut chars = PathParser::new(self.path.chars());

            loop{
                //TODO: make multible comands work
                let cc = match chars.next(){
                    Some(e) => e,
                    None => break
                };
                match cc{
                    'M' => {//move to
                        let x = chars.num().unwrap();
                        let y = chars.num().unwrap();
                        c.move_to(x,y);
                        //println!("move to ({} {})", x, y);
                        before = None;
                    },
                    'm' => {//move to
                        let x = chars.num().unwrap();
                        let y = chars.num().unwrap();
                        c.rel_move_to(x,y);
                        before = None;
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
                        before = None;
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
                        before = None;
                    },
                    'V' => {//vertical line
                        //println!("vertival line");
                        c.line_to(0.0,chars.num().unwrap());
                        before = None;
                    },
                    'v' => {//vertical line
                        //println!("vertival line relative");
                        c.rel_line_to(0.0,chars.num().unwrap());
                        before = None;
                    },
                    'H' => {//horizontal line
                        c.line_to(chars.num().unwrap(),0.0);
                        before = None;
                        //println!("horizontal line");
                    },
                    'h' => {//horizontal line
                        before = None;
                        c.rel_line_to(chars.num().unwrap(),0.0);
                        //println!("horizontal line relative");
                    },
                    'Z' | 'z' => {//close path
                        //println!("close path");
                        before = None;
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
                            //TODO: set before
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

                            //TODO: set before

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
                            let mut x1;
                            let mut y1;
                            match before{
                                None => {
                                    x1 = x2;
                                    y1 = y2;
                                },
                                Some((x,y)) => {
                                    x1 = x;
                                    y1 = y;
                                }
                            }
                            c.rel_curve_to(x2,y2,x1,y1,x,y);
                            //println!("s");
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

                            let mut x1;
                            let mut y1;
                            match before{
                                None => {
                                    x1 = x2;
                                    y1 = y2;
                                },
                                Some((x,y)) => {
                                    x1 = x;
                                    y1 = y;
                                }
                            }

                            c.curve_to(x2,y2,x1,y1,x,y);
                            //println!("S");
                        }
                    },
                    //TODO: implement the following functions
                    'T' => {//quadratische bizar kurven
                        println!("T");
                        unimplemented!();
                    },
                    't' => {//quadratische bizar kurven
                        println!("t");
                        unimplemented!();
                    },
                    'A' => {//Bogenkurven
                        println!("A");
                        unimplemented!();
                    },
                    'a' => {//Bogenkurven
                        println!("a");
                        unimplemented!();
                    },
                    'q' => {//quadratic curve
                        println!("q");
                        unimplemented!();
                    },
                    'Q' => {
                        println!("Q");
                        unimplemented!();
                    },
                    ' ' => {},//ignore whitespaces
                    _ => {
                        println!("something else '{}'", cc);
                    }
                }
                //println!("ch: {}", cc);
            }
            c.stroke();
        });
        (0.0, 0.0)
    }
}
