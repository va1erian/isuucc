use map;
use entity;

use piston::input::*;
use piston::input::ButtonState;
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;

pub struct Directions {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool
}

pub struct GameState {
    pub current_map: map::Map,
    pub isuucc: entity::Isuucc,
    pub directions: Directions
}

impl GameState {
    pub fn consume_event(&mut self, button_event: &ButtonArgs) { 
        match button_event {
            ButtonArgs { state: ButtonState::Press, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::W) => self.directions.move_up = true,
                    Button::Keyboard(Key::A) => self.directions.move_left = true,
                    Button::Keyboard(Key::S) => self.directions.move_down = true,
                    Button::Keyboard(Key::D) => self.directions.move_right = true,
                    _ => {}
                }
            }
            ButtonArgs { state: ButtonState::Release, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::W) => self.directions.move_up = false,
                    Button::Keyboard(Key::A) => self.directions.move_left = false,
                    Button::Keyboard(Key::S) => self.directions.move_down = false,
                    Button::Keyboard(Key::D) => self.directions.move_right = false,
                    _ => {}
                }
            }
        }
    }

    pub fn update(& mut self) {
        let ref mut isuucc = self.isuucc;
        let ref mut map = self.current_map;
        if self.directions.move_right {
            isuucc.entity.pos_x = isuucc.entity.pos_x + isuucc.speed as u32;
        }

        if self.directions.move_up && isuucc.entity.pos_y > 0 {
            isuucc.entity.pos_y = isuucc.entity.pos_y - isuucc.speed as u32;
        }

        if self.directions.move_left && isuucc.entity.pos_x > 0 {
            isuucc.entity.pos_x = isuucc.entity.pos_x - isuucc.speed as u32;
        }

        if self.directions.move_down {
            isuucc.entity.pos_y = isuucc.entity.pos_y + isuucc.speed as u32;
        }
    }
}