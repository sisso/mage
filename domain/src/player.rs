use specs::prelude::*;
use specs_derive::Component;

use crate::components::{Critter, Damageable, Frame, Position, Velocity};
use crate::models::*;
use crate::{cfg, math};
use crate::caster::Caster;

pub fn level_from_score(score: Score) -> Level {
    f32::sqrt(score as f32).floor() as Level
}

#[derive(Debug, Clone)]
pub enum PlayerUpgradeRequest {
    Health,
    Mana,
    Recharge,
    SkillCasting,
    Firebold
}


#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub input_dir: V2, pub mouse_pos: V2,
    pub cast: bool,
    pub upgrade: Option<PlayerUpgradeRequest>,
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub input: PlayerInput,
    score: Score,
    level: Level,
    free_skill_points: SkillPoint,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            input: Default::default(),
            score: 0,
            level: 0,
            free_skill_points: 4,
        }
    }
}

impl Player {
    pub fn update_score(&mut self, score: Score) {
        self.score += score;
        let new_level = level_from_score(self.score);
        if new_level != self.level {
            self.free_skill_points += new_level - self.level;
            self.level = new_level;
            log::debug!("player level up to {}, skill points {}", self.level, self.free_skill_points);
        }
    }

    pub fn next_level_required_score(&self) -> Score {
        (self.score..i32::MAX)
            .into_iter()
            .find(|score| level_from_score(*score) != self.level)
            .unwrap_or(self.score)
    }

    pub fn score(&self) -> Score {
        self.score
    }
    pub fn level(&self) -> Level {
        self.level
    }
    pub fn free_skill_points(&self) -> SkillPoint {
        self.free_skill_points
    }
}

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Critter>,
        WriteStorage<'a, Caster>,
        WriteStorage<'a, Damageable>,
    );

    fn run(
        &mut self,
        (mut players, mut velocities, mut positions, mut critters, mut caster, mut damageables): Self::SystemData,
    ) {
        for (pla, vel, pos, cri, cas, dam) in (
            &mut players,
            &mut velocities,
            &mut positions,
            &critters,
            &mut caster,
            &mut damageables,
        )
            .join()
        {
            // level up
            if let Some(upgrade) = pla.input.upgrade.clone() {
                _ = player_upgrade(pla, dam, cas, upgrade);
            }

            // move
            if pla.input.input_dir.length_squared() <= 0.1 {
                vel.vel = V2::ZERO;
            } else {
                vel.vel = pla.input.input_dir.normalize() * cri.speed;
            }

            // angle
            let mouse_delta = pla.input.mouse_pos - pos.pos;
            pos.angle = math::angle_of(mouse_delta);

            // casting
            if pla.input.cast {
                let rs = cas.cast(&cfg::FIRE_MISSILE);
                if rs.is_ok() {
                    log::debug!("player starting to cast");
                }
            }
        }
    }
}

fn player_upgrade(player: &mut Player, damageable: &mut Damageable, caster: &mut Caster, request:PlayerUpgradeRequest) -> Result<(), ()> {
    if player.free_skill_points <= 0 {
        return Err(());
    }

    player.free_skill_points -= 1;

    match request {
        PlayerUpgradeRequest::Health => {
            damageable.max_hp += 1.0;
        }
        PlayerUpgradeRequest::Mana => {
            caster.max_mana += 1.0;
        }
        PlayerUpgradeRequest::Recharge => {
            caster.mana_recharge += 0.1;
        }
        PlayerUpgradeRequest::SkillCasting => {
            caster.casting_skill += 0.1;
        }
        PlayerUpgradeRequest::Firebold => {
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_level() {
        assert_eq!(0, level_from_score(0));
        assert_eq!(1, level_from_score(1));
        assert_eq!(1, level_from_score(2));
        assert_eq!(1, level_from_score(3));
        assert_eq!(2, level_from_score(4));
        assert_eq!(2, level_from_score(5));
        assert_eq!(2, level_from_score(6));
        assert_eq!(2, level_from_score(7));
        assert_eq!(2, level_from_score(8));
        assert_eq!(3, level_from_score(9));
        assert_eq!(3, level_from_score(10));
        assert_eq!(3, level_from_score(11));
        assert_eq!(3, level_from_score(12));
        assert_eq!(3, level_from_score(13));
        assert_eq!(3, level_from_score(14));
        assert_eq!(3, level_from_score(15));
        assert_eq!(4, level_from_score(16));
    }
}
