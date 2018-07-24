use std::vec::Vec;
use std::path::Path;
use tiled::parse_file;
/*
    A map is basically an array of tiles. These tiles are laid out according to a
    map definition, which is in reality just a csv. We use tiled map editor.
    The tiles are defined in the file assets/tiles.png
    The map definitions are located in assets/<name>.tmx
*/
pub const TILES_FILENAME: &str = "assets/tiles.png";
pub const TILE_SIZE: u32 = 20;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>> //width x heigth
}

impl Map {
    pub fn get_tile_at_coord(&self, pos_x: u32, pos_y: u32) -> &Tile {
        let tile_x = pos_x / TILE_SIZE;
        let tile_y = pos_y / TILE_SIZE;
        return self.tiles.get(tile_x as usize).unwrap()
                         .get(tile_y as usize).unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    description: &'static str,
    pub id: u32,
    pub has_collision: bool
}

const TILE_FLOOR: Tile = Tile {
    description: "Floor",
    id: 1,
    has_collision: false
};

const TILE_ROCK: Tile = Tile {
    description: "Rock",
    id: 2,
    has_collision: true
};

const TILE_VOID: Tile = Tile {
    description: "Void",
    id: 3,
    has_collision: true
};

const TILE_FIRE: Tile = Tile {
    description: "Fire",
    id: 4,
    has_collision: true
};

pub fn load_map(def_filename : String) -> Map {
    println!("Reading filename: {}", def_filename);
    let def_file = Path::new(&def_filename);
    
    let map = parse_file(def_file).unwrap();
    let ref layer = map.layers[0];
    let ref definition: Vec<Vec<u32>> = layer.tiles; //layer.tiles is the csv defining the map

    let width = map.width as usize;
    let height = map.height as usize;
    //we have to transform this into our tiles
    let mut width_vec = Vec::with_capacity(width);
    for i in 0..width {
        let mut height_vec = Vec::with_capacity(height);
        for j in 0..height {
            let tile = id_to_tile(definition[j][i]);
            height_vec.push(tile);
        }
        width_vec.push(height_vec);
    }
    println!("Finished parsing map");
    println!("width is {} and height is {}", width, height);

    return Map {
        width: width,
        height: height,
        tiles: width_vec
    }
}

fn id_to_tile(id: u32) -> Tile {
    match id {
        1 => { return TILE_FLOOR }
        2 => { return TILE_ROCK }
        3 => { return TILE_VOID }
        4 => { return TILE_FIRE }
        _ => { return TILE_FLOOR }
    }
}