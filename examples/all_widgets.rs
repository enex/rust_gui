extern crate gui;

use gui::components::{
    Button, ButtonEvent,
    Row,
    Label,
    Icon,
    Slider, SliderEvent,
    Checkbox, CheckboxEvent,
    TextInput, TextInputEvent};

fn main(){
    let mut sv: f64 = 10.0;
    let mut checked = false;
    let mut tiv = "".to_string();
    gui::Window::new("test",640,480).show(|ctx|{
        ctx.draw(|c|{//set background-color
            c.set_source_rgb(0.9, 0.9, 0.9);
            c.paint();
        });
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

        ctx.add(1, &mut Button::new("Button1".to_string(), 100.0,20.0), Some(|event| match event{
            ButtonEvent::Click => println!("Button1 geclickt"),
            ButtonEvent::Hover => println!("Hover Button1"),
            _ => {}
        }));
        ctx.go_to(5.0,200.0);
        ctx.add(20, &mut Checkbox::new(checked, 20.0,20.0), Some(|event| match event{
            CheckboxEvent::Change(now) => {
                println!("Checkbox change {}", now);
                checked = now;
            }
        }));
        ctx.go_to(150.0,20.0);
        ctx.add(21, &mut TextInput::new(tiv.clone(), "Text".to_string()), Some(|event| match event{
            TextInputEvent::Change(now) => {
                println!("Text-input change {}", now)
                tiv = now;
            },
            //_ => {}
        }));
        ctx.go_to(150.0,50.0);
        ctx.add(22, &mut TextInput::new(tiv.clone(), "Text".to_string()), Some(|event| match event{
            TextInputEvent::Change(now) => {
                println!("Text-input change {}", now)
                tiv = now;
            },
            //_ => {}
        }));
        ctx.go_to(5.0,60.0);
        ctx.add(2, &mut Button::new("Button2".to_string(), 100.0,20.0), Some(|event| match event{
            ButtonEvent::Click => println!("Button2 geclickt"),
            ButtonEvent::Hover => println!("Hover Button2"),
            _ => {}
        }));
        ctx.go_to(10.0,120.0+sv);
        ctx.add(3, &mut Label::new(format!("Wert: {}", sv)), None);

        ctx.go_to(110.0,10.0);
        ctx.add(12, &mut Slider::new(sv,30.0,1.0), Some(|event| match event{
            SliderEvent::Changed(v) => {
                println!("new value: {}", v);
                sv = v;
            },
            _ => {}
        }));//add a slider
        ctx.go_to(10.0,50.0);
        //add an icon described by an svg path
        /*ctx.add(4, &mut Icon::new("M15.41 7.41l-1.41-1.41-6 6 6 6 1.41-1.41-4.58-4.59z", (0.2,0.2,0.2)), None);
        ctx.go_to(40.0,50.0);
        ctx.add(5, &mut Icon::new("M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z", (0.2,0.2,0.2)), None);
        ctx.go_to(70.0,50.0);
        ctx.add(6, &mut Icon::new("M19 6.41l-1.41-1.41-5.59 5.59-5.59-5.59-1.41 1.41 5.59 5.59-5.59 5.59 1.41 1.41 5.59-5.59 5.59 5.59 1.41-1.41-5.59-5.59z", (0.2,0.2,0.2)), None);
        ctx.go_to(100.0,50.0);
        ctx.add(7, &mut Icon::new("M6 18c0 .55.45 1 1 1h1v3.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-3.5h2v3.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-3.5h1c.55 0 1-.45 1-1v-10h-12v10zm-2.5-10c-.83 0-1.5.67-1.5 1.5v7c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-7c0-.83-.67-1.5-1.5-1.5zm17 0c-.83 0-1.5.67-1.5 1.5v7c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-7c0-.83-.67-1.5-1.5-1.5zm-4.97-5.84l1.3-1.3c.2-.2.2-.51 0-.71-.2-.2-.51-.2-.71 0l-1.48 1.48c-.79-.4-1.69-.63-2.64-.63-.96 0-1.86.23-2.66.63l-1.49-1.48c-.2-.2-.51-.2-.71 0-.2.2-.2.51 0 .71l1.31 1.31c-1.48 1.09-2.45 2.84-2.45 4.83h12c0-1.99-.97-3.75-2.47-4.84zm-5.53 2.84h-1v-1h1v1zm5 0h-1v-1h1v1z", (0.2,0.7,0.2)), None);
        ctx.go_to(130.0,50.0);
        ctx.add(8, &mut Icon::new("M11.791,25.229c1.027-0.104,1.162-1.191,0.68-1.666c-0.398-0.392-2.598-2.022-3.171-2.664C9.033,20.6,8.673,20.454,8.52,20.12c-0.352-0.771-0.598-1.869-0.151-2.658c0.081-0.144,0.133-0.078,0.071,0.22c-0.351,1.684,0.746,3.059,0.986,2.354c0.167-0.487,0.013-1.358,0.102-2.051c0.158-1.226,1.273-3.577,1.763-3.712c-0.755-1.398,0.886-2.494,0.866-3.723c-0.014-0.798,0.701,0.982,1.419,1.359c0.802,0.422,1.684-0.794,2.936-1.41c0.354-0.176,0.809-0.376,0.776-0.524c-0.146-0.718-1.644,0.886-2.979,0.939c-0.61,0.024-0.837-0.12-1.072-0.347c-0.712-0.689,0.073-0.115,1.132-0.307c0.471-0.085,0.629-0.163,1.128-0.365c0.5-0.201,1.069-0.5,1.636-0.654c0.395-0.106,0.361-0.402,0.208-0.491c-0.088-0.051-0.219-0.046-0.321,0.133c-0.244,0.419-1.383,0.661-1.74,0.771c-0.457,0.14-0.962,0.271-1.634,0.243c-1.021-0.042-0.782-0.509-1.513-0.928c-0.213-0.122-0.156-0.444,0.129-0.729c0.148-0.148,0.557-0.232,0.76-0.572c0.028-0.047,0.289-0.32,0.494-0.461c0.07-0.049,0.076-1.295-0.562-1.32c-0.543-0.021-0.697,0.398-0.675,0.818c0.022,0.419,0.245,0.765,0.393,0.764c0.285-0.004,0.019,0.311-0.138,0.361c-0.237,0.078-0.562-0.934-0.525-1.418c0.039-0.506,0.303-1.4,0.942-1.383c0.576,0.016,0.993,0.737,0.973,1.983c-0.003,0.211,0.935-0.101,1.247,0.229c0.224,0.236-0.767-2.207,1.438-2.375c0.582,0.111,1.14,0.305,1.371,1.641c-0.086,0.139,0.146,1.07-0.215,1.182c-0.438,0.135-0.707-0.02-0.453-0.438c0.172-0.418,0.004-1.483-0.882-1.42c-0.887,0.064-0.769,1.637-0.526,1.668c0.243,0.031,0.854,0.465,1.282,0.549c1.401,0.271,0.371,1.075,0.555,2.048c0.205,1.099,0.929,0.809,1.578,3.717c0.137,0.177,0.676,0.345,1.199,2.579c0.473,2.011-0.195,3.473,0.938,3.353c0.256-0.026,0.629-0.1,0.792-0.668c0.425-1.489-0.213-3.263-0.855-4.46c-0.375-0.698-0.729-1.174-0.916-1.337c0.738,0.436,1.683,1.829,1.898,2.862c0.286,1.358,0.49,1.934,0.059,3.37c0.25,0.125,0.871,0.39,0.871,0.685c-0.647-0.53-2.629-0.625-2.68,0.646c-0.338,0.008-0.594,0.034-0.811,0.293c-0.797,0.944-0.059,2.842-0.139,3.859c-0.07,0.896-0.318,1.783-0.46,2.683c-0.474-0.019-0.428-0.364-0.274-0.852c0.135-0.431,0.351-0.968,0.365-1.484c0.012-0.467-0.039-0.759-0.156-0.831c-0.118-0.072-0.303,0.074-0.559,0.485c-0.543,0.875-1.722,1.261-2.821,1.397c-1.099,0.138-2.123,0.028-2.664-0.578c-0.186-0.207-0.492,0.058-0.529,0.111c-0.049,0.074,0.18,0.219,0.352,0.533c0.251,0.461,0.49,1.159-0.105,1.479C12.83,26.314,12.316,26.221,11.791,25.229L11.791,25.229zM11.398,25.188c0.395,0.621,1.783,3.232-0.652,3.571c-0.814,0.114-2.125-0.474-3.396-0.784c-1.142-0.279-2.301-0.444-2.949-0.627c-0.391-0.108-0.554-0.25-0.588-0.414c-0.091-0.434,0.474-1.041,0.503-1.555c0.028-0.514-0.188-0.779-0.364-1.199c-0.177-0.42-0.224-0.734-0.081-0.914c0.109-0.141,0.334-0.199,0.698-0.164c0.462,0.047,1.02-0.049,1.319-0.23c0.505-0.309,0.742-0.939,0.516-1.699c0,0.744-0.244,1.025-0.855,1.366c-0.577,0.319-1.467,0.062-1.875,0.416c-0.492,0.427,0.175,1.528,0.12,2.338c-0.042,0.622-0.69,1.322-0.401,1.946c0.291,0.627,1.648,0.695,3.064,0.99c2.012,0.422,3.184,1.153,4.113,1.188c1.356,0.05,1.564-1.342,3.693-1.36c0.621-0.033,1.229-0.052,1.835-0.06c0.688-0.009,1.375-0.003,2.079,0.014c1.417,0.034,0.931,0.773,1.851,1.246c0.774,0.397,2.17,0.241,2.504-0.077c0.451-0.431,1.662-1.467,2.592-1.935c1.156-0.583,3.876-1.588,1.902-2.812c-0.461-0.285-1.547-0.588-1.639-2.676c-0.412,0.366-0.365,2.312,0.784,2.697c1.283,0.431,2.085,1.152-0.301,1.969c-1.58,0.54-1.849,0.706-3.099,1.747c-1.267,1.054-3.145,0.636-2.815-1.582c0.171-1.155,0.269-2.11-0.019-3.114c-0.142-0.49-0.211-1.119-0.114-1.562c0.187-0.858,0.651-1.117,1.106-0.293c0.285,0.519,0.385,1.122,1.408,1.171c1.607,0.077,1.926-1.553,2.439-1.627c0.343-0.05,0.686-1.02,0.425-2.589c-0.28-1.681-1.269-4.332-2.536-5.677c-1.053-1.118-1.717-2.098-2.135-3.497c-0.352-1.175-0.547-2.318-0.475-3.412c0.094-1.417-0.691-3.389-1.943-4.316c-0.782-0.581-2.011-0.893-3.122-0.88c-0.623,0.007-1.21,0.099-1.661,0.343c-1.855,1.008-2.113,2.445-2.086,4.088c0.025,1.543,0.078,3.303,0.254,4.977c-0.208,0.77-1.288,2.227-1.979,3.114C8.59,14.233,8.121,16.01,7.52,17.561c-0.321,0.828-0.862,1.2-0.908,2.265C6.6,20.122,6.61,20.891,6.894,20.672C7.98,19.829,9.343,21.95,11.398,25.188L11.398,25.188zM17.044,2.953c-0.06,0.176-0.3,0.321-0.146,0.443c0.152,0.123,0.24-0.171,0.549-0.281c0.08-0.028,0.449,0.012,0.519-0.164c0.03-0.077-0.19-0.164-0.321-0.291c-0.133-0.125-0.262-0.236-0.386-0.229C16.938,2.451,17.096,2.798,17.044,2.953L17.044,2.953zM18.934,9.35c0.115-0.121,0.174,0.207,0.483,0.402c0.244,0.154,0.481,0.04,0.545,0.354c0.044,0.225-0.097,0.467-0.284,0.436C19.35,10.486,18.596,9.705,18.934,9.35L18.934,9.35zM13.832,7.375c-0.508-0.037-0.543,0.33-0.375,0.324C13.629,7.693,13.523,7.408,13.832,7.375L13.832,7.375zM12.96,6.436c0.06-0.013,0.146,0.09,0.119,0.233c-0.037,0.199-0.021,0.324,0.117,0.325c0.022,0,0.048-0.005,0.056-0.057c0.066-0.396-0.14-0.688-0.225-0.711C12.834,6.178,12.857,6.458,12.96,6.436L12.96,6.436zM16.663,6.268c0.129,0.039,0.253,0.262,0.28,0.504c0.002,0.021,0.168-0.035,0.17-0.088c0.011-0.389-0.321-0.571-0.408-0.562C16.506,6.139,16.562,6.238,16.663,6.268L16.663,6.268zM14.765,7.423c0.463-0.214,0.625,0.118,0.465,0.171C15.066,7.648,15.065,7.345,14.765,7.423L14.765,7.423zM9.178,15.304c-0.219-0.026,0.063-0.19,0.184-0.397c0.131-0.227,0.105-0.511,0.244-0.469s0.061,0.2-0.033,0.461C9.491,15.121,9.258,15.313,9.178,15.304L9.178,15.304z", (0.2,0.2,0.2)), None);
        ctx.go_to(200.0,100.0);
        ctx.add(9, &mut Icon::new("M122.631,69.716l-4.394-2.72c-0.037-0.428-0.079-0.855-0.125-1.28l3.776-3.522c0.384-0.358,0.556-0.888,0.452-1.401  c-0.101-0.515-0.462-0.939-0.953-1.122l-4.827-1.805c-0.121-0.418-0.248-0.833-0.378-1.246l3.011-4.182  c0.307-0.425,0.37-0.978,0.17-1.463c-0.2-0.483-0.637-0.829-1.154-0.914l-5.09-0.828c-0.198-0.386-0.404-0.766-0.612-1.143  l2.139-4.695c0.219-0.478,0.174-1.034-0.118-1.468c-0.291-0.436-0.784-0.691-1.31-0.671l-5.166,0.18  c-0.267-0.334-0.539-0.665-0.816-0.99l1.187-5.032c0.12-0.511-0.031-1.046-0.403-1.417c-0.369-0.37-0.905-0.523-1.416-0.403  l-5.031,1.186c-0.326-0.276-0.657-0.549-0.992-0.816l0.181-5.166c0.02-0.523-0.235-1.02-0.671-1.31  c-0.437-0.292-0.99-0.336-1.467-0.119l-4.694,2.14c-0.379-0.208-0.759-0.414-1.143-0.613l-0.83-5.091  c-0.084-0.516-0.43-0.954-0.914-1.154c-0.483-0.201-1.037-0.136-1.462,0.17l-4.185,3.011c-0.412-0.131-0.826-0.257-1.244-0.377  l-1.805-4.828c-0.183-0.492-0.607-0.853-1.122-0.955c-0.514-0.101-1.043,0.07-1.4,0.452l-3.522,3.779  c-0.425-0.047-0.853-0.09-1.28-0.125l-2.72-4.395c-0.275-0.445-0.762-0.716-1.286-0.716s-1.011,0.271-1.285,0.716l-2.72,4.395  c-0.428,0.035-0.856,0.078-1.281,0.125l-3.523-3.779c-0.357-0.382-0.887-0.553-1.4-0.452c-0.515,0.103-0.939,0.463-1.122,0.955  l-1.805,4.828c-0.418,0.12-0.832,0.247-1.245,0.377l-4.184-3.011c-0.425-0.307-0.979-0.372-1.463-0.17  c-0.483,0.2-0.83,0.638-0.914,1.154l-0.83,5.091c-0.384,0.199-0.764,0.404-1.143,0.613l-4.694-2.14  c-0.477-0.218-1.033-0.173-1.467,0.119c-0.436,0.29-0.691,0.787-0.671,1.31l0.18,5.166c-0.334,0.267-0.665,0.54-0.992,0.816  l-5.031-1.186c-0.511-0.119-1.047,0.033-1.417,0.403c-0.372,0.371-0.523,0.906-0.403,1.417l1.185,5.032  c-0.275,0.326-0.547,0.656-0.814,0.99l-5.166-0.18c-0.521-0.015-1.019,0.235-1.31,0.671c-0.292,0.434-0.336,0.99-0.119,1.468  l2.14,4.695c-0.208,0.377-0.414,0.757-0.613,1.143l-5.09,0.828c-0.517,0.084-0.953,0.43-1.154,0.914  c-0.2,0.485-0.135,1.038,0.17,1.463l3.011,4.182c-0.131,0.413-0.258,0.828-0.378,1.246l-4.828,1.805  c-0.49,0.183-0.851,0.607-0.953,1.122c-0.102,0.514,0.069,1.043,0.452,1.401l3.777,3.522c-0.047,0.425-0.089,0.853-0.125,1.28  l-4.394,2.72c-0.445,0.275-0.716,0.761-0.716,1.286s0.271,1.011,0.716,1.285l4.394,2.72c0.036,0.428,0.078,0.855,0.125,1.28  l-3.777,3.523c-0.383,0.357-0.554,0.887-0.452,1.4c0.102,0.515,0.463,0.938,0.953,1.122l4.828,1.805  c0.12,0.418,0.247,0.833,0.378,1.246l-3.011,4.183c-0.306,0.426-0.371,0.979-0.17,1.462c0.201,0.485,0.638,0.831,1.155,0.914  l5.089,0.828c0.199,0.386,0.403,0.766,0.613,1.145l-2.14,4.693c-0.218,0.477-0.173,1.032,0.119,1.468  c0.292,0.437,0.789,0.692,1.31,0.671l5.164-0.181c0.269,0.336,0.54,0.665,0.816,0.992l-1.185,5.033  c-0.12,0.51,0.031,1.043,0.403,1.414c0.369,0.373,0.906,0.522,1.417,0.402l5.031-1.185c0.327,0.278,0.658,0.548,0.992,0.814  l-0.18,5.167c-0.02,0.523,0.235,1.019,0.671,1.311c0.434,0.291,0.99,0.335,1.467,0.117l4.694-2.139  c0.378,0.21,0.758,0.414,1.143,0.613l0.83,5.088c0.084,0.518,0.43,0.956,0.914,1.155c0.483,0.201,1.038,0.136,1.463-0.169  l4.182-3.013c0.413,0.131,0.828,0.259,1.246,0.379l1.805,4.826c0.183,0.49,0.607,0.853,1.122,0.953  c0.514,0.104,1.043-0.068,1.4-0.452l3.523-3.777c0.425,0.049,0.853,0.09,1.281,0.128l2.72,4.394  c0.274,0.443,0.761,0.716,1.285,0.716s1.011-0.272,1.286-0.716l2.72-4.394c0.428-0.038,0.855-0.079,1.28-0.128l3.522,3.777  c0.357,0.384,0.887,0.556,1.4,0.452c0.515-0.101,0.939-0.463,1.122-0.953l1.805-4.826c0.418-0.12,0.833-0.248,1.246-0.379  l4.183,3.013c0.425,0.305,0.979,0.37,1.462,0.169c0.484-0.199,0.83-0.638,0.914-1.155l0.83-5.088  c0.384-0.199,0.764-0.406,1.143-0.613l4.694,2.139c0.477,0.218,1.032,0.174,1.467-0.117c0.436-0.292,0.69-0.787,0.671-1.311  l-0.18-5.167c0.334-0.267,0.665-0.536,0.991-0.814l5.031,1.185c0.511,0.12,1.047-0.029,1.416-0.402  c0.372-0.371,0.523-0.904,0.403-1.414l-1.185-5.033c0.276-0.327,0.548-0.656,0.814-0.992l5.166,0.181  c0.521,0.021,1.019-0.234,1.31-0.671c0.292-0.436,0.337-0.991,0.118-1.468l-2.139-4.693c0.209-0.379,0.414-0.759,0.612-1.145  l5.09-0.828c0.518-0.083,0.954-0.429,1.154-0.914c0.2-0.483,0.137-1.036-0.17-1.462l-3.011-4.183  c0.13-0.413,0.257-0.828,0.378-1.246l4.827-1.805c0.491-0.184,0.853-0.607,0.953-1.122c0.104-0.514-0.068-1.043-0.452-1.4  l-3.776-3.523c0.046-0.425,0.088-0.853,0.125-1.28l4.394-2.72c0.445-0.274,0.716-0.761,0.716-1.285S123.076,69.991,122.631,69.716z   M93.222,106.167c-1.678-0.362-2.745-2.016-2.385-3.699c0.359-1.681,2.012-2.751,3.689-2.389c1.678,0.359,2.747,2.016,2.387,3.696  S94.899,106.526,93.222,106.167z M91.729,96.069c-1.531-0.328-3.037,0.646-3.365,2.18l-1.56,7.28  c-4.814,2.185-10.16,3.399-15.79,3.399c-5.759,0-11.221-1.274-16.121-3.552l-1.559-7.28c-0.328-1.532-1.834-2.508-3.364-2.179  l-6.427,1.38c-1.193-1.228-2.303-2.536-3.323-3.917h31.272c0.354,0,0.59-0.064,0.59-0.386V81.932c0-0.322-0.236-0.386-0.59-0.386  h-9.146v-7.012h9.892c0.903,0,4.828,0.258,6.083,5.275c0.393,1.543,1.256,6.562,1.846,8.169c0.588,1.802,2.982,5.402,5.533,5.402  h15.583c0.177,0,0.366-0.02,0.565-0.056c-1.081,1.469-2.267,2.859-3.544,4.158L91.729,96.069z M48.477,106.015  c-1.678,0.362-3.33-0.708-3.691-2.389c-0.359-1.684,0.708-3.337,2.386-3.699c1.678-0.359,3.331,0.711,3.691,2.392  C51.222,103.999,50.154,105.655,48.477,106.015z M36.614,57.91c0.696,1.571-0.012,3.412-1.581,4.107  c-1.569,0.697-3.405-0.012-4.101-1.584c-0.696-1.572,0.012-3.41,1.581-4.107C34.083,55.63,35.918,56.338,36.614,57.91z   M32.968,66.553l6.695-2.975c1.43-0.635,2.076-2.311,1.441-3.744l-1.379-3.118h5.423V81.16H34.207  c-0.949-3.336-1.458-6.857-1.458-10.496C32.749,69.275,32.824,67.902,32.968,66.553z M62.348,64.179v-7.205h12.914  c0.667,0,4.71,0.771,4.71,3.794c0,2.51-3.101,3.41-5.651,3.41H62.348z M109.28,70.664c0,0.956-0.035,1.902-0.105,2.841h-3.926  c-0.393,0-0.551,0.258-0.551,0.643v1.803c0,4.244-2.393,5.167-4.49,5.402c-1.997,0.225-4.211-0.836-4.484-2.058  c-1.178-6.626-3.141-8.041-6.241-10.486c3.847-2.443,7.85-6.047,7.85-10.871c0-5.209-3.571-8.49-6.005-10.099  c-3.415-2.251-7.196-2.702-8.216-2.702H42.509c5.506-6.145,12.968-10.498,21.408-12.082l4.786,5.021  c1.082,1.133,2.874,1.175,4.006,0.092l5.355-5.122c11.221,2.089,20.721,9.074,26.196,18.657l-3.666,8.28  c-0.633,1.433,0.013,3.109,1.442,3.744l7.058,3.135C109.216,68.115,109.28,69.381,109.28,70.664z M68.705,28.784  c1.24-1.188,3.207-1.141,4.394,0.101c1.185,1.245,1.14,3.214-0.103,4.401c-1.24,1.188-3.207,1.142-4.394-0.102  C67.418,31.941,67.463,29.972,68.705,28.784z M105.085,58.061c0.695-1.571,2.531-2.28,4.1-1.583  c1.569,0.696,2.277,2.536,1.581,4.107c-0.695,1.572-2.531,2.281-4.101,1.584C105.098,61.473,104.39,59.634,105.085,58.061z", (0.2,0.2,0.2)), None);
        ctx.go_to(-20.0,0.0);
        ctx.add(10, &mut Icon::new("4.872c2.689,0,4.871,2.182,4.871,4.872C20.871,18.689,18.689,20.871,16,20.871z", (0.2,0.2,0.2)), None);
        ctx.add(11, &mut Icon::new("M22,4.582h-2v3.335h2V4.582zM25.416,5.748H23v3.17h-4v-3.17h-6v3.168H9.002V5.748H6.583v21.555h18.833V5.748zM24.418,26.303H7.584V13.988h16.833V26.303zM12,4.582h-2v3.335h2V4.582zM19.428,23.962h1.568v-7.788h-1.277c0,0.067-0.021,0.172-0.061,0.312c-0.066,0.232-0.168,0.419-0.299,0.559c-0.193,0.204-0.443,0.34-0.75,0.408c-0.193,0.043-0.531,0.075-1.014,0.097v1.042h1.832V23.962zM13.673,22.909c-0.489,0-0.827-0.188-1.013-0.564c-0.101-0.203-0.15-0.461-0.15-0.773h-1.504c0.025,0.62,0.15,1.121,0.376,1.504c0.429,0.721,1.194,1.08,2.296,1.08c0.895,0,1.569-0.25,2.026-0.749c0.455-0.5,0.684-1.079,0.684-1.737c0-0.627-0.195-1.121-0.586-1.482c-0.261-0.24-0.461-0.36-0.602-0.36c0.187-0.071,0.365-0.206,0.537-0.403c0.272-0.314,0.408-0.701,0.408-1.16c0-0.647-0.228-1.164-0.684-1.549c-0.456-0.386-1.056-0.578-1.8-0.578c-0.4,0-0.738,0.049-1.014,0.146c-0.276,0.097-0.514,0.236-0.714,0.419c-0.269,0.258-0.465,0.539-0.591,0.843c-0.117,0.348-0.184,0.715-0.198,1.102h1.429c-0.007-0.384,0.074-0.689,0.244-0.919c0.169-0.229,0.435-0.344,0.795-0.344c0.314,0,0.559,0.094,0.731,0.279c0.174,0.187,0.26,0.428,0.26,0.726c0,0.458-0.169,0.763-0.508,0.913c-0.196,0.09-0.543,0.138-1.039,0.145v1.096c0.507,0,0.878,0.049,1.114,0.146c0.414,0.172,0.621,0.514,0.621,1.026c0,0.387-0.112,0.683-0.335,0.889C14.234,22.807,13.973,22.909,13.673,22.909z", (0.2,0.2,0.2)), None);
        */
    });
}
