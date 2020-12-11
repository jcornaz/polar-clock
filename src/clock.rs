use core::time::Duration;

use chrono::{DateTime, Local, Timelike, TimeZone};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use crate::arc::TimeArc;
use crate::vector::Vec2;

pub(crate) struct Clock {
    size: f32,
    now: DateTime<Local>,
    task: IntervalTask,
}

#[derive(Properties, Copy, Clone)]
pub(crate) struct ClockProps {
    pub size: f32
}

impl Default for ClockProps {
    fn default() -> Self {
        Self {
            size: 500.0
        }
    }
}

pub(crate) struct Tick;

impl Component for Clock {
    type Message = Tick;
    type Properties = ClockProps;

    fn create(ClockProps { size }: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            size,
            now: Local::now(),
            task: IntervalService::spawn(
                Duration::from_millis(10),
                link.callback(|_| Tick),
            ),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        self.now = Local::now();
        true
    }

    fn change(&mut self, ClockProps { size }: Self::Properties) -> bool {
        if size != self.size {
            self.size = size;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <svg width=self.size height=self.size>
                <TimeArc
                    center=Vec2::repeat(self.size / 2.0),
                    radius=((self.size / 2.0) - 5.0)
                    width=10.0,
                    color="red",
                    progress=(self.now.nanosecond() as f32 / 1000000000.0),
                />
            </svg>
        }
    }
}
