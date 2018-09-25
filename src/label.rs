use super::*;

pub struct Label {
    pub title: String,
    pub background: bool,
    pub offset: Offset,
}

impl Label {
    pub fn new(title: &impl ToString, offset: Offset) -> Label {
        Label {
            title: title.to_string(),
            background: true,
            offset,
        }
    }
}

impl GuiElement for Label {
    fn render(&mut self, gui: &mut Gui, c: &Context, g: &mut G2d) {
        let position = gui.get_position(self);

        if self.background {
            let hitbox = gui.text_size(&self.title, 24);
            // The hitbox upper left and lower right coordinates
            let start = (position.0, position.1 - hitbox.1);
            rectangle(
                [0.9, 0.9, 0.9, 0.5],
                [start.0 - 2.0, start.1 - 2.0, hitbox.0 + 5.0, hitbox.1 + 5.0],
                c.transform,
                g,
            );
        }

        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 24)
            .draw(
                &self.title,
                &mut gui.glyphs,
                &c.draw_state,
                c.transform.trans(position.0, position.1),
                g,
            )
            .unwrap();
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
        gui.text_size(&self.title, 24)
    }
}
