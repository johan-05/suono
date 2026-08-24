use crate::config::*;
use crate::graphics::{Component, process_colors};
use raylib::prelude::*;
use std::iter::Chain;
use std::slice::Iter;

struct Line {
    start: Vector2,
    end: Vector2,
}

pub struct Timeline {
    //config
    background_color: Color,
    position: GraphicPosition,
    shape: GraphicShape,
    style: GraphicStyle,
    color_scheme: ColorScheme,
    //state
    width: i32,
    height: i32,
    topleft: (i32, i32),
}

impl Timeline {
    pub fn init(config: GraphicConfig) -> Box<dyn Component> {
        let (width, height) = match config.position {
            GraphicPosition::Full => (1280, 720),
            GraphicPosition::Top | GraphicPosition::Bottom => (1280, 360),
            GraphicPosition::Left | GraphicPosition::Right => (640, 720),
            _ => (640, 360),
        };

        let topleft = match config.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0, 0),
            GraphicPosition::Right | GraphicPosition::TopRight => (640, 0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0, 360),
            GraphicPosition::BottomRight => (640, 360),
        };
        return Box::new(Timeline {
            background_color: config.background_color,
            position: config.position,
            shape: config.shape,
            style: config.style,
            color_scheme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }

    fn get_audio_history_iterator<'a>(
        &self,
        audio_history: &'a Vec<i32>,
    ) -> Chain<Iter<'a, i32>, Iter<'a, i32>> {
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];
        return first_slice.into_iter().chain(second_slice.into_iter());
    }

    fn calculate_line_position(&self, i: f32, s: f32, sample_interval: f32) -> Line {
        Line {
            start: Vector2 {
                x: self.topleft.0 as f32 + i * sample_interval,
                y: self.topleft.1 as f32
                    + self.height as f32 / 2.0
                    + (s * self.height as f32 / 360.0),
            },
            end: Vector2 {
                x: self.topleft.0 as f32 + i * sample_interval,
                y: self.topleft.1 as f32 + self.height as f32 / 2.0
                    - (s * self.height as f32 / 360.0),
            },
        }
    }

    fn render_lines(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<i32>) {
        let audio_history_iter = self.get_audio_history_iterator(&audio_history);
        let sample_interval = self.width as f32 / audio_history.len() as f32;

        audio_history_iter.enumerate().for_each(|(i, s)| {
            // t goes from 0.0 at the right border to 1.0 as left border
            let t = i as f32 / audio_history.len() as f32;
            let line_color = process_colors(&self.color_scheme, t.clamp(0.0, 0.9999));
            let line = self.calculate_line_position(i as f32, *s as f32, sample_interval);
            d.draw_line_v(line.start, line.end, line_color);
        });

        /*first_slice.iter().enumerate().for_each(|(i, s)| {
            let color = process_colors(&self.color_scheme, i as f32 / 400.0);
            d.draw_line(
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.topleft.1 + self.height / 2 + (s * self.height / 360),
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.topleft.1 + self.height / 2 - (s * self.height / 360),
                color,
            );
        });

        second_slice.iter().enumerate().for_each(|(i, s)| {
            let color =
                process_colors(&self.color_scheme, (i + (400 - start_index)) as f32 / 400.0);
            d.draw_line(
                ((i + (400 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.topleft.1 + self.height / 2 + (s * self.height / 360),
                ((i + (400 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.topleft.1 + self.height / 2 - (s * self.height / 360),
                color,
            );
        });*/
    }

    fn render_graph(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<i32>) {
        let sample_interval = self.width as f32 / 400.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        let top_graph_points = first_slice
            .iter()
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: (i as f32 * sample_interval) as f32 + self.topleft.0 as f32,
                y: ((self.height / 2 + self.topleft.1) - (s * self.height / 360) - 1) as f32,
            })
            .chain(second_slice.iter().enumerate().map(|(i, s)| Vector2 {
                x: (i + (400 - start_index)) as f32 * sample_interval,
                y: (self.height / 2 - (s * self.height / 360) - 1) as f32,
            }))
            .collect::<Vec<Vector2>>();

        d.draw_spline_bezier_quadratic(
            &top_graph_points,
            3.0,
            Color {
                r: 227,
                g: 158,
                b: 0,
                a: 255,
            },
        );

        let bottom_graph_points = first_slice
            .iter()
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: (i as f32 * sample_interval) as f32,
                y: (self.height / 2 + (s * self.height / 360) + 1) as f32,
            })
            .chain(second_slice.iter().enumerate().map(|(i, s)| Vector2 {
                x: (i + (400 - start_index)) as f32 * sample_interval,
                y: (self.height / 2 + (s * self.height / 360) + 1) as f32,
            }))
            .collect::<Vec<Vector2>>();

        d.draw_spline_bezier_quadratic(
            &bottom_graph_points,
            3.0,
            Color {
                r: 109,
                g: 71,
                b: 177,
                a: 255,
            },
        );
    }

    fn render_dots(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<i32>) {
        let sample_interval = self.width as f32 / 400.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        first_slice.iter().enumerate().for_each(|(i, s)| {
            let color = process_colors(&self.color_scheme, i as f32 / 400.0);
            d.draw_circle(
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 + s + 1 + self.topleft.1,
                3.0,
                color,
            );
            d.draw_circle(
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 - s - 1 + self.topleft.1,
                3.0,
                color,
            );
        });

        second_slice.iter().enumerate().for_each(|(i, s)| {
            let color =
                process_colors(&self.color_scheme, (i + (400 - start_index)) as f32 / 400.0);
            d.draw_circle(
                ((i + (400 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 + s + 1 + self.topleft.1,
                3.0,
                color,
            );

            d.draw_circle(
                ((i + (400 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 - s - 1 + self.topleft.1,
                3.0,
                color,
            );
        });
    }

    fn render_dots_single(&mut self, _d: &mut RaylibDrawHandle, _audio_history: &Vec<i32>) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT");
    }
}

impl Component for Timeline {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        _decoded_audio: &[f32],
        audio_history: &Vec<i32>,
    ) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );
        match self.style {
            GraphicStyle::Lines => self.render_lines(d, audio_history),
            GraphicStyle::Graph => self.render_graph(d, audio_history),
            GraphicStyle::Dots => self.render_dots(d, audio_history),
            GraphicStyle::DotsSingle => self.render_dots_single(d, audio_history),
        }
    }

    fn update(&mut self, new_width: i32, new_height: i32, _sample_count: usize) {
        (self.width, self.height) = match self.position {
            GraphicPosition::Full => (new_width, new_height),
            GraphicPosition::Top | GraphicPosition::Bottom => (new_width, new_height / 2),
            GraphicPosition::Left | GraphicPosition::Right => (new_width / 2, new_height),
            _ => (new_width / 2, new_height / 2),
        };

        self.topleft = match self.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0, 0),
            GraphicPosition::Right | GraphicPosition::TopRight => (self.width, 0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0, self.height),
            GraphicPosition::BottomRight => (self.width, self.height),
        };
    }
}
