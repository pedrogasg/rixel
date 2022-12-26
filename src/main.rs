#[macro_use]
extern crate itertools;
use bevy::prelude::*;
use movement::Movement;
use rand::Rng;
pub mod cell;
pub mod grid;
pub mod movement;

pub const HEIGHT: f32 = 1000.0;
pub const WIDTH: f32 = 1000.0;

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
        .add_startup_system(setup)
        .add_event::<Movement>()
        .insert_resource(movement::Actions::empty(20,20))
        .add_system(movement::keyboard_movement)
        .add_plugin(grid::GridPlugin::new(grid::GridConfig {
            window_height: HEIGHT as u32,
            window_width: WIDTH as u32,
            grid_height: 20,
            grid_width: 20,
        }))
        .add_system(movement::movement)
        .add_system(selected_cell)
        //.add_system(selected_cells)
        .add_system(update_cell)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(Agent { id: 1 })
        .insert(cell::CellPosition::new(0, 0));
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
                    color: Color::BISQUE,
                });
            }
        }
    }
}
fn selected_cells(
    time: Res<Time>,
    mut commands: Commands,
    mut grid_query: Query<(&mut grid::LastUpdate, &mut grid::Grid)>,
) {
    let current_time = time.elapsed_seconds_f64();
    for (mut last_update, grid) in grid_query.iter_mut() {
        let x = rand::thread_rng().gen_range(0..grid.config.grid_width as u32);
        let y = rand::thread_rng().gen_range(0..grid.config.grid_height as u32);
        if current_time - last_update.0 > 1.0 {
            let cell_position = cell::CellPosition::new(x, y);
            let cell_entity = grid.get(&cell_position).unwrap();
            let mut current_cell = commands.entity(cell_entity);
            current_cell.insert(UpdateCell {
                color: Color::BLACK,
            });
            last_update.0 = current_time;
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
