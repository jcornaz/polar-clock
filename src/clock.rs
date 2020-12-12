use core::time::Duration;

use chrono::{DateTime, Local};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use crate::arc::TimeArc;
use crate::time::DateTimeExt;
use crate::vector::Vec2;

//region Settings
const SIZE: f32 = 100.0;
const INNER_PADDING: f32 = 20.0;
const OUTER_PADDING: f32 = 5.0;
const ANIM_DURATION: Duration = Duration::from_secs(2);
const ANIM_DELAY: Duration = Duration::from_millis(100);
//endregion

//region Derived from settings
const CENTER: Vec2 = Vec2::repeat(SIZE / 2.0);
const ARC_WIDTH: f32 = (CENTER.x - INNER_PADDING - OUTER_PADDING) / 5.0;
const HALF_ARC_WIDTH: f32 = ARC_WIDTH / 2.0;
const INNER_RADIUS: f32 = INNER_PADDING + HALF_ARC_WIDTH;
lazy_static! {
    static ref VIEW_BOX: String = format!("0 0 {} {}", SIZE, SIZE);
}
//endregion

pub(crate) struct PolarClock {
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
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            now: Local::now(),
            task: IntervalService::spawn(Duration::from_millis(30), link.callback(|_| ())),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        self.now = Local::now();
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg viewBox=&VIEW_BOX>
                <TimeArc
                    center=CENTER,
                    width=ARC_WIDTH,
                    radius=INNER_RADIUS + (4.0 * ARC_WIDTH)
                    color="darkred",
                    progress=self.minute_progress(),
                    anim_delay=Duration::from_millis(0),
                    anim_duration=ANIM_DURATION,
                />
                <TimeArc
                    center=CENTER,
                    width=ARC_WIDTH,
                    radius=INNER_RADIUS + (3.0 * ARC_WIDTH)
                    color="darkorange",
                    progress=self.hour_progress(),
                    anim_delay=ANIM_DELAY,
                    anim_duration=ANIM_DURATION,
                />
                <TimeArc
                    center=CENTER,
                    width=ARC_WIDTH,
                    radius=INNER_RADIUS + (2.0 * ARC_WIDTH)
                    color="gold",
                    progress=self.day_progress(),
                    anim_delay=(ANIM_DELAY * 2),
                    anim_duration=ANIM_DURATION,
                />
                <TimeArc
                    center=CENTER,
                    width=ARC_WIDTH,
                    radius=INNER_RADIUS + ARC_WIDTH
                    color="darkgreen",
                    progress=self.month_progress(),
                    anim_delay=(ANIM_DELAY * 3),
                    anim_duration=ANIM_DURATION,
                />
                <TimeArc
                    center=CENTER,
                    width=ARC_WIDTH,
                    radius=INNER_RADIUS
                    color="darkblue",
                    progress=self.year_progress(),
                    anim_delay=(ANIM_DELAY * 4),
                    anim_duration=ANIM_DURATION,
                />
            </svg>
        }
    }
}
