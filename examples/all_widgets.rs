extern crate gui;

use gui::components::{Button, ButtonEvent, Row, Label};

fn main(){
    gui::Window::new("test",640,480).show(|ctx|{
        //TODO: do something;
        /*gui!(ctx,//macro as syntactic shugar for the code down there
            1: Button("Hallo Welt") => {
                ButtonEvent::Click => println!("Button1 geclickt");
            },
            2: Button("Tschpss Welt") => {
                ButtonEvent::Click => println!("Button2 geclickt")
            },
            3: Row(){ //child nodes können mit einfachen geschweiften Klammern eingeleited werden
                1: Label("Hallo"),
                2: Label("Ich"),
                3: Label("Heiße"),
                4: Label("Simon"),
            },
            4: Group(){
                1: Line(0,20, 0, )
            }
        )*/

        ctx.add(1, &Button::new("Hallo Welt".to_string(), 100.0,20.0), Some(|event|{
            match event{
                ButtonEvent::Click => println!("Button1 geclickt"),
                _ => {}
            }
        }));
        ctx.go_to(0.0,60.0);
        ctx.add(2, &Button::new("Tschüss Welt".to_string(), 100.0,20.0), Some(|event|{
            match event{
                ButtonEvent::Click => println!("Button2 geclickt"),
                _ => {}
            }
        }));
        ctx.go_to(0.0,120.0);
        ctx.add(3, &Label::new("test label".to_string()), None);
    });
}
