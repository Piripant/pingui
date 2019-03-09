extern crate nalgebra;
extern crate piston_window;

pub mod input;

pub mod button;
pub mod inputbox;
pub mod label;
pub mod multibox;

pub use button::Button;
pub use inputbox::InputBox;
pub use label::Label;
pub use multibox::MultiBox;

use input::InputState;
use piston_window::*;

type Vector = nalgebra::Vector2<f64>;

const FONT: &[u8] = include_bytes!("../assets/FiraSans-Regular.ttf");

pub trait GuiElement {
    fn get_offset(&self) -> &Offset;
    fn set_offset(&mut self, offset: &Offset);
    fn offset(self, offset: &Offset) -> Self;

    fn get_size(&self, gui: &mut Gui) -> (f64, f64);

    fn render(&mut self, gui: &mut Gui, c: &Context, g: &mut G2d);
}

#[derive(Clone)]
pub enum HAlign {
    Right,
    Left,
    Center,
}

#[derive(Clone)]
pub enum VAlign {
    Top,
    Bottom,
    Center,
}

#[derive(Clone)]
pub struct Offset {
    pub relative: (f64, f64),
    pub align: (HAlign, VAlign),
}

pub struct Gui {
    input: InputState,
    pub window_size: (u32, u32),
    glyphs: Glyphs,
}

impl Gui {
    pub fn new(factory: GfxFactory) -> Gui {
        let glyphs = Glyphs::from_bytes(FONT, factory, TextureSettings::new()).unwrap();

        Gui {
            input: InputState::new(),
            window_size: (0, 0),
            glyphs,
        }
    }

    pub fn get_position(&mut self, element: &impl GuiElement) -> (f64, f64) {
        let offset = element.get_offset();
        let size = element.get_size(self);

        let (halign, valign) = &offset.align;
        let relative = offset.relative;

        let x = match halign {
            HAlign::Left => relative.0,
            HAlign::Right => f64::from(self.window_size.0) - relative.0 - size.0,
            HAlign::Center => f64::from(self.window_size.0 / 2) - relative.0,
        };
        let y = match valign {
            VAlign::Top => relative.1,
            VAlign::Bottom => f64::from(self.window_size.1) - relative.1 - size.1,
            VAlign::Center => f64::from(self.window_size.1 / 2) - relative.1,
        };

        (x, y)
    }

    pub fn event(&mut self, e: &Event) {
        self.input.event(e);

        if let Some(dimensions) = e.resize_args() {
            self.window_size.0 = dimensions[0] as u32;
            self.window_size.1 = dimensions[1] as u32;
            println!("{:?}", self.window_size);
        }
    }

    /// Get the text width and height
    fn text_size(&mut self, text: &str, size: u32) -> (f64, f64) {
        use piston_window::character::CharacterCache;

        let width = self.glyphs.width(size, text).unwrap();
        let mut height = 0.0;
        for ch in text.chars() {
            let character = self.glyphs.character(size, ch).unwrap();
            if character.top() > height {
                height = character.top();
            }
        }

        (width, height as f64)
    }
}
