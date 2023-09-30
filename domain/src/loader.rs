use std::sync::Arc;

use glam::Vec2;
use specs::prelude::*;
use crate::caster::Caster;

use crate::cfg;
use crate::models::*;
use crate::player::{Player, PlayerInput};

use super::components::*;

pub fn load_player(world: &mut World, pos: V2) -> Entity {
    world
        .create_entity()
        .with(Position { pos, angle: 0.0 })
        .with(Velocity::default())
        .with(Player::default())
        .with(Team::Player)
        .with(Damageable {
            hp: 100.0,
            max_hp: 100.0,
            kill_score: 0,
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
    owner: Option<Entity>,
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
        .maybe_with(owner.map(|own| Owner { entity: own }))
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
            kill_score: 1,
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
