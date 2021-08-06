use bevy::prelude::*;
use bevy_pyxel_loader::{BevyPyxelPlugin, PyxelFile};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyPyxelPlugin)
        .add_startup_system(startup)
        .add_system(dbg_json)
        .run();
}
fn startup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load::<PyxelFile, &str>("borders.pyxel");
    commands.insert_resource(handle);
}
fn dbg_json(assets: Res<Assets<PyxelFile>>, handle: Res<Handle<PyxelFile>>) {
    if assets.is_changed() {
        if let Some(file) = assets.get(handle.id) {
            dbg!(&file.data);
        }
    }
}
