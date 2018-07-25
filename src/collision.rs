use map;
use game_state;

pub fn collision_map(map: &map::Map, x: i32, y: i32, collision_box: (u32, u32), dir: game_state::Direction) -> (u32, u32) {
    let tz = map::TILE_SIZE;
    let mut new_x: u32;
    let mut new_y: u32;
    let w = collision_box.0 / 2;
    let h = collision_box.1 / 2;

    //check map boundaries
    let map_w = (map.width as u32) * tz;
    let map_h = (map.height as u32) * tz;
    if x - (w as i32) < 0 { new_x = w; }
    else { new_x = x as u32; }
    if y - (h as i32) < 0 { new_y = h; }
    else { new_y = y as u32; }
    if x + (w as i32) > (map_w as i32) { new_x = map_w - w; }
    if y + (h as i32) > (map_h as i32) { new_y = map_h - h; }

    let box_x1 = new_x - w;
    let box_y1 = new_y - h;
    let box_x2 = new_x + w;
    let box_y2 = new_y + h;

    //check tile collision
    //we need to check all possible tiles
    //this means, we start at the first possible tile
    //if going up it is the tile that contains the (x1, y1) point
    //and then jump tz to check the next tile, until we are out of our colliding box
    let collision_box: (u32, u32, u32, u32) = (box_x1, box_y1, box_x2, box_y2);
    match dir {
        game_state::Direction::Up => {
            let tile_x = (box_x1 / tz) * tz;
            let tile_y = (box_y1 / tz) * tz;
            let mut i = tile_x;
            while i < box_x2 {
                let tile_box: (u32, u32, u32, u32) = (i, tile_y, 
                                                      i + tz, tile_y + tz);
                let tile = &map.get_tile_at_coord(i, tile_y);
                if tile.has_collision && is_collision(collision_box, tile_box) {  
                    new_y = tile_y + tz + h;
                }
                i += tz;
            }
        }
        game_state::Direction::Right => {
            let tile_x = (box_x2 / tz) * tz;
            let tile_y = (box_y1 / tz) * tz;
            let mut i = tile_y;
            while i < box_y2 {
                let tile_box: (u32, u32, u32, u32) = (tile_x, i, 
                                                      tile_x + tz, i + tz);
                let tile = &map.get_tile_at_coord(tile_x, i);
                if tile.has_collision && is_collision(collision_box, tile_box) {
                    new_x = tile_x - w;
                }
                i += tz;
            }
        }
        game_state::Direction::Down => {
            let tile_x = (box_x1 / tz) * tz;
            let tile_y = (box_y2 / tz) * tz;
            let mut i = tile_x;
            while i < box_x2 {
                let tile_box: (u32, u32, u32, u32) = (i, tile_y, 
                                                      i + tz, tile_y + tz);
                let tile = &map.get_tile_at_coord(i, tile_y);
                if tile.has_collision && is_collision(collision_box, tile_box) {
                    new_y = tile_y - h;
                }
                i += tz;
            }
        }
        game_state::Direction::Left => {
            let tile_x = (box_x1 / tz) * tz;
            let tile_y = (box_y1 / tz) * tz;
            let mut i = tile_y;
            while i < box_y2 {
                let tile_box: (u32, u32, u32, u32) = (tile_x, i, 
                                                      tile_x + tz, i + tz);
                let tile = &map.get_tile_at_coord(tile_x, i);
                if tile.has_collision && is_collision(collision_box, tile_box) {
                    new_x = tile_x + tz + w;
                }
                i += tz;
            }
        }
    }
    (new_x, new_y)
}

fn is_collision(collisioner: (u32, u32, u32, u32), collisionee: (u32, u32, u32, u32)) -> bool {
    collisioner.0 <= collisionee.2 && collisioner.2 >= collisionee.0 &&
    collisioner.1 <= collisionee.3 && collisioner.3 >= collisionee.1 
}