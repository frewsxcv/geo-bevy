use bevy::prelude::Mesh;
use std::num;

type Vertex = [f32; 3]; // [x, y, z]

#[derive(Default)]
pub struct LineStringMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_line_string(
        &mut self,
        line_string: &geo::LineString,
    ) -> Result<(), num::TryFromIntError> {
        let index_base = self.vertices.len();

        self.vertices.reserve(self.vertices.len());
        self.indices.reserve(self.indices.len() * 2);

        for (i, coord) in line_string.0.iter().enumerate() {
            self.vertices.push([coord.x as f32, coord.y as f32, 0.0f32]);
            if i != line_string.0.len() - 1 {
                self.indices.push(u32::try_from(index_base + i)?);
                self.indices.push(u32::try_from(index_base + i + 1)?);
            }
        }
        Ok(())
    }
}

impl From<LineStringMeshBuilder> for Mesh {
    fn from(line_string_builder: LineStringMeshBuilder) -> Self {
        let vertices = line_string_builder.vertices;
        let indices = line_string_builder.indices;
        let num_vertices = vertices.len();
        let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineList);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        let normals = vec![[0.0, 0.0, 0.0]; num_vertices];
        let uvs = vec![[0.0, 0.0]; num_vertices];

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }
}

impl From<LineStringMeshBuilder> for Option<Mesh> {
    fn from(line_string_mesh_builder: LineStringMeshBuilder) -> Self {
        if line_string_mesh_builder.vertices.is_empty() {
            None
        } else {
            Some(line_string_mesh_builder.into())
        }
    }
}

impl crate::build_mesh::BuildMesh for LineStringMeshBuilder {
    fn build(self) -> Option<crate::GeometryMesh> {
        Option::<Mesh>::from(self).map(|mesh| crate::GeometryMesh::LineString(mesh))
    }
}
