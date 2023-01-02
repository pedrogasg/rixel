#[macro_use]
extern crate itertools;
use bevy::prelude::*;
use ndarray::prelude::*;
use serde::Deserialize;
use std::fs;
pub mod cell;
pub mod grid;
pub mod menu;
pub mod movement;
pub const HEIGHT: f32 = 1000.0;
pub const WIDTH: f32 = 1000.0;

#[derive(Debug, Deserialize)]
struct TestStruct {
    name: String,
    grid: Array2<i8>,
}

#[derive(Debug, Default, Clone, Component)]
pub struct UpdateCell {
    pub color: Color,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Agent {
    pub id: u32,
}

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
        .add_state(AppState::Menu)
        .init_resource::<MainLayout>()
        .add_startup_system(setup)
        .add_plugin(menu::LayoutsMenu)
        .add_event::<movement::Movement>()
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_game))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(game_loaded))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movement::keyboard_movement)
                .with_system(movement::movement)
                .with_system(selected_cell)
                .with_system(update_cell)
                .with_system(keyboard_return),
        )
        .add_plugin(grid::GridPlugin)
        .run();
}

#[derive(Resource)]
struct MainLayout {
    path: String,
}

impl Default for MainLayout {
    fn default() -> Self {
        Self {
            path: "./assets/layouts/capsuleClassic.json".to_string(),
        }
    }
}

#[derive(Component, Default)]
struct AssetPath {
    path: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    Loading,
    InGame,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.35,
            ..Default::default()
        },
        ..Default::default()
    });
}
fn setup_game(mut commands: Commands, main_layout: Res<MainLayout>) {
    let file_content = fs::read_to_string(main_layout.path.clone()).unwrap();
    let test = serde_json::from_str::<TestStruct>(&file_content).unwrap();
    println!("Name of the test {:?}", test.name);
    println!("Serde encode {:?} => {:?}", test, file_content);
    let (grid_width, grid_height) = test.grid.dim();

    commands.insert_resource(grid::GridConfig {
        window_height: HEIGHT as u32,
        window_width: WIDTH as u32,
        grid_height: grid_height as u32,
        grid_width: grid_width as u32,
    });

    commands.insert_resource(movement::Actions::new(test.grid));
}

fn game_loaded(mut state: ResMut<State<AppState>>) {
    state.set(AppState::InGame).unwrap();
}

fn keyboard_return(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();
    }
}

fn selected_cell(
    mut commands: Commands,
    mut grid_query: Query<&mut grid::Grid>,
    mut agent_query: Query<(&Agent, &mut cell::CellPosition)>,
) {
    for grid in grid_query.iter_mut() {
        for (_agent, cell_position) in agent_query.iter_mut() {
            if cell_position.within_map_bounds(&grid.config) {
                let cell_entity = grid.get(&cell_position).unwrap();
                let mut current_cell = commands.entity(cell_entity);
                current_cell.insert(UpdateCell {
                    color: Color::ALICE_BLUE,
                });
            }
        }
    }
}

fn update_cell(
    mut query: Query<(Entity, &Handle<cell::CellMaterial>, &UpdateCell)>,
    mut materials: ResMut<Assets<cell::CellMaterial>>,
    mut commands: Commands,
) {
    for (entity, material_handle, update) in query.iter_mut() {
        let mut material = materials.get_mut(&material_handle).unwrap();

        material.color = update.color;
        let mut current_cell = commands.entity(entity);
        current_cell.remove::<UpdateCell>();
    }
}
