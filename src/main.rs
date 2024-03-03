#![feature(duration_constructors)]

use chrono::{DateTime, Local};
use druid::widget::prelude::*;
use druid::widget::{Button, Flex};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc, WindowState};

mod clock_widget;
mod gif_widget;
use clock_widget::DigitalClock;
use gif_widget::GifWidget;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

pub struct LocalDateTime(DateTime<Local>);

impl LocalDateTime {
    fn into_inner(self) -> DateTime<Local> {
        self.0
    }
}

impl Data for LocalDateTime {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for LocalDateTime {
    fn clone(&self) -> Self {
        LocalDateTime(self.0.clone())
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    start_time: Option<LocalDateTime>,
    started: bool,
    ended: bool,
}

fn main() {
    let window = WindowDesc::new(build_root_widget()).set_window_state(WindowState::Maximized);

    let state = AppState {
        start_time: None,
        started: false,
        ended: false,
    };

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let clock = DigitalClock::new();
    let anim = GifWidget::new();

    let start_stop_button = Button::new(|a: &AppState, _: &Env| match (a.started, a.ended) {
        (true, false) => "Stop",
        (true, true) => "Restart",
        _ => "Start",
    })
    .on_click(|_ctx, a: &mut AppState, _env| match (a.started, a.ended) {
        (true, false) => {
            a.started = false;
            a.ended = false
        }
        _ => {
            a.started = true;
            a.ended = false;
            a.start_time = Some(LocalDateTime(Local::now()));
        }
    });

    Flex::column()
        .with_child(clock)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(anim)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(start_stop_button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .align_vertical(UnitPoint::CENTER)
}
