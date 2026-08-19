use std::sync::Arc;
use bob_engine::runtime::assets::AssetStore;
use bob_engine::runtime::Engine;
use bob_engine::runtime::ecs::{SystemGroup, system_group};
use bob_engine::*;
pub mod systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bob_engine running...");
    let event_loop = EventLoop::new()?; 
    
    // Bootstrap
    let mut engine = Engine::new();
    bootstrap_systems(&mut engine);
    // Do this last
    let mut app = App::new(engine, build_asset_store());
    event_loop.run_app(&mut app)?;
    Ok(())
}
const GAMEPLAY_LOGIC: &str = "gameplay_logic";
fn bootstrap_systems(engine: &mut Engine) {
    let world = engine.entities.get_world(bob_engine::constants::MAIN_WORLD).unwrap();
    engine.entities.add_system_group(GAMEPLAY_LOGIC, SystemGroup::new(&world, system_group::SystemGroupThreading::Main));
    
    let group = engine.entities.get_system_group_mut(GAMEPLAY_LOGIC).unwrap();
    group.register_system(Box::new(systems::TestSpawner::new(engine.input.clone())), 0);
}
fn build_asset_store() -> AssetStore {
    let mut a: AssetStore = AssetStore::new();
    include_asset!(&mut a, "test_texture.png");
    include_asset!(&mut a, "ship_parts.png");
    include_asset!(&mut a, "crappy_fire.png");
    include_asset!(&mut a, "drone.png");
    //include_asset!(&mut a, "test")
    a
}