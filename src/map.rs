use std::vec::Vec;
use std::fs::File;
use std::io::Read;
/*
    A map is basically an array of tiles. These tiles are laid out according to a
    map definition, which is just a csv.
    The tiles are defined in the file assets/tiles.png
    The map definitions are located in assets/<name>.csv
*/
pub const TILES_FILENAME: &str = "assets/tiles.png";
pub const TILE_SIZE: u32 = 20;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>> //width x heigth
}

pub struct Tile {
    description: &'static str,
    pub id: i32,
    has_collision: bool
}

const TILE_FLOOR: Tile = Tile {
    description: "Floor",
    id: 0,
    has_collision: false
};

const TILE_ROCK: Tile = Tile {
    description: "Rock",
    id: 1,
    has_collision: true
};

const TILE_VOID: Tile = Tile {
    description: "Void",
    id: 2,
    has_collision: true
};

const TILE_FIRE: Tile = Tile {
    description: "Fire",
    id: 3,
    has_collision: true
};

const TILE_NULL: Tile = Tile {
    description: "WTF",
    id: -1,
    has_collision: true
};

/*
    This function create a map from the svg file:
        - loads the svg file
        - parses length and width
        - reads the array, and then creates corresponding tile
        - return the generated map
*/
pub fn load_map(def_filename : String) -> Map {
    println!("Reading filename: {}", def_filename);
    let mut def_file = File::open(def_filename)
                            .expect("File not found");
    
    let mut def_file_contents = String::new();
    def_file.read_to_string(&mut def_file_contents)
            .expect("Error while reading contents");

    let lines: Vec<&str> = def_file_contents.split("\n").collect();

    //read the first line, which defines dimensions
    let dimensions: Vec<&str> = lines[0].split(",").collect();
    let height: usize = dimensions[0].trim().parse().unwrap();
    let width: usize = dimensions[1].trim().parse().unwrap();
    println!("Parsed width: {} and height {}", width, height);
    
    let mut width_vec = Vec::with_capacity(width);
    for i in 1..(width+1) {
        let mut height_vec = Vec::with_capacity(height);
        let line: Vec<&str> = lines[i].split(",").collect();
        for id in &line {
            let tile = id_to_tile(id.trim().parse().unwrap());
            height_vec.push(tile);
        }
        width_vec.push(height_vec);
    }

    return Map {
        width: width,
        height: height,
        tiles: width_vec
    };
}

fn id_to_tile(id: i32) -> Tile {
    match id {
        0 => { return TILE_FLOOR }
        1 => { return TILE_ROCK }
        2 => { return TILE_VOID }
        3 => { return TILE_FIRE }
        _ => { return TILE_NULL }
    }
}