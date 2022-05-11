pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        Health {
            max: 20,
            current: 20,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        MoveWander,
        Name(name),
        Health {
            current: hp,
            max: hp,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}


fn egre() -> (i32, String, FontCharType) {
    (5, "Egre".to_string(), to_cp437('E'))
}
fn ogre() -> (i32, String, FontCharType) {
    (3, "Ogre".to_string(), to_cp437('O'))
}