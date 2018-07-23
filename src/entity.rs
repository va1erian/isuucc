use::std::string::String;

pub struct Entity {
    pub pos_x: u32,
    pub pos_y: u32,
    pub facing_direction: f64,
    pub sprite_filename: String
}

pub struct Isuucc {
    pub entity: Entity,
    pub hp: u32,
    pub dmg: f32,
    pub speed: f32,
    pub tear_delay: f32
}

impl Isuucc {
    pub fn new(x: u32, y: u32) -> Isuucc {
        println!("Creating isuucc at position ({},{})", x, y);
        return
        Isuucc {
            entity: Entity {
                pos_x: x,
                pos_y: y,
                facing_direction: 1.0,
                sprite_filename: "assets/isuucc.png".to_string()
            },
            hp: 6,
            dmg: 1.0,
            speed: 1.0,
            tear_delay: 1.0
        }
    }
}