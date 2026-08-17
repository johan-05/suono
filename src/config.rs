use dirs::config_dir;
use raylib::{
    ffi::{BlendMode::BLEND_ALPHA, CSSPalette, RaylibPalette},
    prelude::*,
};
use std::fs;

#[allow(dead_code)]
#[allow(unused_variables)]

const SQUARE_BRACKETS: [char; 2] = ['[', ']'];

pub fn load_config_file() -> Config {
    let config_file_path = config_dir()
        .expect("failed to locate config directory")
        .join("musicrc");

    let config_file = match std::fs::read_to_string(&config_file_path) {
        Ok(file) => file,
        Err(_e) => match fs::copy("./assets/default_config_file", &config_file_path) {
            Ok(_) => {
                let config_file = fs::read_to_string(&config_file_path).unwrap();
                println!(
                    "Config file not found. Default config written to {:?}",
                    config_file_path
                );
                config_file
            }
            Err(e) => panic!("failed to create default config file, {}", e),
        },
    };

    let session_config = parse_config_file(config_file);
    return session_config;
}

#[derive(Debug)]
pub struct Config {
    pub sample_count: usize,
    pub sample_interpolation_scalar: f32,
    pub background_color: Color,
    pub background_image: Option<Image>,
    pub graphics: Vec<GraphicConfig>,
}

impl Config {
    fn default() -> Self {
        return Config {
            sample_count: 200,
            sample_interpolation_scalar: 0.75,
            background_color: Color {
                r: 17,
                g: 17,
                b: 26,
                a: 255,
            },
            background_image: None,
            graphics: Vec::new(),
        };
    }

    fn modify_parameter(&mut self, parameter: &str, value: &str) {
        let parameter = parameter.trim().to_lowercase();
        let value = value.trim();

        match parameter.as_str() {
            "sample_count" => self.sample_count = str::parse::<usize>(value).unwrap_or(200),
            "sample_interpolation_scalar" => {
                self.sample_interpolation_scalar = str::parse::<f32>(value).unwrap_or(0.75)
            }
            "background_color" => {
                self.background_color = Color::from_hex(value).unwrap_or(Color::BLACK);
            }
            "background_image" => {
                self.background_image = match Image::load_image(value) {
                    Ok(image) => Some(image),
                    Err(e) => {
                        println!("failed to load image {}, error: {}", value, e);
                        None
                    }
                };
            }
            _ => println!("Could not set parameter {} to value {}", parameter, value),
        };
    }
}

#[derive(Clone, Debug)]
pub struct GraphicConfig {
    pub graphics_type: GraphicType,
    pub background_color: Color,
    pub position: GraphicPosition,
    pub shape: GraphicShape,
    pub style: GraphicStyle,
    pub color_scheme: ColorScheme,
}

impl GraphicConfig {
    fn default() -> Self {
        return GraphicConfig {
            graphics_type: GraphicType::Timeline,
            background_color: Color {
                r: 17,
                g: 17,
                b: 26,
                a: 255,
            },
            position: GraphicPosition::Full,
            shape: GraphicShape::Line,
            style: GraphicStyle::Graph,
            color_scheme: ColorScheme {
                colors: vec![
                    Color {
                        r: 204,
                        g: 47,
                        b: 0,
                        a: 255,
                    },
                    Color {
                        r: 70,
                        g: 90,
                        b: 178,
                        a: 255,
                    },
                    Color {
                        r: 0,
                        g: 118,
                        b: 104,
                        a: 255,
                    },
                ],
                orientation: Orientation::Horizintal,
                blend: true,
                glow: false,
            },
        };
    }

    fn modify_parameter(&mut self, parameter: &str, value: &str) {
        let parameter = parameter.trim().to_lowercase();
        let value = value.trim();

        match parameter.as_str() {
            "background_color" => {
                if value == "none" {
                    self.background_color = Color::TRANSPARENT
                } else {
                    self.background_color = Color::from_hex(value).unwrap_or(Color::WHITE);
                }
            }
            "position" => {
                self.position = match value {
                    "full" => GraphicPosition::Full,
                    "top" => GraphicPosition::Top,
                    "bottom" => GraphicPosition::Bottom,
                    "left" => GraphicPosition::Left,
                    "right" => GraphicPosition::Right,
                    "topleft" => GraphicPosition::TopLeft,
                    "topright" => GraphicPosition::TopRight,
                    "bottomleft" => GraphicPosition::BottomLeft,
                    "bottomright" => GraphicPosition::BottomRight,
                    _ => GraphicPosition::Full,
                };
            }
            "shape" => {
                self.shape = match value {
                    "line" => GraphicShape::Line,
                    "circle" => GraphicShape::Circle,
                    _ => GraphicShape::Line,
                }
            }
            "style" => {
                self.style = match value {
                    "lines" => GraphicStyle::Lines,
                    "graph" => GraphicStyle::Graph,
                    "dots" => GraphicStyle::Dots,
                    "dots_single" => GraphicStyle::DotsSingle,
                    _ => GraphicStyle::Lines,
                }
            }
            "type" => {
                self.graphics_type = match value {
                    "timeline" => GraphicType::Timeline,
                    "spectrogram" => GraphicType::Spectrogram,
                    "waveform" => GraphicType::Waveform,
                    _ => GraphicType::Timeline,
                }
            }
            "color_scheme" => {
                let values_trimmed = value.trim_matches(&SQUARE_BRACKETS);

                let colors: Vec<Color> = values_trimmed
                    .split(", ")
                    .map(|color| Color::from_hex(color).unwrap_or(Color::TURQUOISE))
                    .collect();

                self.color_scheme.colors = colors;
            }
            "color_blend" => {
                self.color_scheme.blend = match value {
                    "true" => true,
                    "false" => false,
                    _ => false,
                }
            }
            "color_glow" => {
                self.color_scheme.glow = match value {
                    "true" => true,
                    "false" => false,
                    _ => false,
                }
            }
            _ => println!("Could not set parameter {} to value {}", parameter, value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GraphicType {
    Spectrogram,
    Waveform,
    Timeline,
}

#[derive(Clone, Copy, Debug)]
pub enum GraphicPosition {
    Full,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug)]
pub enum GraphicShape {
    Line,
    Circle,
}

#[derive(Clone, Copy, Debug)]
pub enum GraphicStyle {
    Lines,
    Graph,
    Dots,
    DotsSingle,
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Vertical,
    Horizintal,
    //Diagonal(u8), Proceed if you dare!
}

#[derive(Clone, Debug)]
pub struct ColorScheme {
    pub colors: Vec<Color>,
    pub orientation: Orientation,
    pub blend: bool,
    pub glow: bool,
}

fn parse_config_file(file: String) -> Config {
    let mut session_config = Config::default();
    let mut current_graphic: Option<GraphicConfig> = None;

    file.lines()
        .filter(|l| l.len() > 1)
        .filter(|l| l.chars().nth(0).unwrap() != '#')
        .for_each(|l| {
            if l.contains("[graphic]") {
                match current_graphic {
                    Some(ref current) => {
                        session_config.graphics.push(current.clone());
                        current_graphic = Some(GraphicConfig::default());
                    }
                    None => current_graphic = Some(GraphicConfig::default()),
                };
            }

            match l.split_once(" = ") {
                Some((parameter, value)) => match current_graphic {
                    Some(ref mut current) => current.modify_parameter(parameter, value),
                    None => session_config.modify_parameter(parameter, value),
                },
                None => (),
            }
        });

    match current_graphic {
        Some(current_graphic) => session_config.graphics.push(current_graphic),
        None => (),
    }

    return session_config;
}
