use bob_engine::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bob_engine running...");
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    
    let mut eng = app.engine.expect("BIG PROBLEM BUDDY THE ENGINE DOESN'T EXIST");

    add_assets(&mut eng);
    Ok(())
}
use bob_engine::runtime::assets::AssetStore;
use bob_engine::runtime::Engine;
fn add_assets(engine: &mut Engine) {
    include_asset!(engine.asset_store.take().unwrap(), "../assets/test_texture.png");
}