#[macro_use]
extern crate itertools;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod cell;
mod grid;

pub const HEIGHT: f32 = 1280.0;
pub const WIDTH: f32 = 1280.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: WIDTH,
                        height: HEIGHT,
                        title: "Rixel".to_string(),
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .add_startup_system(setup)
        .add_plugin(grid::GridPlugin::new(30, 30))
        //.add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection{
            left: -(WIDTH / 2.),
            right: WIDTH / 2.,
            top: -(HEIGHT / 2.),
            bottom: HEIGHT / 2.,
            ..Default::default()
        },
        ..Default::default()
    });
}
