use specs::prelude::*;
use specs_derive::Component;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn to_x_delta(dir: &Direction) -> i32 {
  match dir {
    Direction::UP => 0,
    Direction::DOWN => 0,
    Direction::LEFT => -1,
    Direction::RIGHT => 1,
  }
}

pub fn to_y_delta(dir: &Direction) -> i32 {
  match dir {
    Direction::UP => -1,
    Direction::DOWN => 1,
    Direction::LEFT => 0,
    Direction::RIGHT => 0,
  }
}

pub trait Movable {
    fn move_(&mut self, dir: &Direction) -> ();
}

#[derive(Component)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Movable for Position {
    fn move_(&mut self, dir: &Direction) -> () {
        match dir {
            Direction::UP | Direction::DOWN => self.y += to_y_delta(&dir),
            Direction::LEFT | Direction::RIGHT => self.x += to_x_delta(&dir),
        }
    }
}
