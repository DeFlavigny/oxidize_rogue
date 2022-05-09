mod player_input;
mod map_renderers;
mod entity_render;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(map_renderers::map_render_system())
    .add_system(entity_render::entity_render_system())
    .build()
}