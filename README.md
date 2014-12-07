# GUI for Rust

Little an simple gui library for rust inspired by react.
gui and uses cairo as backend library for rendering.
But this will may change in the future.

## Konzept

Ein Programm besteht aus mehreren Komponenten. Jedes dieser komponenten kann dafür sorgen, das
es gecachd wird, und das es einen Zustand besitzt.

Der Baum mit den Componenten, wird allerdings nicht beahalten, sondern wenn benötigt lazy
erstellt. Das heißt es wird kein zusätzlicher Speicherplatz benötigt wenn sich nichts ändert,
und Änderungen können schnell und einfach geschehen.

Jedes element besitzt eine Render-funktion. In dieser werden die Elemente initialisiert und an
den context geschickt. Hierbei bekommt jedes Objekt eine ID um.

# Events
vom Context selbst werden Events bereitgestellt, die von jedem element aboniert werden könne.
Wenn eines dieser Events auslöst wird die Render-funktion des gesuchten elements aufgerufen, und
das Event gehandled.

Außerdem können Componenten selbs event bereitstellen. Diese werden vom Vontext verwalted und
gehen immer vom Child-Element zum Parent-Element, so das das Parent element dieße behandeln kann.
Events können mit der Funktion `ctx.emit(name: Eq, info: <Info>);` ausgelößst werden. Sie werden
übertragen, so bald die Funktion beendet ist.
Überlegung: Event möglicherweiße Als rückgabeparamter ansehen.

# Performance
Um eine gute performance zu erreichen, können gezeichnete Bereiche gecached werden, so dass sie nur
einmal gezeichnet werden. das ist mit der Funktion `ctx.cach_all();` für das Ganze lement und mit
`ctx.cach();` für ein einzelnes Element möglich.

## Zeichnen
Es wird immer von oben nach unten gezeichnet das heißt bei der Anordnung ist die Reihenfolge zu
beachten. Um die Position einzelner Komponenten zu bestimmen kann die Parent funktion deren Position
bestimmen.

## Syntactic shugar
```
struct Component{
    state: Option<()>,
    //other paramters
}
impl Element for Component{
    fn draw(); //function to draw raw things, it will be called after render
    fn render(&self, ctx: context) -> Element {
        ctx.add(ListView::new().childs(|ctx|{
            ctx.add(1,Button::new("OK"),|event|{
                match event{
                    Click => self.test()
                }
            })
        }))

        gui!(ctx,
            ListView(){
                Button("OK")=>{
                    Click => self.test()
                }
                Button(text: "No") => {
                    Click => self.abord()
                }
                Button("quit") => {
                    Click => self.exit()
                }
            }
        );
    }
    gui_state!(state, Option<()>);//generates acces functions to the state
}
impl Elemen{
    fn exit(){

    },
    //ein paar setter funktionen
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
