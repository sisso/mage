use std::sync::Arc;

use crate::models::DeltaTime;
use crate::spell::{Spell, SpellAtLevel, SpellCode, SpellEffect};

#[derive(Clone, Debug)]
pub struct Cfg {
    pub spells: Vec<Spell>,
}

pub const MODEL_MAGIC_MISSILE: &str = "magic_missile";
pub const MODEL_ENEMY_1: &str = "enemy_1";

impl Default for Cfg {
    fn default() -> Self {
        let firebold = Spell {
            spell_code: Arc::from("firebold"),
            per_level: vec![
                SpellAtLevel {
                    mana_cost: 2.0,
                    effect: SpellEffect::Projectile {
                        damage: 10.0,
                        speed: 500.0,
                        ttl: DeltaTime(5.0),
                    },
                    cast_complexity: 0.5,
                    calm_down_complexity: 0.1,
                },
                SpellAtLevel {
                    mana_cost: 2.0,
                    effect: SpellEffect::Projectile {
                        damage: 20.0,
                        speed: 500.0,
                        ttl: DeltaTime(5.0),
                    },
                    cast_complexity: 0.5,
                    calm_down_complexity: 0.1,
                },
                SpellAtLevel {
                    mana_cost: 1.0,
                    effect: SpellEffect::Projectile {
                        damage: 20.0,
                        speed: 500.0,
                        ttl: DeltaTime(5.0),
                    },
                    cast_complexity: 0.5,
                    calm_down_complexity: 0.1,
                },
            ],
        };

        Cfg {
            spells: vec![firebold],
        }
    }
}

impl Cfg {
    pub fn find_spell(&self, code: SpellCode) -> Option<&Spell> {
        self.spells.iter().find(|s| s.spell_code == code)
    }
}
