use super::components::*;
use crate::cfg;
use crate::models::*;
use glam::Vec2;
use specs::prelude::*;
use std::sync::Arc;

pub fn load_player(world: &mut World, pos: V2) -> Entity {
    world
        .create_entity()
        .with(Position { pos, angle: 0.0 })
        .with(Velocity::default())
        .with(Player {
            input: PlayerInput::default(),
        })
        .with(Team::Player)
        .with(Damageable {
            hp: 100.0,
            max_hp: 100.0,
        })
        .with(Critter { speed: 100.0 })
        .with(HasModel {
            model: Arc::from("player"),
        })
        .with(Caster::default())
        .with(Collider {
            shape: Shape::Circle,
            scale: 12.0,
            sensor: false,
        })
        .build()
}

pub fn create_magic_missile<B: Builder>(
    builder: B,
    pos: Position,
    dir: Vec2,
    damage: Damage,
    speed: Speed,
    deadline: TotalTime,
) -> B {
    builder
        .with(pos)
        .with(DamageCollider {
            damage,
            affects: Team::Enemy,
            disposable: true,
        })
        .with(Velocity { vel: dir * speed })
        .with(HasModel {
            model: Arc::from(cfg::MODEL_MAGIC_MISSILE),
        })
        .with(Collider {
            shape: Shape::Circle,
            scale: 2.5,
            sensor: true,
        })
        .with(Deadline { deadline })
}

pub fn new_critter<B: Builder>(builder: B, pos: Position) -> B {
    builder
        .with(pos)
        .with(HasModel {
            model: Arc::from(cfg::MODEL_ENEMY_1),
        })
        .with(Critter { speed: 50.0 })
        .with(Team::Enemy)
        .with(Damageable {
            hp: 10.0,
            max_hp: 10.0,
        })
        .with(Velocity {
            vel: Default::default(),
        })
        .with(DamageCollider {
            damage: 1.0,
            affects: Team::Player,
            disposable: false,
        })
        .with(Collider {
            shape: Shape::Circle,
            scale: 12.0,
            sensor: false,
        })
        .with(Ai::FollowPlayer)
}
