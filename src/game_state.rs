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

pub struct ArrowsDirectionEvent {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool
}

pub struct GameState {
    pub current_map: map::Map,
    pub isuucc: entity::Isuucc,
    pub tear: entity::Tear,
    pub direction_event: DirectionEvent,
    pub arrows_direction_event: ArrowsDirectionEvent
}

impl GameState {
    pub fn consume_event(&mut self, button_event: &ButtonArgs) { 
        match button_event {
            ButtonArgs { state: ButtonState::Press, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::Z) => self.direction_event.move_up = true,
                    Button::Keyboard(Key::Q) => self.direction_event.move_left = true,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = true,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = true,
                    Button::Keyboard(Key::Up) => {if (self.tear.entity.facing_direction == 0.0 || self.tear.entity.facing_direction == 1.0) {
                                                  self.arrows_direction_event.move_up = true;
                                                  self.arrows_direction_event.move_left = false;
                                                  self.arrows_direction_event.move_down = false;
                                                  self.arrows_direction_event.move_right = false;
                                                  self.tear.entity.facing_direction = 1.0;}},
                    Button::Keyboard(Key::Left) => {if (self.tear.entity.facing_direction == 0.0 || self.tear.entity.facing_direction == 2.0) {
                                                  self.arrows_direction_event.move_up = false;
                                                  self.arrows_direction_event.move_left = true;
                                                  self.arrows_direction_event.move_down = false;
                                                  self.arrows_direction_event.move_right = false;
                                                  self.tear.entity.facing_direction = 2.0;}},
                    Button::Keyboard(Key::Down) => {if (self.tear.entity.facing_direction == 0.0 || self.tear.entity.facing_direction == 3.0) {
                                                  self.arrows_direction_event.move_up = false;
                                                  self.arrows_direction_event.move_left = false;
                                                  self.arrows_direction_event.move_down = true;
                                                  self.arrows_direction_event.move_right = false;
                                                  self.tear.entity.facing_direction = 3.0;}},
                    Button::Keyboard(Key::Right) => {if (self.tear.entity.facing_direction == 0.0 || self.tear.entity.facing_direction == 4.0) {
                                                   self.arrows_direction_event.move_up = false;
                                                  self.arrows_direction_event.move_left = false;
                                                  self.arrows_direction_event.move_down = false;
                                                  self.arrows_direction_event.move_right = true;
                                                  self.tear.entity.facing_direction = 4.0;}},
                    _ => {}
                }
            }
            ButtonArgs { state: ButtonState::Release, button: but, scancode: _ } => {
                match but {
                    Button::Keyboard(Key::Z) => self.direction_event.move_up = false,
                    Button::Keyboard(Key::Q) => self.direction_event.move_left = false,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = false,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = false,
                    _ => {}
                }
            }
        }
    }

    pub fn update(&mut self) {
        let ref mut isuucc = self.isuucc;
        let map = &self.current_map;
        if self.direction_event.move_right { isuucc.move_direction(map, Direction::Right); }
        if self.direction_event.move_up { isuucc.move_direction(map, Direction::Up); }
        if self.direction_event.move_left { isuucc.move_direction(map, Direction::Left); }
        if self.direction_event.move_down { isuucc.move_direction(map, Direction::Down); }

        let ref mut tear = self.tear;
        if(self.arrows_direction_event.move_right) { 
            if(!tear.entity.visible && tear.entity.facing_direction == 4.0) {
                tear.entity.pos_x = isuucc.entity.pos_x;
                tear.entity.pos_y = isuucc.entity.pos_y;
                tear.entity.visible = true;
            }
            tear.move_direction(map, Direction::Right);
        }
        if(self.arrows_direction_event.move_up) {
            if(!tear.entity.visible && tear.entity.facing_direction == 1.0) {
                tear.entity.pos_x = isuucc.entity.pos_x;
                tear.entity.pos_y = isuucc.entity.pos_y;
                tear.entity.visible = true;
            }
            tear.move_direction(map, Direction::Up);
        }
        if(self.arrows_direction_event.move_left ) {
            if(!tear.entity.visible && tear.entity.facing_direction == 2.0) {
                tear.entity.pos_x = isuucc.entity.pos_x;
                tear.entity.pos_y = isuucc.entity.pos_y;
                tear.entity.visible = true;
            }
            tear.move_direction(map, Direction::Left);
        }
        if(self.arrows_direction_event.move_down) {
            if(!tear.entity.visible && tear.entity.facing_direction == 3.0) {
                tear.entity.pos_x = isuucc.entity.pos_x;
                tear.entity.pos_y = isuucc.entity.pos_y;
                tear.entity.visible = true;
            }
            tear.move_direction(map, Direction::Down);
        }

        if(tear.entity.has_collision) {
            tear.entity.visible = false;
            tear.entity.facing_direction = 0.0;
        }

    }
}