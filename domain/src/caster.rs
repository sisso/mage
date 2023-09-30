use specs::prelude::*;
use specs_derive::Component;
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

    pub fn has_cast(&self) -> Option<&Spell> {
        match &self.casting {
            CasterState::Cast { spell } => Some(spell),
            _ => None,
        }
    }
}

