pub use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MoveWander)]
pub fn move_wander(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut movers = <(&mut Point, &MoveWander)>::query();
    movers.iter_mut(ecs)
    .for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0,4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            3 => Point::new(0, 1),
            _ => Point::new(0, 0),
        } + *pos;

        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}
