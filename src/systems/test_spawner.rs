use bob_engine::runtime::assets::AssetHandle;
use bob_engine::runtime::ecs::SystemBase;
use bob_engine::StableID;
use bob_engine::runtime::math::Float2;
use bob_engine::runtime::rendering::renderer::PipelineKey;
use bob_engine::runtime::rendering::*;
use bob_engine::runtime::ecs::core_components::Transform;
pub struct TestSpawner {
    
}
#[bob_engine::component]
struct TestMover {
    
}
impl SystemBase for TestSpawner {
    fn on_start(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        let entity = world.create_entity();
        world.add_component_safe(entity, Transform::new(Float2::ZERO, 0.0));
        world.add_component_safe(entity, TestMover {});
        world.add_component_safe(entity, sprite_rendering::components::Sprite::new(true, [0.0, 0.0], [1.0, 1.0]));
        world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(0, None), PipelineKey::Sprite));
        println!("Test spawner started");
    }
    fn on_update(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        
    }
    fn on_destroy(&mut self, world: &std::sync::Arc<bob_engine::runtime::ecs::DynamicWorld>) {
        
    }
}