use log::LevelFilter;
use rand::prelude::StdRng;
use rand::SeedableRng;
use specs::prelude::*;

use crate::components::*;
use crate::error::GameError;
use crate::events::Events;
use crate::models::*;
use crate::systems::*;

pub mod cfg;
pub mod components;
pub mod damage;
pub mod error;
pub mod events;
pub mod loader;
pub mod math;
pub mod models;
pub mod systems;
pub mod utils;

#[macro_export]
macro_rules! unwrap_or_continue {
    ($res:expr) => {
        match $res {
            Some(value) => value,
            None => continue,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($res:expr) => {
        match $res {
            Some(value) => value,
            None => return,
        }
    };
}

pub struct Api {
    pub world: World,
    pub enemy_system: EnemySpawnerSystem,
}

impl Default for Api {
    fn default() -> Self {
        _ = env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .try_init();

        log::info!("starting api");

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Player>();
        world.register::<Critter>();
        world.register::<Caster>();
        world.register::<DamageCollider>();
        world.register::<Deadline>();
        world.register::<HasModel>();
        world.register::<Ai>();
        world.register::<Collider>();
        world.register::<Damageable>();
        world.register::<Team>();
        world.register::<Owner>();

        Self {
            world,
            enemy_system: EnemySpawnerSystem::default(),
        }
    }
}

impl Api {
    pub fn start_scenery(&mut self, params: SceneryParams) -> Result<(), GameError> {
        let start_position = {
            let v = params.screen_size * 0.5;
            V2::new(v.x, v.y)
        };

        self.world.insert(Frame::default());
        self.world.insert(Events::default());
        self.world.insert(StdRng::seed_from_u64(params.seed));
        self.world.insert(Contacts::default());
        self.world.insert(params);

        self.enemy_system = Default::default();

        loader::load_player(&mut self.world, start_position);

        Ok(())
    }

    pub fn set_player_input(&mut self, input: PlayerInput) -> Result<(), GameError> {
        let mut player_repo = self.world.write_storage::<Player>();
        for (pla,) in (&mut player_repo,).join() {
            pla.input = input.clone();
        }
        Ok(())
    }

    pub fn update(&mut self, delta_time: DeltaTime) -> Result<(), GameError> {
        {
            let mut frame = self.world.write_resource::<Frame>();
            frame.update(delta_time);
        }

        let mut system = DeadlineSystem {};
        system.run_now(&mut self.world);

        let mut system = PlayerSystem {};
        system.run_now(&mut self.world);

        let mut system = VelocitySystem {};
        system.run_now(&mut self.world);

        let mut system = ColliderSystem {};
        system.run_now(&mut self.world);

        let mut system = DamageColliderSystem {};
        system.run_now(&mut self.world);

        let mut system = CasterSystem {};
        system.run_now(&mut self.world);

        self.enemy_system.run_now(&mut self.world);

        let mut system = AiSystem {};
        system.run_now(&mut self.world);

        self.world.maintain();

        Ok(())
    }

    pub fn take_events(&mut self) -> Events {
        let mut events = self.world.write_resource::<Events>();
        events.take()
    }
}
