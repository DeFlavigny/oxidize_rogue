use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState){
    let new_state = match turn_state {
        TurnState::Ready => TurnState::PlayerTurn,
        TurnState::PlayerTurn => TurnState::WorldTurn,
        TurnState::WorldTurn => TurnState::Ready,
    };

    *turn_state = new_state;
}