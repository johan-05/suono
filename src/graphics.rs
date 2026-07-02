use crate::config::*;
use raylib::prelude::*;

pub trait Component {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_rasults: &Vec<f32>,
        sample_size: usize,
        decoded_audio: &Vec<(f32, f32)>,
        audio_history: [i32; 800],
    );
    fn update(&mut self, new_width: i32, new_height: i32);
}

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
        return Box::new(Spectrogram {
            background_color: config.background_color,
            position: config.position,
            shape: config.shape,
            style: config.style,
            color_scheme: config.color_scheme,
            width: 1280,
            height: 720,
            topleft: (0, 0),
        });
    }
}

impl Component for Spectrogram {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
        _decoded_audio: &Vec<(f32, f32)>,
        _audio_history: [i32; 800],
    ) {
        let sample_interval_x = self.width as f32 / sample_count as f32;
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        match self.shape {
            GraphicShape::Line => {
                let point_positions = fft_results
                    .iter()
                    .enumerate()
                    .map(|(i, r)| ((i as f32 * sample_interval_x) as i32, *r as i32))
                    .collect::<Vec<(i32, i32)>>();

                match self.style {
                    GraphicStyle::Lines => {
                        point_positions
                            .into_iter()
                            .enumerate()
                            .for_each(|(i, (x, h))| {
                                let color = RAINBOW[10 * i / sample_count];

                                d.draw_line(x, self.height, x, self.height - (h * 3) as i32, color);
                            })
                    }

                    GraphicStyle::Graph => {
                        point_positions.windows(2).enumerate().for_each(|(i, p)| {
                            let color = RAINBOW[10 * i / sample_count];

                            d.draw_line(
                                p[0].0,
                                self.height - (p[0].1 * 3) as i32,
                                p[1].0,
                                self.height - (p[1].1 * 3) as i32,
                                color,
                            );
                        })
                    }
                    GraphicStyle::Dots => {
                        point_positions
                            .into_iter()
                            .enumerate()
                            .for_each(|(i, (x, h))| {
                                let color = RAINBOW[10 * i / sample_count];

                                d.draw_circle(x, self.height - (h * 3) as i32, 3.0, color);
                            })
                    }
                }
            }
            GraphicShape::Circle => {}
        }

        /*for (i, freq) in fft_results.iter().enumerate() {
        let color = RAINBOW[10 * i / sample_count];
        d.draw_line(
            (i as f32 * sample_interval_x) as i32,
            self.height,
            (i as f32 * sample_interval_x) as i32,
            self.height - (freq * 3.0) as i32,
            color,
        );
        }*/
    }

    fn update(&mut self, new_width: i32, new_height: i32) {
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

pub struct Waveform {
    //config
    background_color: Color,
    position: GraphicPosition,
    shape: GraphicShape,
    style: GraphicStyle,
    color_shceme: ColorScheme,
    //state
    width: i32,
    height: i32,
    topleft: (i32, i32),
}

impl Waveform {
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

        return Box::new(Waveform {
            background_color: config.background_color,
            position: config.position,
            shape: config.shape,
            style: config.style,
            color_shceme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }
}

impl Component for Waveform {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        decoded_audio: &Vec<(f32, f32)>,
        _audio_history: [i32; 800],
    ) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        let sample_interval = self.width as f32 / 200.0;

        let points: Vec<Vector2> = decoded_audio
            .iter()
            .step_by(8)
            .enumerate()
            .map(|(i, s)| Vector2 {
                x: i as f32 * sample_interval + self.topleft.0 as f32,
                y: (self.height as f32 / 2.0) + (s.1 / 45.0) + self.topleft.1 as f32,
            })
            .collect();

        d.draw_spline_bezier_quadratic(
            &points,
            3.0,
            Color {
                r: 204,
                g: 47,
                b: 0,
                a: 255,
            },
        );
    }

    fn update(&mut self, new_width: i32, new_height: i32) {
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

    fn render_dots(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        _decoded_audio: &Vec<(f32, f32)>,
        audio_history: [i32; 800],
    ) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        let sample_interval = self.width as f32 / 800.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        first_slice.iter().enumerate().for_each(|(i, s)| {
            let color = blend_colors(&self.color_scheme.colors, i as f32 / 800.0);
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
            let color = blend_colors(
                &self.color_scheme.colors,
                (i + (800 - start_index)) as f32 / 800.0,
            );
            d.draw_circle(
                ((i + (800 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 + s + 1 + self.topleft.1,
                3.0,
                color,
            );

            d.draw_circle(
                ((i + (800 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 - s - 1 + self.topleft.1,
                3.0,
                color,
            );
        });
    }

    fn render_lines(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        _decoded_audio: &Vec<(f32, f32)>,
        audio_history: [i32; 800],
    ) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        let sample_interval = self.width as f32 / 800.0;
        let start_index = audio_history.iter().position(|s| *s == 0).unwrap_or(0);
        let first_slice = &audio_history[start_index..];
        let second_slice = &audio_history[..start_index];

        first_slice.iter().enumerate().for_each(|(i, s)| {
            let color = blend_colors(&self.color_scheme.colors, i as f32 / 800.0);
            d.draw_line(
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 + s + 1 + self.topleft.1,
                (i as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 - s - 1 + self.topleft.1,
                color,
            );
        });

        second_slice.iter().enumerate().for_each(|(i, s)| {
            let color = blend_colors(
                &self.color_scheme.colors,
                (i + (800 - start_index)) as f32 / 800.0,
            );
            d.draw_line(
                ((i + (800 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 + s + self.topleft.1,
                ((i + (800 - start_index)) as f32 * sample_interval) as i32 + self.topleft.0,
                self.height / 2 - s + self.topleft.1,
                color,
            );
        });
    }

    fn render_graph(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _sample_count: usize,
        _decoded_audio: &Vec<(f32, f32)>,
        audio_history: [i32; 800],
    ) {
        d.draw_rectangle(
            self.topleft.0,
            self.topleft.1,
            self.width,
            self.height,
            self.background_color,
        );

        let sample_interval = self.width as f32 / 800.0;
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
                x: (i + (800 - start_index)) as f32 * sample_interval,
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
                x: (i + (800 - start_index)) as f32 * sample_interval,
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
}

impl Component for Timeline {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_results: &Vec<f32>,
        sample_count: usize,
        decoded_audio: &Vec<(f32, f32)>,
        audio_history: [i32; 800],
    ) {
        match self.style {
            GraphicStyle::Lines => {
                self.render_lines(d, fft_results, sample_count, decoded_audio, audio_history)
            }
            GraphicStyle::Dots => {
                self.render_dots(d, fft_results, sample_count, decoded_audio, audio_history)
            }
            GraphicStyle::Graph => {
                self.render_graph(d, fft_results, sample_count, decoded_audio, audio_history)
            }
        }
    }

    fn update(&mut self, new_width: i32, new_height: i32) {
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

fn blend_colors(colors: &Vec<Color>, t: f32) -> Color {
    if colors.len() == 1 {
        return colors[0];
    };
    let fade_count = (colors.len() - 1) as f32;
    let color_index = f32::floor(fade_count * t);
    let true_t = fade_count * t - color_index as f32;
    return fade_rgb(
        colors[color_index as usize],
        colors[color_index as usize + 1],
        true_t,
    );
}

fn fade_rgb(from: Color, to: Color, t: f32) -> Color {
    let clamp = |x: f32| x.clamp(0.0, 1.0);
    let t = clamp(t * t * (3.0 - 2.0 * t));

    let r = (from.r as f32 + (to.r as f32 - from.r as f32) * t).round() as u8;
    let g = (from.g as f32 + (to.g as f32 - from.g as f32) * t).round() as u8;
    let b = (from.b as f32 + (to.b as f32 - from.b as f32) * t).round() as u8;
    let a = (from.a as f32 + (to.a as f32 - from.a as f32) * t).round() as u8;

    return Color {
        r: r,
        g: g,
        b: b,
        a: a,
    };
}

const RAINBOW: [Color; 10] = [
    Color {
        r: 204,
        g: 47,
        b: 0,
        a: 255,
    },
    Color {
        r: 219,
        g: 102,
        b: 0,
        a: 255,
    },
    Color {
        r: 227,
        g: 158,
        b: 0,
        a: 255,
    },
    Color {
        r: 118,
        g: 184,
        b: 13,
        a: 255,
    },
    Color {
        r: 0,
        g: 118,
        b: 104,
        a: 255,
    },
    Color {
        r: 0,
        g: 100,
        b: 134,
        a: 255,
    },
    Color {
        r: 0,
        g: 124,
        b: 181,
        a: 255,
    },
    Color {
        r: 70,
        g: 90,
        b: 178,
        a: 255,
    },
    Color {
        r: 109,
        g: 71,
        b: 177,
        a: 255,
    },
    Color {
        r: 135,
        g: 59,
        b: 156,
        a: 255,
    },
];
