use std::f32::consts::PI;
use std::iter::FromIterator;

use yew::prelude::*;

use crate::svg::*;
use crate::vector::*;

const ARC_WIDTH: f32 = 20.0;
const ARC_HALF_WIDTH: f32 = ARC_WIDTH / 2.0;

pub(crate) struct TimeArk;

impl Component for TimeArk {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg width="1000" height="1000">
                <Arc center=Vec2 { x: 500.0, y: 500.0 } radius=100.0 color="black" progress=0.66 />
            </svg>
        }
    }
}

struct Arc {
    props: ArcProp,
}

#[derive(Properties, Clone, PartialEq)]
struct ArcProp {
    center: Vec2,
    radius: f32,
    color: String,
    progress: f32,
}

impl Component for Arc {
    type Message = ();
    type Properties = ArcProp;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props == self.props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let angle = 2.0 * PI * self.props.progress;

        let center = self.props.center;
        let start_outer = Vec2 {
            x: 0.0,
            y: -self.props.radius - ARC_HALF_WIDTH,
        };
        let start_inner = Vec2 {
            x: 0.0,
            y: -self.props.radius + ARC_HALF_WIDTH,
        };
        let end_outer = start_outer.rotate(angle);
        let end_inner = start_inner.rotate(angle);

        let path_data = PathData::from_iter(vec![
            PathSegment::MoveTo(center + start_outer),
            PathSegment::Arc {
                radius: Vec2 {
                    x: self.props.radius + ARC_HALF_WIDTH,
                    y: self.props.radius + ARC_HALF_WIDTH,
                },
                x_axis_rotation: 0.0,
                large_arc_flag: angle > PI,
                sweep_flag: true,
                end: center + end_outer,
            },
            PathSegment::LineTo(center + end_inner),
            PathSegment::Arc {
                radius: Vec2 {
                    x: self.props.radius - ARC_HALF_WIDTH,
                    y: self.props.radius - ARC_HALF_WIDTH,
                },
                x_axis_rotation: 0.0,
                large_arc_flag: angle > PI,
                sweep_flag: false,
                end: center + start_inner,
            },
            PathSegment::Close,
        ]);

        html! {
            <path d=path_data.to_string() fill=self.props.color />
        }
    }
}
