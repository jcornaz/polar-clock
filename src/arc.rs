use core::time::Duration;
use std::f32::consts::PI;

use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::timeout::TimeoutTask;
use yew::services::{IntervalService, TimeoutService};

use crate::svg::*;
use crate::vector::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct TimeArcProp {
    pub center: Vec2,
    pub radius: f32,
    pub width: f32,
    pub color: &'static str,
    pub progress: f32,
    pub text: String,
    pub anim_delay: Duration,
    pub anim_duration: Duration,
}

pub(crate) enum TimeArcMsg {
    StartAnimation,
    TickAnimation(f32),
}

pub(crate) struct TimeArc {
    props: TimeArcProp,
    link: ComponentLink<Self>,
    animation: AnimTask,
    progress: f32,
}

impl Component for TimeArc {
    type Message = TimeArcMsg;
    type Properties = TimeArcProp;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            progress: props.progress,
            animation: AnimTask::None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TimeArcMsg::StartAnimation => {
                let interval_duration = Duration::from_millis(20);
                let delta =
                    interval_duration.as_secs_f32() / self.props.anim_duration.as_secs_f32();

                self.animation = AnimTask::Animate(IntervalService::spawn(
                    interval_duration,
                    self.link
                        .callback(move |_| TimeArcMsg::TickAnimation(delta)),
                ));
                false
            }
            TimeArcMsg::TickAnimation(delta) => {
                self.progress -= delta;
                if self.progress < self.props.progress {
                    self.progress = self.props.progress;
                    self.animation = AnimTask::None;
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props == self.props {
            false
        } else {
            if self.progress <= props.progress {
                // The progress advanced, we immediately update
                self.progress = props.progress;
            } else {
                // The progress has been reset, we start the animation
                if matches!(self.animation, AnimTask::None) {
                    self.animation = AnimTask::Delay(TimeoutService::spawn(
                        props.anim_delay,
                        self.link.callback(|_| TimeArcMsg::StartAnimation),
                    ));
                }
            }
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let angle = 2.0 * PI * self.progress;

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

        let outer_end = outer_start.rotate(angle);
        let inner_end = inner_start.rotate(angle);

        let path_data = PathData::with_capacity(5)
            .move_to(center + outer_start)
            .arc_to(
                center + outer_end,
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
            .line_to(center + inner_end)
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

        let (text_angle, text_color, text_anchor) = if angle >= PI {
            (angle - (PI / 180.0), "white", "end")
        } else {
            (angle + (PI / 180.0), self.props.color, "start")
        };

        let text_pos =
            Vec2::new(0.0, -inner_radius - (0.2 * self.props.width)).rotate(text_angle) + center;
        let text_transform = format!(
            "rotate({} {} {})",
            -((2.0 * PI) - text_angle).to_degrees(),
            text_pos.x,
            text_pos.y
        );
        html! {
            <>
                <path d=path_data fill=self.props.color />
                <text
                    x=text_pos.x
                    y=text_pos.y
                    font-size=(self.props.width * 0.6)
                    transform=text_transform
                    fill=text_color
                    text-anchor=text_anchor
                >{ &self.props.text }</text>
            </>
        }
    }
}

#[must_use]
enum AnimTask {
    Delay(TimeoutTask),
    Animate(IntervalTask),
    None,
}
