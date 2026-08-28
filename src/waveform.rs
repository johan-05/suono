use crate::config::*;
use crate::graphics::{Component, process_colors};
use ffi::rlSetLineWidth;
use raylib::prelude::*;

pub struct Waveform {
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

impl Waveform {
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

        return Box::new(Waveform {
            background_color: config.background_color,
            position: config.position,
            style: config.style,
            color_scheme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }

    fn render_lines(&mut self, d: &mut RaylibDrawHandle, decoded_audio: &[f32]) {
        let sample_interval = self.width as f32 / 256.0;

        let points: Vec<Vector2> = decoded_audio
            .into_iter()
            .step_by(8)
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: i as f32 * sample_interval + self.topleft.0 as f32,
                y: (self.height as f32 / 2.0) + (s * self.height as f32) + self.topleft.1 as f32,
            })
            .collect();

        points.into_iter().enumerate().for_each(|(i, vec)| {
            let color = process_colors(&self.color_scheme, i as f32 / 256.0);

            d.draw_line_v(
                Vector2 {
                    x: vec.x,
                    y: self.topleft.1 as f32 + self.height as f32 / 2.0,
                },
                vec,
                color,
            );
        });
    }

    fn render_graph(&mut self, d: &mut RaylibDrawHandle, decoded_audio: &[f32]) {
        let sample_interval = self.width as f32 / 256.0;

        let points: Vec<Vector2> = decoded_audio
            .iter()
            .step_by(8)
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: i as f32 * sample_interval + self.topleft.0 as f32,
                y: (self.height as f32 / 2.0) + (s * self.height as f32) + self.topleft.1 as f32,
            })
            .collect();

        points.windows(2).enumerate().for_each(|(i, points)| {
            let t = i as f32 / 256.0;
            let color = process_colors(&self.color_scheme, t);
            d.draw_line_v(points[0], points[1], color);
        });
    }

    fn render_dots(&mut self, d: &mut RaylibDrawHandle, decoded_audio: &[f32]) {
        let sample_interval = self.width as f32 / 256.0;

        let points: Vec<Vector2> = decoded_audio
            .into_iter()
            .step_by(8)
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: i as f32 * sample_interval + self.topleft.0 as f32,
                y: (self.height as f32 / 2.0) + (s * self.height as f32) + self.topleft.1 as f32,
            })
            .collect();

        points.into_iter().enumerate().for_each(|(i, vec)| {
            let color = process_colors(&self.color_scheme, i as f32 / 256.0);

            d.draw_line_dashed(
                Vector2 {
                    x: vec.x,
                    y: self.topleft.1 as f32 + self.height as f32 / 2.0,
                },
                vec,
                4,
                4,
                color,
            );
        });
    }

    fn render_dots_single(&mut self, d: &mut RaylibDrawHandle, decoded_audio: &[f32]) {
        let sample_interval = self.width as f32 / 256.0;

        let points: Vec<Vector2> = decoded_audio
            .into_iter()
            .step_by(8)
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: i as f32 * sample_interval + self.topleft.0 as f32,
                y: (self.height as f32 / 2.0) + (s * self.height as f32) + self.topleft.1 as f32,
            })
            .collect();

        points.into_iter().enumerate().for_each(|(i, vec)| {
            let color = process_colors(&self.color_scheme, i as f32 / 256.0);

            d.draw_circle_v(vec, 0.5 * sample_interval, color);
        });
    }
}

impl Component for Waveform {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        decoded_audio: &[f32],
        _audio_history: &Vec<f32>,
    ) {
        d.draw_rectangle(
            self.topleft.0 as i32,
            self.topleft.1 as i32,
            self.width as i32,
            self.height as i32,
            self.background_color,
        );

        match self.style {
            GraphicStyle::Lines => self.render_lines(d, decoded_audio),
            GraphicStyle::Graph => self.render_graph(d, decoded_audio),
            GraphicStyle::Dots => self.render_dots(d, decoded_audio),
            GraphicStyle::DotsSingle => self.render_dots_single(d, decoded_audio),
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
        let line_width = 0.40 * new_width as f32 / sample_count as f32;
        unsafe { rlSetLineWidth(line_width) };
    }
}
