use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    Ready,
    PlayerTurn,
    WorldTurn,
}