#[derive(Default)]
pub struct PointMeshBuilder {
    points: Vec<geo::Point>,
}

impl PointMeshBuilder {
    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_point(&mut self, point: &geo::Point) {
        self.points.push(*point);
    }
}

impl crate::build_mesh::BuildMesh for PointMeshBuilder {
    fn build(self) -> Option<crate::GeometryMesh> {
        if self.points.is_empty() {
            None
        } else {
            Some(crate::GeometryMesh::Point(self.points))
        }
    }
}
