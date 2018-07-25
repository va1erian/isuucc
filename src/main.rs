extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate tiled;

mod map;
mod game_state;
mod renderer;
mod entity;
mod collision;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
/*
    A game consists mainly in a simple loop:
        loadInitialResources()
        while(true) {
            userInput();
            updateGameState();
                updatePlayerPosition();
                checkCollisions()
                updateAI();
            drawScreen();
        }
*/
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Map Test", [640, 480])
                                            .opengl(opengl)
                                            .exit_on_esc(true)
                                            .build()
                                            .unwrap();

    let map = map::load_map("assets/map1.tmx".to_string());
    let isuucc = entity::Isuucc::new(map.width as u32 * map::TILE_SIZE / 2, map.height as u32 * map::TILE_SIZE / 2); //center isuucc to the map
    let ref mut game_state = game_state::GameState {
        current_map: map,
        isuucc: isuucc,
        direction_event: game_state::DirectionEvent {
            move_up: false,
            move_down: false,
            move_right: false,
            move_left: false
        },
        fire_event: game_state::FireEvent {
            fire_direction: game_state::Direction::Down,
            firing: false
        },
        last_fired: 0 as f64,
        tears: Vec::new()
    };
    
    let mut renderer = renderer::Renderer::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(b) = e.button_args() {
            game_state.consume_event(&b);
        }

        if let Some(u) = e.update_args() {
            game_state.update(u);
        }

        if let Some(r) = e.render_args() {
            renderer.render(game_state, &r);
        }
    }
}