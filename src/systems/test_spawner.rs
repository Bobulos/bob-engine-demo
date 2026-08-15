use bob_engine::runtime::assets::AssetHandle;
use bob_engine::runtime::ecs::SystemBase;
use bob_engine::StableID;
use bob_engine::runtime::math::Float2;
use bob_engine::runtime::rendering::renderer::PipelineKey;
use bob_engine::runtime::rendering::sprite_rendering::components::{Sprite, SpriteFrame};
use bob_engine::runtime::rendering::*;
use bob_engine::runtime::ecs::core_components::Transform;
use bob_engine::runtime::rendering::sprite_rendering::{SpriteAnimation, SpriteSheetBinder};
use std::vec::Vec;
use std::time;
pub struct TestSpawner {
    binder: SpriteSheetBinder,
    accumulator: usize,
}
impl TestSpawner {
    pub fn new() -> Self {
        Self { 
            binder: SpriteSheetBinder::new(6, 1, Some(SpriteAnimation::new(vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [3.0, 0.0], [4.0, 0.0], [5.0, 0.0]]))),
            accumulator: 0,
        }
    }
}
#[bob_engine::component]
struct TestMover {
    
}
impl SystemBase for TestSpawner {
    fn on_start(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {

        let entity = world.create_entity();
        world.add_component_safe(entity, Transform::new(Float2::ZERO, 0.0));
        world.add_component_safe(entity, TestMover {});
        world.add_component_safe(entity, self.binder.new_sprite_at_frame([0.0, 0.0]));
        world.add_component_safe(entity, SpriteFrame::new());
        world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(1, None), PipelineKey::Sprite));

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
        if self.accumulator >= 100 {
            world.for_each2_mut_both::<Sprite, SpriteFrame>(|_entity, sprite, sprite_frame| {
                self.binder.run_animation(sprite, sprite_frame);
            });
            self.accumulator = 0
        } else {
            self.accumulator += 1;
        }
    }
    fn on_destroy(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        
    }
}