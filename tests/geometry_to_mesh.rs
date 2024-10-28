use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};
use geo_bevy::*;
use geo_types::geometry::*;

const P_0: [f64; 2] = [0., 0.];
const P_1: [f64; 2] = [1., 0.];
const P_2: [f64; 2] = [1., 1.];
const P_3: [f64; 2] = [0., 1.];

const I_0: [f64; 2] = [0.25, 0.25];
const I_1: [f64; 2] = [0.75, 0.25];
const I_2: [f64; 2] = [0.75, 0.75];
const I_3: [f64; 2] = [0.25, 0.75];

#[test]
pub fn builds_mesh_from_line() {
    let mesh = line_to_mesh(&Line::new(P_0, P_1)).expect("Vertices.");
    assert_eq!([P_0, P_1].as_slice(), mesh_to_indices(&mesh));
}

pub fn builds_no_mesh_from_emty_line() {
    let line = Line::new(P_0, P_0);
    assert!(line_to_mesh(&line).is_err())
}

#[test]
pub fn builds_mesh_from_line_string() {
    let indices = vec![P_0, P_1, P_2];
    let mesh = line_string_to_mesh(&LineString::new(
        indices.clone().into_iter().map(Coord::from).collect(),
    ))
    .expect("Vertices.");
    assert_eq!(indices, mesh_to_indices(&mesh));
}

#[test]
pub fn builds_mesh_from_multi_line_string() {
    let indices = vec![P_0, P_1, P_2];
    let meshes = multi_line_string_to_mesh(&MultiLineString::new(vec![indices.clone().into()]));
    assert_eq!(indices, mesh_to_indices(meshes.unwrap().first().unwrap()));
}

#[test]
pub fn builds_mesh_from_polygon() {
    let exterior_ring = vec![P_0, P_1, P_2, P_3];
    let interior_ring = vec![I_0, I_1, I_2, I_3];

    let PolygonMesh {
        mesh,
        exterior_mesh,
        interior_meshes,
    } = polygon_to_mesh(&Polygon::new(
        exterior_ring.clone().into(),
        vec![interior_ring.clone().into()],
    ))
    .expect("Vertices");

    assert_eq!(exterior_ring, mesh_to_indices(&mesh)[0..4]);
    assert_eq!(exterior_ring, mesh_to_indices(&exterior_mesh));
    assert_eq!(interior_ring, mesh_to_indices(&interior_meshes[0]));
}

#[test]
pub fn builds_mesh_from_multi_polygon() {
    let exterior_ring = vec![P_0, P_1, P_2, P_3];

    let polygon_meshes = multi_polygon_to_mesh(&MultiPolygon::new(vec![Polygon::new(
        exterior_ring.clone().into(),
        vec![],
    )]));

    assert_eq!(
        exterior_ring,
        mesh_to_indices(&polygon_meshes.unwrap().first().unwrap().exterior_mesh)
    );
}

#[test]
pub fn builds_mesh_from_rect() {
    let PolygonMesh { mesh, .. } = rect_to_mesh(&Rect::new(P_0, P_2)).expect("Vertices");
    assert_eq!([P_0, P_3, P_2, P_1].as_slice(), mesh_to_indices(&mesh))
}

#[test]
pub fn builds_mesh_from_triangle() {
    let indices = [P_0, P_1, P_3];
    let PolygonMesh { mesh, .. } = triangle_to_mesh(Triangle::new(
        indices[0].into(),
        indices[1].into(),
        indices[2].into(),
    ))
    .expect("Vertices");
    assert_eq!(indices.as_slice(), mesh_to_indices(&mesh))
}

fn mesh_to_indices(mesh: &Mesh) -> Vec<[f64; 2]> {
    let VertexAttributeValues::Float32x3(vertices) = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("Populated vertices")
    else {
        panic!("Expected f32 vertices.")
    };

    let mut indices = vertices
        .iter()
        .map(|vec3| [vec3[0] as f64, vec3[1] as f64])
        .collect::<Vec<[f64; 2]>>();

    if indices.first().unwrap() == indices.last().unwrap() {
        indices.pop();
    }
    indices
}
