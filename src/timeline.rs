use crate::config::*;
use crate::graphics::{Component, process_colors};
use itertools::Itertools;
use raylib::prelude::*;
use std::iter::Chain;
use std::slice::Iter;

struct Line {
    start: Vector2,
    end: Vector2,
}

impl Line {
    fn clamp_y(&mut self, height: f32) {
        let len = f32::abs(self.start.y - self.end.y);
        let diff = len - height;
        if diff > 0.0 {
            self.start.y -= diff / 2.0;
            self.end.y += diff / 2.0;
        }
    }
}

struct GraphSegment {
    top: Line,
    bottom: Line,
}

impl GraphSegment {
    fn clamp_y(&mut self, height: f32) {
        let len_start = f32::abs(self.top.start.y - self.bottom.start.y);
        let diff_start = len_start - height;
        if diff_start > 0.0 {
            self.top.start.y -= diff_start / 2.0;
            self.bottom.start.y += diff_start / 2.0;
        }

        let len_end = f32::abs(self.top.end.y - self.bottom.end.y);
        let diff_end = len_end - height;
        if diff_end > 0.0 {
            self.top.end.y -= diff_end / 2.0;
            self.bottom.end.y += diff_end / 2.0;
        }
    }
}

pub struct Timeline {
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

impl Timeline {
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
        return Box::new(Timeline {
            background_color: config.background_color,
            position: config.position,
            style: config.style,
            color_scheme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }

    fn get_audio_history_iterator<'a>(
        &self,
        audio_history: &'a Vec<f32>,
    ) -> Chain<Iter<'a, f32>, Iter<'a, f32>> {
        let start_index = audio_history.iter().position(|s| *s == 0.0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];
        return first_slice.into_iter().chain(second_slice.into_iter());
    }

    fn calculate_vertical_line_position(&self, i: f32, s: f32, sample_interval: f32) -> Line {
        let mut line = Line {
            start: Vector2 {
                x: self.topleft.0 + i * sample_interval,
                y: self.topleft.1 + self.height / 2.0 + (s * self.height / 800.0),
            },
            end: Vector2 {
                x: self.topleft.0 + i * sample_interval,
                y: self.topleft.1 + self.height / 2.0 - (s * self.height / 800.0),
            },
        };
        line.clamp_y(self.height);
        return line;
    }

    fn calculate_graph_line_position(
        &self,
        i: f32,
        s1: f32,
        s2: f32,
        sample_interval: f32,
    ) -> GraphSegment {
        let mut graph_segment = GraphSegment {
            top: Line {
                start: Vector2 {
                    x: self.topleft.0 + i * sample_interval,
                    y: self.topleft.1 + self.height / 2.0 + (s1 * self.height / 800.0),
                },
                end: Vector2 {
                    x: self.topleft.0 + (i + 1.0) * sample_interval,
                    y: self.topleft.1 + self.height / 2.0 + (s2 * self.height / 800.0),
                },
            },
            bottom: Line {
                start: Vector2 {
                    x: self.topleft.0 + i * sample_interval,
                    y: self.topleft.1 as f32 + self.height / 2.0 - (s1 * self.height / 800.0),
                },
                end: Vector2 {
                    x: self.topleft.0 + (i + 1.0) * sample_interval,
                    y: self.topleft.1 + self.height / 2.0 - (s2 * self.height / 800.0),
                },
            },
        };

        graph_segment.clamp_y(self.height);
        return graph_segment;
    }

    fn render_lines(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<f32>, gain: f32) {
        let audio_history_iter = self.get_audio_history_iterator(&audio_history);
        let sample_interval = self.width as f32 / audio_history.len() as f32;

        audio_history_iter.enumerate().for_each(|(i, s)| {
            let t = i as f32 / audio_history.len() as f32;
            let line_color = process_colors(&self.color_scheme, t.clamp(0.0, 0.9999));
            let line = self.calculate_vertical_line_position(i as f32, *s * gain, sample_interval);
            d.draw_line_v(line.start, line.end, line_color);
        });
    }

    fn render_graph(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<f32>, gain: f32) {
        let audio_history_iter = self.get_audio_history_iterator(&audio_history);
        let sample_interval = self.width as f32 / audio_history.len() as f32;

        audio_history_iter
            .tuple_windows()
            .enumerate()
            .for_each(|(i, (s1, s2))| {
                let t = i as f32 / audio_history.len() as f32;
                let line_color = process_colors(&self.color_scheme, t.clamp(0.0, 0.9999));
                let graph_segment = self.calculate_graph_line_position(
                    i as f32,
                    *s1 * gain,
                    *s2 * gain,
                    sample_interval,
                );
                d.draw_line_v(graph_segment.top.start, graph_segment.top.end, line_color);
                d.draw_line_v(
                    graph_segment.bottom.start,
                    graph_segment.bottom.end,
                    line_color,
                );
            });
    }

    fn render_dots(&mut self, d: &mut RaylibDrawHandle, audio_history: &Vec<f32>, gain: f32) {
        let audio_history_iter = self.get_audio_history_iterator(&audio_history);
        let sample_interval = self.width as f32 / audio_history.len() as f32;

        audio_history_iter.enumerate().for_each(|(i, s)| {
            let t = i as f32 / audio_history.len() as f32;
            let line_color = process_colors(&self.color_scheme, t.clamp(0.0, 0.9999));
            let line = self.calculate_vertical_line_position(i as f32, *s * gain, sample_interval);
            d.draw_line_dashed(line.start, line.end, 1, 8, line_color);
        });
    }

    fn render_dots_single(
        &mut self,
        _d: &mut RaylibDrawHandle,
        _audio_history: &Vec<f32>,
        _gain: f32,
    ) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT");
    }
}

impl Component for Timeline {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _decoded_audio: &[f32],
        audio_history: &Vec<f32>,
        gain: f32,
    ) {
        d.draw_rectangle(
            self.topleft.0 as i32,
            self.topleft.1 as i32,
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
        match self.style {
            GraphicStyle::Lines => self.render_lines(d, audio_history, gain),
            GraphicStyle::Graph => self.render_graph(d, audio_history, gain),
            GraphicStyle::Dots => self.render_dots(d, audio_history, gain),
            GraphicStyle::DotsSingle => self.render_dots_single(d, audio_history, gain),
        }
    }

    fn update(&mut self, new_width: f32, new_height: f32) {
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
    }
}
