use crate::{line_string::LineStringMeshBuilder, PreparedMesh};

#[derive(Default)]
pub struct PolygonMeshBuilder {
    polygon: bevy_earcutr::PolygonMeshBuilder,
    exterior: LineStringMeshBuilder,
    interiors: Vec<LineStringMeshBuilder>,
}

impl PolygonMeshBuilder {
    pub fn add_polygon_components(
        &mut self,
        polygon: &geo::Polygon,
    ) -> Result<(), std::num::TryFromIntError> {
        self.polygon
            .add_earcutr_input(crate::polygon_to_earcutr_input(polygon));
        self.exterior.add_line_string(polygon.exterior())?;
        for interior in polygon.interiors() {
            let mut interior_line_string_builder = LineStringMeshBuilder::default();
            interior_line_string_builder.add_line_string(interior)?;
            self.interiors.push(interior_line_string_builder);
        }

        Ok(())
    }
}

impl crate::BuildMesh for PolygonMeshBuilder {
    fn build(self) -> Option<PreparedMesh> {
        self.polygon
            .build()
            .map(|polygon_mesh| PreparedMesh::Polygon {
                polygon_mesh,
                exterior_mesh: self.exterior.into(),
                interior_meshes: self
                    .interiors
                    .into_iter()
                    .map(|interior_builder| interior_builder.into())
                    .collect(),
            })
    }
}
