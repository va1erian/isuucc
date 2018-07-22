use map;
use game_state;

use piston::input::*;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::collections::HashMap;
use image::{open, GenericImage, imageops};

pub struct Renderer {
    pub game_state: game_state::GameState,
    pub gl: GlGraphics,
    tiles_loaded: bool,
    tiles_mapping: HashMap<u32, Texture>
}

impl Renderer {
    pub fn new(game_state: game_state::GameState, gl: GlGraphics) -> Renderer {
        Renderer {
            game_state: game_state,
            gl: gl,
            tiles_loaded: false,
            tiles_mapping: HashMap::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        Renderer::load_tiles(self);
        Renderer::render_map(self, args);
    }

    fn load_tiles(&mut self) {
        if !(self.tiles_loaded) {
            self.tiles_loaded = true;
            let ref mut tiles_image = open(map::TILES_FILENAME).unwrap();
            let tz = map::TILE_SIZE;
            let width = tiles_image.dimensions().0 / tz;
            let height = tiles_image.dimensions().1 / tz;

            for i in 0..width {
                for j in 0.. height {
                    let id = width * j + i;
                    let tile = imageops::crop(tiles_image, i*tz, j*tz, tz, tz).to_image();
                    let texture = Texture::from_image(&tile, &TextureSettings::new());
                    self.tiles_mapping.insert(id as u32, texture);
                }
            }
        }
    }

    fn render_map(&mut self, args: &RenderArgs) {
        use graphics::*;

        let map = &self.game_state.current_map;
        let tiles_mapping = &self.tiles_mapping;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for i in 0..map.width {
                for j in 0..map.height {
                    let tile = &map.tiles[i][j];
                    let id = tile.id - 1;
                    match tiles_mapping.get(&id) {
                        Some(ref texture) => {
                            let tz = map::TILE_SIZE as f64;
                            let x = i as f64 * tz;
                            let y = j as f64 * tz;
                            let transform = c.transform.trans(x, y);
                            image(*texture, transform, gl);
                        }
                        None => {}
                    };
                }
            }
        });
    }
}