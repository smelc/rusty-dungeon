use specs::prelude::*;
use specs_derive::Component;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait Movable {
    fn move_(&mut self, dir: Direction) -> ();
}

#[derive(Component)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Movable for Position {
    fn move_(&mut self, dir: Direction) -> () {
        match dir {
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
        }
    }
}
