use crate::cell;
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

pub struct GridPlugin {
    width: u32,
    height: u32,
}

impl Default for GridPlugin {
    fn default() -> Self {
        Self {
            width: 5,
            height: 5,
        }
    }
}
impl GridPlugin {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let grid_size = GridSize {
            width: self.width,
            height: self.height,
        };
        app.add_plugin(Material2dPlugin::<cell::CellMaterial>::default())
            .insert_resource(grid_size)
            .add_startup_system(spawn_cells);
    }
}

#[derive(Resource)]
struct GridSize {
    width: u32,
    height: u32,
}

fn spawn_cells(
    windows: Res<Windows>,
    grid_size: ResMut<GridSize>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<cell::CellMaterial>>,
) {
    let (width,heigth) = match windows.get_primary() {
        Some(window) =>  (window.width(), window.height()),
        None => (1024., 1024.)
    };
    let size = width / grid_size.width as f32;
    let left = (width / 2.) - (size / 2.);
    let top = (heigth / 2.) - (size / 2.);
    for (i, j) in iproduct!(0..grid_size.width, 0..grid_size.height) {
        let color = if i % 2 == 0 {
            Color::ALICE_BLUE
        } else {
            Color::DARK_GRAY
        };

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(cell::Cell::new(size).into()).into(),
            material: materials.add(cell::CellMaterial::new(color)),
            transform: Transform::from_xyz((size * i as f32) - left, (size * j as f32) - top, 0.),
            ..default()
        });
    }
}
