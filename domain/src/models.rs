use std::sync::Arc;

use specs::Entity;

pub type Radians = f32;
pub type V2 = glam::f32::Vec2;
pub type V2I = glam::i32::IVec2;

pub type Score = i32;

pub type Mana = f32;
pub type Tick = u64;
pub type Damage = f32;
pub type Hp = f32;
pub type Radius = f32;
pub type Speed = f32;
pub type ModelRef = &'static str;
pub type Model = Arc<str>;
/// How much a work a caster need to execute to be able to cast a spell
pub type CastComplexity = f32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeltaTime(pub f32);

impl DeltaTime {
    pub fn as_seconds_f32(&self) -> f32 {
        self.0
    }

    pub fn add_seconds(self, seconds: f32) -> Self {
        DeltaTime(self.as_seconds_f32() + seconds)
    }

    pub fn mult(self, m: f32) -> Self {
        DeltaTime(self.as_seconds_f32() * m)
    }
}

impl From<f32> for DeltaTime {
    fn from(value: f32) -> Self {
        DeltaTime(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TotalTime(pub f64);

impl From<f64> for TotalTime {
    fn from(value: f64) -> Self {
        TotalTime(value)
    }
}

impl TotalTime {
    pub fn as_seconds_f64(&self) -> f64 {
        self.0
    }

    /// Or equal
    pub fn is_after(&self, time: TotalTime) -> bool {
        self.0 >= time.0
    }

    /// Or equal
    pub fn is_before(&self, time: TotalTime) -> bool {
        self.0 <= time.0
    }

    pub fn add(&self, delta: DeltaTime) -> TotalTime {
        TotalTime(self.0 + delta.0 as f64)
    }

    pub fn sub(&self, other: TotalTime) -> DeltaTime {
        DeltaTime((self.0 - other.0) as f32)
    }
}

impl std::ops::Add<DeltaTime> for TotalTime {
    type Output = TotalTime;

    fn add(self, rhs: DeltaTime) -> TotalTime {
        TotalTime(self.0 + rhs.as_seconds_f32() as f64)
    }
}

impl std::ops::Sub<DeltaTime> for DeltaTime {
    type Output = DeltaTime;

    fn sub(self, rhs: DeltaTime) -> DeltaTime {
        DeltaTime(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone)]
pub struct SceneryParams {
    pub screen_size: V2,
    pub seed: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub input_dir: V2,
    pub mouse_pos: V2,
    pub cast: bool,
}

#[derive(Debug, Clone)]
pub struct CastPoint {
    pub pos: V2,
    pub angle: Radians,
}

#[derive(Debug, Clone)]
pub enum Kind {
    Projectile {
        damage: Damage,
        speed: Speed,
        ttl: DeltaTime,
    },
    ExplosiveProject {
        damage: Damage,
        speed: Speed,
        radius: Radius,
    },
    Area {
        damage: Damage,
        radius: Radius,
    },
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub mana_cost: Mana,
    pub cast_complexity: CastComplexity,
    pub calm_down_complexity: CastComplexity,
    pub kind: Kind,
}

impl Spell {
    pub fn time_to_cast(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.cast_complexity / casting_skill)
    }

    pub fn time_to_calm(&self, casting_skill: CastComplexity) -> DeltaTime {
        DeltaTime(self.calm_down_complexity / casting_skill)
    }
}

// #[derive(Debug, Clone)]
// pub struct Casting {
//     pub spell: Spell,
//     /// how much user already process the spell, if complexity is > spell.complexity, is cast
//     pub chant: CastComplexity,
// }

#[derive(Debug, Clone)]
pub enum CasterState {
    Idle,
    Cast {
        spell: Spell,
    },
    Casting {
        spell: Spell,
        /// decrement until zero
        progress: CastComplexity,
    },
    CalmDown {
        /// decrement until zero
        progress: CastComplexity,
    },
}

impl CasterState {
    pub fn has_casted(&self) -> bool {
        match self {
            Self::Cast { .. } => true,
            _ => false,
        }
    }

    pub fn get_calm_down(&self) -> Option<CastComplexity> {
        match self {
            Self::CalmDown { progress } => Some(*progress),
            _ => None,
        }
    }

    pub fn is_idle(&self) -> bool {
        match self {
            CasterState::Idle => true,
            _ => false,
        }
    }
    pub fn get_casting(&self) -> Option<CastComplexity> {
        match self {
            Self::Casting { progress, .. } => Some(*progress),
            _ => None,
        }
    }
}

impl Default for CasterState {
    fn default() -> Self {
        CasterState::Idle
    }
}

#[derive(Default, Clone, Debug)]
pub struct Contacts {
    // contact between two solid objects
    contacts: Vec<(Entity, Entity)>,
    // contacts between where at least one entity is a sensor
    // detects: Vec<(Entity, Entity)>,
}

impl Contacts {
    pub fn clear(&mut self) {
        self.contacts.clear();
        // self.detects.clear();
    }

    pub fn push(&mut self, e1: Entity, e2: Entity, _any_sensor: bool) {
        // if any_sensor {
        //     log::trace!("sensor contact {:?} {:?}", e1, e2);
        //     self.detects.push((e1, e2));
        // } else {
        //     log::trace!("collider contact {:?} {:?}", e1, e2);
        self.contacts.push((e1, e2));
        // }
    }

    pub fn list(&self) -> &Vec<(Entity, Entity)> {
        &self.contacts
    }
}
