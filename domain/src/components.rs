use specs::prelude::*;
use specs_derive::Component;

use crate::models::*;

#[derive(Component, Debug, Clone, Default)]
pub struct Position {
    pub pos: V2,
    pub angle: Radians,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Velocity {
    pub vel: V2,
}

#[derive(Component, Debug, Clone)]
pub struct HasModel {
    pub model: Model,
}

#[derive(Component, Debug, Clone)]
pub struct Critter {
    pub speed: Speed,
}

#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub tick: Tick,
    pub delta_time: DeltaTime,
    pub total_time: TotalTime,
}

impl Frame {
    pub fn update(&mut self, delta_time: DeltaTime) {
        self.tick += 1;
        self.delta_time = delta_time;
        self.total_time = self.total_time.add(delta_time);
    }
}

#[derive(Component, Debug, Clone)]
pub struct Damageable {
    pub hp: Hp,
    pub max_hp: Hp,
    pub kill_score: Score,
}

#[derive(Component, Debug, Clone)]
pub struct DamageCollider {
    pub damage: Damage,
    /// only objects of this team will receive damage
    pub affects: Team,
    /// is removed after hit
    pub disposable: bool,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Deadline {
    pub deadline: TotalTime,
}

#[derive(Component, Debug, Clone)]
pub enum Ai {
    FollowPlayer,
}

#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub shape: Shape,
    pub scale: f32,
    pub sensor: bool,
}

#[derive(Component, Debug, Clone)]
pub enum Shape {
    Circle,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Component)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Component)]
pub struct Owner {
    pub entity: Entity,
}

#[cfg(test)]
mod test {
    use super::*;
}
