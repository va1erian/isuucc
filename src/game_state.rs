use map;
use entity;

use entity::Mobile;
use piston::input::*;
use piston::input::ButtonState;
use piston::input::keyboard::Key;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct DirectionEvent {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool
}

pub struct GameState {
    pub current_map: map::Map,
    pub isuucc: entity::Isuucc,
    pub direction_event: DirectionEvent
}

impl GameState {
    pub fn consume_event(&mut self, button_event: &ButtonArgs) { 
        match button_event {
            ButtonArgs { state: ButtonState::Press, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::W) => self.direction_event.move_up = true,
                    Button::Keyboard(Key::A) => self.direction_event.move_left = true,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = true,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = true,
                    _ => {}
                }
            }
            ButtonArgs { state: ButtonState::Release, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::W) => self.direction_event.move_up = false,
                    Button::Keyboard(Key::A) => self.direction_event.move_left = false,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = false,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = false,
                    _ => {}
                }
            }
        }
    }

    pub fn update(& mut self) {
        let ref mut isuucc = self.isuucc;
        let map = &self.current_map;
        if self.direction_event.move_right { isuucc.move_direction(map, Direction::Right); }
        if self.direction_event.move_up { isuucc.move_direction(map, Direction::Up); }
        if self.direction_event.move_left { isuucc.move_direction(map, Direction::Left); }
        if self.direction_event.move_down { isuucc.move_direction(map, Direction::Down); }
    }
}