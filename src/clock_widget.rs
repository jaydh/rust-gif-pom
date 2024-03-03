use chrono::{DateTime, Local, Timelike};
use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Color, FontFamily};
use std::time::Duration;

const POMO_DURATION: Duration = Duration::from_mins(25);

use crate::AppState;

pub struct DigitalClock {}

impl DigitalClock {
    pub fn new() -> Self {
        DigitalClock {}
    }
    fn get_current_time(&self, now: DateTime<Local>) -> String {
        format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
    }
    fn get_time_left(&self, start_time: DateTime<Local>, now: DateTime<Local>) -> String {
        let ttl = start_time + POMO_DURATION;
        let diff = ttl - now;
        let hours = diff.num_hours();
        let minutes = diff.num_minutes() % 60;
        let seconds = diff.num_seconds() % 60;

        format!(
            "Time left: {} hours, {} minutes, {} seconds",
            hours, minutes, seconds
        )
    }
}

impl Widget<AppState> for DigitalClock {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.constrain((400.0, 400.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        if data.started {
            if let Some(start_time) = &data.start_time {
                let now = Local::now();
                let time_str = self.get_current_time(now);
                let time_left_str = self.get_time_left(start_time.clone().into_inner(), now);

                let text_layout = ctx
                    .text()
                    .new_text_layout(time_str)
                    .font(FontFamily::MONOSPACE, 48.0)
                    .text_color(Color::from_hex_str("#FFFFFF").unwrap())
                    .build()
                    .unwrap();

                let text_width = text_layout.size().width;
                let text_height = text_layout.size().height;

                let size = ctx.size();
                let x_offset = (size.width - text_width) / 2.0;
                let y_offset = (size.height - text_height) / 2.0;

                let text_ttl_layout = ctx
                    .text()
                    .new_text_layout(time_left_str)
                    .font(FontFamily::MONOSPACE, 24.0)
                    .text_color(Color::from_hex_str("#FF0000").unwrap())
                    .build()
                    .unwrap();

                let text_width = text_ttl_layout.size().width;
                let text_height = text_ttl_layout.size().height;

                let size = ctx.size();
                let x_ttl_offset = (size.width - text_width) / 2.0;
                let y_ttl_offset = (size.height - text_height) / 2.0 + 100.0;

                ctx.draw_text(&text_layout, (x_offset, y_offset));
                ctx.draw_text(&text_ttl_layout, (x_ttl_offset, y_ttl_offset));
            }
        }
    }
}
