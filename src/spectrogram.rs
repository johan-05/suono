use crate::config::*;
use crate::graphics::{process_colors, Component};
use ffi::rlSetLineWidth;
use raylib::prelude::*;

#[allow(dead_code)]
pub struct Spectrogram {
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

//#[allow(dead_code)]
impl Spectrogram {
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

        return Box::new(Spectrogram {
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
                    (i as f32 * sample_interval_x) as i32,
                    (*r * self.height as f32 / 4.0) as i32,
                )
            })
            .collect::<Vec<(i32, i32)>>();

        point_positions
            .into_iter()
            .enumerate()
            .for_each(|(i, (x, h))| {
                let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);
                d.draw_line_v(
                    Vector2 {
                        x: (x + self.topleft.0) as f32,
                        y: (self.height + self.topleft.1) as f32,
                    },
                    Vector2 {
                        x: (x + self.topleft.0) as f32,
                        y: (self.topleft.1 + self.height - (h)) as f32,
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
            .map(|(i, r)| {
                (
                    (i as f32 * sample_interval_x) as i32,
                    (*r * self.height as f32 / 4.0) as i32,
                )
            })
            .collect::<Vec<(i32, i32)>>();

        point_positions.windows(2).enumerate().for_each(|(i, p)| {
            let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);

            d.draw_line(
                p[0].0,
                self.height - (p[0].1) as i32,
                p[1].0,
                self.height - (p[1].1) as i32,
                color,
            );
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
            .map(|(i, r)| {
                (
                    (i as f32 * sample_interval_x) as i32,
                    (*r * self.height as f32 / 4.0) as i32,
                )
            })
            .collect::<Vec<(i32, i32)>>();

        point_positions
            .into_iter()
            .enumerate()
            .for_each(|(i, (x, h))| {
                let color = process_colors(&self.color_scheme, i as f32 / sample_count as f32);
                d.draw_line_dashed(
                    Vector2 {
                        x: (x + self.topleft.0) as f32,
                        y: (self.height + self.topleft.1) as f32,
                    },
                    Vector2 {
                        x: (x + self.topleft.0) as f32,
                        y: (self.topleft.1 + self.height - h) as f32,
                    },
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
                self.topleft.0 + p.0,
                self.height - (self.topleft.1 + p.1),
                2.0,
                color,
            );
        })
    }

    fn render_round_lines(
        &mut self,
        _d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
    ) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT")
    }

    fn render_round_graph(
        &mut self,
        _d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
    ) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT")
    }

    fn render_round_dots(
        &mut self,
        _d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
    ) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT")
    }

    fn render_round_dots_single(
        &mut self,
        _d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
    ) {
        unimplemented!("UNIMPLEMENTED BRUH MOMENT")
    }
}

impl Component for Spectrogram {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
        _decoded_audio: &[f32],
        _audio_history: [i32; 400],
    ) {
        // background
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        // main graphic
        match (self.shape, self.style) {
            (GraphicShape::Flat, GraphicStyle::Lines) => {
                self.render_flat_lines(d, fft_results, sample_count);
            }
            (GraphicShape::Flat, GraphicStyle::Graph) => {
                self.render_flat_graph(d, fft_results, sample_count);
            }
            (GraphicShape::Flat, GraphicStyle::Dots) => {
                self.render_flat_dots(d, fft_results, sample_count);
            }
            (GraphicShape::Flat, GraphicStyle::DotsSingle) => {
                self.render_flat_dots_single(d, fft_results, sample_count);
            }
            (GraphicShape::Circle, GraphicStyle::Lines) => {
                self.render_round_lines(d, fft_results, sample_count);
            }
            (GraphicShape::Circle, GraphicStyle::Graph) => {
                self.render_round_graph(d, fft_results, sample_count);
            }
            (GraphicShape::Circle, GraphicStyle::Dots) => {
                self.render_round_dots(d, fft_results, sample_count);
            }
            (GraphicShape::Circle, GraphicStyle::DotsSingle) => {
                self.render_round_dots_single(d, fft_results, sample_count);
            }
        }
    }

    fn update(&mut self, new_width: i32, new_height: i32, sample_count: usize) {
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

        //TODO: user defined line thickness instead of hard coding it
        //                 ↓
        let line_width = 0.60 * new_width as f32 / sample_count as f32;
        unsafe { rlSetLineWidth(line_width) };
    }
}
