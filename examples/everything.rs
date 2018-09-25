extern crate pingui;
extern crate piston_window;

use button::Button;
use inputbox::InputBox;
use label::Label;
use multibox::MultiBox;
use pingui::*;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gui = Gui::new(window.factory.clone());
    let mut label = Label::new(
        &25,
        Offset {
            align: (HAlign::Left, VAlign::Top),
            relative: (20.0, 50.0),
        },
    );

    let mut inputbox = InputBox::new(
        &"Im not dead",
        Offset {
            align: (HAlign::Left, VAlign::Bottom),
            relative: (40.0, 50.0),
        },
    );

    let mut multi = MultiBox::new(
        &"Belli",
        10.0,
        3,
        Offset {
            align: (HAlign::Right, VAlign::Center),
            relative: (0.0, 50.0),
        },
    ).value(0, &2)
        .value(1, &5)
        .value(2, &5);

    let mut button = Button::new(
        &"5",
        Offset {
            align: (HAlign::Right, VAlign::Top),
            relative: (0.0, 80.0),
        },
    );

    while let Some(e) = window.next() {
        gui.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            gui.event(&e);

            if button.is_pressed() {
                println!("ye");
            }

            label.render(&mut gui, &c, g);
            inputbox.render(&mut gui, &c, g);
            multi.render(&mut gui, &c, g);
            button.render(&mut gui, &c, g);
        });
    }
}
