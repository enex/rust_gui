/*!
things for drawing
```rust
path!(
M 23.,23.;

);
```
*/
use std::fmt::{self, Debug};
use primitives::Rect;

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

/// a structure containing all data for a path
pub struct Path<I:AsRef<[PathInstr]>, V:AsRef<[f32]>>{
	pub instr: I,
	pub vals: V,
}

impl<I:AsRef<[PathInstr]>, V:AsRef<[f32]>> Path<I, V>{
	/// check if point is in path
	pub fn is_point_in_path(&self, x: f32, y: f32) -> bool{
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
