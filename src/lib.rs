use bevy::prelude::*;
use geo::algorithm::coords_iter::CoordsIter;
use std::num::TryFromIntError;

mod line_string;
mod point;
mod polygon;

pub enum PreparedMesh {
    Point(Vec<geo::Point>),
    LineString {
        mesh: Mesh,
    },
    Polygon {
        polygon_mesh: Mesh,
        exterior_mesh: Mesh,
        interior_meshes: Vec<Mesh>,
    },
}

trait BuildMesh {
    fn build(self) -> Option<PreparedMesh>;
}

#[derive(Default)]
pub struct BuildBevyMeshesContext {
    point_mesh_builder: point::PointMeshBuilder,
    line_string_mesh_builder: line_string::LineStringMeshBuilder,
    polygon_mesh_builder: polygon::PolygonMeshBuilder,
}

pub fn build_bevy_meshes<G: BuildBevyMeshes>(
    geo: &G,
) -> Result<impl Iterator<Item = PreparedMesh>, TryFromIntError> {
    let mut ctx = BuildBevyMeshesContext::default();

    info_span!("Populating Bevy mesh builder").in_scope(|| geo.populate_mesh_builders(&mut ctx))?;

    info_span!("Building Bevy meshes").in_scope(|| {
        Ok([
            ctx.point_mesh_builder.build(),
            ctx.line_string_mesh_builder.build(),
            ctx.polygon_mesh_builder.build(),
        ]
        .into_iter()
        .flatten())
    })
}

pub trait BuildBevyMeshes {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError>;
}

impl BuildBevyMeshes for geo::Point {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        ctx.point_mesh_builder.add_point(self)
    }
}

impl BuildBevyMeshes for geo::LineString {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        ctx.line_string_mesh_builder.add_line_string(self)
    }
}

impl BuildBevyMeshes for geo::Polygon {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        ctx.polygon_mesh_builder.add_polygon_components(self)
    }
}

impl BuildBevyMeshes for geo::MultiPoint {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        for point in &self.0 {
            point.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::MultiLineString {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        for line_string in &self.0 {
            line_string.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::MultiPolygon {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        for polygon in &self.0 {
            polygon.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl BuildBevyMeshes for geo::Line {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        geo::LineString::new(vec![self.start, self.end]).populate_mesh_builders(ctx)
    }
}

impl BuildBevyMeshes for geo::Triangle {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        self.to_polygon().populate_mesh_builders(ctx)
    }
}

impl BuildBevyMeshes for geo::Rect {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        self.to_polygon().populate_mesh_builders(ctx)
    }
}

impl BuildBevyMeshes for geo::Geometry {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        match self {
            geo::Geometry::Point(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::Line(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::LineString(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::Polygon(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::MultiPoint(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::MultiLineString(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::MultiPolygon(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::GeometryCollection(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::Triangle(g) => g.populate_mesh_builders(ctx)?,
            geo::Geometry::Rect(g) => g.populate_mesh_builders(ctx)?,
        };
        Ok(())
    }
}

impl BuildBevyMeshes for geo::GeometryCollection {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext,
    ) -> Result<(), TryFromIntError> {
        for g in self {
            g.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

fn polygon_to_earcutr_input(polygon: &geo::Polygon) -> bevy_earcutr::EarcutrInput {
    let mut vertices = Vec::with_capacity(polygon.coords_count() * 2);
    let mut interior_indices = Vec::with_capacity(polygon.interiors().len());
    debug_assert!(polygon.exterior().0.len() >= 4);

    flat_line_string_coords_2(polygon.exterior(), &mut vertices);

    for interior in polygon.interiors() {
        debug_assert!(interior.0.len() >= 4);
        interior_indices.push(vertices.len() / 2);
        flat_line_string_coords_2(interior, &mut vertices);
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
