use crate::{line_string::LineStringMeshBuilder, PreparedMesh};
use geo::algorithm::coords_iter::CoordsIter;

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
            .add_earcutr_input(Self::polygon_to_earcutr_input(polygon));
        self.exterior.add_line_string(polygon.exterior())?;
        for interior in polygon.interiors() {
            let mut interior_line_string_builder = LineStringMeshBuilder::default();
            interior_line_string_builder.add_line_string(interior)?;
            self.interiors.push(interior_line_string_builder);
        }

        Ok(())
    }

    fn polygon_to_earcutr_input(polygon: &geo::Polygon) -> bevy_earcutr::EarcutrInput {
        let mut vertices = Vec::with_capacity(polygon.coords_count() * 2);
        let mut interior_indices = Vec::with_capacity(polygon.interiors().len());
        debug_assert!(polygon.exterior().0.len() >= 4);

        Self::flat_line_string_coords_2(polygon.exterior(), &mut vertices);

        for interior in polygon.interiors() {
            debug_assert!(interior.0.len() >= 4);
            interior_indices.push(vertices.len() / 2);
            Self::flat_line_string_coords_2(interior, &mut vertices);
        }

        bevy_earcutr::EarcutrInput {
            vertices,
            interior_indices,
        }
    }

    fn flat_line_string_coords_2(line_string: &geo::LineString, vertices: &mut Vec<f64>) {
        for coord in &line_string.0 {
            vertices.push(coord.x);
            vertices.push(coord.y);
        }
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
