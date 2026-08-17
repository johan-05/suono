use crate::config::*;
use raylib::prelude::*;

#[allow(unused_variables)]
#[allow(dead_code)]

pub trait Component {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        fft_rasults: &Vec<f32>,
        sample_size: usize,
        decoded_audio: &[f32],
        audio_history: [i32; 400],
    );
    fn update(&mut self, new_width: i32, new_height: i32, sample_count: usize);
}

//                                      t between 0.0 and 1.0
pub fn process_colors(color_scheme: &ColorScheme, t: f32) -> Color {
    if color_scheme.blend {
        return blend_colors(&color_scheme.colors, t);
    } else {
        return index_colors(&color_scheme.colors, t);
    }
}

//                               t between 0.0 and 1.0
fn index_colors(colors: &Vec<Color>, t: f32) -> Color {
    let index = f32::floor(t * colors.len() as f32);
    return colors[index as usize];
}

//                                t between 0.0 and 1.0
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
