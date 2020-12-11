use std::fmt::{self, Display, Formatter};

use crate::vector::Vec2;

pub(crate) enum PathSegment {
    MoveTo(Vec2),
    LineTo(Vec2),
    Arc {
        radius: Vec2,
        x_axis_rotation: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
        end: Vec2,
    },
    Close,
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PathSegment::MoveTo(Vec2 { x, y }) => write!(f, "M {} {}", x, y),
            PathSegment::LineTo(Vec2 { x, y }) => write!(f, "L {} {}", x, y),
            PathSegment::Arc {
                radius: Vec2 { x: rx, y: ry },
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                end: Vec2 { x, y },
            } => {
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

struct PathData(Vec<PathSegment>);

impl std::fmt::Display for PathData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for segment in &self.0 {
            write!(f, "{} ", segment)?;
        }
        std::fmt::Result::Ok(())
    }
}
