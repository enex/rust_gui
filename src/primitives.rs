/*!
Primitives to draw everything needed backend independent and all in the same
consistent way
*/

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Circle{
    pub r: f32,
    pub cx: f32,
    pub cy: f32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Ellipse{
    pub rx: f32,
    pub ry: f32,
    pub cx: f32,
    pub cy: f32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Line{
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Image{
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    //TODO: add the data
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Path{
    pub cords: Vec<f32>,
    pub ops:   Vec<()>,//TODO: fully implement this
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Polygon{
    pub cords: Vec<(f32,f32)>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Polyline{
    pub cords: Vec<(f32,f32)>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Rect{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    //maybe also implement rounden corners
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Text{
    pub x: f32,
    pub y: f32,
    pub text: String,
}
