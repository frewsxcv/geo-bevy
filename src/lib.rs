use bevy::prelude::{info_span, Mesh};
use build_mesh::BuildMesh;
use geo_traits::*;
use line_string::LineStringMeshBuilder;
use polygon::PolygonMeshBuilder;
use std::iter;

pub use point::SpritePosition;
pub use polygon::PolygonMesh;

mod build_mesh;
mod line_string;
mod point;
mod polygon;

pub fn line_to_mesh(line: impl LineTrait) -> Result<Mesh, Error> {
    let mut mesh_builder = LineStringMeshBuilder::new();
    mesh_builder.add_coords(iter::once(line.start()).chain(iter::once(line.end())))?;
    mesh_builder.try_into()
}

pub fn line_string_to_mesh(line_string: impl LineStringTrait) -> Result<Mesh, Error> {
    let mut mesh_builder = LineStringMeshBuilder::new();
    mesh_builder.add_coords(line_string.coords())?;
    mesh_builder.try_into()
}

pub fn multi_line_string_to_mesh(
    multi_line_string: impl MultiLineStringTrait,
) -> Result<Vec<Mesh>, Error> {
    let mut line_string_meshes = Vec::with_capacity(multi_line_string.num_line_strings());

    for line_string in multi_line_string.line_strings() {
        line_string_meshes.push(line_string_to_mesh(line_string)?);
    }

    Ok(line_string_meshes)
}

pub fn polygon_to_mesh<Scalar: geo_types::CoordFloat>(
    polygon: impl PolygonTrait<T = Scalar>,
) -> Result<PolygonMesh, Error> {
    let mut mesh_builder = PolygonMeshBuilder::new();
    mesh_builder.add_polygon(&polygon)?;
    mesh_builder.try_into()
}

pub fn multi_polygon_to_mesh<Scalar: geo_types::CoordFloat>(
    multi_polygon: impl MultiPolygonTrait<T = Scalar>,
) -> Result<Vec<PolygonMesh>, Error> {
    let polygons = multi_polygon.polygons();
    let mut polygon_meshes = Vec::with_capacity(polygons.len());
    for polygon in polygons {
        polygon_meshes.push(polygon_to_mesh(polygon)?);
    }

    Ok(polygon_meshes)
}

pub fn rect_to_mesh<Scalar: geo_types::CoordFloat>(
    rect: impl RectTrait<T = Scalar>,
) -> Result<PolygonMesh, Error> {
    let mut mesh_builder = PolygonMeshBuilder::default();
    mesh_builder.add_polygon_from_exterior_coords(
        [
            (rect.min().x(), rect.min().y()),
            (rect.min().x(), rect.max().y()),
            (rect.max().x(), rect.max().y()),
            (rect.max().x(), rect.min().y()),
            (rect.min().x(), rect.min().y()),
        ]
        .into_iter(),
    )?;
    PolygonMesh::try_from(mesh_builder)
}

pub fn triangle_to_mesh<Scalar: geo_types::CoordFloat>(
    triangle: impl TriangleTrait<T = Scalar>,
) -> Result<PolygonMesh, Error> {
    let mut mesh_builder = PolygonMeshBuilder::default();
    mesh_builder.add_polygon_from_exterior_coords(
        [
            (triangle.first().x(), triangle.first().y()),
            (triangle.second().x(), triangle.second().y()),
            (triangle.third().x(), triangle.third().y()),
            (triangle.first().x(), triangle.first().y()),
        ]
        .into_iter(),
    )?;
    PolygonMesh::try_from(mesh_builder)
}

pub fn geometry_to_mesh<Scalar: geo_types::CoordFloat>(
    geometry: impl GeometryTrait<T = Scalar>,
) -> Result<GeometryMesh, Error> {
    let mut ctx = build_mesh::BuildBevyMeshesContext::default();

    info_span!("Populating Bevy mesh builder")
        .in_scope(|| build_mesh::populate_geometry_mesh_builders(&geometry, &mut ctx))?;

    info_span!("Building Bevy meshes").in_scope(|| {
        [
            ctx.point_mesh_builder.build(),
            ctx.line_string_mesh_builder.build(),
            ctx.polygon_mesh_builder.build(),
        ]
        .into_iter()
        .find(|prepared_mesh| prepared_mesh.is_ok())
        .unwrap_or(Err(Error::CouldNotBuildMesh))
    })
}

pub enum GeometryMesh {
    Point(Vec<SpritePosition>),
    LineString(Mesh),
    Polygon(polygon::PolygonMesh),
}

#[derive(Debug)]
pub enum Error {
    CouldNotBuildMesh,
    CouldNotConvertToF32,
    EmptyGeometry,
    BevyEarcutr(bevy_earcutr::Error),
}
