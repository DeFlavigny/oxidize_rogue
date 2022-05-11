mod player_input;
mod map_renderers;
mod entity_render;
mod collisions;
mod move_wander;
mod end_turn;

use crate::prelude::*;

use self::{entity_render::entity_render, collisions::collisions};

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(move_wander::move_wander_system())
    .build()
}

pub fn input_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .build()
}

pub fn player_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn world_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(move_wander::move_wander_system())
    .flush()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}
