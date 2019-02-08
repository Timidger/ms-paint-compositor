use wlroots::{input::keyboard::Key,
              xkbcommon::xkb::keysyms};

#[derive(Debug, Default)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.red as f32 / 255.0,
         self.green as f32 / 255.0,
         self.blue as f32 / 255.0,
         // alpha
         1.0]
    }
}

#[derive(Debug, Default)]
pub struct ColorState {
    pub editing_color: Option<String>,
    pub color: Color
}

impl ColorState {
    pub fn update_key(&mut self, key: Key) {
        let color = match self.editing_color.as_mut() {
            None => return,
            Some(color) => color
        };
        let number = match key {
            keysyms::KEY_0 ..= keysyms::KEY_9 =>
                std::char::from_digit(key - keysyms::KEY_0, 10).unwrap(),
            keysyms::KEY_a => 'a',
            keysyms::KEY_b => 'b',
            keysyms::KEY_c => 'c',
            keysyms::KEY_d => 'd',
            keysyms::KEY_e => 'e',
            keysyms::KEY_f => 'f',
            // TODO Delete
            _ => return
        };
        color.push(number)
    }
}
