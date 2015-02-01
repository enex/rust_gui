extern crate gui;

use gui::components::{Button};
//TODO: insert widgets here

///a macro to call ctx.add function more convinient and with less writing
macro_rules! gui{
    ($target:ident ,
        $(
            $name:ident ( $($arg_val:expr),* )
            $( . $prop_name ( $( $prop_arg_val ),* ) )*
        ),+
    ) => ({
        $({
            $target.add(
                $name::new($($arg_val),*)
                $(.$prop_name($($prop_arg_val),*))* ,
                None
            );
        })+
    })
}

fn main(){
    gui::Window::new("test",640,480).show(|ctx|{
        ctx.add(1, &Button::new("test-Button",270.0,50.0));
    });
    //gui::Window::new("test2", 640, 480).show(|ctx| ctx.add());
}
