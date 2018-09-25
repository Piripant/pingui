use super::*;

use label::Label;

pub struct Button {
    pressed: bool,
    label: Label,
}

impl Button {
    pub fn new(title: &impl ToString, offset: Offset) -> Button {
        let label = Label::new(title, offset);
        Button {
            pressed: false,
            label,
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }
}

impl GuiElement for Button {
    fn render(&mut self, gui: &mut Gui, c: &Context, g: &mut G2d) {
        let position = gui.get_position(self);
        let hitbox = gui.text_size(&self.label.title, 24);
        let start = (position.0, position.1 - hitbox.1);
        let end = (position.0 + hitbox.0, position.1);

        self.pressed = false;
        if gui.input.pressed_mouse.is_some() {
            let cursor = gui.input.cursor;
            if cursor.x > start.0 && cursor.x < end.0 && cursor.y > start.1 && cursor.y < end.1 {
                self.pressed = true;
                gui.input.consume();
            } else {
                self.pressed = false;
            }
        }

        self.label.render(gui, c, g);
    }

    fn get_offset(&self) -> &Offset {
        &self.label.offset
    }

    fn set_offset(&mut self, offset: &Offset) {
        self.label.offset = offset.clone();
    }

    fn offset(mut self, offset: &Offset) -> Self {
        self.set_offset(offset);
        self
    }

    fn get_size(&self, gui: &mut Gui) -> (f64, f64) {
        gui.text_size(&self.label.title, 24)
    }
}
