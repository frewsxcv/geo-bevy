use geo::geometry::*;

pub trait BuildMesh<Scalar: geo::CoordFloat> {
    fn build(self) -> Result<crate::GeometryMesh<Scalar>, crate::Error>;
}

pub struct BuildBevyMeshesContext<Scalar: geo::CoordFloat> {
    pub point_mesh_builder: crate::point::PointMeshBuilder<Scalar>,
    pub line_string_mesh_builder: crate::line_string::LineStringMeshBuilder,
    pub polygon_mesh_builder: crate::polygon::PolygonMeshBuilder<Scalar>,
}

impl<Scalar: geo::CoordFloat> Default for BuildBevyMeshesContext<Scalar> {
    fn default() -> Self {
        Self {
            point_mesh_builder: Default::default(),
            line_string_mesh_builder: Default::default(),
            polygon_mesh_builder: Default::default(),
        }
    }
}

pub trait BuildBevyMeshes<Scalar: geo::CoordFloat> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error>;
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Point<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        ctx.point_mesh_builder.add_point(self);
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for LineString<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        ctx.line_string_mesh_builder
            .add_line_string(self.coords().copied())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Polygon<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        ctx.polygon_mesh_builder.add_polygon(self)
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for MultiPoint<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        for point in &self.0 {
            point.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for MultiLineString<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        for line_string in &self.0 {
            line_string.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for MultiPolygon<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        for polygon in &self.0 {
            polygon.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Line<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        LineString::from(self).populate_mesh_builders(ctx)?;
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Triangle<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        self.to_polygon().populate_mesh_builders(ctx)?;
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Rect<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        self.to_polygon().populate_mesh_builders(ctx)?;
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for Geometry<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        match self {
            Geometry::Point(g) => g.populate_mesh_builders(ctx)?,
            Geometry::Line(g) => g.populate_mesh_builders(ctx)?,
            Geometry::LineString(g) => g.populate_mesh_builders(ctx)?,
            Geometry::Polygon(g) => g.populate_mesh_builders(ctx)?,
            Geometry::MultiPoint(g) => g.populate_mesh_builders(ctx)?,
            Geometry::MultiLineString(g) => g.populate_mesh_builders(ctx)?,
            Geometry::MultiPolygon(g) => g.populate_mesh_builders(ctx)?,
            Geometry::GeometryCollection(g) => g.populate_mesh_builders(ctx)?,
            Geometry::Triangle(g) => g.populate_mesh_builders(ctx)?,
            Geometry::Rect(g) => g.populate_mesh_builders(ctx)?,
        };
        Ok(())
    }
}

impl<Scalar: geo::CoordFloat> BuildBevyMeshes<Scalar> for GeometryCollection<Scalar> {
    fn populate_mesh_builders(
        &self,
        ctx: &mut BuildBevyMeshesContext<Scalar>,
    ) -> Result<(), crate::Error> {
        for g in self {
            g.populate_mesh_builders(ctx)?;
        }
        Ok(())
    }
}
