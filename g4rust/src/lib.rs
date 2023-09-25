use godot::prelude::*;

pub mod controller;
pub mod player;

struct GameApi;

#[gdextension]
unsafe impl ExtensionLibrary for GameApi {}
