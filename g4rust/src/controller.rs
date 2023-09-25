use godot::bind::{godot_api, GodotClass};
use godot::engine::{Engine, Node, NodeExt, NodeVirtual};
use godot::obj::Base;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Controller {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Controller {

}

#[godot_api]
impl NodeVirtual for Controller {
    fn init(base: Base<Node>) -> Self {
        if Engine::singleton().is_editor_hint() {
        }

        Controller {
            base: base,
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
        } else {
        }

        godot_print!("ready");
    }

    fn process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }


    }
}
