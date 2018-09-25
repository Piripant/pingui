use super::*;

use std::str::FromStr;

pub struct InputBox {
    pub(crate) active: bool,
    description: String,
    pub show_desc: bool,
    pub(crate) input: String,
    offset: Offset,
}

impl InputBox {
    pub fn new(description: &impl ToString, offset: Offset) -> InputBox {
        InputBox {
            active: false,
            description: description.to_string(),
            input: String::new(),
            show_desc: true,
            offset,
        }
    }

    /// Set the input box value from type T
    pub fn value(mut self, value: &impl ToString) -> InputBox {
        self.input = value.to_string();
        self
    }

    /// Get the input box value parsed to type T
    pub fn input(&self, value: &mut impl FromStr) {
        if !self.active {
            let new_value = self.input.parse();
            if new_value.is_ok() {
                *value = new_value.ok().unwrap();
            }
        }
    }
}

impl GuiElement for InputBox {
    fn render(&mut self, gui: &mut Gui, c: &Context, g: &mut G2d) {
        let position = gui.get_position(self);

        let mut full_text = if self.show_desc {
            format!("{}: {}", self.description, self.input)
        } else {
            self.input.clone()
        };

        if full_text.is_empty() {
            full_text = " ".to_string();
        }

        let hitbox = gui.text_size(&full_text, 24);
        // The hitbox upper left and lower right coordinates
        let start = (position.0, position.1 - hitbox.1);
        let end = (position.0 + hitbox.0, position.1);
        let color = if self.active {
            [0.8, 0.8, 0.8, 0.75]
        } else {
            [0.9, 0.9, 0.9, 0.5]
        };

        rectangle(
            color,
            [start.0 - 2.0, start.1 - 2.0, hitbox.0 + 5.0, hitbox.1 + 5.0],
            c.transform,
            g,
        );

        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 24)
            .draw(
                &full_text,
                &mut gui.glyphs,
                &c.draw_state,
                c.transform.trans(position.0, position.1),
                g,
            )
            .unwrap();

        if gui.input.pressed_mouse.is_some() {
            let cursor = gui.input.cursor;
            if cursor.x > start.0 && cursor.x < end.0 && cursor.y > start.1 && cursor.y < end.1 {
                self.active = true;
                gui.input.consume();
            } else {
                self.active = false;
            }
        }

        if gui.input.pressed_keys.contains(&Key::Return) {
            self.active = false;
        }

        if self.active {
            for key in &gui.input.pressed_keys {
                match key {
                    &Key::Backspace => {
                        self.input.pop();
                    }
                    &Key::Escape => {
                        self.input.clear();
                    }
                    &Key::Minus | &Key::NumPadMinus => {
                        self.input.push('-');
                    }
                    key => {
                        println!("{:?}", key);
                        let code = key.code();
                        let character = code as u8 as char;
                        self.input.push(character);
                    }
                }
            }

            gui.input.processed();
        }
    }

    fn get_offset(&self) -> &Offset {
        &self.offset
    }

    fn set_offset(&mut self, offset: &Offset) {
        self.offset = offset.clone();
    }

    fn offset(mut self, offset: &Offset) -> Self {
        self.set_offset(offset);
        self
    }

    fn get_size(&self, gui: &mut Gui) -> (f64, f64) {
        let mut full_text = if self.show_desc {
            format!("{}: {}", self.description, self.input)
        } else {
            self.input.clone()
        };

        if full_text.is_empty() {
            full_text = " ".to_string();
        }

        gui.text_size(&full_text, 24)
    }
}
