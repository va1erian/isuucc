use ggez::{Context, GameResult, timer};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use net::NetChan;
use std::collections::{LinkedList, HashMap};
use std::string;
use tiled;

pub trait Entity {
  fn think(&self, dt : f32, Context : &mut Context);
  fn draw(&self,  Context: &mut Context);
}

pub struct GameEngine {
  state : Option<Box<dyn Entity>>
}

struct Resources {
  textures: HashMap<String, graphics::Image>
}

struct GameState {
  channel : NetChan,
  level: tiled::Map,
  entities: LinkedList<Box<dyn Entity>>
}

impl GameState {
  fn load_resources() -> Resources {
    let map = HashMap::new();
    


    Resources {
      textures: map
    }
  }
}

impl GameEngine { 
  pub fn new() -> GameEngine {
    GameEngine {
      state: None,
    }
  }

  pub fn to_game() {

  }
}

impl event::EventHandler for GameEngine {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      const DESIRED_FPS: u32 = 60;

      while timer::check_update_time(ctx, DESIRED_FPS) {
        if let Some(ref state) =  self.state {
          state.think(1.0, ctx);
        }
      }
      Ok(())
  }

  fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {
      if let Some(ref state) =  self.state {
          state.draw(ctx);
      }
    Ok(())
  }

  fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
    println!(
        "Key pressed: {:?}, modifier {:?}, repeat: {}",
        keycode, keymod, repeat
    );
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
  }


}

impl Entity for GameState {
  fn think(&self, dt : f32, Context : &mut Context){

  }


  fn draw(&self, Context: &mut Context,) {
  }
}