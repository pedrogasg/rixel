use bevy::{
    math::{UVec2, Vec2},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, Mesh, MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use crate::grid::{self, GridConfig};
use wgpu::{PrimitiveTopology, VertexFormat};
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct CellPosition {
    pub x: u32,
    pub y: u32,
}

impl CellPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn to_index(&self, grid_config: &grid::GridConfig) -> usize {
        ((self.y * grid_config.grid_width) + self.x) as usize
    }

    pub fn to_screen_position(&self, grid_config: &grid::GridConfig) -> (f32, f32) {
        let GridConfig {
            grid_width,
            grid_height,
            window_width,
            window_height,
        } = grid_config;

        let size_x = (*window_width / *grid_width) as f32;
        let size_y = (*window_height / *grid_height) as f32;
        let left = (*window_width as f32 / 2.) - (size_x / 2.);
        let top = (*window_height as f32 / 2.) - (size_y / 2.);
        //let left = (window_width / 2) as f32;
        //let top = (window_height / 2) as f32;
        (
            (size_x * self.x as f32) - left,
            0.0 - ((size_y * self.y as f32) - top),
        )
    }

    pub fn within_map_bounds(&self, grid_config: &grid::GridConfig) -> bool {
        self.x < grid_config.grid_width && self.y < grid_config.grid_height
    }
}

impl From<CellPosition> for UVec2 {
    fn from(pos: CellPosition) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}

impl From<&CellPosition> for UVec2 {
    fn from(pos: &CellPosition) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}

impl From<UVec2> for CellPosition {
    fn from(v: UVec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<CellPosition> for Vec2 {
    fn from(pos: CellPosition) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl From<&CellPosition> for Vec2 {
    fn from(pos: &CellPosition) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    x_size: f32,
    y_size: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x_size: 0.5,
            y_size: 0.5,
        }
    }
}

impl Cell {
    pub fn new(x_size: f32, y_size: f32) -> Self {
        Self { x_size, y_size }
    }

    pub const ATTRIBUTE_PROPS: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Props", 3, VertexFormat::Float32x2);
}

impl From<Cell> for Mesh {
    fn from(cell: Cell) -> Self {
        let Cell { x_size, y_size } = cell;
        let x_radius = x_size / 2.0 as f32;
        let y_radius = y_size / 2.0 as f32;
        let positions = vec![
            [0.0, 0.0, 0.0],
            [x_radius, y_radius, 0.0],
            [x_radius, -y_radius, 0.0],
            [-x_radius, -y_radius, 0.0],
            [-x_radius, y_radius, 0.0],
        ];
        let normals = vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ];

        let uvs = vec![
            [0.0, 0.0],
            [1.0, 1.0],
            [1.0, -1.0],
            [-1.0, -1.0],
            [-1.0, 1.0],
        ];

        let properties = vec![
            [0.0, 0.0],
            [-1.0, -1.0],
            [-1.0, 1.0],
            [1.0, 1.0],
            [1.0, -1.0],
        ];

        let indices = vec![1, 4, 0, 4, 3, 0, 3, 2, 0, 2, 1, 0];
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Cell::ATTRIBUTE_PROPS, properties);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CellMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl CellMaterial {
    /// Creates a regular polygon in the `XY` plane
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overriden.
impl Material2d for CellMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
