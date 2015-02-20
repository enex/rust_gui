#![crate_name = "rui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax)]
#![feature(core)]
#![feature(libc)]
#![feature(std_misc)]
#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate libc;
//extern crate cairo;
extern crate glutin;
extern crate gfx;
extern crate cgmath;

pub use glutin::{Event, ElementState, MouseCursor, MouseButton, VirtualKeyCode, Api, WindowBuilder};
pub use window::Window;
pub use context::Context;
pub use id::ID;
pub use draw2d::Paper;

use cgmath::FixedArray;
use cgmath::{Matrix, Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};
use gfx::{Device, DeviceExt, ToSlice};
use gfx::batch::RefBatch;

pub mod context;
pub mod components;
mod id;
mod window;
mod state;
pub mod draw2d;

//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]
//TODO: add android support

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	///event type the widget returns
	//type Event;
	type State = ();

	/// In this function only rendering to the screen and atouching event listeners is done
	/// the state of the component gets passed as a imutable reference, so this rutine is not
	/// able to change anything.
	/// It returns the (width, hight) of the area affected by the render method
	fn render(&self, ctx: &mut Context);

	/// Method which is used by the layout engine to get the size of a component
	/// by default the size will be calculated by using the render function with
	/// blocked drawing, but if speed matters this function can provide faster information
	#[inline(always)]
	fn size(&self) -> (f64, f64) {
		(0.0,0.0)
	}
}

/// A trait which should be implemented by every Widget featuring child widgets
/// like layout or Tabs or somthing like this.
pub trait Children{
	/// builder pattern like function which makes defining child nodes possible
	fn children() -> Self;
}

/// This has to be implemented by the root component of an application. This is
/// the only component which holds its state by default and is able to mutade
/// its propreties.
pub trait App{
	fn render(&mut self, ctx: &mut Context);
	fn close(&mut self){}
	fn start(&mut self){}
}


#[vertex_format]
#[derive(Copy)]
struct Vertex {
    #[name = "a_Pos"]
    pos: [f32; 2],

    #[name = "a_Color"]
    color: [f32; 4],
}

//The shaders, they are loaded from an external file
static VERTEX_SRC: &'static [u8] = include_bytes!("vertex-shader.vert");
static FRAGMENT_SRC: &'static [u8] = include_bytes!("fragment-shader.frag");

pub fn show<F>(window: &mut glutin::Window, draw: F) where F: Fn(&mut Paper) {
	let (w, h) = window.get_inner_size().unwrap();
	let mut frame = gfx::Frame::new(w as u16, h as u16);
	
	let mut device = gfx::GlDevice::new(|s| window.get_proc_address(s));
	let mut renderer = device.create_renderer();
	let mut context = gfx::batch::Context::new();
	
	let c = [0.5,0.5,1.,1.];
	
	let mut vertex_data = vec![
        Vertex { pos: [ -0.7, -0.7 ], color: [1.0, 0.0, 0.8,1.] },
        Vertex { pos: [  0.7, -0.7 ], color: [0.0, 1.0, 0.8,1.] },
        Vertex { pos: [  0.0,  0.7 ], color: [0.0, 0.0, 1.0,1.] },
        
        Vertex { pos: [ -0.5, -0.5 ], color: [0.0, 1.0, 0.0, 0.1] },
        Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0, 0.4] },
        Vertex { pos: [  0.0,  0.5 ], color: [0.0, 1.0, 0.0, 0.4] },
        
		Vertex { pos: [ -0.2, -0.2 ], color: c },
		Vertex { pos: [  0.2, -0.2 ], color: c },
		Vertex { pos: [  0.2,  0.2 ], color: c },
		Vertex { pos: [ -0.2, -0.2 ], color: c },
		Vertex { pos: [  0.2, 0.2 ], color: c },
		Vertex { pos: [  -0.2,  0.2 ], color: c },
    ];
    
    let mesh = device.create_mesh(&vertex_data);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

    let program = device.link_program(VERTEX_SRC, FRAGMENT_SRC)
                        .ok().expect("Failed to link program");

    let mut renderer = device.create_renderer();

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };
    let state = gfx::DrawState::new();
	
	let mut paper = Paper::new();//TODO: initialize with gl context
	
	let mut zoom:f64 = 0.0;
	let mut mouse_pos: (f32, f32) = (0., 0.);
	
	//let context = load(&window);
	while !window.is_closed() {
		window.wait_events();
		
		for event in window.poll_events() {
			match event{
				Event::Resized(w, h) => {//handle resizes
					frame.width = w as u16;
					frame.height = h as u16;
				},
				Event::MouseWheel(v) => {
					//TODO: zoom in and out
					zoom += (v as f64);
					println!("Zoom level: {}", zoom);
				},
				Event::MouseMoved(p) => {
					mouse_pos = (
						(p.0 as f32 / frame.width as f32)*2. -1.,
						-(p.1 as f32 / frame.height as f32)*2. +1.
					);
				},
				Event::MouseInput(glutin::ElementState::Pressed, _) => {
					let x = mouse_pos.0;
					let y = mouse_pos.1;
					vertex_data.push(Vertex { pos: [ x-0.1 , y-0.1 ], color: c });
					vertex_data.push(Vertex { pos: [ x-0.1, y+0.1 ], color: c });
					vertex_data.push(Vertex { pos: [ x, y ], color: c });
					println!("new data at ({}, {})", x, y);
				},
				_ => ()
			}
			println!("{:?}", event);
		}
		
		renderer.reset();
		renderer.clear(clear_data, gfx::COLOR, &frame);
		
		let mesh = device.create_mesh(&vertex_data);
		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);
		
		renderer.draw(&(&mesh, slice, &program, &(), &state), &frame).unwrap();
		device.submit(renderer.as_buffer());
		
		(draw)(&mut paper);
		//TODO: make drawing here
		
		window.swap_buffers();
	}
}
