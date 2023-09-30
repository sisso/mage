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

impl Collider {
    pub fn is_colliding(&self, _pos: V2, _other: &Collider, _other_pos: V2) -> bool {
        todo!()
    }
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
    use crate::caster::Caster;
    use super::*;

    const SPELL: Spell = Spell {
        mana_cost: 5.0,
        cast_complexity: 1.0,
        calm_down_complexity: 1.0,
        kind: Kind::Projectile {
            damage: 1.0,
            speed: 1.0,
            ttl: DeltaTime(1.0),
        },
    };

    #[test]
    fn test_caster_casting() {
        let mut c = new_caster();
        assert!(c.cast(&SPELL).is_ok());
        assert!(c.casting.get_casting().is_some());
        let mana_after_cast = c.max_mana - SPELL.mana_cost;
        assert_eq!(mana_after_cast, c.mana);
    }

    #[test]
    fn test_caster_without_mana() {
        let mut c = new_caster();
        let not_enough_mana = SPELL.mana_cost - 1.0;
        c.mana = not_enough_mana;
        assert!(c.cast(&SPELL).is_err());
        assert!(c.casting.is_idle());
        assert_eq!(not_enough_mana, c.mana);
    }

    fn new_caster() -> Caster {
        let mut c = Caster::default();
        c.max_mana = 10.0;
        c.mana = c.max_mana;
        c
    }
}
