use map;
use game_state;

use graphics::*;
use image::{open, GenericImage, imageops};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::*;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;

pub struct Renderer {
    pub gl: GlGraphics,
    resources_loaded: bool,
    sprites: HashMap<String, Texture>,
    tiles: HashMap<u32, Texture>
}

impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer {
            gl: gl,
            resources_loaded: false,
            sprites: HashMap::new(),
            tiles: HashMap::new()
        }
    }

    pub fn render(&mut self, game_state: &mut game_state::GameState, args: &RenderArgs) {
        self.load_resources(game_state);
        self.render_map(game_state, args);
        self.render_isuucc(game_state, args);
    }

    fn load_resources(&mut self, game_state: &mut game_state::GameState) {
        if !(self.resources_loaded) {
            self.resources_loaded = true;
            let ref mut tiles_image = open(map::TILES_FILENAME).unwrap();
            let tz = map::TILE_SIZE;
            let width = tiles_image.dimensions().0 / tz;
            let height = tiles_image.dimensions().1 / tz;

            for i in 0..width {
                for j in 0.. height {
                    let id = width * j + i;
                    let tile = imageops::crop(tiles_image, i*tz, j*tz, tz, tz).to_image();
                    let texture = Texture::from_image(&tile, &TextureSettings::new());
                    self.tiles.insert(id as u32, texture);
                }
            }

            let isuucc_texture = Texture::from_path(&Path::new(&game_state.isuucc.entity.sprite_filename), &TextureSettings::new()).unwrap();
            game_state.isuucc.entity.init_bounding_box(isuucc_texture.get_width(), isuucc_texture.get_height());
            self.sprites.insert("isuucc".to_string(), isuucc_texture);
            self.sprites.insert("full_heart".to_string(), Texture::from_path(&Path::new("assets/full_heart.png"), &TextureSettings::new()).unwrap());
            self.sprites.insert("half_heart".to_string(), Texture::from_path(&Path::new("assets/half_heart.png"), &TextureSettings::new()).unwrap());
        }
    }

    fn render_map(&mut self, game_state: &game_state::GameState, args: &RenderArgs) {
        let map = &game_state.current_map;
        let tiles = &self.tiles;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for i in 0..map.width {
                for j in 0..map.height {
                    let tile = &map.tiles[i][j];
                    let id = tile.id - 1;
                    match tiles.get(&id) {
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

    fn render_isuucc(&mut self, game_state: &game_state::GameState, args: &RenderArgs) {
        let isuucc = &game_state.isuucc;
        let sprites = &self.sprites;
        self.gl.draw(args.viewport(), |c, gl| {
            //render isuucc sprite
            let isuucc_texture = sprites.get("isuucc").unwrap();
            let transform = c.transform.trans(isuucc.entity.pos_x as f64 - isuucc_texture.get_width() as f64 / 2.0, 
                                              isuucc.entity.pos_y as f64 - isuucc_texture.get_height() as f64 / 2.0);
            image(isuucc_texture, transform, gl);

            //render isuucc hp
            //determine how many hearts to draw
            let hp = isuucc.hp;
            let full = hp / 2;
            let half = hp % 2;
            let init_pos = map::TILE_SIZE as f64 / 2.0;
            for i in 0..full as usize {
                let texture = sprites.get("full_heart").unwrap();
                let transform = c.transform.trans(init_pos + (i as f64 * map::TILE_SIZE as f64) - (texture.get_width() as f64 / 2.0), 
                                                  init_pos - (texture.get_width() as f64 / 2.0));
                image(texture, transform, gl);
            }

            for i in 0..half as usize {
                let texture = sprites.get("half_heart").unwrap();
                let pos = i + full as usize;
                let transform = c.transform.trans(init_pos + (pos as f64 * map::TILE_SIZE as f64) - (texture.get_width() as f64 / 2.0), 
                                                  init_pos - (texture.get_width() as f64 / 2.0));
                image(texture, transform, gl);
            }
        });
    }
}