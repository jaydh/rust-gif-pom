use anyhow::{Error, Result};
use druid::widget::prelude::*;
use druid::TimerToken;
use image::codecs::gif::GifDecoder;
use image::*;
use std::collections::HashMap;
use std::fs::File;
use std::time::{Duration, SystemTime};

use crate::AppState;

const TIMER_INTERVAL: Duration = Duration::from_millis(50);

struct GifData {
    data: Vec<Frame>,
    dimensions: (u32, u32),
}

pub struct GifWidget {
    start_time: SystemTime,
    timer_id: Option<TimerToken>,
    gif_map: HashMap<String, GifData>,
}

impl GifWidget {
    pub fn new() -> Self {
        let gif_map: HashMap<String, GifData> = vec!["src/squirtle-sax.gif", "src/thinking.gif"]
            .iter()
            .map(|key| {
                (
                    key.to_string(),
                    GifData {
                        data: load_gif(key).unwrap(),
                        dimensions: image_dimensions(key).unwrap(),
                    },
                )
            })
            .collect();

        GifWidget {
            start_time: SystemTime::now(),
            timer_id: None,
            gif_map,
        }
    }
}

impl Widget<AppState> for GifWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = Some(ctx.request_timer(TIMER_INTERVAL));
            }
            Event::Timer(_) => {
                ctx.request_anim_frame();
            }
            Event::AnimFrame(_interval) => {
                ctx.request_paint();
                self.timer_id = Some(ctx.request_timer(TIMER_INTERVAL));
            }
            _ => (),
        }
    }

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
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _app_data: &AppState,
        _env: &Env,
    ) -> Size {
        let max_dimension = self.gif_map.values().fold((0, 0), |max_dim, gif_data| {
            let (max_width, max_height) = max_dim;
            let (width, height) = gif_data.dimensions;
            (max_width.max(width), max_height.max(height))
        });

        bc.constrain((max_dimension.0.into(), max_dimension.1.into()))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, app_data: &AppState, _env: &Env) {
        let gif_data = match app_data.started {
            true => self.gif_map.get("src/thinking.gif").unwrap(),
            false => self.gif_map.get("src/squirtle-sax.gif").unwrap(),
        };
        let vec_size = gif_data.data.len();
        let time_since = SystemTime::now()
            .duration_since(self.start_time)
            .unwrap()
            .as_millis();
        let index = (time_since / TIMER_INTERVAL.as_millis()) as usize % vec_size;

        let img = ctx
            .make_image(
                gif_data.dimensions.0.try_into().unwrap(),
                gif_data.dimensions.1.try_into().unwrap(),
                gif_data.data[index].clone().buffer(),
                druid::piet::ImageFormat::RgbaSeparate,
            )
            .expect("failed to create image");
        let rect = ctx.size().to_rect();
        ctx.draw_image(&img, rect, druid::piet::InterpolationMode::NearestNeighbor);
    }
}

fn load_gif(file_path: &str) -> Result<Vec<Frame>, Error> {
    let f = File::open(file_path).unwrap();
    let decoder = GifDecoder::new(f).unwrap();
    let frames = decoder.into_frames().collect_frames().unwrap();
    Ok(frames)
}
