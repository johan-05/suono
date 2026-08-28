use crate::config::*;
use crate::graphics::{Component, process_colors};
use ffi::rlSetLineWidth;
use itertools::Itertools;
use raylib::prelude::*;

#[allow(dead_code)]
pub struct Spectrogram {
    //config
    background_color: Color,
    position: GraphicPosition,
    style: GraphicStyle,
    color_scheme: ColorScheme,
    //state
    width: f32,
    height: f32,
    topleft: (f32, f32),
}

//#[allow(dead_code)]
impl Spectrogram {
    pub fn init(config: GraphicConfig) -> Box<dyn Component> {
        let (width, height) = match config.position {
            GraphicPosition::Full => (1280.0, 720.0),
            GraphicPosition::Top | GraphicPosition::Bottom => (1280.0, 360.0),
            GraphicPosition::Left | GraphicPosition::Right => (640.0, 720.0),
            _ => (640.0, 360.0),
        };

        let topleft = match config.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0.0, 0.0),
            GraphicPosition::Right | GraphicPosition::TopRight => (640.0, 0.0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0.0, 360.0),
            GraphicPosition::BottomRight => (640.0, 360.0),
        };

        return Box::new(Spectrogram {
            background_color: config.background_color,
            position: config.position,
            style: config.style,
            color_scheme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }

    fn render_flat_lines(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
    ) {
        let sample_interval_x = self.width as f32 / sample_count as f32;

        let point_positions = fft_results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                (
                    (i as f32 * sample_interval_x),
                    (*r * self.height as f32 / 4.0),
                )
            })
            .collect::<Vec<(f32, f32)>>();

        point_positions
            .into_iter()
            .enumerate()
            .for_each(|(i, (x, h))| {
                let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);
                d.draw_line_v(
                    Vector2 {
                        x: x + self.topleft.0,
                        y: self.height + self.topleft.1,
                    },
                    Vector2 {
                        x: x + self.topleft.0,
                        y: self.topleft.1 + self.height - h,
                    },
                    color,
                );
            });
    }

    fn render_flat_graph(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
    ) {
        let sample_interval_x = self.width as f32 / sample_count as f32;

        let point_positions = fft_results
            .iter()
            .enumerate()
            .map(|(i, r)| Vector2 {
                x: i as f32 * sample_interval_x,
                y: self.height - (*r * self.height as f32 / 4.0),
            })
            .collect::<Vec<Vector2>>();

        point_positions
            .iter()
            .tuple_windows()
            .enumerate()
            .for_each(|(i, (p1, p2))| {
                let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);
                d.draw_line_v(*p1, *p2, color);
            })
    }

    fn render_flat_dots(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
    ) {
        let sample_interval_x = self.width as f32 / sample_count as f32;

        let point_positions = fft_results
            .iter()
            .enumerate()
            .map(|(i, r)| Vector2 {
                x: i as f32 * sample_interval_x,
                y: *r * self.height / 4.0,
            })
            .collect::<Vec<Vector2>>();

        point_positions.into_iter().enumerate().for_each(|(i, v)| {
            let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);
            d.draw_line_dashed(
                Vector2 {
                    x: v.x,
                    y: self.height + self.topleft.1,
                },
                v,
                4,
                4,
                color,
            )
        });
    }

    fn render_flat_dots_single(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
    ) {
        let sample_interval_x = self.width as f32 / sample_count as f32;

        let point_positions = fft_results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                (
                    (i as f32 * sample_interval_x) as i32,
                    (*r * self.height as f32 / 4.0) as i32,
                )
            })
            .collect::<Vec<(i32, i32)>>();

        point_positions.into_iter().enumerate().for_each(|(i, p)| {
            let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);

            d.draw_circle(
                self.topleft.0 as i32 + p.0,
                self.height as i32 - (self.topleft.1 as i32 + p.1),
                2.0,
                color,
            );
        })
    }
}

impl Component for Spectrogram {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
        _decoded_audio: &[f32],
        _audio_history: &Vec<f32>,
    ) {
        // background
        d.draw_rectangle(
            self.topleft.0 as i32,
            self.topleft.1 as i32,
            self.width as i32,
            self.height as i32,
            self.background_color,
        );

        // main graphic
        match self.style {
            GraphicStyle::Lines => {
                self.render_flat_lines(d, fft_results, sample_count);
            }
            GraphicStyle::Graph => {
                self.render_flat_graph(d, fft_results, sample_count);
            }
            GraphicStyle::Dots => {
                self.render_flat_dots(d, fft_results, sample_count);
            }
            GraphicStyle::DotsSingle => {
                self.render_flat_dots_single(d, fft_results, sample_count);
            }
        }
    }

    fn update(&mut self, new_width: f32, new_height: f32, sample_count: usize) {
        (self.width, self.height) = match self.position {
            GraphicPosition::Full => (new_width, new_height),
            GraphicPosition::Top | GraphicPosition::Bottom => (new_width, new_height / 2.0),
            GraphicPosition::Left | GraphicPosition::Right => (new_width / 2.0, new_height),
            _ => (new_width / 2.0, new_height / 2.0),
        };

        self.topleft = match self.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0.0, 0.0),
            GraphicPosition::Right | GraphicPosition::TopRight => (self.width, 0.0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0.0, self.height),
            GraphicPosition::BottomRight => (self.width, self.height),
        };

        //TODO: user defined line thickness instead of hard coding it
        //                 ↓
        let line_width = 0.60 * new_width as f32 / sample_count as f32;
        unsafe { rlSetLineWidth(line_width) };
    }
}
