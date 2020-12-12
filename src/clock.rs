use core::time::Duration;

use chrono::{DateTime, Datelike, Local, Timelike};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use crate::arc::TimeArc;
use crate::vector::Vec2;

#[derive(Properties, Copy, Clone)]
pub(crate) struct ClockProps {
    pub size: f32,
}

impl Default for ClockProps {
    fn default() -> Self {
        Self { size: 500.0 }
    }
}

pub(crate) struct Tick;

pub(crate) struct PolarClock {
    size: f32,
    now: DateTime<Local>,
    #[allow(dead_code)]
    task: IntervalTask,
}

impl PolarClock {
    fn minute_progress(&self) -> f32 {
        ((self.now.second() as f32 * 1_000_000_000.0) + self.now.nanosecond() as f32)
            / 60_000_000_000.0
    }

    fn hour_progress(&self) -> f32 {
        ((self.now.minute() as f32 * 60.0) + self.now.second() as f32) / 3600.0
    }

    fn day_progress(&self) -> f32 {
        ((self.now.hour() as f32 * 60.0) + self.now.minute() as f32) / 1440.0
    }

    fn month_progress(&self) -> f32 {
        ((self.now.day() as f32 * 24.0) + self.now.hour() as f32) / (30.0 * 24.0)
        // FIXME
    }
    fn year_progress(&self) -> f32 {
        11.5 / 12.0 // FIXME
    }
}

impl Component for PolarClock {
    type Message = Tick;
    type Properties = ClockProps;

    fn create(ClockProps { size }: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            size,
            now: Local::now(),
            task: IntervalService::spawn(Duration::from_millis(30), link.callback(|_| Tick)),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        self.now = Local::now();
        true
    }

    fn change(&mut self, ClockProps { size }: Self::Properties) -> bool {
        self.size = size;
        true
    }

    fn view(&self) -> Html {
        let center = Vec2::repeat(self.size / 2.0);

        html! {
            <svg width=self.size height=self.size>
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 5.0)
                    width=10.0,
                    color="blue",
                    progress=self.minute_progress(),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 15.0)
                    width=10.0,
                    color="green",
                    progress=self.hour_progress(),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 25.0)
                    width=10.0,
                    color="red",
                    progress=self.day_progress(),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 35.0)
                    width=10.0,
                    color="black",
                    progress=self.month_progress(),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 45.0)
                    width=10.0,
                    color="yellow",
                    progress=self.year_progress(),
                />
            </svg>
        }
    }
}
