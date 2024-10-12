use bevy::prelude::Mesh;

type Vertex = [f32; 3]; // [x, y, z]

#[derive(Default)]
pub struct LineStringMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    /// Logs error if self.vertices.len() + linestring coords > u32::MAX (4_294_967_295).
    pub fn add_line_string<Scalar, I>(&mut self, line_string_iter: I) -> Result<(), crate::Error>
    where
        Scalar: geo::CoordFloat,
        I: Iterator<Item = geo::Coord<Scalar>>,
    {
        let index_base = self.vertices.len();

        // Reserve space for vertices and indices
        self.vertices.reserve(self.vertices.len());
        self.indices.reserve(self.indices.len() * 2);

        let mut last_index = None;
        for (i, coord) in line_string_iter.enumerate() {
            self.vertices.push([
                coord.x.to_f32().ok_or(crate::Error::CouldNotConvertToF32)?,
                coord.y.to_f32().ok_or(crate::Error::CouldNotConvertToF32)?,
                0.0,
            ]);

            if let Some(last) = last_index {
                self.indices.push(last as u32);
                self.indices.push((index_base + i) as u32);
            }
            last_index = Some(index_base + i);
        }

        Ok(())
    }
}

impl TryFrom<LineStringMeshBuilder> for Mesh {
    type Error = crate::Error;

    fn try_from(line_string_mesh_builder: LineStringMeshBuilder) -> Result<Self, Self::Error> {
        if line_string_mesh_builder.vertices.is_empty() {
            Err(crate::Error::EmptyGeometry)
        } else {
            let vertices = line_string_mesh_builder.vertices;
            let indices = line_string_mesh_builder.indices;
            let num_vertices = vertices.len();
            let mut mesh = Mesh::new(
                bevy::render::render_resource::PrimitiveTopology::LineList,
                Default::default(),
            );
            mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            let normals = vec![[0.0, 0.0, 0.0]; num_vertices];
            let uvs = vec![[0.0, 0.0]; num_vertices];

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

            Ok(mesh)
        }
    }
}

impl<Scalar: geo::CoordFloat> crate::build_mesh::BuildMesh<Scalar> for LineStringMeshBuilder {
    fn build(self) -> Result<crate::GeometryMesh<Scalar>, crate::Error> {
        Ok(crate::GeometryMesh::LineString(self.try_into()?))
    }
}
