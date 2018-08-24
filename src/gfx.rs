use ggez::{GameResult, Context};
use ggez::graphics::{Rect, Point2, DrawParam, draw};
use ggez::graphics::spritebatch::SpriteBatch;
use engine::Resources;

struct Viewport(Rect);

pub fn draw_tilemap(context: &mut Context, res : &Resources, layer: usize)
   -> GameResult<()> {
  let layer = &res.map.layers[layer];
  let image = res.textures.get("tileset").unwrap().clone(); // TODO better load handling
  let img_info = &res.map.tilesets[0].images[0]; //TODO something better too..

  let tile_h = img_info.height as f32 / 16 as f32;
  let tile_w = img_info.width as f32/ 16 as f32;
  let tile_h_ratio =  1.0 / tile_h;
  let tile_w_ratio = 1.0 / tile_w;

  let mut spritebatch = SpriteBatch::new(image);

  for (col_idx, row) in layer.tiles.iter().enumerate() {
    for (row_idx, tile) in row.iter().enumerate() {
      if *tile == 0u32 {
        continue;
      }

      let tile = tile - 1;
      let draw_params = DrawParam {
        src: Rect {
          x: tile_w_ratio * (tile as f32 % tile_w  as f32),
          y: tile_h_ratio * (tile as f32 / tile_w as f32).floor(),
          w: tile_w_ratio as f32,
          h: tile_h_ratio as f32
        },
        dest: Point2::new(row_idx as f32 * 16.0, col_idx as f32 * 16.0),
        ..
        Default::default()
      };
      spritebatch.add(draw_params);
      println!("{:?}", draw_params);
    }
  }

  draw(context, &spritebatch, Point2::new(0f32,0f32), 0f32)?;
  Ok(())
}


pub fn draw_sprite(context: &mut Context) {

}