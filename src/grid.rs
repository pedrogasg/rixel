use crate::cell;
use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;
#[derive(Resource, Component, Reflect, Default, Clone, Copy, Debug, Hash)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

impl GridSize {
    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
pub struct Grid {
    cells: Vec<Option<Entity>>,
    pub size: GridSize,
}

impl Grid {
    /// Creates a new tile storage that is empty.
    pub fn empty(size: GridSize) -> Self {
        Self {
            cells: vec![None; size.count()],
            size,
        }
    }

    /// Gets a tile entity for the given tile position, if an entity is associated with that tile
    /// position.
    ///
    /// Panics if the given `cell_position` does lie within the extents of the underlying tile map.
    pub fn get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        self.cells[cell_position.to_index(&self.size)]
    }

    /// Gets a tile entity for the given tile position, if:
    /// 1) the tile position lies within the underlying tile map's extents *and*
    /// 2) there is an entity associated with that tile position;
    /// otherwise it returns `None`.
    pub fn checked_get(&self, cell_position: &cell::CellPosition) -> Option<Entity> {
        if cell_position.within_map_bounds(&self.size) {
            self.cells[cell_position.to_index(&self.size)]
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
        self.cells[cell_position.to_index(&self.size)].replace(cell_entity);
    }

    /// Sets a tile entity for the given tile position, if the tile position lies within the
    /// underlying tile map's extents.
    ///
    /// If there is an entity already at that position, it will be replaced.
    pub fn checked_set(&mut self, cell_position: &cell::CellPosition, cell_entity: Entity) {
        if cell_position.within_map_bounds(&self.size) {
            self.cells[cell_position.to_index(&self.size)].replace(cell_entity);
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
        self.cells[cell_position.to_index(&self.size)].take();
    }

    /// Remove any stored entity at the given tile position, if the given `cell_position` does lie within
    /// the extents of the underlying map.
    ///
    /// Otherwise, nothing is done.
    pub fn checked_remove(&mut self, cell_position: &cell::CellPosition) {
        if cell_position.within_map_bounds(&self.size) {
            self.cells[cell_position.to_index(&self.size)].take();
        }
    }
}

#[derive(Bundle, Debug, Default, Clone)]
pub struct GridBundle {
    pub grid_size: GridSize,
    pub grid: Grid,
}

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
            .add_startup_system(spawn_cells)
            .add_system(selected_cells)
            .add_system(update_cell);
    }
}

#[derive(Component)]
struct LastUpdate(f64);

#[derive(Debug, Default, Clone)]
#[derive(Component)]
pub struct UpdateCell {
    pub color: Color,
}

fn spawn_cells(
    windows: Res<Windows>,
    grid_size: ResMut<GridSize>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<cell::CellMaterial>>,
) {
    let (width, heigth) = match windows.get_primary() {
        Some(window) => (window.width(), window.height()),
        None => (1024., 1024.),
    };

    let grid_entity = commands.spawn_empty().id();

    let mut grid = Grid::empty(grid_size.clone());

    let size = (width / grid_size.width as f32).floor();
    let left = ((width / 2.) - (size / 2.)).floor();
    let top = ((heigth / 2.) - (size / 2.)).floor();

    for (i, j) in iproduct!(0..grid_size.width, 0..grid_size.height) {
        let color = Color::ALICE_BLUE;

        let cell_position = cell::CellPosition::new(i, j);
        let handle = materials.add(cell::CellMaterial::new(color));

        let cell_id = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(cell::Cell::new(size).into()).into(),
                material: handle,
                transform: Transform::from_xyz(
                    (size * i as f32) - left,
                    (size * j as f32) - top,
                    0.,
                ),
                ..default()
            })
            .insert(Name::new(format!("Cell {} {}", i, j)))
            .id();

        grid.set(&cell_position, cell_id);
    }

    commands
        .entity(grid_entity)
        .insert(GridBundle {
            grid_size: grid_size.clone(),
            grid: grid,
        })
        .insert(LastUpdate(0.0))
        .insert(Name::new("Grid"));
}

fn selected_cells(
    time: Res<Time>,
    grid_size: ResMut<GridSize>,
    mut commands: Commands,
    mut grid_query: Query<(&mut LastUpdate, &mut Grid)>,
) {
    let x = rand::thread_rng().gen_range(0..grid_size.width as u32);
    let y = rand::thread_rng().gen_range(0..grid_size.height as u32);
    let current_time = time.elapsed_seconds_f64();
    for (mut last_update, grid) in grid_query.iter_mut() {
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
    
    for (entity, material_handle, update) in query.iter_mut(){
        let mut material = materials.get_mut(&material_handle).unwrap();

        material.color = update.color;
        let mut current_cell = commands.entity(entity);
        current_cell.remove::<UpdateCell>();
    }
}

