use map;
use game_state;

use image::{open, GenericImage, imageops};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::*;
use std::collections::HashMap;
use std::path::Path;

pub struct Renderer<'a> {
    pub game_state: game_state::GameState<'a>,
    pub gl: GlGraphics,
    resources_loaded: bool,
    sprites: HashMap<&'a str, Texture>,
    tiles: HashMap<u32, Texture>
}

impl<'a> Renderer<'a> {
    pub fn new(game_state: game_state::GameState<'a>, gl: GlGraphics) -> Renderer {
        Renderer {
            game_state: game_state,
            gl: gl,
            resources_loaded: false,
            sprites: HashMap::new(),
            tiles: HashMap::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        Renderer::load_resources(self);
        Renderer::render_map(self, args);
        Renderer::render_isuucc(self, args);
    }

    fn load_resources(&mut self) {
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

            self.sprites.insert("isuucc", Texture::from_path(&Path::new(self.game_state.isuucc.entity.sprite_filename), &TextureSettings::new()).unwrap());
            self.sprites.insert("full_heart", Texture::from_path(&Path::new("assets/full_heart.png"), &TextureSettings::new()).unwrap());
            self.sprites.insert("half_heart", Texture::from_path(&Path::new("assets/half_heart.png"), &TextureSettings::new()).unwrap());
        }
    }

    fn render_map(&mut self, args: &RenderArgs) {
        use graphics::*;

        let map = &self.game_state.current_map;
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

    fn render_isuucc(&mut self, args: &RenderArgs) {
        use graphics::*;
        let isuucc = &self.game_state.isuucc;
        let sprites = &self.sprites;
        self.gl.draw(args.viewport(), |c, gl| {
            //render isuucc sprite
            let isuucc_texture = sprites.get("isuucc").unwrap();
            let transform = c.transform.trans(isuucc.entity.posX as f64 - isuucc_texture.get_width() as f64 / 2.0, 
                                              isuucc.entity.posY as f64 - isuucc_texture.get_height() as f64 / 2.0);
            image(isuucc_texture, transform, gl);

            //render isuucc hp
            //determine how many hearts to draw
            let hp = isuucc.hp;
            let full = hp / 2;
            let half = hp % 2;
            let initPos = map::TILE_SIZE as f64 / 2.0;
            for i in 0..full as usize {
                let texture = sprites.get("full_heart").unwrap();
                let transform = c.transform.trans(initPos + (i as f64 * map::TILE_SIZE as f64) - (texture.get_width() as f64 / 2.0), 
                                                  initPos - (texture.get_width() as f64 / 2.0));
                image(texture, transform, gl);
            }

            for i in 0..half as usize {
                let texture = sprites.get("half_heart").unwrap();
                let pos = i + full as usize;
                let transform = c.transform.trans(initPos + (pos as f64 * map::TILE_SIZE as f64) - (texture.get_width() as f64 / 2.0), 
                                                  initPos - (texture.get_width() as f64 / 2.0));
                image(texture, transform, gl);
            }
        });
    }
}