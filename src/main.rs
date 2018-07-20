extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::{Image, clear};
use std::path::Path;
use std::{thread, time};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {

    fn render(&mut self, args: &RenderArgs, array: &[Texture; 10], i: usize) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
            image(&array[i], transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 1.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("nique ta mere, Hadrien", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut array: [Texture; 10] = [
        Texture::from_path(&Path::new("resources/gif0.gif"), &TextureSettings::new()).unwrap(),        
        Texture::from_path(&Path::new("resources/gif1.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif2.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif3.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif4.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif5.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif6.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif7.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif8.gif"), &TextureSettings::new()).unwrap(),
        Texture::from_path(&Path::new("resources/gif9.gif"), &TextureSettings::new()).unwrap()
    ];
    let mut i = 0;
    let mut start = time::Instant::now();
    let cent_millis = time::Duration::from_millis(100);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &array, i);
            if(start.elapsed() > cent_millis) {
                i = (&i + 1) % 10;
                start = time::Instant::now();
            }
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
