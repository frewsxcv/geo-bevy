use crate::line_string::LineStringMeshBuilder;
use bevy::prelude::Mesh;
use geo::geometry::{LineString, Polygon};
use geo::CoordsIter;

pub struct PolygonMesh {
    pub mesh: Mesh,
    pub exterior_mesh: Mesh,
    pub interior_meshes: Vec<Mesh>,
}

pub struct PolygonMeshBuilder<Scalar: geo::CoordFloat> {
    polygon: bevy_earcutr::PolygonMeshBuilder<Scalar>,
    exterior: LineStringMeshBuilder,
    interiors: Vec<LineStringMeshBuilder>,
}

impl<Scalar: geo::CoordFloat> Default for PolygonMeshBuilder<Scalar> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Scalar: geo::CoordFloat> PolygonMeshBuilder<Scalar> {
    pub fn new() -> Self {
        Self {
            polygon: bevy_earcutr::PolygonMeshBuilder::default(),
            exterior: LineStringMeshBuilder::default(),
            interiors: Vec::new(),
        }
    }

    pub fn add_polygon(&mut self, polygon: &Polygon<Scalar>) -> Result<(), crate::Error> {
        self.polygon
            .add_earcutr_input(Self::polygon_to_earcutr_input(polygon));
        self.exterior
            .add_line_string(polygon.exterior().0.iter().copied())?;
        for interior in polygon.interiors() {
            let mut interior_line_string_builder = LineStringMeshBuilder::default();
            interior_line_string_builder.add_line_string(interior.0.iter().copied())?;
            self.interiors.push(interior_line_string_builder);
        }
        Ok(())
    }

    fn polygon_to_earcutr_input(polygon: &Polygon<Scalar>) -> bevy_earcutr::EarcutrInput<Scalar> {
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

    fn flat_line_string_coords_2(line_string: &LineString<Scalar>, vertices: &mut Vec<Scalar>) {
        for coord in &line_string.0 {
            vertices.push(coord.x);
            vertices.push(coord.y);
        }
    }
}

impl<Scalar: geo::CoordFloat> TryFrom<PolygonMeshBuilder<Scalar>> for PolygonMesh {
    type Error = crate::Error;

    fn try_from(polygon_mesh_builder: PolygonMeshBuilder<Scalar>) -> Result<Self, Self::Error> {
        polygon_mesh_builder
            .polygon
            .build()
            .map_err(crate::Error::BevyEarcutr)
            .and_then(|polygon_mesh| {
                let exterior_mesh = Mesh::try_from(polygon_mesh_builder.exterior)?;
                let interior_meshes = polygon_mesh_builder
                    .interiors
                    .into_iter()
                    .map(Mesh::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(PolygonMesh {
                    mesh: polygon_mesh,
                    exterior_mesh,
                    interior_meshes,
                })
            })
    }
}

impl<Scalar: geo::CoordFloat> crate::build_mesh::BuildMesh<Scalar> for PolygonMeshBuilder<Scalar> {
    fn build(self) -> Result<crate::GeometryMesh<Scalar>, crate::Error> {
        Ok(crate::GeometryMesh::Polygon(PolygonMesh::try_from(self)?))
    }
}
