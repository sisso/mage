use specs::prelude::*;
use specs::Entity;

use crate::events::Events;
use crate::models::Hp;
use crate::player::Player;
use crate::unwrap_or_return;

use super::components::*;

#[derive(Debug)]
pub struct Hit {
    pub source: Entity,
    pub target: Entity,
    pub amount: Hp,
}

pub fn process_hit(
    hit: Hit,
    entities: &Entities,
    events: &mut WriteExpect<Events>,
    owners: &ReadStorage<Owner>,
    players: &mut WriteStorage<Player>,
    damageables: &mut WriteStorage<Damageable>,
) {
    log::trace!("{:?} receive {:?}", hit.target, hit);

    let damageable = unwrap_or_return!(damageables.get_mut(hit.target));
    damageable.hp -= hit.amount;

    if damageable.hp < 0.0 {
        log::trace!("{:?} died, deleting it", hit.target);
        entities.delete(hit.target).unwrap();
        events.removed.push(hit.target);

        if let Some(owner) = owners.get(hit.source) {
            if let Some(player) = players.get_mut(owner.entity) {
                player.update_score(damageable.kill_score);
            }
        }
    }
}
