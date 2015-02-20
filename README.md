# GUI for Rust

https://github.com/reem/rust-typemap

Little an simple gui library for rust inspired by react.
gui and uses nanovg as backend library for rendering.

https://github.com/TomBebbington/glutin should be used as glue to generate the
opengl context, but it seems not jet ready so sdl2 is used.

nanovg will be used instead of cairo which I used at first to test my idee, because
it is more lightweight and i need opengl anyway for other reasons, furthermore
most of cairos functions aren't needed.

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
