# GUI for Rust

Little an simple gui library for rust which is designed very close to the react
gui and uses rust-piston as backend library for rendering and sdl2 for windowing.
But this will may change in the future, without any api changes.

## Tree of the GUI
- Root
  - 1.Child
  - 2.Child
    - 2.Child-1.Child
    - 2.Child-2.Child
  - 3.Child
  - 4.Child
  - 5.Child

Werend des renderns wird auf dem stack der bereich für die gui informationen reserviert und
die Komponenten erzeugt. Wenn komponenten Event listener registrieren, oder mit viel
aufwand verbunden sind werden diese kopiert und in einem externen Baum gespeichert.
Lößt nun einer dießer Events aus, wird die registrierte Funktion mit event-beschreibung aufgerufen.
Wird hierbei der Zustand der Komponente verändert, wird diese neu gezeichnet.

## Konzept

Ein Programm besteht aus mehreren Komponenten. Jedes dießer komponenten kann dafür sorgen, das
es gecachd wird, und das es einen Zustand besitzt.

### Initiales Rendern

Beim initialen Rendern werden render funktion der Haupt app aufgerufen und ein Baum konstruiert der alle Knoten beinhalted, die entweder
 - Eine Eigenen Zustand haben
 - Events empfangen
 - gecachte ergebnise haben
Alle weiteren Nodes werden jedes mal neu gezeichnet und es ist nicht nötig ihren Zustand zu speichern, somit ist der speicherverbrauch sehr gering und die Perfomance zimlich hos

### diffing

beim aufrufen der render Funktion werden neue Nodes konstruiert und alte übernommen. Dabei sollen nicht jedes mal neue erstellt, sondern bereits vorhandene Cachs genutzt werden. Ebenso muss der State zugeordnet werden. Dießer Prozess geschieht anhand der Koordinaten und Propertys, oder wenn spezifiziert anhand einer Nummer(aus Performance gründen zu bevorzugen).


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
        ListView::new()
            .add(Button::new()
                .text("OK")
                .onClick(|this| this.test();)
            ).add(Button::new()
                .text("No")
                .onClick(|this| this.abord();)
            ).add(Button::new()
                .text("quit")
                .onClick(|this| this.exit(); )
            );

        //syntactic shugar to speed up development
        gui!{
            ListView{
                Button{
                    text: "OK",
                    on click: {this.test();},
                }
                Button{
                    text: "No",
                    on click: {this.abord();},
                }
                Button{
                    text: "quit",
                    on click: {this.exit();}
                }
            }
        }
    }

    #[inline]
    fn set_state(&mut self, state: Option<()>){
        self.state = state
    }
    #[inline]
    fn get_state(&self) -> Option<()>{
        self.state
    }
}
impl Elemen{
    fn exit(){

    },
    //ein paar setter funktionen
}
```
