# GUI for Rust

https://github.com/reem/rust-typemap

Little an simple gui library for rust inspired by react.
gui and uses nanovg as backend library for rendering.

https://github.com/TomBebbington/glutin should be used as glue to generate the
opengl context, but it seems not jet ready so sdl2 is used.

nanovg will be used instead of cairo which I used at first to test my idee, because
it is more lightweight and i need opengl anyway for other reasons, furthermore
most of cairos functions aren't needed.

Everything should be abstracted into the widget api including all drawing-operations.
This allows to switch the backend without changing the api. Aditionaly it simplifyes
the usage of the api because there are only some widgets you should know about and
not more.
Events should be hendled as discussed earlier, for convinience the api could be constructed
in a way that every event that will propagate up to the top level has only one event type,
this way mapping is much simpler


## Macros
[...]                container for child nodes
{...}                indicates some code for this context
|$event:ident|{...}  some event handling code
$name:ident(...)     a new widget
```rust
data!(
	Group[
		Path("M12,54L4,4L34,100"),
		Button(text="Hallo")|event|{
			//handle the events here
			//
		}//==Button("Hallo")
	]
);
widget! Button(
	(
		text: &str = "",
		icon: Option<Icon> = None,
	){
		Path(...)
	}
);
```

```
Button{
	x: 10
	y: 100
	text: "test"

	Name{
		x: 100
		y: 10
		text: "wer auch immer"
	}
}
```
## TODO:
 - make component system work
 - build some basic components
   - button
   - radio
   - label
   - lineEdit
   - numberInput
   - multilineEdit
   - Tabs
   - List
   - Box-Layout
 - create macro for easier interaction
 - animation
 - caching
 - state management
 - documentation
 - theming
 - publish

## Screenshots

![all_widgets example](./screenshot_all_widgets.png)
