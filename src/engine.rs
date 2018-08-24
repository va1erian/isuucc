use ggez::{Context, GameResult, timer};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use net::NetChan;
use std::collections::{LinkedList, HashMap};
use input::Input;
use tiled;
use std::path::Path;
use game_state::GameState;

pub trait Entity {
  fn think(&self, dt : f32, context : &mut Context);
  fn draw(&self, context: &mut Context);
}


pub type UpdateResult = GameResult<Option<Box<dyn State>>>; //state can optionoally transition to other state
pub type RenderResult = GameResult<()>;

pub trait State {
  fn update(&self, dt : f32, context : &mut Context, engine: &GameEngine) -> UpdateResult;
  fn render(&self, context: &mut Context, engine: &GameEngine) -> RenderResult;
  fn load_resources(&self, Context: &mut Context) -> GameResult<Option<Resources>>;
}

pub struct NullState;

impl State for NullState {
  
  fn update(&self, _dt : f32, _context : &mut Context, _engine: &GameEngine) -> UpdateResult {
    Ok(None)
  }

  fn render(&self, _context: &mut Context, _engine: &GameEngine) -> RenderResult {
    Ok(())
  }

  fn load_resources(&self, _context: &mut Context) -> GameResult<Option<Resources>> {
    Ok(None)
  }
}

pub struct Resources {
  pub textures: HashMap<String, graphics::Image>,
  pub map: tiled::Map
}

pub struct GameEngine {
  state : Box<dyn State>,
  pub resources: Option<Resources>,
  pub input: Input
}

impl GameEngine { 
  pub fn new() -> GameEngine {
    GameEngine {
      state: Box::new(NullState{}),
      resources : None,
      input : Input::new()
    }
  }

  pub fn to_game(&mut self, channel : NetChan, level: String) {
    let game_state = GameState::new(channel, level);
    self.state = Box::new(game_state);
    println!("waht");
    self.resources = None;
  }   
}

impl event::EventHandler for GameEngine {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      const DESIRED_FPS: u32 = 60;

      if let None = self.resources {
        self.resources = self.state.load_resources(ctx).unwrap();
      }

      while timer::check_update_time(ctx, DESIRED_FPS) {
          self.state.update(1.0, ctx, self)?;
      }
      Ok(())
  }

  fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    self.state.render(ctx, self)?;
    graphics::present(ctx);
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