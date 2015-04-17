
/// macro which simplifyes the definition of a widget
///
/// ```
/// widget!(ListItem<T>{
///     type Event = enum{
///         Hover,
///         Click,
///         Leave,
///     };
///     data: <T>,
/// } -> |self, ctx|{
///     //add widgets and what you want to do
/// });

macro_rules! widget_type{
    ($name:ident, $tp:ty) => (type $name = ($tp););
    ($name:ident) => (type $name = ());
}

#[unstable]
#[macro_export]
macro_rules! widget{
    ($name:ident{
        $( $field:ident : $tp:ty ),*
        $( type $name = $state:ty ),* //TODO: allow enum type definition in here
    } -> | $s:ident, $ctx:ident | $body:expr ) => (
        #[derive(Copy, Clone)]
        pub struct $name{
            $(pub $field : $p )+
        }

        impl Widget for $name{
            fn render( &$s, $ctx :&mut Context<$event, $state> ){
                $body
            }
        }
    )
}//TODO: make it ready

/// macro to quickly define setters implementing the builder Pattern for
/// a given type. This can be useful for widgets
#[unstable]
#[macro_export]
macro_rules! setter{
    //TODO: custom function for stter
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

/// macro to perform the exact same thing as you do with ctx.add() and similat
/// methods but a bit shorter
///
/// gui!(ctx,
/// 	Button(text:"Hallo") => {
///			Click => /*do something on click*/
/// 	});
///
/// ctx.add_event(Button{
/// 	text: "Hallo",
/// 	Default::default(),
/// }, |event| match event{
/// 	Click => /*do something on click*/
/// });
#[unstable]
#[macro_export]
macro_rules! gui{
    //TODO: add id
    //TODO: for more than one item
    //TODO: event handling
    //TODO: child nodes
    ($ctx:ident, $widg:ty{ //one Widget without callback or so
        $( $field:ident : $val:expr ),*
    }) => ({
        use std::default::Default;

        $ctx.add($widg{
            $( $field : $val )*
        });
    });
}
