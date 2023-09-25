use domain::spell;
use godot::bind::{godot_api, GodotClass};
use godot::engine::global::{Key, MouseButton};
use godot::engine::utilities::abs;
use godot::engine::{AnimatedSprite2D, Engine, NodeExt, NodeVirtual, RigidBody2D};
use godot::obj::Base;
use godot::prelude::utilities::atan2;
use godot::prelude::*;
use godot::sys::get_interface;
use std::cmp;
use std::panic::panic_any;

pub struct Attributes {
    pub mana: f32,
    pub max_mana: f32,
    pub recharge: f32,
}

#[derive(GodotClass)]
#[class(base = RigidBody2D)]
pub struct Player {
    #[base]
    base: Base<RigidBody2D>,
    move_force: f32,
    max_velocity: f32,
    rotation_speed: f64,
    attributes: Attributes,
}

#[godot_api]
impl Player {
    pub fn cast(&mut self, spell: spell::Spell) {
        godot_print!("cast speel");
    }
}

#[godot_api]
impl NodeVirtual for Player {
    fn init(base: Base<RigidBody2D>) -> Self {
        if Engine::singleton().is_editor_hint() {}

        Player {
            base: base,
            move_force: 50000.0,
            max_velocity: 100.0,
            rotation_speed: 360.0_f64.to_radians(),
            attributes: Attributes {
                mana: 100.0,
                max_mana: 100.0,
                recharge: 1.0,
            },
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
        } else {
        }
    }

    fn process(&mut self, delta_time: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        let viewport = self.base.get_viewport().expect("viewport not found");

        // read inputs
        let mut force = Vector2::new(0.0, 0.0);
        let input = Input::singleton();
        if input.is_key_pressed(Key::KEY_D) {
            force += Vector2::RIGHT;
        }
        if input.is_key_pressed(Key::KEY_A) {
            force += Vector2::LEFT;
        }
        if input.is_key_pressed(Key::KEY_S) {
            force += Vector2::DOWN;
        }
        if input.is_key_pressed(Key::KEY_W) {
            force += Vector2::UP;
        }

        // update mana
        self.attributes.mana += self.attributes.recharge * delta_time as f32;
        if self.attributes.mana > self.attributes.max_mana {
            self.attributes.mana = self.attributes.max_mana;
        }

        // player movement
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let vel = self.base.get_linear_velocity();
        let desired = force * self.max_velocity;
        let mut delta: Vector2 = desired - vel;
        if delta.length_squared() > 1.0 {
            delta = delta.normalized();
        }
        self.base
            .apply_force(delta * self.move_force, Vector2::ZERO);

        // update animation
        if force.length_squared() > 0.01 {
            animated_sprite.set_animation("walk".into());
        } else {
            animated_sprite.set_animation("idle".into());
        }

        // rotate to the camera
        let player_position = self.base.get_position();
        let mouse_pos = viewport.get_mouse_position();
        let delta_mouse_position = mouse_pos - player_position;
        let angle_mouse_position =
            atan2(delta_mouse_position.y as f64, delta_mouse_position.x as f64);
        let sprite_angle = animated_sprite.get_rotation();
        let new_sprite_angle = lerp_angle(
            sprite_angle as f32,
            angle_mouse_position as f32,
            (self.rotation_speed * delta_time) as f32,
        );
        animated_sprite.set_rotation(new_sprite_angle.as_f64());

        // player casting
        if input.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            // cast spell
            self.cast(spell::FIRE_MISSILE);
        }
    }
}
