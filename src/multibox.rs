use super::*;
use inputbox::InputBox;
use label::Label;
use std::str::FromStr;

pub struct MultiBox {
    description: Label,
    boxes: Vec<InputBox>,
    spacing: f64,
    offset: Offset,
}

impl MultiBox {
    pub fn new(description: &impl ToString, spacing: f64, n_inputs: usize, offset: Offset) -> MultiBox {
        let mut boxes = Vec::with_capacity(n_inputs);
        for i in 0..n_inputs {
            let mut offset = offset.clone();
            offset.relative.0 += (i + 1) as f64 * spacing;
            let mut input_box = InputBox::new(&"".to_string(), offset);
            input_box.show_desc = false;
            boxes.push(input_box);
        }

        MultiBox {
            description: Label::new(description, offset.clone()),
            boxes,
            spacing,
            offset,
        }
    }

    /// Set the input box value from type T
    pub fn value(mut self, index: usize, value: &impl ToString) -> MultiBox {
        self.boxes[index].input = value.to_string();
        self
    }

    /// Get the input box value parsed to type T
    pub fn input(&self, index: usize, value: &mut impl FromStr) {
        if !self.boxes[index].active {
            let new_value = self.boxes[index].input.parse();
            if new_value.is_ok() {
                *value = new_value.ok().unwrap();
            }
        }
    }
}

impl GuiElement for MultiBox {
    fn render(&mut self, gui: &mut Gui, c: &Context, g: &mut G2d) {
        self.description.render(gui, c, g);

        let mut end = self.description.get_size(gui);
        let mut offset = self.offset.clone();
        for input_box in &mut self.boxes {
            match self.offset.align.0 {
                HAlign::Left | HAlign::Center => offset.relative.0 += end.0 + self.spacing,
                HAlign::Right => offset.relative.0 -= self.spacing,
            }
            input_box.set_offset(&offset);
            input_box.render(gui, c, g);
            end = input_box.get_size(gui);
        }
    }

    fn get_offset(&self) -> &Offset {
        &self.offset
    }

    fn set_offset(&mut self, offset: &Offset) {
        self.offset = offset.clone();
        for (i, input) in self.boxes.iter_mut().enumerate() {
            let mut offset = self.offset.clone();
            offset.relative.0 += self.spacing * i as f64;
            input.set_offset(&offset);
        }
    }

    fn offset(mut self, offset: &Offset) -> Self {
        self.set_offset(offset);
        self
    }

    fn get_size(&self, _gui: &mut Gui) -> (f64, f64) {
        (0.0, 0.0)
    }
}
