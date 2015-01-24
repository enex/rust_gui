use Widget;
use Context;

///Layout for placing everything in a row
pub struct Row<F> where F: Fn(&mut Context<()>){
    pub spacing: f64,
    pub render_childs: F,
}

impl<F> Row<F> where F: Fn(&mut Context<()>){
	/// Create a new layout element with a defined spacing and a function
	/// to render its child nodes. The child nodes. These nodes are rendered
	/// foreahead to measure their size and then there position is calculated
	/// based on this measurements. This is relatively processing intensive
	/// and thus will be chached
	pub fn new(spacing: f64, render_childs: F) -> Row<F>{
		Row{
			render_childs: render_childs,
			spacing: spacing
		}
	}
}

impl<F> Widget for Row<F> where F: Fn(&mut Context<()>){
	type Event = ();

	fn render(&self, ctx: &mut Context<()>) {
		//TODO: check if changed, and already calculated, if so
			//TODO: create a fake context
			//TODO: call the render_childs funktion with this context
			//TODO: save the extracted sizes as a state for later use
			//TODO: calculate position

		(self.render_childs)(ctx);//call with real context to the screen
	}
}


///Layout for placing into a column
#[derive(Copy)]
pub struct Col{
    pub spacing: f64,
}

///Layout management for all widgets.
///It specifies the size of a widget
#[derive(Copy)]
pub enum Layout{
    ///make it large enough for its content
    Fit,
    ///make it as large as the params (width, height)
    Fill(f64,f64),
}
