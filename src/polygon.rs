use crate::PreparedMesh;
use bevy_render::prelude::*;
use geo::TriangulateEarcut;

#[derive(Default)]
pub struct PolygonMeshBuilder {
    pub polygons: Vec<geo::Polygon>,
}

#[derive(Default)]
pub struct EarcutrResult {
    pub vertices: Vec<f64>,
    pub triangle_indices: Vec<usize>,
}

impl EarcutrResult {
    fn merge(&mut self, mut other: EarcutrResult) {
        let base_triangle_index = self.vertices.len() / 2;
        for other_triangle_index in other.triangle_indices {
            self.triangle_indices
                .push(other_triangle_index + base_triangle_index);
        }
        self.vertices.append(&mut other.vertices);
    }
}

impl PolygonMeshBuilder {
    pub fn build(self, color: Color) -> Option<PreparedMesh> {
        let mut earcutr_result = EarcutrResult::default();
        for polygon in self.polygons {
            let outcome = polygon.triangulate_earcut_vertexes();
            earcutr_result.merge(EarcutrResult {
                vertices: outcome.vertexes,
                triangle_indices: outcome.triangle_indexes,
            });
        }
        let mesh = build_mesh_from_earcutr(earcutr_result, 0.);
        // build_mesh_from_earcutr(earcutr_result, z_index)
        Some(crate::PreparedMesh::Polygon { mesh, color })
    }
}

pub fn build_mesh_from_earcutr(earcutr_result: EarcutrResult, z_index: f32) -> Mesh {
    let indices = earcutr_result
        .triangle_indices
        .into_iter()
        .map(|n| u32::try_from(n).unwrap())
        .collect::<Vec<_>>();
    let vertices = earcutr_result
        .vertices
        .chunks(2)
        .map(|n| [n[0] as f32, n[1] as f32, z_index])
        .collect::<Vec<_>>();
    crate::build_mesh_from_vertices(
        bevy_render::render_resource::PrimitiveTopology::TriangleList,
        vertices,
        indices,
    )
}
