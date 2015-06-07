//! Some basic components for the day to day use
//! but they are not jet all ready, so choose only components marked as stable for productive use.

macro_rules! setter{
    (
        $e:ty,
        $( $prop:ident : $prop_type:ty ),+
    ) => (
        impl<'a> $e{
            $(
                pub fn $prop(&'a mut self, $prop: $prop_type) -> &'a mut $e{
                    self.$prop = $prop;
                    self
                }
            )+
        }
    )
}

//pub use self::layout::{Row, Col, Layout};

//pub use self::button::{Button, ButtonEvent};
//pub use self::label::Label;

//pub use self::image::Icon;
//pub use self::slider::{Slider, SliderEvent};
//pub use self::checkbox::{Checkbox, CheckboxEvent};
//pub use self::text_input::{TextInput, TextInputEvent};
//pub use self::icon::Icon;
//pub use self::icon::fa;

//pub mod image;
//pub mod text_input;
//pub mod tabs;

//pub mod slider;
//pub mod checkbox;
//pub mod layout;
//#[macro_use]
//pub mod button;
//pub mod label;

//pub mod icon;
