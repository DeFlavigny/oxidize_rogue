use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &mut SubWorld,
    #[resource] camera: &Camera){
        
}