use super::models::*;

#[derive(Debug, Clone)]
pub enum SpellKind {
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
pub struct Spell {
    pub mana_cost: Mana,
    pub cast_complexity: CastComplexity,
    pub calm_down_complexity: CastComplexity,
    pub kind: SpellKind,
}

impl Spell {
    pub fn time_to_cast(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.cast_complexity / casting_skill)
    }

    pub fn time_to_calm(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.calm_down_complexity / casting_skill)
    }
}
