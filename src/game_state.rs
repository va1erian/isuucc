use map;
use entity;

pub struct GameState<'a> {
    pub current_map: map::Map,
    pub isuucc: entity::Isuucc<'a>
}