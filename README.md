# GUI for Rust

Little an simple gui library for rust inspired by react.
gui and uses cairo as default backend library for rendering.

Everything should be abstracted into the widget api including all drawing-operations.
This allows to switch the backend without changing the api. Aditionaly it simplifyes
the usage of the api because there are only some widgets you should know about and
not more.

## Macros
**this is not jet implemented and might change**

| pattern             | description                          |
|---------------------|--------------------------------------|
| [...]               | container for child nodes            |
| {...}               | indicates some code for this context |
| |$event:ident|{...} | some event handling code             |
| $name:ident(...)    | a new widget                         |

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

## Links
https://github.com/reem/rust-typemap
https://github.com/TomBebbington/glutin
