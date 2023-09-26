use specs::prelude::*;
use specs_derive::Component;

use crate::components::{Caster, Critter, Frame, Position, Velocity};
use crate::models::*;
use crate::{cfg, math};

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
