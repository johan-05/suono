use crate::config::*;

use crate::graphics::Component;
use crate::graphics::Spectrogram;
use crate::graphics::Timeline;
use crate::graphics::Waveform;
const FLAG_WINDOW_RESIZABLE: u32 = 4; // source: trust me bro!
use ffi::rlSetLineWidth;
use ffi::SetConfigFlags;
use raylib::prelude::*;

use std::f32::consts::{E, PI};

extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
use psimple::Simple;
use pulse::sample::{Format, Spec};
use pulse::stream::Direction;

use rayon::prelude::*;

#[allow(dead_code)]
pub struct Suono {
    //config
    pub sample_count: usize,
    pub sample_interpolation_scalar: f32,
    pub background_color: Color,
    pub background_image: Option<Image>,
    //pub graphics: Vec<GraphicElement>,
    pub graphic_elements: Vec<Box<dyn Component>>,
    //state
    window_height: i32,
    window_width: i32,
    target_frequencies: Vec<f32>,
    raw_audio_buffer: [u8; 3200],
    decoded_audio_buffer: Vec<(f32, f32)>,
    audio_history: [i32; 800],
    fft_results: Vec<f32>,
    //raylib stuff
    pub rl: RaylibHandle,
    thread: RaylibThread,
    rl_audio: RaylibAudio,
    //PulseAudio stuff
    pub pa_stream: Simple,
    audio_format: Spec,
}

#[allow(dead_code)]
impl Suono {
    pub fn init(config: Config) -> Self {
        let (rl, thread) = raylib::init()
            .size(1280, 720)
            .title("Trap Nation ripoff™")
            .build();

        let rl_audio = RaylibAudio::init_audio_device().expect("failed to initialize audio");

        let (pa_stream, audio_format) = pa_init();

        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();

        let line_width = 0.60 * window_width as f32 / config.sample_count as f32;
        unsafe { rlSetLineWidth(line_width) };

        unsafe { SetConfigFlags(FLAG_WINDOW_RESIZABLE) };

        let graphic_elements = Suono::create_graphic_elements(config.graphics);

        let target_frequencies = (0..config.sample_count)
            .map(|i| 20.0 * E.powf(6.3 * i as f32 / config.sample_count as f32))
            .collect::<Vec<f32>>();

        let raw_audio_buffer = [0; 3200];
        let audio_history = [0; 800];
        let decoded_audio_buffer: Vec<(f32, f32)> = Vec::new();

        let fft_results = vec![0.0; config.sample_count];
        return Suono {
            sample_count: config.sample_count,
            sample_interpolation_scalar: config.sample_interpolation_scalar,
            background_color: config.background_color,
            background_image: config.background_image,
            graphic_elements,
            window_width,
            window_height,
            target_frequencies,
            raw_audio_buffer,
            decoded_audio_buffer,
            audio_history,
            fft_results,
            rl,
            thread,
            rl_audio,
            pa_stream,
            audio_format,
        };
    }

    pub fn update_screen_dimensions(&mut self) {
        let new_width = self.rl.get_screen_width();
        let new_height = self.rl.get_screen_height();
        if new_width != self.window_width || new_height != self.window_height {
            println!("changed");
            self.window_width = new_width;
            self.window_height = new_height;
            self.graphic_elements
                .iter_mut()
                .for_each(|g| g.update(new_width, new_width));

            let line_width = 0.60 * new_width as f32 / self.sample_count as f32;
            unsafe { rlSetLineWidth(line_width) };
        }
    }

    pub fn update_audio_data(&mut self) {
        self.pa_stream.read(&mut self.raw_audio_buffer).unwrap();
        self.decoded_audio_buffer = self
            .raw_audio_buffer
            .chunks(2)
            .enumerate()
            .map(|(i, b)| {
                (i as f32 / 44100.0, unsafe {
                    *(&b[0] as *const u8 as *const i16) as f32
                })
            })
            .collect::<Vec<(f32, f32)>>();

        self.decoded_audio_buffer
            .chunks(100)
            .map(|c| c.iter().map(|s| f32::abs(s.1)).sum::<f32>())
            .map(|n| n / 4000.0)
            .for_each(|n| {
                let index = self.audio_history.iter().position(|i| *i == 0).unwrap_or(0);
                if index != 799 {
                    self.audio_history[index] = n as i32;
                    self.audio_history[index + 1] = 0;
                } else {
                    self.audio_history[index] = n as i32;
                    self.audio_history[0] = 0;
                }
            });

        let res = fft_custom(
            self.decoded_audio_buffer.as_slice(),
            &self.target_frequencies,
        );

        for (i, f) in self.fft_results.iter_mut().enumerate() {
            *f = res[i] + *f * self.sample_interpolation_scalar;
        }
    }

    fn create_graphic_elements(graphic_configs: Vec<GraphicConfig>) -> Vec<Box<dyn Component>> {
        graphic_configs
            .into_iter()
            .map(|config| match config.graphics_type {
                GraphicType::Spectrogram => Spectrogram::init(config),
                GraphicType::Waveform => Waveform::init(config),
                GraphicType::Timeline => Timeline::init(config),
            })
            .collect::<Vec<Box<dyn Component>>>()
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(self.background_color);

        for graphic in &mut self.graphic_elements {
            graphic.render(
                &mut d,
                &self.fft_results,
                self.sample_count,
                &self.decoded_audio_buffer,
                self.audio_history,
            );
        }
    }
}

pub fn pa_init() -> (Simple, Spec) {
    let spec = Spec {
        format: Format::S16NE,
        channels: 1,
        rate: 44100,
    };

    let stream = Simple::new(
        None,
        "Suono",
        Direction::Record,
        //Some("alsa_output.pci-0000_03_00.6.HiFi__Headphones__sink.monitor"),
        Some("alsa_output.pci-0000_03_00.6.HiFi__Speaker__sink.monitor"),
        "Music",
        &spec,
        None,
        None,
    )
    .expect("amogus");

    return (stream, spec);
}

fn fft_custom(data: &[(f32, f32)], target_frequencies: &Vec<f32>) -> Vec<f32> {
    target_frequencies
        .par_iter()
        .map(|target_f| {
            let forier_data = data
                .into_iter()
                .map(|d| {
                    (
                        d.1 * f32::sin(2.0 * PI * d.0 * target_f),
                        d.1 * f32::cos(2.0 * PI * d.0 * target_f),
                    )
                })
                .fold((0.0, 0.0), |acc, d| (acc.0 + d.0, acc.1 + d.1));
            let mass_center = (
                forier_data.0 / data.len() as f32,
                forier_data.1 / data.len() as f32,
            );
            return f32::powf(
                mass_center.0 * mass_center.0 + mass_center.1 * mass_center.1,
                0.25,
            );
        })
        .collect::<Vec<f32>>()
}
