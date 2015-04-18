/*!
Primitives to draw everything needed backend independent and all in the same
consistent way
*/

pub use draw::Path;

//TODO: add stoke and fill somehow
//TODO: zero copy and zero alloc
//TODO: use lazy statics to store paths that would otherwise go away and prevent allocation on new creation
//TODO: consider removing Polyline, Polygon, Circle, Line and just use Path

#[derive(Clone, Debug)]
pub struct Circle{
    pub r: f32,
    pub cx: f32,
    pub cy: f32,
}

#[derive(Clone, Debug)]
pub struct Ellipse{
    pub rx: f32,
    pub ry: f32,
    pub cx: f32,
    pub cy: f32,
}

#[derive(Clone, Debug)]
pub struct Line{
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Clone, Debug)]
pub struct Image{
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    //TODO: add the data
}

#[derive(Clone, Debug)]
pub struct Polygon{
    pub cords: Vec<(f32, f32)>,
}

#[derive(Clone, Debug)]
pub struct Polyline{
    pub cords: Vec<(f32,f32)>,
}

#[derive(Clone, Debug)]
pub struct Rect{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    //maybe also implement rounden corners
}

#[derive(Clone, Debug)]
pub struct Text<'a>{
    pub x: f32,
    pub y: f32,
    pub text: &'a str,
}
