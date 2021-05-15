use bevy::prelude::{Vec2, Vec3};
use std::ops::Add;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub absolute: bool
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position {
            x, y, z,
            absolute: false
        }
    }

    pub fn new_2d(x: f32, y: f32) -> Self {
        Position::new(x, y, 0f32)
    }

    // 相对位置到绝对位置
    pub fn convert(mut self, ww: f32, wh: f32) -> Self {
        /*if !self.absolute {
            self.x = self.x / ARENA_WIDTH as f32 * ww - (ww / 2.0) + (ww / ARENA_WIDTH as f32 / 2.0);
            self.y = self.y / ARENA_HEIGHT as f32 * wh - (wh / 2.0) + (wh / ARENA_HEIGHT as f32 / 2.0);
        } else {
            self.x = self.x - ww / 2f32;
            self.y = self.y - wh / 2f32;
        }*/
        self.x = self.x - ww / 2f32;
        self.y = self.y - wh / 2f32;
        self
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(rhs.x + self.x, rhs.y + self.y, rhs.z + self.z)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
            .and(self.y.partial_cmp(&other.y))
    }
}

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        Vec2::new(pos.x, pos.y)
    }
}

impl From<Position> for Vec3 {
    fn from(pos: Position) -> Self {
        Vec3::new(pos.x, pos.y, pos.z)
    }
}

impl From<&Position> for Vec3 {
    fn from(pos: &Position) -> Self {
        Vec3::new(pos.x, pos.y, pos.z)
    }
}
