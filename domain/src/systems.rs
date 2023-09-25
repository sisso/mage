use std::collections::HashSet;

use rand::prelude::*;
use specs::prelude::*;

use crate::events::Events;
use crate::models::{Contacts, DeltaTime, Kind, SceneryParams, TotalTime, V2};
use crate::{cfg, unwrap_or_continue, unwrap_or_return};
use crate::{loader, math};

use super::components::*;

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

pub struct DeadlineSystem;

impl<'a> System<'a> for DeadlineSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Deadline>,
        ReadExpect<'a, Frame>,
        WriteExpect<'a, Events>,
    );

    fn run(&mut self, (entities, poisons, frame, mut events): Self::SystemData) {
        for (poi, e) in (&poisons, &entities).join() {
            if poi.deadline.is_before(frame.total_time) {
                log::debug!("deleting {:?} by deadline", e);
                events.removed.push(e);
                entities.delete(e).expect("fail to delete entity");
            }
        }
    }
}

pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, Frame>,
    );

    fn run(&mut self, (velocities, mut positions, frame): Self::SystemData) {
        for (vel, pos) in (&velocities, &mut positions).join() {
            pos.pos = pos.pos + vel.vel * frame.delta_time.as_seconds_f32();
        }
    }
}

pub struct CasterSystem;

impl<'a> System<'a> for CasterSystem {
    type SystemData = (
        WriteStorage<'a, Caster>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Frame>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, Events>,
    );

    fn run(
        &mut self,
        (mut casters, positions, frame, mut entities, updates, mut events): Self::SystemData,
    ) {
        for (cas, pos) in (&mut casters, &positions).join() {
            cas.update(frame.delta_time);

            if let Some(spell) = cas.has_cast() {
                let casting_pos = pos.pos + V2::from_angle(pos.angle) * 50.0;

                match spell.kind {
                    Kind::Projectile { damage, speed, ttl } => {
                        let e = loader::create_magic_missile(
                            updates.create_entity(&mut entities),
                            Position {
                                pos: casting_pos,
                                angle: pos.angle,
                            },
                            V2::from_angle(pos.angle),
                            damage,
                            speed,
                            frame.total_time.add(ttl),
                        )
                        .build();
                        log::debug!("casting spell {:?}", e);
                        events.added.push(e);
                    }
                    _ => todo!(),
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct EnemySpawnerSystem {
    next_spawn: TotalTime,
}

impl<'a> System<'a> for EnemySpawnerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, Events>,
        ReadExpect<'a, Frame>,
        ReadExpect<'a, SceneryParams>,
        WriteExpect<'a, StdRng>,
    );

    fn run(
        &mut self,
        (mut entities, updates, mut events, frame, params, mut rng): Self::SystemData,
    ) {
        if frame.total_time.is_before(self.next_spawn) {
            return;
        }
        self.next_spawn = frame.total_time.add(DeltaTime(3.0));

        let side = rng.gen_range(0..4);
        let position = match side {
            // top
            0 => Position {
                pos: V2::new(rng.gen_range(0..params.screen_size.x as i32) as f32, 0.0),
                angle: std::f32::consts::PI * 0.5,
            },
            // down
            1 => Position {
                pos: V2::new(
                    rng.gen_range(0..params.screen_size.x as i32) as f32,
                    params.screen_size.y,
                ),
                angle: std::f32::consts::PI * -0.5,
            },
            // left
            2 => Position {
                pos: V2::new(0.0, rng.gen_range(0..params.screen_size.y as i32) as f32),
                angle: 0.0,
            },
            // right
            3 => Position {
                pos: V2::new(
                    params.screen_size.x,
                    rng.gen_range(0..params.screen_size.y as i32) as f32,
                ),
                angle: std::f32::consts::PI,
            },
            _ => panic!("non expected random number"),
        };

        let critter = loader::new_critter(updates.create_entity(&mut entities), position).build();
        events.added.push(critter);

        log::debug!(
            "spawning critter {:?}, next spawn on {:?}",
            critter,
            self.next_spawn
        );
    }
}

pub struct AiSystem {}

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Ai>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Critter>,
    );

    fn run(
        &mut self,
        (players, ais, mut positions, mut velocities, mut critters): Self::SystemData,
    ) {
        // find player position
        let (_, player_pos) = unwrap_or_return!((&players, &positions).join().next());
        let player_pos = player_pos.to_owned();

        for (ai, pos, vel, cri) in (&ais, &mut positions, &mut velocities, &mut critters).join() {
            match ai {
                Ai::FollowPlayer => {
                    let dir = (player_pos.pos - pos.pos).normalize();
                    pos.angle = math::angle_of(dir);
                    let target_vel = cri.speed * dir;
                    vel.vel = target_vel;
                }
            }
        }
    }
}

pub struct ColliderSystem {}

impl<'a> System<'a> for ColliderSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Collider>,
        Write<'a, Contacts>,
    );

    fn run(&mut self, (entities, mut positions, colliders, mut contacts): Self::SystemData) {
        contacts.clear();

        let mut impulses = vec![];
        let mut checked = HashSet::new();

        for (e1, pos1, col1) in (&entities, &positions, &colliders).join() {
            for (e2, pos2, col2) in (&entities, &positions, &colliders).join() {
                if e1 == e2 {
                    continue;
                }

                if checked.contains(&(e1, e2)) {
                    continue;
                }

                if let Some((v1, v2)) = resolve_collision(pos1.pos, col1, pos2.pos, col2) {
                    let is_sensor = col1.sensor || col2.sensor;
                    contacts.push(e1, e2, is_sensor);

                    if !is_sensor {
                        impulses.push((e1, v1));
                        impulses.push((e2, v2));
                    }
                }

                checked.insert((e1, e2));
                checked.insert((e2, e1));
            }
        }

        for (e, v) in impulses {
            let p = positions.get_mut(e).unwrap();
            log::trace!("applying {:?} {:?} on {:?}", e, v, p.pos);
            p.pos += v;
        }
    }
}

fn resolve_collision(pos1: V2, col1: &Collider, pos2: V2, col2: &Collider) -> Option<(V2, V2)> {
    match (&col1.shape, &col2.shape) {
        (Shape::Circle, Shape::Circle) => {
            let v = pos2 - pos1;
            let distance = v.length();
            let impact_distance = col1.scale + col2.scale - distance;
            if impact_distance > 0.0 {
                let v = v.normalize();
                Some((v * -impact_distance, v * impact_distance))
            } else {
                None
            }
        } // _ => panic!("unsupported shapes"),
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use glam::Vec2;

    use super::*;

    #[test]
    fn test_resolve_collision() {
        let contact = resolve_collision(
            Vec2::ZERO,
            &Collider {
                shape: Shape::Circle,
                scale: 10.0,
                sensor: false,
            },
            V2::new(12.0, 5.0),
            &Collider {
                shape: Shape::Circle,
                scale: 5.0,
                sensor: false,
            },
        );

        assert!(contact.is_some());
        let contact = contact.unwrap();

        assert_abs_diff_eq!(-1.846154, contact.0.x);
        assert_abs_diff_eq!(-0.7692308, contact.0.y);
        assert_abs_diff_eq!(1.846154, contact.1.x);
        assert_abs_diff_eq!(0.7692308, contact.1.y);
    }
}

pub struct DamageColliderSystem {}

impl<'a> System<'a> for DamageColliderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DamageCollider>,
        ReadStorage<'a, Team>,
        WriteStorage<'a, Damageable>,
        ReadExpect<'a, Contacts>,
        WriteExpect<'a, Events>,
    );

    fn run(
        &mut self,
        (entities, damage_colliders, teams, mut damageables, contacts, mut events): Self::SystemData,
    ) {
        let mut damages = vec![];

        for (a, b) in contacts.list().iter().copied() {
            let a_team = teams.get(a);
            let a_damage = damage_colliders.get(a);
            let a_damageable = damageables.get(a);

            let b_team = teams.get(b);
            let b_damage = damage_colliders.get(b);
            let b_damageable = damageables.get(b);

            // check if a can damage b
            match (a_damage, b_team, b_damageable) {
                (Some(a_damage), Some(b_team), Some(_)) if a_damage.affects == *b_team => {
                    damages.push((b, a_damage.damage));
                    if a_damage.disposable {
                        log::trace!("{:?} hit {:?}, deleting it", a, b);
                        entities.delete(a).unwrap();
                        events.removed.push(a);
                    }
                }
                _ => {}
            }

            // check if b can damage a
            match (b_damage, a_team, a_damageable) {
                (Some(b_damage), Some(a_team), Some(_)) if b_damage.affects == *a_team => {
                    damages.push((a, b_damage.damage));
                    if b_damage.disposable {
                        log::trace!("{:?} hit {:?}, deleting it", b, a);
                        entities.delete(b).unwrap();
                        events.removed.push(b);
                    }
                }
                _ => {}
            }
        }

        for (e, damage) in damages {
            log::trace!("{:?} receive {:?}", e, damage);

            let damageable = unwrap_or_continue!(damageables.get_mut(e));
            damageable.hp -= damage;

            if damageable.hp < 0.0 {
                log::trace!("{:?} died, deleting it", e);
                entities.delete(e).unwrap();
                events.removed.push(e);
            }
        }
    }
}
