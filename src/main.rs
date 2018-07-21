extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate tiled;
extern crate rand;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::Image;
use graphics::math::{Matrix2d };
use std::path::Path;
use std::string::String;
use tiled::Layer;
use graphics::ImageSize;
use rand::prelude::*;

type Position = [f64; 2];

enum GameState {
    Title,
    Intermission,
    Game,
    Score
}

struct Game {
    state : GameState,
    gl: GlGraphics, // OpenGL drawing backend.
    level : Level
}

struct Level {
    name: String,
    tiles: Option<Layer>,
    entities: Vec<Sprite>
}

impl Level {
    pub fn new(s: String) -> Level {
        Level {
            name : s,
            tiles: None,
            entities : vec![]
        }
    }

    pub fn put_sprite(&mut self, s : Sprite) -> &Level {
        self.entities.push(s);
        self
    }
}

struct Sprite {
    texture: Texture,
    pos: Position,
    rotation: f64, //in radian
    bbox: [f64; 4],
    direction: f64 // in radian
}

impl Sprite {
    pub fn new(t : Texture) -> Sprite {
        Sprite {
            texture : t,
            pos: [0.0, 0.0],
            rotation : 0.0,
            bbox: [0.0, 0.0, 32.0, 32.0],
            direction : 0.0
        }
    }

    pub fn set_pos(&mut self, x : f64, y : f64) {
        self.pos = [x, y];
    }

    pub fn x(&self) -> f64 {
        self.pos[0]
    }

    pub fn y(&self) -> f64 {
        self.pos[1]
    }

    pub fn set_dir(&mut self, dir : f64)  {
        self.direction = dir;
    } 
}

trait Entity {
    fn draw(&self, args: &RenderArgs, t: Matrix2d, backend: &mut GlGraphics);
    fn update(&mut self, args: &UpdateArgs);
}


impl Entity for Sprite {
    fn draw(&self, _args: &RenderArgs, t: Matrix2d, backend: &mut GlGraphics) {
        use graphics::Transformed;
        let ref draw_state: graphics::DrawState = Default::default();
        let (w, h) = self.texture.get_size();
        let transformed = t.trans(self.x(), self.y())
            .rot_rad(self.rotation)
            .trans( - (w as f64 / 2.0), - (h as f64 / 2.0) );

        Image::new().draw(&self.texture, draw_state, transformed, backend);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let new_x = self.x() + self.direction.cos();
        let new_y = self.y() + self.direction.sin();
        let (w, h) = self.texture.get_size();

        self.set_pos(new_x, new_y);
        if new_x < 0.0 || new_x > 640.0  - w as f64 / 2.0 || new_y < 0.0  || new_y  > 480.0 - h as f64 / 2.0 {
            let new_angle = self.direction + std::f64::consts::PI / 2.0;
            self.set_dir(new_angle);
        }
        self.rotation += 1.0 * args.dt;
    }
}

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

impl Game {
    fn render(&mut self, args  : &RenderArgs) {
        let ref entities = self.level.entities;
        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::clear;

            clear(GREEN, gl);
            for entity in entities {
                entity.draw(args, c.transform, gl );
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let ref mut entities = self.level.entities;

        for mut entity in entities {
            entity.update(args);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("nique ta mere, Alexis", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut level = Level::new(String::from("TBD"));

    for i in 0..100 {
        let path_str = format!("resources/img{}.png", i % 3);
        let path = Path::new(&path_str);
        let tex =  Texture::from_path(&path, &TextureSettings::new()).unwrap();
        let (w, h) = tex.get_size();

        let mut sprite = Sprite::new(tex);
        sprite.set_pos(random::<f64>() * (640.0 - w as f64),random::<f64>() * (480.0 - h as f64)) ;
        sprite.set_dir(random::<f64>() * (2.0 * std::f64::consts::PI));
        level.put_sprite(sprite);
    }

    // Create a new game and run it.
    let mut app = Game {
        state: GameState::Game,
        gl: GlGraphics::new(opengl),
        level : level
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
