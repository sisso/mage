use std::sync::Arc;

use super::models::*;

pub type SpellLevel = i32;
pub type SpellCode = Arc<str>;

#[derive(Debug, Clone)]
pub struct SpellBookEntry {
    pub level: Level,
    pub spell: Spell,
}

/// list of available spells on the level know of each spell
#[derive(Debug, Clone, Default)]
pub struct SpellBook {
    pub spells: Vec<SpellBookEntry>,
}

/// list spells per level
#[derive(Debug, Clone)]
pub struct Spell {
    pub spell_code: SpellCode,
    pub per_level: Vec<SpellAtLevel>,
}

#[derive(Debug, Clone)]
pub enum SpellEffect {
    Projectile {
        damage: Damage,
        speed: Speed,
        ttl: DeltaTime,
    },
    ExplosiveProject {
        damage: Damage,
        speed: Speed,
        radius: Radius,
    },
    Area {
        damage: Damage,
        radius: Radius,
    },
}

#[derive(Debug, Clone)]
pub struct SpellAtLevel {
    pub mana_cost: Mana,
    pub cast_complexity: CastComplexity,
    pub calm_down_complexity: CastComplexity,
    pub effect: SpellEffect,
}

impl SpellAtLevel {
    pub fn time_to_cast(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.cast_complexity / casting_skill)
    }

    pub fn time_to_calm(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.calm_down_complexity / casting_skill)
    }
}
