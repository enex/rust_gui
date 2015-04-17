/*!
Primitives to draw everything needed backend independent and all in the same
consistent way
*/

pub use draw::Path;

//TODO: zero copy and zero alloc
//TODO: use lazy statics to store paths that would otherwise go away and prevent allocation on new creation

#[derive(Clone, Debug)]
pub struct Circle{
    pub r: f64,
    pub cx: f64,
    pub cy: f64,
}

#[derive(Clone, Debug)]
pub struct Ellipse{
    pub rx: f64,
    pub ry: f64,
    pub cx: f64,
    pub cy: f64,
}

#[derive(Clone, Debug)]
pub struct Line{
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug)]
pub struct Image{
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    //TODO: add the data
}

#[derive(Clone, Debug)]
pub struct Polygon{
    pub cords: Vec<(f64,f64)>,
}

#[derive(Clone, Debug)]
pub struct Polyline{
    pub cords: Vec<(f64,f64)>,
}

#[derive(Clone, Debug)]
pub struct Rect{
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    //maybe also implement rounden corners
}

#[derive(Clone, Debug)]
pub struct Text<'a>{
    pub x: f64,
    pub y: f64,
    pub text: &'a str,
}
