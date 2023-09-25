use domain::models::V2;
use gdnative::prelude::Vector2;
use specs::Entity;

pub fn v2g(v: V2) -> Vector2 {
    Vector2::new(v.x, v.y)
}

pub fn g2v(v: Vector2) -> V2 {
    V2::new(v.x, v.y)
}

// pretty but broken encode of entity, it display id:1 gen: 2 as 2000001, but it didnt' support all
// possible values of a entity
pub fn encode_entity(entity: Entity) -> u64 {
    let high = entity.gen().id() as u64 * 1_000_000;
    let low = entity.id() as u64;
    return high + low;
}

// pretty but broken decode of entity
pub fn decode_entity(value: u64) -> (u32, i32) {
    let high = value / 1_000_000;
    let low = value % 1_000_000;
    (low as u32, high as i32)
}
