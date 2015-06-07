/*!
Everything necessary to implement a new backend.
*/

pub use Color;
pub use Transform;
pub use draw::*;
pub use ID;

use glutin;

/// Backend which should be implemented to support drawing operations
/// the first backend will be cairo + OpenGL but other backends should follow
pub trait Backend{
	/// create a new backend.
	fn new(window: &glutin::Window) -> Self;

	/// start drawing the path. The size of the Frame is provided.
	fn begin(&mut self, _:i32, _:i32){}

	/// load a font form a given path and make it available for later use
	/// if it is called with the same font more than one time nothing
	/// should happen
	fn load_font(&mut self, name: &str, path: &str) -> Result<(),()>;

	fn font_face(&mut self, font: &str);

	fn font_size(&mut self, size: f32);

	fn font_color(&mut self, color: Color);

    fn text(&self, x: f32, y: f32, text: &str) -> f32;

	/// reset transformation so the normal transformation matrix is the current
	fn reset_transform(&mut self){
		self.set_transform(Transform::normal())
	}

	/// set the current transformation matrix
	fn set_transform(&mut self, t: Transform);

	/// get the current transformation matrix
	fn get_transform(&self) -> Transform;

	fn draw_path<P:AsPath>(&mut self, P);

	/// end of the drawing operation for one frame
	fn end(&mut self){}
}

/// Implementation of Backend for debugging purposes, it can eather log every call
/// or can save everything and can then be queried afterward or it can just be
/// used as a placeholder if drawing is not necessary.
pub struct DebugBackend{
	log: bool,
	fonts: Vec<String>,
}

impl DebugBackend{
	/// create a new Debug backend without logging enabled
	pub fn new() -> DebugBackend{
		DebugBackend{
			log: false,
			fonts: Vec::new(),
		}
	}
	/// create a backend with logging enabled
	pub fn log() -> DebugBackend{
		let mut n = DebugBackend::new();
		n.log = true;
		n
	}
}

impl Backend for DebugBackend{
	fn new(window: &glutin::Window) -> Self{
		DebugBackend{
			log: false,
			fonts: Vec::new(),
		}
	}
	fn begin(&mut self, w:i32, h:i32){
		if self.log{
			println!("% begin({}, {})", w, h);
		}
	}
	fn load_font(&mut self, name: &str, path: &str) -> Result<(),()>{
		if self.log{
			println!("% load_font(name:{:?}, path:{:?})", name, path);
		}
		Ok(())
	}
	fn font_face(&mut self, font: &str){
		if self.log{
			println!("% font_face(font:{:?})", font);
		}
	}
	fn font_size(&mut self, size: f32){
		if self.log{
			println!("% font_size(size:{:?})", size);
		}
	}
	fn font_color(&mut self, color: Color){
		if self.log{
			println!("% font_color(color:{:?})", color);
		}
	}
    fn text(&self, x: f32, y: f32, text: &str) -> f32{
		if self.log{
			println!("% text(x:{:?}, y:{:?}, text:{:?})", x,y,text);
		}
		0.
    }
	fn reset_transform(&mut self){
		if self.log{
			println!("% reset_transform()");
		}
	}
	fn set_transform(&mut self, t:Transform){
		if self.log{
			println!("% set_transform({:?})",t);
		}
	}
	fn get_transform(&self) -> Transform{
		unimplemented!()
	}
	fn draw_path<P:AsPath>(&mut self, p:P){
		unimplemented!()
	}
	fn end(&mut self){
		unimplemented!()
	}
}
