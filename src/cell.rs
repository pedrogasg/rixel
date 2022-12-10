use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, Mesh, MeshVertexBufferLayout, MeshVertexAttribute},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use wgpu::{PrimitiveTopology, VertexFormat};

/// A regular polygon in the `XY` plane
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    size: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self { size: 0.5 }
    }
}

impl Cell {
    /// Creates a regular polygon in the `XY` plane
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub const ATTRIBUTE_PROPS: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Props", 3, VertexFormat::Float32x2);
}

impl From<Cell> for Mesh {
    fn from(cell: Cell) -> Self {
        let Cell { size } = cell;
        let radius = size / 2.0 as f32;
        let positions = vec![
            [0.0, 0.0, 0.0],
            [radius, radius, 0.0],
            [radius, -radius, 0.0],
            [-radius, -radius, 0.0],
            [-radius, radius, 0.0],
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
    color: Color,
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