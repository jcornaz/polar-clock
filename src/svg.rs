use std::fmt::{self, Display, Formatter};

use crate::vector::Vec2;

pub(crate) struct PathData(Vec<PathSegment>);

impl PathData {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn move_to(&mut self, point: Vec2) -> &mut Self {
        self.0.push(PathSegment::MoveTo(point));
        self
    }

    pub fn line_to(&mut self, point: Vec2) -> &mut Self {
        self.0.push(PathSegment::LineTo(point));
        self
    }

    pub fn arc_to(&mut self, point: Vec2, def: ArcDef) -> &mut Self {
        self.0.push(PathSegment::Arc(point, def));
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.0.push(PathSegment::Close);
        self
    }
}

impl std::fmt::Display for PathData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for segment in &self.0 {
            write!(f, "{} ", segment)?;
        }
        std::fmt::Result::Ok(())
    }
}

pub(crate) struct ArcDef {
    pub radius: Vec2,
    pub x_axis_rotation: f32,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
}

enum PathSegment {
    MoveTo(Vec2),
    LineTo(Vec2),
    Arc(Vec2, ArcDef),
    Close,
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PathSegment::MoveTo(Vec2 { x, y }) => write!(f, "M {} {}", x, y),
            PathSegment::LineTo(Vec2 { x, y }) => write!(f, "L {} {}", x, y),
            PathSegment::Arc(
                Vec2 { x, y },
                ArcDef {
                    radius: Vec2 { x: rx, y: ry },
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                },
            ) => {
                write!(
                    f,
                    "A {} {} {} {} {} {} {}",
                    rx,
                    ry,
                    x_axis_rotation,
                    if *large_arc_flag { 1 } else { 0 },
                    if *sweep_flag { 1 } else { 0 },
                    x,
                    y
                )
            }
            PathSegment::Close => write!(f, "Z"),
        }
    }
}
