mod player_input;
mod map_renderers;
mod entity_render;
mod collisions;
mod move_wander;
mod end_turn;
mod hud;
mod tooltip;

use crate::prelude::*;

pub fn input_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(tooltip::monster_tooltip_system())
    .add_system(hud::hud_system())
    .build()
}

pub fn player_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
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
    .add_system(hud::hud_system())
    .add_system(end_turn::end_turn_system())
    .build()
}
