use crate::models::{DeltaTime};
use crate::spell::{SpellKind, Spell};

pub const FIRE_MISSILE: Spell = Spell {
    mana_cost: 2.0,
    kind: SpellKind::Projectile {
        damage: 10.0,
        speed: 500.0,
        ttl: DeltaTime(5.0),
    },
    cast_complexity: 0.5,
    calm_down_complexity: 0.1,
};

pub const MODEL_MAGIC_MISSILE: &str = "magic_missile";
pub const MODEL_ENEMY_1: &str = "enemy_1";
