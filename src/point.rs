pub struct PointMeshBuilder<Scalar: geo::CoordFloat> {
    points: Vec<geo::Point<Scalar>>,
}

impl<Scalar: geo::CoordFloat> Default for PointMeshBuilder<Scalar> {
    fn default() -> Self {
        Self { points: Vec::new() }
    }
}

impl<Scalar: geo::CoordFloat> PointMeshBuilder<Scalar> {
    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_point(&mut self, point: &geo::Point<Scalar>) {
        self.points.push(*point);
    }
}

impl<Scalar: geo::CoordFloat> crate::build_mesh::BuildMesh<Scalar> for PointMeshBuilder<Scalar> {
    fn build(self) -> Result<crate::GeometryMesh<Scalar>, crate::Error> {
        if self.points.is_empty() {
            Err(crate::Error::EmptyGeometry)
        } else {
            Ok(crate::GeometryMesh::Point(self.points))
        }
    }
}
