use specs::prelude::*;
use specs_derive::Component;

use crate::spell::{Spell, SpellAtLevel, SpellBook, SpellBookEntry, SpellCode};

use super::models::*;

#[derive(Component, Debug, Clone)]
pub struct Caster {
    pub mana: Mana,
    pub max_mana: Mana,
    /// how much mana is recharge until max per second
    pub mana_recharge: Mana,
    // time needed to finish a cast
    pub casting: CasterState,
    pub casting_skill: CastComplexity,
    pub spell_book: SpellBook,
}

impl Default for Caster {
    fn default() -> Self {
        Caster {
            mana: 10.0,
            max_mana: 10.0,
            mana_recharge: 1.0,
            casting_skill: 1.0,
            casting: CasterState::Idle,
            spell_book: SpellBook::default(),
        }
    }
}

impl Caster {
    pub fn new(spells: &Vec<Spell>) -> Caster {
        let mut caster = Caster::default();
        let spell_book = SpellBook {
            spells: spells
                .iter()
                .map(|spell| SpellBookEntry {
                    level: 0,
                    spell: spell.clone(),
                })
                .collect(),
        };
        caster.spell_book = spell_book;
        caster
    }

    pub fn cast(&mut self, spell: SpellCode) -> Result<(), ()> {
        let spell = self.get_spell(spell)?.clone();
        self.cast_spell_at_level(spell)
    }

    fn cast_spell_at_level(&mut self, spell: SpellAtLevel) -> Result<(), ()> {
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
            log::trace!("recharging mana {:.0}/{:.0}", new_mana, self.max_mana);
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
                log::trace!("casting progress {:.2}", progress);
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

    pub fn has_cast(&self) -> Option<&SpellAtLevel> {
        match &self.casting {
            CasterState::Cast { spell } => Some(spell),
            _ => None,
        }
    }

    pub fn get_spell(&self, code: SpellCode) -> Result<&SpellAtLevel, ()> {
        self.spell_book
            .spells
            .iter()
            .find(|e| e.spell.spell_code == code)
            .ok_or(())
            .and_then(|e| e.spell.per_level.get(e.level as usize).ok_or(()))
    }
}

#[cfg(test)]
mod test {
    use crate::caster::Caster;
    use crate::spell::{Spell, SpellAtLevel, SpellBookEntry, SpellEffect};

    use super::*;

    const SPELL_CODE: &'static str = "spell";

    const SPELL: SpellAtLevel = SpellAtLevel {
        mana_cost: 5.0,
        cast_complexity: 1.0,
        calm_down_complexity: 1.0,
        effect: SpellEffect::Projectile {
            damage: 1.0,
            speed: 1.0,
            ttl: DeltaTime(1.0),
        },
    };

    fn new_caster() -> Caster {
        let mut c = Caster::default();
        c.max_mana = 10.0;
        c.mana = c.max_mana;
        c.spell_book = SpellBook {
            spells: vec![SpellBookEntry {
                level: 0,
                spell: Spell {
                    spell_code: SpellCode::from(SPELL_CODE),
                    per_level: vec![SPELL],
                },
            }],
        };
        c
    }

    #[test]
    fn test_caster_casting() {
        let mut c = new_caster();
        assert!(c.cast(SpellCode::from(SPELL_CODE)).is_ok());
        assert!(c.casting.get_casting().is_some());
        let mana_after_cast = c.max_mana - SPELL.mana_cost;
        assert_eq!(mana_after_cast, c.mana);
    }

    #[test]
    fn test_caster_without_mana() {
        let mut c = new_caster();
        let not_enough_mana = SPELL.mana_cost - 1.0;
        c.mana = not_enough_mana;
        assert!(c.cast(SpellCode::from(SPELL_CODE)).is_err());
        assert!(c.casting.is_idle());
        assert_eq!(not_enough_mana, c.mana);
    }
}
