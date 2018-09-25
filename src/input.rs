use Vector;
use piston_window::*;

/// The current state of input (buttons pressed, released, held, etc...)
pub struct InputState {
    pub last_cursor: Vector,
    pub cursor: Vector,
    pub pressed_mouse: Option<MouseButton>,
    pub held_mouse: Option<MouseButton>,
    pub released_mouse: Option<MouseButton>,
    pub mouse_wheel: f64,
    pub pressed_keys: Vec<Key>,
    pub held_keys: Vec<Key>,
    pub released_keys: Vec<Key>,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            last_cursor: Vector::new(0.0, 0.0),
            cursor: Vector::new(0.0, 0.0),
            pressed_mouse: None,
            held_mouse: None,
            released_mouse: None,
            mouse_wheel: 0.0,
            pressed_keys: Vec::new(),
            held_keys: Vec::new(),
            released_keys: Vec::new(),
        }
    }

    /// Called once all the input is been processed
    /// Sets as held all the keys/mouse buttons that were pressed
    pub fn processed(&mut self) {
        self.last_cursor = self.cursor;

        // Everything that was now pressed becomes held
        if self.pressed_mouse.is_some() {
            self.held_mouse = self.pressed_mouse;
            self.pressed_mouse = None;
        }

        // Everything that was now pressed becomes held
        for _ in 0..self.pressed_keys.len() {
            let key = self.pressed_keys.remove(0);
            self.held_keys.push(key);
        }

        // Remove all the released keys
        self.released_keys.clear();
        self.released_mouse = None;

        self.mouse_wheel = 0.0;
    }

    /// Consumes all input a.k.a. deletes it
    pub fn consume(&mut self) {
        self.pressed_mouse = None;
        self.held_mouse = None;
        self.released_mouse = None;

        self.pressed_keys.clear();
        self.released_keys.clear();
        self.held_keys.clear();

        self.mouse_wheel = 0.0;
    }

    /// Updates the current Input State
    pub fn event(&mut self, e: &Event) {
        e.mouse_cursor(|x, y| {
            self.cursor.x = x;
            self.cursor.y = y;
        });

        e.mouse_scroll(|_dx, dy| {
            self.mouse_wheel += dy;
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            // Add the key only if it wasn't already added
            if !self.pressed_keys.contains(&key) && !self.held_keys.contains(&key) {
                self.pressed_keys.push(key);
            }
        };

        // Remove the release keys form the held and pressed keys
        // (In case one key was pressed and released before the events were processed)
        if let Some(Button::Keyboard(key)) = e.release_args() {
            for i in 0..self.pressed_keys.len() {
                if self.pressed_keys[i] == key {
                    self.pressed_keys.remove(i);
                    break;
                }
            }

            for i in 0..self.held_keys.len() {
                if self.held_keys[i] == key {
                    self.held_keys.remove(i);
                    break;
                }
            }

            self.released_keys.push(key);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            self.pressed_mouse = Some(button);
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            self.pressed_mouse = None;
            self.held_mouse = None;
            self.released_mouse = Some(button);
        }
    }
}
