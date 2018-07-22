extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate tiled;

mod map;
mod game_state;
mod renderer;

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

    let game_state = game_state::GameState {
        current_map: map::load_map(String::from("assets/map1.tmx"))
    };

    let mut renderer = renderer::Renderer::new(game_state, GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            renderer.render(&r);
        }
    }
}