use ggez::{Context, GameResult, timer};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use net::NetChan;
use std::collections::{LinkedList, HashMap};
use input::Input;
pub trait Entity {
  fn think(&self, dt : f32, context : &mut Context);
  fn draw(&self, context: &mut Context);
}

pub trait State {
  fn update(&self, dt : f32, context : &mut Context);
  fn render(&self, context: &mut Context);
  fn load_resources(&self) -> Option<Resources>;
}

pub struct NullState;

impl State for NullState {
  
  fn update(&self, dt : f32, context : &mut Context) {}

  fn render(&self, context: &mut Context) {}

  fn load_resources(&self) -> Option<Resources> {
    None
  }
}

pub struct GameEngine {
  state : Box<dyn State>,
  resources: Resources,
  input: Input
}

pub struct Resources {
  textures: HashMap<String, graphics::Image>
}

impl Resources {
  fn new() -> Resources {
    Resources {
      textures: HashMap::new()
    }
  }
}

struct GameState {
  channel : NetChan,
  entities: LinkedList<Box<dyn Entity>>
}

impl GameState {
  fn new(channel : NetChan, level_name : String) -> GameState {
    GameState {
      channel: channel,
      entities: LinkedList::new()
    }
  }
}

impl State for GameState {

  fn update(&self, dt : f32, context : &mut Context) {

  }

  fn render(&self, context: &mut Context) {

  }

  fn load_resources(&self) -> Option<Resources> {
    let mut res = Resources::new();

    Some(res)
  }
}

impl GameEngine { 
  pub fn new() -> GameEngine {
    GameEngine {
      state: Box::new(NullState{}),
      resources : Resources::new(),
      input : Input::new()
    }
  }

  pub fn to_game(&mut self, channel : NetChan, level: String) {
    let game_state = GameState::new(channel, level);
    self.state = Box::new(game_state);
  }
}

impl event::EventHandler for GameEngine {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      const DESIRED_FPS: u32 = 60;

      while timer::check_update_time(ctx, DESIRED_FPS) {
          self.state.update(1.0, ctx);
      }
      Ok(())
  }

  fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {
    self.state.render(ctx);
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