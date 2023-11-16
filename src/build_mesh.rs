use geo::geometry::*;

pub trait BuildMesh {
    fn build(self) -> Option<crate::GeometryMesh>;
}

#[derive(Default)]
pub struct BuildBevyMeshesContext {
    pub point_mesh_builder: crate::point::PointMeshBuilder,
    pub line_string_mesh_builder: crate::line_string::LineStringMeshBuilder,
    pub polygon_mesh_builder: crate::polygon::PolygonMeshBuilder,
}

pub trait BuildBevyMeshes {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext);
}

impl BuildBevyMeshes for Point {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.point_mesh_builder.add_point(self);
    }
}

impl BuildBevyMeshes for LineString {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.line_string_mesh_builder.add_line_string(self);
    }
}

impl BuildBevyMeshes for Polygon {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        ctx.polygon_mesh_builder.add_polygon(self);
    }
}

impl BuildBevyMeshes for MultiPoint {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for point in &self.0 {
            point.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for MultiLineString {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for line_string in &self.0 {
            line_string.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for MultiPolygon {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for polygon in &self.0 {
            polygon.populate_mesh_builders(ctx);
        }
    }
}

impl BuildBevyMeshes for Line {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        LineString::from(self).populate_mesh_builders(ctx);
    }
}

impl BuildBevyMeshes for Triangle {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        self.to_polygon().populate_mesh_builders(ctx);
    }
}

impl BuildBevyMeshes for Rect {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        self.to_polygon().populate_mesh_builders(ctx);
    }
}

impl BuildBevyMeshes for Geometry {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        match self {
            Geometry::Point(g) => g.populate_mesh_builders(ctx),
            Geometry::Line(g) => g.populate_mesh_builders(ctx),
            Geometry::LineString(g) => g.populate_mesh_builders(ctx),
            Geometry::Polygon(g) => g.populate_mesh_builders(ctx),
            Geometry::MultiPoint(g) => g.populate_mesh_builders(ctx),
            Geometry::MultiLineString(g) => g.populate_mesh_builders(ctx),
            Geometry::MultiPolygon(g) => g.populate_mesh_builders(ctx),
            Geometry::GeometryCollection(g) => g.populate_mesh_builders(ctx),
            Geometry::Triangle(g) => g.populate_mesh_builders(ctx),
            Geometry::Rect(g) => g.populate_mesh_builders(ctx),
        };
    }
}

impl BuildBevyMeshes for GeometryCollection {
    fn populate_mesh_builders(&self, ctx: &mut BuildBevyMeshesContext) {
        for g in self {
            g.populate_mesh_builders(ctx);
        }
    }
}
