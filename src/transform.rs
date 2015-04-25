use std::ops;
use std::default::Default;
use std::f32;

/// Transformation matrix used to manipulate the result
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Transform(pub [f32; 6]);

impl Transform{
	pub fn null() -> Transform{
		Transform([0.;6])
	}
	/// normal transformation matrix
	///
	/// 	[1, 0, 0, 1, 0, 0]
	pub fn normal() -> Transform{
		Transform([1., 0., 0., 1., 0., 0.])
	}
	pub fn translated(x: f32, y: f32) -> Transform{
		Transform([1., 0.,0., 1., x, y])
	}
	pub fn scaled(x: f32, y: f32) -> Transform{
		Transform([x, 0., 0., y, 0., 0.])
	}

	/// Sets the transform to translation matrix
	pub fn translate(&mut self, x: f32, y:f32) {
		self.0[4] += x;
		self.0[5] += y;
	}
	/// Sets the transform to scale matrix.
	pub fn scale(&mut self, x: f32, y: f32) {
        let mut t = self.0;
        t[0] = x; t[1] = 0.0;
        t[2] = 0.0; t[3] = y;
        t[4] = 0.0; t[5] = 0.0;
	}
	/// Sets the transform to rotate matrix. Angle is specified in radians
	pub fn rotate(&mut self, a: f32){
        let mut t = self.0;
		let cs = a.cos();
        let sn = a.sin();
		t[0] = cs; t[1] = sn;
		t[2] = -sn; t[3] = cs;
		t[4] = 0.0; t[5] = 0.0;
	}
	/// Sets the transform to skew-x matrix. Angle is specified in radians
	pub fn skew_x(&mut self, a: f32){
        let mut t = self.0;
		t[0] = 1.0; t[1] = 0.0;
		t[2] = a.tan(); t[3] = 1.0;
		t[4] = 0.0; t[5] = 0.0;
	}
	/// Sets the transform to skew-y matrix. Angle is specified in radians
	pub fn skew_y(&mut self, a: f32){
        let mut t = self.0;
		t[0] = 1.0; t[1] = a.tan();
		t[2] = 0.0; t[3] = 1.0;
		t[4] = 0.0; t[5] = 0.0;
	}
	/// Sets the transform to the result of multiplication of two transforms, of A = A*B
	pub fn multiply(&mut self, s: Transform){
		let mut t = self.0;
		let s = s.0;

		let t0 = t[0] * s[0] + t[1] * s[2];
		let t2 = t[2] * s[0] + t[3] * s[2];
		let t4 = t[4] * s[0] + t[5] * s[2] + s[4];
		t[1] = t[0] * s[1] + t[1] * s[3];
		t[3] = t[2] * s[1] + t[3] * s[3];
		t[5] = t[4] * s[1] + t[5] * s[3] + s[5];
		t[0] = t0;
		t[2] = t2;
		t[4] = t4;
	}
	/// Sets the transform to the result of multiplication of two transforms, of A = B*A
	pub fn premultiply(&mut self, s: Transform){
        let mut s2 = s.clone();
        s2.multiply(*self);
        *self = s2;
	}
	/// Sets the destination to inverse of specified transform.
    /// Returns true if the inverse could be calculated, else false.
	pub fn inverse(&mut self, a:Transform) -> bool{
        let mut t = self.0;
        let t = [
            (t[0] as f64),
            (t[1] as f64),
            (t[2] as f64),
            (t[3] as f64),
            (t[4] as f64),
            (t[5] as f64)
        ];
		let det: f64 = t[0] * t[3] - t[2] * t[1];
        let invdet = 1.0 / det;

        let mut inv = self.0;
        inv[0] = (t[3] * invdet) as f32;
        //...

        true
	}
	/// Transform a point by given transform.
	pub fn point(&mut self, x: f32, y: f32) -> (f32, f32){
        let t = self.0;
		(
            x*t[0] + y*t[2] + t[4],
            x*t[1] + y*t[3] + t[5]
        )
	}
}
impl Default for Transform{
    fn default() -> Transform{
        Transform::normal()
    }
}
impl ops::Add for Transform{
	type Output = Transform;

	fn add(self, rhs: Transform) -> Transform{
		Transform([
			self.0[0] + rhs.0[0],
			self.0[1] + rhs.0[1],
			self.0[2] + rhs.0[2],
			self.0[3] + rhs.0[3],
			self.0[4] + rhs.0[4],
			self.0[5] + rhs.0[5],
		])
	}
}
impl ops::Sub for Transform{
	type Output = Transform;

	fn sub(self, rhs: Transform) -> Transform{
		Transform([
			self.0[0] - rhs.0[0],
			self.0[1] - rhs.0[1],
			self.0[2] - rhs.0[2],
			self.0[3] - rhs.0[3],
			self.0[4] - rhs.0[4],
			self.0[5] - rhs.0[5],
		])
	}
}
impl ops::Neg for Transform{
	type Output = Transform;

	fn neg(self) -> Transform{
		Transform([
			-self.0[0],
			-self.0[1],
			-self.0[2],
			-self.0[3],
			-self.0[4],
			-self.0[5],
		])
	}
}
impl ops::Mul for Transform{
	type Output = Transform;

	fn mul(self, rhs: Transform) -> Transform{
		let mut s = self.clone();
        s.multiply(rhs);
        s
	}
}

pub fn deg_to_rad(deg: f32) -> f32{
    deg / 180. * f32::consts::PI
}
pub fn rad_to_deg(rad: f32) -> f32{
    rad / f32::consts::PI * 180.
}

#[test]
fn test_transform_add(){
	let a = Transform([1., 2., 3., 4., 5., 6.]);
	let b = Transform([6., 5., 4., 3., 2., 1.]);
	assert_eq!(a+b, Transform([7.;6]));
}
fn test_mulitply(){
	//let a = Transform::translated(100.,50.);
	//let b = Transform::translated(20., 40.);
	//assert_eq!(a*b, );
}
