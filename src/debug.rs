/*!
Some things usefull for debuging like a DebugBackend which just
logs all draw calls.
*/

use Backend;
use primitives;

/// Backend which logs all draw calls to the terminal
pub struct DebugBackend;

impl Backend for DebugBackend{
    fn load_font(&mut self, path: &str){
        println!("load_font({:?})", path);
    }
    fn draw_line(&mut self, line: primitives::Line){
        println!("draw_line({:?})", path);
    }
    fn draw_rect(&mut self, rect: primitives::Rect){
		println!("draw_rect({:?})", rect);
	}
    fn draw_circle(&mut self, c: primitives::Circle){
        println!("draw_circle({:?})", c)
    }
	fn draw_path(&mut self, p: primitives::Path){
        println!("draw_path({:?})", p)
    }
	fn draw_polygon(&mut self, p: primitives::Polygon){
        println!("draw_polygon({:?})", p)
    }
}
