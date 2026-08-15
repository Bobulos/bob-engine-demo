use bob_engine::runtime::assets::AssetHandle;
use bob_engine::runtime::ecs::SystemBase;
use bob_engine::StableID;
use bob_engine::runtime::input::{Input, KeyCode, MouseButton, input};
use bob_engine::runtime::math::Float2;
use bob_engine::runtime::phys::{RigidBody, Shape};
use bob_engine::runtime::rendering::renderer::PipelineKey;
use bob_engine::runtime::rendering::sprite_rendering::components::{Sprite, SpriteFrame};
use bob_engine::runtime::rendering::*;
use bob_engine::runtime::ecs::core_components::Transform;
use bob_engine::runtime::rendering::sprite_rendering::{SpriteAnimation, SpriteSheetBinder};
use std::sync::{Arc, RwLock};
use std::vec::Vec;
use std::time;
pub struct TestSpawner {
    input: Arc<RwLock<Input>>,
    binder_ship: SpriteSheetBinder,
    binder_fire: SpriteSheetBinder,
    accumulator: usize,
}
impl TestSpawner {
    pub fn new(input: Arc<RwLock<Input>>) -> Self {
        Self { 
            input,
            binder_ship: SpriteSheetBinder::new(6, 1, Some(SpriteAnimation::new(vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [3.0, 0.0], [4.0, 0.0], [5.0, 0.0]]))),
            binder_fire: SpriteSheetBinder::new(2, 1, Some(SpriteAnimation::new(vec![[0.0, 0.0], [1.0, 0.0]]))),
            accumulator: 0,
        }
    }
}
#[bob_engine::component]
struct ShipPart {
    
}

#[bob_engine::component]
struct Flame {
    
}
impl SystemBase for TestSpawner {
    fn on_start(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {

        let entity = world.create_entity();
        world.add_component_safe(entity, Transform::new(Float2::ZERO, 0.0));
        world.add_component_safe(entity, ShipPart {});
        world.add_component_safe(entity, self.binder_ship.new_sprite_at_frame([5.0, 0.0]));
        world.add_component_safe(entity, SpriteFrame::new());
        world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

        let entity = world.create_entity();
        world.add_component_safe(entity, Transform::new(Float2::new(0.0, -1.0), 0.0));
        world.add_component_safe(entity, Flame {});
        world.add_component_safe(entity, self.binder_fire.new_sprite_at_frame([0.0, 0.0]));
        world.add_component_safe(entity, SpriteFrame::new());
        world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(2, None), PipelineKey::Sprite));

        // let entity = world.create_entity();
        // world.add_component_safe(entity, Transform::new(Float2::new(0.7, 0.0), 0.0));
        // world.add_component_safe(entity, TestMover {});
        // world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [1.0/6.0, 0.0], [1.0/6.0, 1.0]));
        // world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

        // let entity = world.create_entity();
        // world.add_component_safe(entity, Transform::new(Float2::new(2.0, 0.0), 0.0));
        // world.add_component_safe(entity, TestMover {});
        // world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [2.0/6.0, 0.0], [1.0/6.0, 1.0]));
        // world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

        // let entity = world.create_entity();
        // world.add_component_safe(entity, Transform::new(Float2::new(3.0, 0.0), 0.0));
        // world.add_component_safe(entity, TestMover {});
        // world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [3.0/6.0, 0.0], [1.0/6.0, 1.0]));
        // world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

        // let entity = world.create_entity();
        // world.add_component_safe(entity, Transform::new(Float2::new(4.0, 0.0), 0.0));
        // world.add_component_safe(entity, TestMover {});
        // world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [4.0/6.0, 0.0], [1.0/6.0, 1.0]));
        // world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

        // let entity = world.create_entity();
        // world.add_component_safe(entity, Transform::new(Float2::new(5.0, 0.0), 0.0));
        // world.add_component_safe(entity, TestMover {});
        // world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [5.0/6.0, 0.0], [1.0/6.0, 1.0]));
        // world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));
        println!("Test spawner started");
    }
    fn on_update(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        if self.accumulator >= 60/60 {
            // world.for_each3_mut_all::<Sprite, SpriteFrame, ShipPart>(|_entity, sprite, sprite_frame, _| {
            //     self.binder_ship.run_animation(sprite, sprite_frame);
            // });
            world.for_each3_mut_all::<Sprite, SpriteFrame, Flame>(|_entity, sprite, sprite_frame, _| {
                self.binder_fire.run_animation(sprite, sprite_frame);
            });
            self.accumulator = 0
        } else {
            self.accumulator += 1;
        }
        let input = self.input.read().unwrap();
        if input.get_mouse_button_pressed(MouseButton::Left) {
            let pos = input.mouse_world_position;
            let entity = world.create_entity();
            world.add_component_safe(entity, Transform::new(pos, 0.0));
            world.add_component_safe(entity, Flame {});
            world.add_component_safe(entity, self.binder_fire.new_sprite_at_frame([0.0, 0.0]));
            world.add_component_safe(entity, SpriteFrame::new());
            world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(2, None), PipelineKey::Sprite));
        }
        if input.get_mouse_button_pressed(MouseButton::Right) {
            let pos = input.mouse_world_position;
            let entity = world.create_entity();
            world.add_component_safe(entity, Transform::new(pos, 0.0));
            world.add_component_safe(entity, ShipPart {});
            world.add_component_safe(entity, self.binder_ship.new_sprite_at_frame([3.0, 0.0]));
            world.add_component_safe(entity, SpriteFrame::new());
            world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));
            let mut rb = RigidBody::new(Shape::Rect { half_w: 0.5, half_h: 0.5 }, 1.0, pos, 0.0);
            if input.get_key_down(KeyCode::ShiftLeft) {
                rb.apply_force((Float2::ZERO - pos) * 30.0);
            }
            world.add_component_safe(entity, rb);
        }
        if input.get_mouse_button_down(MouseButton::Middle) {
            let pos = input.mouse_world_position;
            let entity = world.create_entity();
            world.add_component_safe(entity, Transform::new(pos, 0.0));
            world.add_component_safe(entity, ShipPart {});
            world.add_component_safe(entity, self.binder_ship.new_sprite_at_frame([4.0, 0.0]));
            world.add_component_safe(entity, SpriteFrame::new());
            world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));
            let mut rb = RigidBody::new(Shape::Rect { half_w: 0.5, half_h: 0.5 }, 1.0, pos, 0.0);
            if input.get_key_down(KeyCode::ShiftLeft) {
                rb.apply_force((Float2::ZERO - pos) * 30.0);
            }
            world.add_component_safe(entity, rb);
        }
    }
    fn on_destroy(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        
    }
}