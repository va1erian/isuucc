use map;
use entity;

use entity::Mobile;
use entity::Attack;
use piston::input::*;
use piston::input::ButtonState;
use piston::input::keyboard::Key;

#[derive(Copy, Clone,Debug)]
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

#[derive(Copy, Clone)]
pub struct FireEvent {
    pub fire_direction: Direction,
    pub firing: bool
    
}

pub struct GameState {
    pub current_map: map::Map,
    pub isuucc: entity::Isuucc,
    pub direction_event: DirectionEvent,
    pub fire_event: FireEvent,
    pub last_fired: f64,
    pub tears: Vec<entity::Tear>
}

impl GameState {
    pub fn consume_event(&mut self, button_event: &ButtonArgs) { 
        match button_event {
            ButtonArgs { state: ButtonState::Press, button: but, scancode: _ } => {
                match but {
                    // Movement keys
                    Button::Keyboard(Key::W) => self.direction_event.move_up = true,
                    Button::Keyboard(Key::A) => self.direction_event.move_left = true,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = true,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = true,
                    // Attack keys
                    Button::Keyboard(Key::Up) => self.fire_event = FireEvent {fire_direction: Direction::Up, firing: true},
                    Button::Keyboard(Key::Left) => self.fire_event = FireEvent {fire_direction: Direction::Left, firing: true},
                    Button::Keyboard(Key::Down) => self.fire_event = FireEvent {fire_direction: Direction::Down, firing: true},
                    Button::Keyboard(Key::Right) => self.fire_event = FireEvent {fire_direction: Direction::Right, firing: true},
                    _ => {}
                }
            }
            ButtonArgs { state: ButtonState::Release, button: but, scancode: _ } => {
                match but {
                    // Movement keys
                    Button::Keyboard(Key::W) => self.direction_event.move_up = false,
                    Button::Keyboard(Key::A) => self.direction_event.move_left = false,
                    Button::Keyboard(Key::S) => self.direction_event.move_down = false,
                    Button::Keyboard(Key::D) => self.direction_event.move_right = false,
                    // Attack keys
                    Button::Keyboard(Key::Up) => self.fire_event = FireEvent {fire_direction: Direction::Up, firing: false},
                    Button::Keyboard(Key::Left) => self.fire_event = FireEvent {fire_direction: Direction::Left, firing: false},
                    Button::Keyboard(Key::Down) => self.fire_event = FireEvent {fire_direction: Direction::Down, firing: false},
                    Button::Keyboard(Key::Right) => self.fire_event = FireEvent {fire_direction: Direction::Right, firing: false},
                    _ => {}
                }
            }
        }
    }

    pub fn update(& mut self, ua: UpdateArgs ) {
        self.last_fired = self.last_fired + ua.dt;
        let ref mut isuucc = self.isuucc;
        let map = &self.current_map;
        if self.direction_event.move_right { isuucc.move_direction(map, Direction::Right); }
        if self.direction_event.move_up { isuucc.move_direction(map, Direction::Up); }
        if self.direction_event.move_left { isuucc.move_direction(map, Direction::Left); }
        if self.direction_event.move_down { isuucc.move_direction(map, Direction::Down); }
        if self.fire_event.firing && self.last_fired > isuucc.tear_delay as f64 {
            let spawned_tear = entity::Tear::new(isuucc.entity.pos_x,isuucc.entity.pos_y);
           // isuucc.fire_towards(self.fire_event.fire_direction, &spawned_tear);
            self.tears.push(spawned_tear);
            self.last_fired = 0 as f64;
        }
    }
}