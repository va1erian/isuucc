pub struct Entity<'a> {
    pub posX: u32,
    pub posY: u32,
    pub facing_direction: f64,
    pub sprite_filename: &'a str
}

pub struct Isuucc<'a> {
    pub entity: Entity<'a>,
    pub hp: u32,
    pub dmg: f32,
    pub speed: f32,
    pub tear_delay: f32
}

impl<'a> Isuucc<'a> {
    pub fn new(x: u32, y: u32) -> Isuucc<'a> {
        println!("Creating isuucc at position ({},{})", x, y);
        return
        Isuucc {
            entity: Entity {
                posX: x,
                posY: y,
                facing_direction: 1.0,
                sprite_filename: "assets/isuucc.png"
            },
            hp: 4,
            dmg: 1.0,
            speed: 1.0,
            tear_delay: 1.0
        }
    }
}