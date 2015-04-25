/*!
things for drawing paths should be made with the `path!` macro because this is in most
cases more efficient and simpler.
*/
use std::fmt::{self, Debug};
use primitives::Rect;
use Color;

//TODO: check num arguments in path macro
#[macro_export]
macro_rules! conv_path(
	(M) => (PathInstr::Move);
	(L) => (PathInstr::Line);
	(Q) => (PathInstr::QuadCurve);
	(B) => (PathInstr::BeizierCurve);
	(A) => (PathInstr::Arc);
	(Z) => (PathInstr::ClosePath);
);

/// a macro which makes it much easier to use Paths. It is also more efficient
/// than the second way because no allocation is needed.
///
/// the following two ways of constructing a graph result in the exact same path
///
/// ```
/// #[macro_use] extern crate rui;
/// use rui::draw::{Path, PathInstr};
/// use rui::primitives;
///
/// # fn main(){
/// let path = path!(M:10,10; L:200,200; L:300,200; L:500,400; Z:);
///
/// let mut path = primitives::Path::new();
/// path.move_to(10.,10.);
/// path.line_to(200.,200.);
/// path.line_to(300.,200.);
/// path.line_to(500.,400.);
/// path.close_path();
/// # }
/// ```
#[macro_export]
#[macro_use(conv_path)]
macro_rules! path(
	( $( $pi:ident : $( $val:expr ),* );+ )=>({
		Path{
			instr: [ $( conv_path!( $pi ), )+ ],
			vals: [ $( $( $val as f32, )* )+ ]
		}
	})
);

#[test]
fn test_path_macro(){
	use self::PathInstr::*;

	let p = path!(M:12,3; L:100,30);

	assert_eq!(p.instr.as_ref(), &[Move, Line]);
	assert_eq!(p.vals.as_ref(), &[12.,3.,100.,30.]);
}

/// enum of path instructions
#[derive(Debug, Clone, PartialEq)]
pub enum PathInstr{
	/// Move to a given position
	Move,
	/// Line to a given pos
	Line,

	QuadCurve,

	BeizierCurve,

	Arc,

	ClosePath,
}
impl PathInstr{
	/// number of argument associated with this instruction
	pub fn num_args(self) -> usize{
		use self::PathInstr::*;

		match self{
			Move => 2,
			Line => 2,
			QuadCurve => 4,
			BeizierCurve => 6,
			Arc => 5,
			ClosePath => 0,
		}
	}
}

pub trait AsPath{
	///path instructions neccessary to draw the path
	fn instructions(&self) -> &[PathInstr];
	/// values for the points in the path
	fn values(&self) -> &[f32];

	/// return a filled path
	fn fill(self, color: Color) -> Filled<Self> where Self:Sized{
		Filled{
			p: self,
			color: color,
		}
	}
	/// return a stroked path
	fn stroke(self, width: f32, color: Color) -> Stroked<Self> where Self:Sized{
		Stroked{
			p: self,
			width: width,
			color: color,
		}
	}

	/// how the path is filled for rendering
	fn get_fill(&self) -> Option<Color>{None}
	/// how the path is stroked for rendering
	fn get_stroke(&self) -> Option<(f32, Color)>{None}
}
impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> AsPath for (I, V){
	fn instructions(&self) -> &[PathInstr]{
		self.0.as_ref()
	}
	fn values(&self) -> &[f32]{
		self.1.as_ref()
	}
}
impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> AsPath for Path<I, V>{
	fn instructions(&self) -> &[PathInstr]{
		self.instr.as_ref()
	}
	fn values(&self) -> &[f32]{
		self.vals.as_ref()
	}
}
/// a structure containing all data for a path
pub struct Path<I:AsRef<[PathInstr]>, V:AsRef<[f32]>>{
	pub instr: I,
	pub vals: V,
}

impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> Path<I, V>{
	/// check if point is in path (x,y)
	pub fn is_point_in_path(&self, _: f32, _: f32) -> bool{
		unimplemented!()
	}

	/// return a minimal rectangle the path would fit in
	/// this is usefull for layout operations
	pub fn min_rect(&self) -> Rect{
		unimplemented!()
	}
}

impl Path<[PathInstr; 5], [f32; 8]>{
	/// create a path whic is a rect
	pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Path<[PathInstr; 5], [f32; 8]>{
		path!(M:x,y; L:(x+width),y; L:(x+width),(y+height); L:x,(y+height); Z:)
	}
}

impl Path<Vec<PathInstr>, Vec<f32>>{
	/// create a new path with vecor items
	pub fn new() -> Path<Vec<PathInstr>, Vec<f32>>{
		Path{
			instr: Vec::new(),
			vals: Vec::new(),
		}
	}
}

/// api to construct a path the way it is done in nanovg or HTML5 Canvas
/// this is only available if vectors are used.
impl Path<Vec<PathInstr>, Vec<f32>>{
	pub fn move_to(&mut self, x: f32, y: f32){
		self.instr.push(PathInstr::Move);
		self.vals.push_all(&[x,y]);
	}

	pub fn line_to(&mut self, x: f32, y: f32){
		self.instr.push(PathInstr::Line);
		self.vals.push_all(&[x,y]);
	}

	pub fn bezier_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32){
		self.instr.push(PathInstr::BeizierCurve);
		self.vals.push_all(&[c1x,c1y,c2x,c2y,x,y]);
	}

	pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32){
		self.instr.push(PathInstr::QuadCurve);
		self.vals.push_all(&[cx,cy,x,y]);
	}

	pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32){
		self.instr.push(PathInstr::Arc);
		self.vals.push_all(&[x1,y1,x2,y2,radius]);
	}

	pub fn close_path(&mut self){
		self.instr.push(PathInstr::ClosePath);
	}

	/// clear the path so you can start again
	pub fn clear(&mut self){
		self.instr.truncate(0);
		self.vals.truncate(0);
	}
}

/// print a svg compatible path description
impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> Debug for Path<I, V>{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
		let mut vi = self.vals.as_ref().iter();
		macro_rules! tv(($i:ident)=>(match $i.next(){Some(e)=>e,_=>return Ok(())}.clone()));

		for i in self.instr.as_ref().iter(){
			use self::PathInstr::*;

			try!(match i{
				&Move => write!(f, "M{:?},{:?} ", tv!(vi), tv!(vi)),
				&Line => write!(f, "L{:?},{:?} ", tv!(vi), tv!(vi)),
				&QuadCurve => write!(f, "Q{:?},{:?},{:?},{:?} ", tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
				&BeizierCurve => write!(f, "C{:?},{:?},{:?},{:?},{:?},{:?} ", tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
				&Arc => write!(f, "A{:?},{:?},{:?},{:?},{:?} ", tv!(vi), tv!(vi), tv!(vi), tv!(vi), tv!(vi)),
				&ClosePath => write!(f, "Z "),
			})
		}
		Ok(())
	}
}

impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> Path<I, V>{
	/// fill the path with a givent thing, it is then a normal widget
	pub fn fill() {

	}

	/// stroke the path
	pub fn stroke() {

	}
}

pub struct Filled<P>{
	p: P,
	pub color: Color,
}
impl<P:AsPath> AsPath for Filled<P>{
	fn instructions(&self) -> &[PathInstr]{
		self.p.instructions()
	}
	fn values(&self) -> &[f32]{
		self.p.values()
	}

	/// how the path is filled for rendering
	fn get_fill(&self) -> Option<Color>{
		Some(self.color.clone())
	}
	/// how the path is stroked for rendering
	fn get_stroke(&self) -> Option<(f32, Color)>{
		self.p.get_stroke()
	}
}
pub struct Stroked<P>{
	p: P,
	pub color: Color,
	pub width: f32,
}
impl<P:AsPath> AsPath for Stroked<P>{
	fn instructions(&self) -> &[PathInstr]{
		self.p.instructions()
	}
	fn values(&self) -> &[f32]{
		self.p.values()
	}

	/// how the path is filled for rendering
	fn get_fill(&self) -> Option<Color>{
		self.p.get_fill()
	}
	/// how the path is stroked for rendering
	fn get_stroke(&self) -> Option<(f32, Color)>{
		Some((self.width, self.color.clone()))
	}
}

//TODO: fill and stoke with Patterns.

#[test]
fn test_paht_vec(){
	use self::PathInstr::*;

	let mut p = Path::new();
	p.move_to(0.,0.);
	p.line_to(100.,100.);
	p.line_to(200.,30.);
	p.close_path();
	assert_eq!(p.instr, &[Move, Line, Line, ClosePath]);
	assert_eq!(p.vals, &[0.,0.,100.,100.,200.,30.]);
}
