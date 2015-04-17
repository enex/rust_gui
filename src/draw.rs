//Just some thought on makeing drawing a little bit better

/*
path!(
M 23.,23.;

);
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
	/// a
	QuadCurve,

	BeizierCurve,

	Arc,

	ClosePath,
}

/// a structure containing all data for a path
#[derive(Clone, Debug)]
pub struct Path{
	instr: Vec<PathInstr>,
	vals: Vec<f32>,
	//TODO: add fill and stroke
}

impl Path{
	fn new() -> Path{
		Path{
			instr: Vec::new(),
			vals: Vec::new(),
		}
	}
}

/// api to construct a path the way it is done in nanovg or HTML5 Canvas
impl Path{
	fn move_to(&mut self, x: f32, y: f32){
		self.instr.push(PathInstr::Move);
		self.vals.push_all(&[x,y]);
	}

	fn line_to(&mut self, x: f32, y: f32){
		self.instr.push(PathInstr::Line);
		self.vals.push_all(&[x,y]);
	}

	fn bezier_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32){
		self.instr.push(PathInstr::BeizierCurve);
		self.vals.push_all(&[c1x,c1y,c2x,c2y,x,y]);
	}

	fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32){
		self.instr.push(PathInstr::QuadCurve);
		self.vals.push_all(&[cx,cy,x,y]);
	}

	fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32){
		self.instr.push(PathInstr::Arc);
		self.vals.push_all(&[x1,y1,x2,y2,radius]);
	}

	fn close_path(&mut self){
		self.instr.push(PathInstr::ClosePath);
	}
}
