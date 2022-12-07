#[macro_use]
extern crate itertools;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod cell;
mod grid;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(grid::GridPlugin::new(3,3))
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
