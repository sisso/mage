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
pub struct Player {
    pub input: PlayerInput,
    pub score: Score,
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
pub struct Caster {
    pub mana: Mana,
    pub max_mana: Mana,
    /// how much mana is recharge until max per second
    pub mana_recharge: Mana,
    // time needed to finish a cast
    pub casting: CasterState,
    pub casting_skill: CastComplexity,
}

impl Default for Caster {
    fn default() -> Self {
        Caster {
            mana: 10.0,
            max_mana: 10.0,
            mana_recharge: 1.0,
            casting_skill: 1.0,
            casting: CasterState::Idle,
        }
    }
}

impl Caster {
    pub fn cast(&mut self, spell: &Spell) -> Result<(), ()> {
        if !self.casting.is_idle() {
            return Err(());
        }

        if self.mana < spell.mana_cost {
            return Err(());
        }

        self.mana -= spell.mana_cost;

        self.casting = CasterState::Casting {
            spell: spell.clone(),
            progress: spell.cast_complexity,
        };

        Ok(())
    }

    pub fn update(&mut self, delta_time: DeltaTime) {
        // update mana
        if self.mana < self.max_mana {
            let new_mana = self.mana + self.mana_recharge * delta_time.as_seconds_f32();
            log::debug!("recharging mana {:.0}/{:.0}", new_mana, self.max_mana);
            self.mana = self.max_mana.min(new_mana);
        } else {
            self.mana = self.max_mana;
        }

        // update casting
        let cast_skill = self.casting_skill * delta_time.as_seconds_f32();
        match &mut self.casting {
            CasterState::Idle => {}
            CasterState::Casting { spell, progress } => {
                *progress -= cast_skill;
                log::debug!("casting progress {:.2}", progress);
                if *progress <= 0.0 {
                    let spell = spell.clone();
                    self.casting = CasterState::Cast { spell };
                }
            }
            CasterState::Cast { spell } => {
                log::debug!(
                    "casted complete, starting calm down {:.2}",
                    spell.calm_down_complexity
                );
                self.casting = CasterState::CalmDown {
                    progress: spell.calm_down_complexity,
                };
            }
            CasterState::CalmDown { progress } => {
                *progress -= cast_skill;
                if *progress < 0.0 {
                    log::debug!("calm down complete, casting state is idle");
                    self.casting = CasterState::Idle;
                }
            }
        }
    }

    pub fn has_cast(&self) -> Option<&Spell> {
        match &self.casting {
            CasterState::Cast { spell } => Some(spell),
            _ => None,
        }
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
