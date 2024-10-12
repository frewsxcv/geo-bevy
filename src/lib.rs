use bevy::prelude::{info_span, Mesh};
use build_mesh::{BuildBevyMeshes, BuildMesh};
use geo::geometry::*;
use line_string::LineStringMeshBuilder;
use polygon::PolygonMeshBuilder;

pub use polygon::PolygonMesh;

mod build_mesh;
mod line_string;
mod point;
mod polygon;

pub fn line_to_mesh(line: &Line) -> Result<Mesh, Error> {
    line_string_to_mesh(&line.into())
}

pub fn line_string_to_mesh(line_string: &LineString) -> Result<Mesh, Error> {
    let mut mesh_builder = LineStringMeshBuilder::default();
    mesh_builder.add_line_string(line_string.coords().copied())?;
    mesh_builder.try_into()
}

pub fn multi_line_string_to_mesh(multi_line_string: &MultiLineString) -> Result<Vec<Mesh>, Error> {
    let line_strings = &multi_line_string.0;
    let mut line_string_meshes = Vec::with_capacity(line_strings.len());

    for line_string in line_strings {
        line_string_meshes.push(line_string_to_mesh(line_string)?);
    }

    Ok(line_string_meshes)
}

pub fn polygon_to_mesh<Scalar: geo::CoordFloat>(
    polygon: &Polygon<Scalar>,
) -> Result<PolygonMesh, Error> {
    let mut mesh_builder = PolygonMeshBuilder::default();
    mesh_builder.add_polygon(polygon)?;
    PolygonMesh::try_from(mesh_builder)
}

pub fn multi_polygon_to_mesh(multi_polygon: &MultiPolygon) -> Result<Vec<PolygonMesh>, Error> {
    let polygons = &multi_polygon.0;
    let mut polygon_meshes = Vec::with_capacity(polygons.len());
    for polygon in polygons {
        polygon_meshes.push(polygon_to_mesh(polygon)?);
    }

    Ok(polygon_meshes)
}

pub fn rect_to_mesh<Scalar: geo::CoordFloat>(rect: &Rect<Scalar>) -> Result<PolygonMesh, Error> {
    polygon_to_mesh(&rect.to_polygon())
}

pub fn triangle_to_mesh<Scalar: geo::CoordFloat>(
    triangle: &Triangle<Scalar>,
) -> Result<PolygonMesh, Error> {
    polygon_to_mesh(&triangle.to_polygon())
}

pub fn geometry_to_mesh<Scalar: geo::CoordFloat>(
    geometry: &Geometry<Scalar>,
) -> Result<GeometryMesh<Scalar>, Error> {
    let mut ctx = build_mesh::BuildBevyMeshesContext::default();

    info_span!("Populating Bevy mesh builder")
        .in_scope(|| geometry.populate_mesh_builders(&mut ctx))?;

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

pub fn geometry_collection_to_mesh<Scalar: geo::CoordFloat>(
    geometry_collection: &GeometryCollection<Scalar>,
) -> Result<Vec<GeometryMesh<Scalar>>, Error> {
    let mut geometry_meshes = Vec::with_capacity(geometry_collection.len());
    for geometry in geometry_collection {
        geometry_meshes.push(geometry_to_mesh(geometry)?);
    }

    Ok(geometry_meshes)
}

pub enum GeometryMesh<Scalar: geo::CoordFloat> {
    Point(Vec<Point<Scalar>>),
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
