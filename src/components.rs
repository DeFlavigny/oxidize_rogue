pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveWander;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}
#[derive(Clone, PartialEq)]
pub struct Name (pub String);
