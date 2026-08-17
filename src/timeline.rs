use crate::config::*;
use crate::graphics::{process_colors, Component};
use raylib::prelude::*;

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

    fn render_lines(&mut self, d: &mut RaylibDrawHandle, audio_history: [i32; 400]) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );
        let sample_interval = self.width as f32 / 400.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        first_slice.iter().enumerate().for_each(|(i, s)| {
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
        });
    }

    fn render_graph(&mut self, d: &mut RaylibDrawHandle, audio_history: [i32; 400]) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        let sample_interval = self.width as f32 / 400.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        let top_graph_points = first_slice
            .iter()
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: (i as f32 * sample_interval) as f32,
                y: (self.height / 2 - s - 1) as f32,
            })
            .chain(second_slice.iter().enumerate().map(|(i, s)| Vector2 {
                x: (i + (400 - start_index)) as f32 * sample_interval,
                y: (self.height / 2 - s - 1) as f32,
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
                y: (self.height / 2 + s + 1) as f32,
            })
            .chain(second_slice.iter().enumerate().map(|(i, s)| Vector2 {
                x: (i + (400 - start_index)) as f32 * sample_interval,
                y: (self.height / 2 + s + 1) as f32,
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

    fn render_dots(&mut self, d: &mut RaylibDrawHandle, audio_history: [i32; 400]) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

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

    fn render_dots_single(&mut self, _d: &mut RaylibDrawHandle, _audio_history: [i32; 400]) {
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
        audio_history: [i32; 400],
    ) {
        match self.style {
            GraphicStyle::Lines => self.render_lines(d, audio_history),
            GraphicStyle::Graph => self.render_graph(d, audio_history),
            GraphicStyle::Dots => self.render_dots(d, audio_history),
            GraphicStyle::DotsSingle => {
                self.render_dots_single(d, audio_history);
            }
        }
    }

    fn update(&mut self, new_width: i32, new_height: i32, _sample_count: usize) {
        println!("printed");
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
            GraphicPosition::Right | GraphicPosition::TopRight => (self.width / 2, 0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0, self.height / 2),
            GraphicPosition::BottomRight => (self.width / 2, self.height / 2),
        };
    }
}
