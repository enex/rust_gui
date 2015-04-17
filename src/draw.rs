/*!
things for drawing
```rust
path!(
M 23.,23.;

);
```
*/

macro_rules! conv_path(
	(M) => (PathInstr::Move);
	(L) => (PathInstr::Line);
	(Q) => (PathInstr::QuadCurve);
	(B) => (PathInstr::BeizierCurve);
	(A) => (PathInstr::Arc);
	(C) => (PathInstr::ClosePath);
);

#[macro_export]
macro_rules! path(
	( $( $pi:ident : $( $val:expr ),* );+ )=>({
		(
			&[ $( conv_path!( $pi ), )+ ],
			&[ $( $( $val, )* )+ ]
		)
	})
);

#[test]
fn test_path(){
	use self::PathInstr::*;

	assert_eq!(
		path!(M:12.,3.; L:100.,30.),
		(
			&[Move, Line],
			&[12.,3.,100.,30.]
		)
	);
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
#[derive(Clone, Debug)]
pub struct Path{
	pub instr: Vec<PathInstr>,
	pub vals: Vec<f64>,
	//TODO: add fill and stroke
}

impl Path{
	pub fn new() -> Path{
		Path{
			instr: Vec::new(),
			vals: Vec::new(),
		}
	}
	/// parse a svg path description and construct a new path out of it.
	pub fn from_svg(s: &str) -> Path{
		unimplemented!()
	}
}

/// api to construct a path the way it is done in nanovg or HTML5 Canvas
impl Path{
	pub fn move_to(&mut self, x: f64, y: f64){
		self.instr.push(PathInstr::Move);
		self.vals.push_all(&[x,y]);
	}

	pub fn line_to(&mut self, x: f64, y: f64){
		self.instr.push(PathInstr::Line);
		self.vals.push_all(&[x,y]);
	}

	pub fn bezier_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64){
		self.instr.push(PathInstr::BeizierCurve);
		self.vals.push_all(&[c1x,c1y,c2x,c2y,x,y]);
	}

	pub fn quad_to(&mut self, cx: f64, cy: f64, x: f64, y: f64){
		self.instr.push(PathInstr::QuadCurve);
		self.vals.push_all(&[cx,cy,x,y]);
	}

	pub fn arc_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64){
		self.instr.push(PathInstr::Arc);
		self.vals.push_all(&[x1,y1,x2,y2,radius]);
	}

	pub fn close_path(&mut self){
		self.instr.push(PathInstr::ClosePath);
	}
}
