//! very simple library for basic 2d drawing operations like nanovg

/// This is the "Paper" you can draw on by using the privides api, it is
/// for 2d drawing and quite similar to cairo or nanovg but simpler
/// 
/// ```
/// use rui::Paper;
///
/// let mut p = Paper::new();
/// p.move_to(0.,0.);
///	p.line_to(12.,12.);
/// p.line_to(120.,120.);
/// p.sup(|p| );
/// ```
///
/// Proposal this could be done in future with
/// path!(M100,180 L40,10 L190,120 L160,10 z);
/// where variables could be used like so:
/// path!(M{x},180 L40,(x+2));
/// but maybe the paper api is easyer even thought the macro aproach could reach
/// higher speeds
pub struct Paper{
	///start position of the path
	start: (f32,f32),
	///Transformations
	xform: [f32; 6],
	fill_stye: Color,
	line_width: f32,
	line_style: Color,
}

pub type Color = [f32; 4];

///enum describing elements of a path
pub enum Path{
	Move(f32, f32),
	Line(f32, f32),
	Curve(f32, f32, f32),
}

/// Drawing commands
enum Commands{
	MoveTo = 0,
	LineTo = 1,
	ClosePath = 2,
	//TODO: add others
}

pub enum LineCap {
	BUTT,
	ROUND,
	SQUARE,
	BEVEL,
	MITER,
}

impl Paper{
	///make a new drawing context
	pub fn new() -> Paper{
		Paper{
			start: (0., 0.),
			xform: [1.,1.,0.,1.,1.,0.],
			fill_stye: [1.,1.,1.,1.],
			line_width: 1.0,
			line_style: [0.,0.,0.,1.],
		}
	}
	
	/// draw to new paper with inherited state of current paper without changing
	/// it. It is like the save and recover method in other librarys. Right now
	/// it is not possible to use one Paper or its sub papers on differend Threads
	/// and it might not be implemented soon, this way it is ensured nothing false
	/// happens
	pub fn sub<F>(&mut self, f: F) where F: Fn(&mut Paper){
		let mut p = Paper{
			start: self.start,
			xform: self.xform,
			fill_stye: self.fill_stye,
			line_width: self.line_width,
			line_style: self.line_style,
		};
		(f)(&mut p);
	}
	
	///reset current render state to default
	pub fn reset(&mut self){
		*self = Paper::new();
	}
	
	///set the stroke color
	pub fn stroke_color(&mut self, c: Color){
		self.line_style = c;
	}
	
	//TODO: add some other options
	pub fn fill_color(&mut self, c: Color){
		self.fill_stye = c;
	}
	
	///set the stroke width
	pub fn stroke_width(&mut self, w: f32){
		self.line_width = w;
	}
	
	///transform the context by a matrix
	pub fn transform(&mut self, a1: f32, b1: f32, c1: f32, a2: f32, b2: f32, c2: f32){
		self.xform = [a1, b1, c1, a2, b2, c2];
	}
	///translate the context
	pub fn translate(&mut self, x: f32, y: f32){
		self.xform[2] = x;
		self.xform[5] = y;
	}
	///rotate the whole context
	pub fn rotate(&mut self, angle: f32){
		//TODO: transform the vector with sinus and cosinus
	}
	///scale the context by given value
	pub fn scale(&mut self, x: f32, y: f32){
		self.xform[0] = self.xform[0] * x;
		self.xform[1] = self.xform[1] * y;
		self.xform[3] = self.xform[3] * x;
		self.xform[4] = self.xform[4] * y;
	}
	
	/// start the path drawing
	pub fn beginPath(&mut self){
		
	}
	pub fn move_to(&mut self, x: f32, y: f32){
		
	}
	pub fn line_to(&mut self, x: f32, y: f32){
		
	}
	pub fn bezier_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32){
		
	}
	pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32){
		
	}
	/// Adds an arc segment at the corner defined by the last path point, and two specified points.
	pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32){
		/*
	float theta = 2 * 3.1415926 / float(num_segments); 
	float tangetial_factor = tanf(theta);//calculate the tangential factor 

	float radial_factor = cosf(theta);//calculate the radial factor 
	
	float x = r;//we start at angle = 0 

	float y = 0; 
    
	glBegin(GL_LINE_LOOP); 
	for(int ii = 0; ii < num_segments; ii++) 
	{ 
		glVertex2f(x + cx, y + cy);//output vertex 
        
		//calculate the tangential vector 
		//remember, the radial vector is (x, y) 
		//to get the tangential vector we flip those coordinates and negate one of them 

		float tx = -y; 
		float ty = x; 
        
		//add the tangential vector 

		x += tx * tangetial_factor; 
		y += ty * tangetial_factor; 
        
		//correct using the radial factor 

		x *= radial_factor; 
		y *= radial_factor; 
	} 
	glEnd(); 
*/
	}
	///close the path
	pub fn close_path(&mut self){
		
	}
	///fill the path with current style
	pub fn fill(&mut self){
		
	}
	pub fn stroke(&mut self){
		
	}
}
