use std::f32::consts::PI;

use yew::prelude::*;

use crate::svg::*;
use crate::vector::*;

pub(crate) struct TimeArc {
    props: TimeArcProp,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct TimeArcProp {
    pub center: Vec2,
    pub radius: f32,
    pub width: f32,
    pub color: String,
    pub progress: f32,
}

impl Component for TimeArc {
    type Message = ();
    type Properties = TimeArcProp;

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

        let inner_radius = self.props.radius - (self.props.width / 2.0);
        let outer_radius = inner_radius + self.props.width;

        let center = self.props.center;

        let inner_start = Vec2 {
            x: 0.0,
            y: -inner_radius,
        };
        let outer_start = Vec2 {
            x: 0.0,
            y: -outer_radius,
        };

        let path_data = PathData::with_capacity(5)
            .move_to(center + outer_start)
            .arc_to(
                center + outer_start.rotate(angle),
                ArcDef {
                    radius: Vec2 {
                        x: outer_radius,
                        y: outer_radius,
                    },
                    x_axis_rotation: 0.0,
                    large_arc_flag: angle > PI,
                    sweep_flag: true,
                },
            )
            .line_to(center + inner_start.rotate(angle))
            .arc_to(
                center + inner_start,
                ArcDef {
                    radius: Vec2 {
                        x: inner_radius,
                        y: inner_radius,
                    },
                    x_axis_rotation: 0.0,
                    large_arc_flag: angle > PI,
                    sweep_flag: false,
                },
            )
            .close()
            .to_string();

        html! {
            <path d=path_data fill=self.props.color />
        }
    }
}
