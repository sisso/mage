use specs::prelude::*;
use specs_derive::Component;

use crate::components::{Caster, Critter, Frame, Position, Velocity};
use crate::models::*;
use crate::{cfg, math};

pub fn level_from_score(score: Score) -> Level {
    f32::sqrt(score as f32).floor() as Level
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub input_dir: V2,
    pub mouse_pos: V2,
    pub cast: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub input: PlayerInput,
    pub score: Score,
    pub level: Level,
}

impl Player {
    pub fn update_score(&mut self, score: Score) {
        self.score += score;
        self.level = level_from_score(self.score);
    }

    pub fn next_level_required_score(&self) -> Score {
        (self.score..i32::MAX)
            .into_iter()
            .find(|score| level_from_score(*score) != self.level)
            .unwrap_or(self.score)
    }
}

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Critter>,
        WriteStorage<'a, Caster>,
        ReadExpect<'a, Frame>,
    );

    fn run(
        &mut self,
        (players, mut velocities, mut positions, critters, mut caster, _frame): Self::SystemData,
    ) {
        for (pla, vel, pos, cri, cas) in (
            &players,
            &mut velocities,
            &mut positions,
            &critters,
            &mut caster,
        )
            .join()
        {
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
