use crate::{cell, movement, UpdateCell};
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Resource, Component, Reflect, Default, Clone, Copy, Debug, Hash)]
pub struct GridConfig {
    pub grid_width: u32,
    pub grid_height: u32,
    pub window_width: u32,
    pub window_height: u32,
}

impl GridConfig {
    pub fn count(&self) -> usize {
        (self.grid_width * self.grid_height) as usize
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
pub struct Grid {
    cells: Vec<Option<Entity>>,
    pub config: GridConfig,
}

impl Grid {
    pub fn empty(config: GridConfig) -> Self {
        Self {
            cells: vec![None; config.count()],
            config,
        }
    }

    pub fn get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        self.cells[cell_position.to_index(&self.config)]
    }

    pub fn checked_get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        if cell_position.within_map_bounds(&self.config) {
            self.cells[cell_position.to_index(&self.config)]
        } else {
            None
        }
    }

    pub fn set(&mut self, cell_position: &cell::CellPosition, cell_entity: Entity) {
        self.cells[cell_position.to_index(&self.config)].replace(cell_entity);
    }

    pub fn checked_set(&mut self, cell_position: &cell::CellPosition, cell_entity: Entity) {
        if cell_position.within_map_bounds(&self.config) {
            self.cells[cell_position.to_index(&self.config)].replace(cell_entity);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<Entity>> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<Entity>> {
        self.cells.iter_mut()
    }

    pub fn remove(&mut self, cell_position: &cell::CellPosition) {
        self.cells[cell_position.to_index(&self.config)].take();
    }

    pub fn checked_remove(&mut self, cell_position: &cell::CellPosition) {
        if cell_position.within_map_bounds(&self.config) {
            self.cells[cell_position.to_index(&self.config)].take();
        }
    }
}

#[derive(Bundle, Debug, Default, Clone)]
pub struct GridBundle {
    pub grid_size: GridConfig,
    pub grid: Grid,
}

pub struct GridPlugin {
    pub grid_config: GridConfig,
}

impl Default for GridPlugin {
    fn default() -> Self {
        Self {
            grid_config: GridConfig {
                grid_width: 5,
                grid_height: 5,
                window_width: 1024,
                window_height: 1024,
            },
        }
    }
}
impl GridPlugin {
    pub fn new(grid_config: GridConfig) -> Self {
        Self { grid_config }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Material2dPlugin::<cell::CellMaterial>::default())
            .insert_resource(self.grid_config)
            .add_startup_system_to_stage(StartupStage::Startup, spawn_cells)
            .add_startup_system_to_stage(StartupStage::PostStartup, adding_walls);
    }
}

#[derive(Component)]
pub struct LastUpdate(pub f64);

fn spawn_cells(
    grid_config: ResMut<GridConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<cell::CellMaterial>>,
) {
    let grid_entity = commands.spawn_empty().id();

    let mut grid = Grid::empty(grid_config.clone());
    let size = (grid_config.window_width / grid_config.grid_width) as f32;
    for (i, j) in iproduct!(0..grid_config.grid_width, 0..grid_config.grid_height) {
        let color = Color::ALICE_BLUE;

        let cell_position = cell::CellPosition::new(i, j);
        let handle = materials.add(cell::CellMaterial::new(color));
        let (x, y) = cell_position.to_screen_position(&grid_config);
        let cell_id = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(cell::Cell::new(size).into()).into(),
                material: handle,
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            })
            .insert(Name::new(format!("Cell {} {}", i, j)))
            .id();

        grid.set(&cell_position, cell_id);
    }

    commands
        .entity(grid_entity)
        .insert(GridBundle {
            grid_size: grid_config.clone(),
            grid: grid,
        })
        .insert(LastUpdate(0.0))
        .insert(Name::new("Grid"));
}

fn adding_walls(
    mut commands: Commands,
    mut grid_query: Query<&mut Grid>,
    actions: Res<movement::Actions>,
) {
    for grid in grid_query.iter_mut() {
        for cell_position in actions.get_walls().iter() {
            if cell_position.within_map_bounds(&grid.config) {
                let cell_entity = grid.get(&cell_position).unwrap();
                let mut current_cell = commands.entity(cell_entity);
                current_cell.insert(UpdateCell {
                    color: Color::VIOLET,
                });
            }
        }
    }
}