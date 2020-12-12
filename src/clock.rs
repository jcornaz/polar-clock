use core::time::Duration;

use chrono::{DateTime, Local};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use crate::arc::TimeArc;
use crate::time::DateTimeExt;
use crate::vector::Vec2;

lazy_static! {
    static ref ANIM_DURATION: Duration = Duration::from_secs(2);
}

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
        (self.now.millisecond_of_minute() as f32) / 60_000.0
    }

    fn hour_progress(&self) -> f32 {
        (self.now.millisecond_of_hour() as f32) / 3_600_000.0
    }

    fn day_progress(&self) -> f32 {
        (self.now.second_of_day() as f32) / 86_400.0
    }

    fn month_progress(&self) -> f32 {
        (self.now.minute_of_month() as f32) / (self.now.max_minute_of_month() as f32)
    }

    fn year_progress(&self) -> f32 {
        (self.now.hour_of_year() as f32) / (self.now.max_hour_of_year() as f32)
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
                    anim_delay=Duration::from_millis(0),
                    anim_duration=Duration::from_secs(2),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 15.0)
                    width=10.0,
                    color="green",
                    progress=self.hour_progress(),
                    anim_delay=Duration::from_millis(100),
                    anim_duration=Duration::from_secs(2),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 25.0)
                    width=10.0,
                    color="red",
                    progress=self.day_progress(),
                    anim_delay=Duration::from_millis(200),
                    anim_duration=Duration::from_secs(2),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 35.0)
                    width=10.0,
                    color="black",
                    progress=self.month_progress(),
                    anim_delay=Duration::from_millis(300),
                    anim_duration=Duration::from_secs(2),
                />
                <TimeArc
                    center=center,
                    radius=((self.size / 2.0) - 45.0)
                    width=10.0,
                    color="yellow",
                    progress=self.year_progress(),
                    anim_delay=Duration::from_millis(400),
                    anim_duration=Duration::from_secs(2),
                />
            </svg>
        }
    }
}
