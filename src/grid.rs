use crate::cell;
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;
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
    /// Creates a new tile storage that is empty.
    pub fn empty(size: GridConfig) -> Self {
        Self {
            cells: vec![None; size.count()],
            config: size,
        }
    }

    /// Gets a tile entity for the given tile position, if an entity is associated with that tile
    /// position.
    ///
    /// Panics if the given `cell_position` does lie within the extents of the underlying tile map.
    pub fn get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        self.cells[cell_position.to_index(&self.config)]
    }

    /// Gets a tile entity for the given tile position, if:
    /// 1) the tile position lies within the underlying tile map's extents *and*
    /// 2) there is an entity associated with that tile position;
    /// otherwise it returns `None`.
    pub fn checked_get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        if cell_position.within_map_bounds(&self.config) {
            self.cells[cell_position.to_index(&self.config)]
        } else {
            None
        }
    }

    /// Sets a tile entity for the given tile position.
    ///
    /// If there is an entity already at that position, it will be replaced.
    ///
    /// Panics if the given `cell_position` does lie within the extents of the underlying tile map.
    pub fn set(&mut self, cell_position: &cell::CellPosition, cell_entity: Entity) {
        self.cells[cell_position.to_index(&self.config)].replace(cell_entity);
    }

    /// Sets a tile entity for the given tile position, if the tile position lies within the
    /// underlying tile map's extents.
    ///
    /// If there is an entity already at that position, it will be replaced.
    pub fn checked_set(&mut self, cell_position: &cell::CellPosition, cell_entity: Entity) {
        if cell_position.within_map_bounds(&self.config) {
            self.cells[cell_position.to_index(&self.config)].replace(cell_entity);
        }
    }

    /// Returns an iterator with all of the positions in the grid.
    pub fn iter(&self) -> impl Iterator<Item = &Option<Entity>> {
        self.cells.iter()
    }

    /// Returns mutable iterator with all of the positions in the grid.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<Entity>> {
        self.cells.iter_mut()
    }

    /// Remove entity at the given tile position, if there was one, leaving `None` in its place.
    ///
    /// Panics if the given `cell_position` does lie within the extents of the underlying tile map.
    pub fn remove(&mut self, cell_position: &cell::CellPosition) {
        self.cells[cell_position.to_index(&self.config)].take();
    }

    /// Remove any stored entity at the given tile position, if the given `cell_position` does lie within
    /// the extents of the underlying map.
    ///
    /// Otherwise, nothing is done.
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
            .add_startup_system(spawn_cells)
            .add_system(selected_cells)
            .add_system(update_cell);
    }
}

#[derive(Component)]
struct LastUpdate(f64);

#[derive(Debug, Default, Clone, Component)]
pub struct UpdateCell {
    pub color: Color,
}

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

fn selected_cells(
    time: Res<Time>,
    mut commands: Commands,
    mut grid_query: Query<(&mut LastUpdate, &mut Grid)>,
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
