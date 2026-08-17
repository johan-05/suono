use crate::config::*;
use crate::graphics::Component;
use crate::spectrogram::Spectrogram;
use crate::timeline::Timeline;
use crate::waveform::Waveform;

use ffi::SetConfigFlags;
use raylib::ffi::Rectangle;
use raylib::prelude::*;
const FLAG_WINDOW_RESIZABLE: u32 = 4; // source: trust me bro!

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{DeviceId, Stream, StreamConfig};

use std::f32::consts::{E, PI};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

#[allow(dead_code)]
#[allow(unused_variables)]

pub struct Suono {
    //config
    pub sample_count: usize,
    pub sample_interpolation_scalar: f32,
    pub background_color: Color,
    pub background_image: Option<Texture2D>,
    pub graphic_elements: Vec<Box<dyn Component>>,
    //state
    window_height: i32,
    window_width: i32,
    target_frequencies: Vec<f32>,
    decoded_audio_buffer: [f32; 2048],
    audio_history: [i32; 400],
    fft_results: Vec<f32>,
    //raylib stuff
    pub rl: RaylibHandle,
    thread: RaylibThread,
    //pipewire stuff
    pub audio_stream: Stream,
    audio_data_arc: Arc<Mutex<[f32; 2048]>>,
}

#[allow(dead_code)]
impl Suono {
    pub fn init(config: Config) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(1279, 720)
            .title("Trap Nation ripoff™")
            .build();

        //let rl_audio = RaylibAudio::init_audio_device().expect("audio init failed");

        let audio_data_arc = Arc::new(Mutex::new([0.0; 2048]));
        let audio_stream = pipewire_init(audio_data_arc.clone());
        audio_stream.play().expect("could not play stream");

        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();

        unsafe { SetConfigFlags(FLAG_WINDOW_RESIZABLE) };

        let mut background_image: Option<Texture2D> = None;
        if let Some(background_image_file) = config.background_image {
            background_image = rl
                .load_texture_from_image(&thread, &background_image_file)
                .ok();
        }

        let graphic_elements = Suono::create_graphic_elements(config.graphics);

        let target_frequencies = create_target_frequenzies(config.sample_count);

        let audio_history = [0; 400];
        let decoded_audio_buffer = [0.0; 2048];

        let fft_results = vec![0.0; config.sample_count];

        return Suono {
            sample_count: config.sample_count,
            sample_interpolation_scalar: config.sample_interpolation_scalar,
            background_color: config.background_color,
            background_image: background_image,
            graphic_elements,
            window_width,
            window_height,
            target_frequencies,
            decoded_audio_buffer,
            audio_history,
            fft_results,
            rl,
            thread,
            //rl_audio,
            audio_stream,
            audio_data_arc,
        };
    }

    pub fn update_screen_dimensions(&mut self) {
        let new_width = self.rl.get_screen_width();
        let new_height = self.rl.get_screen_height();
        //println!("{}, {}", &new_width, &new_height);
        if new_width != self.window_width || new_height != self.window_height {
            println!("changed height {new_height}, width {new_width}");
            self.window_width = new_width;
            self.window_height = new_height;
            self.graphic_elements
                .iter_mut()
                .for_each(|g| g.update(new_width, new_height, self.sample_count));
        }
    }

    pub fn update_audio_data(&mut self) {
        let m = self.audio_data_arc.lock().expect("could not lock");
        self.decoded_audio_buffer.copy_from_slice(m.as_slice());

        //homemade circular buffer that records volume as a function of time
        self.decoded_audio_buffer
            .chunks(350)
            .map(|c| c.iter().map(|s| f32::abs(*s)).sum::<f32>())
            .for_each(|n| {
                let index = self.audio_history.iter().position(|i| *i == 0).unwrap_or(0);
                if index != self.audio_history.len() - 1 {
                    self.audio_history[index] = 5 * n as i32;
                    self.audio_history[index + 1] = 0;
                } else {
                    self.audio_history[index] = 5 * n as i32;
                    self.audio_history[0] = 0;
                }
            });

        let res = fft_custom(
            self.decoded_audio_buffer.as_slice(),
            &self.target_frequencies,
        );

        // println!("{:?}", res);

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

        if let Some(background_image) = &self.background_image {
            d.draw_texture_pro(
                background_image,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: background_image.width as f32,
                    height: background_image.height as f32,
                },
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: self.window_width as f32,
                    height: self.window_height as f32,
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                Color::WHITE,
            )
        }
        for graphic in &mut self.graphic_elements {
            graphic.render(
                &mut d,
                &self.fft_results,
                self.sample_count,
                &self.decoded_audio_buffer.as_slice(),
                self.audio_history,
            );
        }
    }
}

fn pipewire_init(audio_data_arc: Arc<Mutex<[f32; 2048]>>) -> Stream {
    let host = cpal::default_host();
    let device = host
        .device_by_id(&DeviceId::new(host.id(), "pipewire"))
        .expect("pipewire not found");

    let stream_config = StreamConfig {
        channels: 2,
        sample_rate: 44100,
        buffer_size: cpal::BufferSize::Fixed(1024),
    };

    let stream = device
        .build_input_stream(
            stream_config,
            move |source_data: &[f32], _| {
                let mut m = audio_data_arc.lock().expect("could not lock");
                m.copy_from_slice(source_data);
                // println!("{:?}", source_data[0..300].to_vec());
            },
            move |error| println!("Audio stream error {}", error),
            Some(std::time::Duration::from_secs(5)),
        )
        .expect("failed to make de strim");

    let _com = std::process::Command::new("bash")
        .arg("-c")
        .arg("~/rust/suono/src/pw_redirect_sources.sh")
        .output()
        .unwrap();

    return stream;
}

// RTFM https://en.wikipedia.org/wiki/Fast_Fourier_transform#FFT_algorithms_specialized_for_real_or_symmetric_data
// pretty standard implementation with a bit of e^ix = cos(x) + i*sin(x) shenanigans
fn fft_custom(data: &[f32], target_frequencies: &Vec<f32>) -> Vec<f32> {
    target_frequencies
        .par_iter()
        .map(|target_f| {
            let forier_data = data
                .into_iter()
                .enumerate()
                .map(|(i, d)| {
                    let t = i as f32 / 44100.0;
                    (
                        d * f32::sin(2.0 * PI * t * target_f),
                        d * f32::cos(2.0 * PI * t * target_f),
                    )
                })
                .fold((0.0, 0.0), |acc, d| (acc.0 + d.0, acc.1 + d.1));
            let mass_center = (
                forier_data.0 / data.len() as f32,
                forier_data.1 / data.len() as f32,
            );
            return f32::powf(
                mass_center.0 * mass_center.0 + mass_center.1 * mass_center.1,
                0.15, // little bit of extra softening of high values + freq equalization
            ) * f32::powf(*target_f, 0.17);
        })
        .collect::<Vec<f32>>()
}

// return a Vec of length "sample_count" with number logarithmically destributed between 0 and 20*e^(6.3) witch is 10 891
// for no other readon than that it looks good
fn create_target_frequenzies(sample_count: usize) -> Vec<f32> {
    (0..sample_count)
        .map(|i| 16.0 * E.powf(5.8 * i as f32 / sample_count as f32))
        .collect::<Vec<f32>>()
}
