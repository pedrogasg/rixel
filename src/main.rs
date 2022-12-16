#[macro_use]
extern crate itertools;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use grid::GridConfig;
pub mod cell;
pub mod grid;

pub const HEIGHT: f32 = 1024.0;
pub const WIDTH: f32 = 1024.0;
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
        .add_plugin(grid::GridPlugin::new(GridConfig {
            window_height: HEIGHT as u32,
            window_width: WIDTH as u32,
            grid_height: 24,
            grid_width: 24,
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            left: -(WIDTH / 2.).floor(),
            right: (WIDTH / 2.).floor(),
            top: -(HEIGHT / 2.).floor(),
            bottom: (HEIGHT / 2.).floor(),
            scale: 1.35,
            ..Default::default()
        },
        ..Default::default()
    });
}
