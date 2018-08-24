use ggez::{Context, GameResult};
use ggez::graphics;
use net::NetChan;
use std::collections::{LinkedList, HashMap};
use tiled;
use std::path::Path;
use engine::*;
use gfx;

pub struct GameState {
  channel : NetChan,
  level_name: String,
  entities: LinkedList<Box<dyn Entity>>
}

impl GameState {
  pub fn new(chan: NetChan, level_name : String) -> GameState {
    GameState {
      channel: chan,
      level_name: level_name,
      entities: LinkedList::new()
    }
  }
}

impl State for GameState {
  fn update(&self, dt : f32, context : &mut Context, engine: &GameEngine) -> UpdateResult {
    if let None = &engine.resources {
      return Ok(None);
    } else if let Some(res) = &engine.resources {
    }
    Ok(None)
  }

  fn render(&self, context: &mut Context, engine: &GameEngine) -> RenderResult {
    let res = engine.resources.as_ref().unwrap();
    gfx::draw_tilemap(context, res, 0)?;
    gfx::draw_tilemap(context, res, 1)?;
    Ok(())
  }


  fn load_resources(&self, context: &mut Context) -> GameResult<Option<Resources>> {
    let map_name = format!("/{}.tmx", self.level_name);
    let path = Path::new(&map_name);
    println!("loading map {:?}", path);

    let file = context.filesystem.open(path)?;
    let map = tiled::parse(file).unwrap();

    let image_name = format!("/{}.png", self.level_name);
    let image_path = Path::new(&image_name);
    let image  = graphics::Image::new(context, image_path)?;

    let mut texture_map = HashMap::new();
    texture_map.insert("tileset".to_owned(), image);

    Ok(Some(Resources {
      textures: texture_map,
      map : map
    }))
  }
}
