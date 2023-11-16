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

pub fn line_to_mesh(line: &Line) -> Option<Mesh> {
    line_string_to_mesh(&line.into())
}

pub fn line_string_to_mesh(line_string: &LineString) -> Option<Mesh> {
    let mut mesh_builder = LineStringMeshBuilder::default();
    mesh_builder.add_line_string(line_string);
    mesh_builder.into()
}

pub fn multi_line_string_to_mesh(multi_line_string: &MultiLineString) -> Vec<Mesh> {
    let line_strings = &multi_line_string.0;
    let mut line_string_meshes = Vec::with_capacity(line_strings.len());

    for line_string in line_strings {
        if let Some(line_string_mesh) = line_string_to_mesh(line_string) {
            line_string_meshes.push(line_string_mesh);
        }
    }

    line_string_meshes
}

pub fn polygon_to_mesh(polygon: &Polygon) -> Option<PolygonMesh> {
    let mut mesh_builder = PolygonMeshBuilder::default();
    mesh_builder.add_polygon(polygon);
    mesh_builder.into()
}

pub fn multi_polygon_to_mesh(multi_polygon: &MultiPolygon) -> Vec<PolygonMesh> {
    let polygons = &multi_polygon.0;
    let mut polygon_meshes = Vec::with_capacity(polygons.len());
    for polygon in polygons {
        if let Some(polygon_mesh) = polygon_to_mesh(polygon) {
            polygon_meshes.push(polygon_mesh);
        }
    }

    polygon_meshes
}

pub fn rect_to_mesh(rect: &Rect) -> Option<PolygonMesh> {
    polygon_to_mesh(&rect.to_polygon())
}

pub fn triangle_to_mesh(triangle: &Triangle) -> Option<PolygonMesh> {
    polygon_to_mesh(&triangle.to_polygon())
}

pub fn geometry_to_mesh(geometry: &Geometry) -> Option<GeometryMesh> {
    let mut ctx = build_mesh::BuildBevyMeshesContext::default();

    info_span!("Populating Bevy mesh builder")
        .in_scope(|| geometry.populate_mesh_builders(&mut ctx));

    info_span!("Building Bevy meshes").in_scope(|| {
        [
            ctx.point_mesh_builder.build(),
            ctx.line_string_mesh_builder.build(),
            ctx.polygon_mesh_builder.build(),
        ]
        .into_iter()
        .find(|prepared_mesh| prepared_mesh.is_some())
        .unwrap_or_default()
    })
}

pub fn geometry_collection_to_mesh(geometry_collection: &GeometryCollection) -> Vec<GeometryMesh> {
    let mut geometry_meshes = Vec::with_capacity(geometry_collection.len());
    for geometry in geometry_collection {
        if let Some(geometry_mesh) = geometry_to_mesh(geometry) {
            geometry_meshes.push(geometry_mesh);
        }
    }

    geometry_meshes
}

pub enum GeometryMesh {
    Point(Vec<Point>),
    LineString(Mesh),
    Polygon(polygon::PolygonMesh),
}
