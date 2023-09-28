use gdnative::prelude::*;
use specs::prelude::*;

use domain::components::*;
use domain::error::GameError;
use domain::models::{DeltaTime, SceneryParams};
use domain::player::*;

use crate::utils::*;

mod utils;

pub type Id = u64;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct GameApi {
    api: domain::Api,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct GameApiInput {
    pub mouse_pos: Vector2,
    pub mouse_press: bool,
    pub input: Vector2,
    pub delta_time: f32,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct CasterDto {
    pub mana: f32,
    pub max_mana: f32,
    pub casting: f32,
    pub calm_down: f32,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct CritterDto {
    pub hp: f32,
    pub max_hp: f32,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct ObjDto {
    pub id: Id,
    pub pos: Vector2,
    pub angle: f32,
    pub current_speed: f32,
    pub model: String,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct ObjChangeDto {
    pub id: Id,
    pub pos: Vector2,
    pub angle: f32,
    pub current_speed: f32,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct PlayerDto {
    pub critter: CritterDto,
    pub caster: CasterDto,
    pub obj: ObjChangeDto,
    pub score: i32,
    pub score_next_level: i32,
    pub level: i32,
    pub free_skill_points: i32,
}

#[derive(ToVariant, FromVariant, Debug, Clone, Default)]
pub struct GameApiOutput {
    pub player: PlayerDto,
    pub objects: Vec<ObjChangeDto>,
    pub added: Vec<ObjDto>,
    pub removed: Vec<Id>,
}

#[methods]
impl GameApi {
    /// The "constructor" of the class.
    fn new(_base: &Node) -> Self {
        GameApi::default()
    }

    #[method]
    fn _ready(&self, #[base] base: &Node) {
        godot_print!("API ready {}!", base.to_string());
    }

    #[method]
    pub fn start_scenery(&mut self, screen_size: Vector2) {
        self.api
            .start_scenery(SceneryParams {
                screen_size: g2v(screen_size),
                seed: 0,
            })
            .expect("fail to start scenery");
    }

    #[method]
    pub fn new_run_update_input(&self) -> GameApiInput {
        GameApiInput::default()
    }

    #[method]
    pub fn run_update(&mut self, input: GameApiInput) -> GameApiOutput {
        self.set_player_input(PlayerInput {
            input_dir: g2v(input.input),
            mouse_pos: g2v(input.mouse_pos),
            cast: input.mouse_press,
        })
        .expect("fail to set player input");

        let mut added = vec![];
        let mut removed = vec![];

        self.api
            .update(DeltaTime(input.delta_time))
            .expect("fail to run update");

        let events = self.api.take_events();
        for id in events.removed {
            removed.push(encode_entity(id));
        }

        for id in events.added {
            if let Ok(data) = self.get_object(id) {
                added.push(data);
            } else {
                log::warn!("could not find obj id {:?}", id);
            }
        }

        let player_dto = self.get_player_data().expect("fail get player data");

        let objects_dto = self
            .list_objects()
            .expect("fail to list objects")
            .into_iter()
            .collect();

        GameApiOutput {
            player: player_dto,
            objects: objects_dto,
            added: added,
            removed: removed,
        }
    }
}

impl GameApi {
    pub fn get_player_data(&self) -> Result<PlayerDto, GameError> {
        let position_repo = self.api.world.read_storage::<Position>();
        let player_repo = self.api.world.read_storage::<Player>();
        let critter_repo = self.api.world.read_storage::<Critter>();
        let velocities_repo = self.api.world.read_storage::<Velocity>();
        let caster_repo = self.api.world.read_storage::<Caster>();
        let entities = self.api.world.entities();
        let damagables = self.api.world.read_storage::<Damageable>();

        let (e, pos, pla, _cri, vel, cas, dam) = (
            &entities,
            &position_repo,
            &player_repo,
            &critter_repo,
            &velocities_repo,
            &caster_repo,
            &damagables,
        )
            .join()
            .next()
            .ok_or(GameError::Str("player not found"))?;

        let caster_dto = CasterDto {
            mana: cas.mana,
            max_mana: cas.max_mana,
            casting: cas.casting.get_casting().unwrap_or(0.0),
            calm_down: cas.casting.get_calm_down().unwrap_or(0.0),
        };

        Ok(PlayerDto {
            obj: ObjChangeDto {
                id: encode_entity(e),
                pos: v2g(pos.pos),
                angle: pos.angle,
                current_speed: vel.vel.length(),
            },
            critter: CritterDto {
                hp: dam.hp,
                max_hp: dam.max_hp,
            },
            caster: caster_dto,
            score: pla.score(),
            score_next_level: pla.next_level_required_score(),
            level: pla.level(),
            free_skill_points: pla.free_skill_points(),
        })
    }

    pub fn set_player_input(&mut self, input: PlayerInput) -> Result<(), GameError> {
        self.api.set_player_input(input)
    }

    pub fn get_object(&self, id: Entity) -> Result<ObjDto, GameError> {
        let position_repo = self.api.world.read_storage::<Position>();
        let player_repo = self.api.world.read_storage::<Player>();
        let critter_repo = self.api.world.read_storage::<Critter>();
        let velocities_repo = self.api.world.read_storage::<Velocity>();
        let model_repo = self.api.world.read_storage::<HasModel>();

        let mut bs = BitSet::new();
        bs.add(id.id());

        for (_, pos, _, _cri, vel, model) in (
            &bs,
            &position_repo,
            !&player_repo,
            critter_repo.maybe(),
            &velocities_repo,
            &model_repo,
        )
            .join()
        {
            return Ok(ObjDto {
                id: encode_entity(id),
                pos: v2g(pos.pos),
                angle: pos.angle,
                current_speed: vel.vel.length(),
                model: model.model.to_string(),
            });
        }

        Err(GameError::Str("entity not found"))
    }

    pub fn list_objects(&self) -> Result<Vec<ObjChangeDto>, GameError> {
        let position_repo = self.api.world.read_storage::<Position>();
        let player_repo = self.api.world.read_storage::<Player>();
        let critter_repo = self.api.world.read_storage::<Critter>();
        let velocities_repo = self.api.world.read_storage::<Velocity>();
        let entities = self.api.world.entities();

        let mut result = vec![];

        for (e, pos, _, _cri, vel) in (
            &entities,
            &position_repo,
            !&player_repo,
            critter_repo.maybe(),
            &velocities_repo,
        )
            .join()
        {
            result.push(ObjChangeDto {
                id: encode_entity(e),
                pos: v2g(pos.pos),
                angle: pos.angle,
                current_speed: vel.vel.length(),
            })
        }

        Ok(result)
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<GameApi>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
