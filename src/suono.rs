use crate::circular::Circular2D;
use crate::config::*;
use crate::graphics::Graphic;

use ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE;
use ffi::Rectangle;
use ffi::SetConfigFlags;
use ffi::rlLoadShaderBuffer;

use raylib::ffi::{RL_STATIC_DRAW, RL_STREAM_DRAW};
use raylib::prelude::*;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{DeviceId, StreamConfig};

use std::f32::consts::{E, PI};
use std::os::raw::c_uint;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const ONE_HUNDRED_MS: u64 = 100_000_000; //the number of nanoseconds that equals 100ms

const AUDIO_BUFFER_ID: c_uint = 0;
const FFT_BUFFER_ID: c_uint = 1;
const COLOR_BUFFER_ID: c_uint = 2;
const AUDIO_HISTORY_BUFFER_ID: c_uint = 3;

pub struct Suono {
    //config
    pub sample_interpolation_scalar: f32,
    pub background_color: Color,
    pub background_image: Option<Texture2D>,
    pub graphic_elements: Vec<Graphic>,
    //state
    window_height: f32,
    window_width: f32,
    gain: f32,
    target_frequencies: Vec<f32>,
    pub audio_data: AudioData,
    pub audio_history: Vec<f32>,
    fft_results: Vec<f32>,
    fft_history: Circular2D<f32>,
    audio_buffer: u32,
    fft_buffer: u32,
    audio_history_buffer: u32,
    //raylib stuff
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    //pipewire stuff
    audio_data_arc: Arc<Mutex<AudioData>>,
}

impl Suono {
    pub fn init(config: Config) -> Self {
        let (mut rl, thread) = raylib::init().size(1279, 720).title("Suono").build();
        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height() as f32;
        rl.set_target_fps(60);
        unsafe { SetConfigFlags(FLAG_WINDOW_RESIZABLE as u32) };

        let start_time = Instant::now();
        let audio_data_arc = Arc::new(Mutex::new(AudioData {
            buffer: [0.0; 2048],
            timestamp: 0,
            program_start: start_time,
        }));

        pipewire_init(audio_data_arc.clone());

        let background_image = config
            .background_image
            .as_ref()
            .map(|img| rl.load_texture_from_image(&thread, &img).ok())
            .flatten();

        let graphic_elements = Suono::create_graphic_elements(&config, &mut rl, &thread);
        let target_frequencies = create_target_frequencies(config.sample_count);

        let audio_history = vec![0.0; config.timeline_length];
        let fft_results = vec![0.0; config.sample_count];
        let mut fft_history = Circular2D::with_dimentions((3, 2));
        println!("{:?}", fft_history);

        fft_history.push(&[1., 2.]).unwrap();
        fft_history.push(&[3., 4.]).unwrap();
        fft_history.push(&[5., 6.]).unwrap();
        fft_history.push(&[7., 8.]).unwrap();
        fft_history.push(&[9., 10.]).unwrap();

        println!("{:?}, {:?}", fft_history, &fft_history[1]);

        let audio_data = AudioData {
            buffer: [0.0; 2048],
            timestamp: 0,
            program_start: start_time,
        };

        let (audio_buffer, fft_buffer, audio_history_buffer) = Suono::attach_gpu_buffers(
            &audio_data.buffer,
            &fft_results,
            &audio_history,
            config.colors,
        );

        return Suono {
            sample_interpolation_scalar: config.sample_interpolation_scalar,
            background_color: config.background_color,
            gain: config.gain,
            background_image,
            audio_buffer,
            fft_buffer,
            fft_history,
            audio_history_buffer,
            graphic_elements,
            window_width,
            window_height,
            target_frequencies,
            audio_data,
            audio_history,
            fft_results,
            rl,
            thread,
            audio_data_arc,
        };
    }

    pub fn update_screen_dimensions(&mut self) {
        let new_width = self.rl.get_screen_width() as f32;
        let new_height = self.rl.get_screen_height() as f32;

        if new_width != self.window_width || new_height != self.window_height {
            self.window_width = new_width;
            self.window_height = new_height;
            self.graphic_elements
                .iter_mut()
                .for_each(|g| g.update(new_width, new_height, &mut self.rl, &self.thread));
        }
    }

    pub fn update_audio(&mut self) {
        self.update_audio_buffer();

        let res = fft_custom(self.audio_data.buffer.as_slice(), &self.target_frequencies);
        for (i, f) in self.fft_results.iter_mut().enumerate() {
            *f = res[i] + *f * self.sample_interpolation_scalar;
        }

        unsafe {
            ffi::rlBindShaderBuffer(self.audio_buffer, AUDIO_BUFFER_ID);
            ffi::rlUpdateShaderBuffer(
                self.audio_buffer,
                self.audio_data.buffer.as_ptr() as *const c_void,
                self.audio_data.buffer.len() as u32 * size_of::<f32>() as u32,
                0,
            );
            ffi::rlBindShaderBuffer(self.fft_buffer, FFT_BUFFER_ID);
            ffi::rlUpdateShaderBuffer(
                self.fft_buffer,
                self.fft_results.as_ptr() as *const c_void,
                self.fft_results.len() as u32 * size_of::<f32>() as u32,
                0,
            );
            ffi::rlBindShaderBuffer(self.audio_history_buffer, AUDIO_HISTORY_BUFFER_ID);
            ffi::rlUpdateShaderBuffer(
                self.audio_history_buffer,
                self.audio_history.as_ptr() as *const c_void,
                self.audio_history.len() as u32 * size_of::<f32>() as u32,
                0,
            );
        }
    }

    fn update_audio_buffer(&mut self) {
        let m = self.audio_data_arc.lock().expect("could not lock");
        let program_runtime = (Instant::now() - self.audio_data.program_start).as_nanos() as u64;
        if program_runtime - m.timestamp > ONE_HUNDRED_MS {
            self.audio_data.buffer.fill(0.0);
        } else {
            self.audio_data.buffer.copy_from_slice(m.buffer.as_slice());
            self.audio_data.timestamp = m.timestamp;
        }
        drop(m);
        self.update_audio_history();
    }

    //homemade circular buffer that records volume as a function of time
    fn update_audio_history(&mut self) {
        self.audio_data
            .buffer
            .chunks(350)
            .map(|c| c.iter().map(|s| f32::abs(*s)).sum::<f32>())
            .for_each(|n| {
                let index = self
                    .audio_history
                    .iter()
                    .position(|i| *i == 0.0)
                    .unwrap_or(0);
                if index != self.audio_history.len() - 1 {
                    self.audio_history[index] = n / 100. + 0.005;
                    self.audio_history[index + 1] = 0.0;
                } else {
                    self.audio_history[index] = n / 100. + 0.005;
                    self.audio_history[0] = 0.0;
                }
            });
    }

    fn create_graphic_elements(
        config: &Config,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Vec<Graphic> {
        let graphic_elemts = config
            .graphics
            .iter()
            .map(|cfg| {
                Graphic::init(
                    &cfg,
                    config.sample_count,
                    config.timeline_length,
                    config.gain,
                    rl,
                    thread,
                )
            })
            .collect::<Vec<Graphic>>();
        return graphic_elemts;
    }

    fn attach_gpu_buffers(
        audio_data: &[f32; 2048],
        fft_results: &Vec<f32>,
        audio_history: &Vec<f32>,
        colors: Vec<Vector4>,
    ) -> (u32, u32, u32) {
        let audio_buffer = unsafe {
            rlLoadShaderBuffer(
                audio_data.len() as u32 * size_of::<f32>() as u32,
                0 as *const c_void,
                RL_STREAM_DRAW as i32,
            )
        };
        let fft_buffer = unsafe {
            rlLoadShaderBuffer(
                fft_results.len() as u32 * size_of::<f32>() as u32,
                0 as *const c_void,
                RL_STREAM_DRAW as i32,
            )
        };
        let audio_history_buffer = unsafe {
            rlLoadShaderBuffer(
                audio_history.len() as u32 * size_of::<f32>() as u32,
                0 as *const c_void,
                RL_STREAM_DRAW as i32,
            )
        };
        let color_buffer = unsafe {
            rlLoadShaderBuffer(
                colors.len() as u32 * size_of::<Vector4>() as u32,
                0 as *const c_void,
                RL_STATIC_DRAW as i32,
            )
        };
        unsafe {
            ffi::rlBindShaderBuffer(color_buffer, COLOR_BUFFER_ID);
            ffi::rlUpdateShaderBuffer(
                color_buffer,
                colors.as_ptr() as *const c_void,
                colors.len() as u32 * size_of::<Vector4>() as u32,
                0,
            );
        }
        return (audio_buffer, fft_buffer, audio_history_buffer);
    }

    pub fn render(&mut self) {
        let fps_string = "FPS: ".to_owned() + &self.rl.get_fps().to_string().as_str();
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
                    width: self.window_width,
                    height: self.window_height,
                },
                Vector2::ZERO,
                0.0,
                Color::WHITE,
            )
        }

        let timeline_offset = self
            .audio_history
            .iter()
            .position(|i| *i == 0.0)
            .unwrap_or(0) as i32;

        for graphic in &mut self.graphic_elements {
            graphic.render(&mut d, self.gain, timeline_offset);
        }
        unsafe { ffi::EndShaderMode() };

        d.draw_text(&fps_string, 10, 10, 16, Color::WHITE);
    }
}

pub struct AudioData {
    pub buffer: [f32; 2048],
    timestamp: u64,
    program_start: Instant,
}

fn pipewire_init(audio_data_arc: Arc<Mutex<AudioData>>) {
    let host = cpal::default_host();
    let device = host
        .device_by_id(&DeviceId::new(host.id(), "pipewire"))
        .expect("pipewire not found");

    create_audio_stream(device, audio_data_arc);

    let _com = std::process::Command::new("bash")
        .arg("-c")
        .arg("~/rust/suono/src/pw_redirect_sources.sh")
        .output()
        .unwrap();
}

fn create_audio_stream(device: cpal::Device, audio_data_arc: Arc<Mutex<AudioData>>) {
    let stream_config = StreamConfig {
        channels: 2,
        sample_rate: 44100,
        buffer_size: cpal::BufferSize::Fixed(1124),
    };

    let stream = device
        .build_input_stream(
            stream_config,
            move |source_data: &[f32], s| {
                let mut m = audio_data_arc.lock().expect("could not lock");
                let waveform_root = source_data
                    .iter()
                    .enumerate()
                    .take(200)
                    .min_by(|(_i1, s1), (_i2, s2)| s1.abs().total_cmp(&s2.abs()))
                    .map(|(i, _s)| i)
                    .unwrap_or(0usize);
                m.buffer
                    .copy_from_slice(&source_data[waveform_root..waveform_root + 2048]);
                m.timestamp = s.timestamp().callback.as_nanos() as u64
            },
            move |error| println!("Audio stream error {}", error),
            Some(std::time::Duration::from_secs(1)),
        )
        .expect("failed to make de strim");

    stream.play().expect("unable to play stream");
    Box::leak(Box::new(stream));
}

// theory: https://en.wikipedia.org/wiki/Fast_Fourier_transform#FFT_algorithms_specialized_for_real_or_symmetric_data
// pretty standard implementation with a bit of e^ix = cos(x) + i*sin(x) shenanigans
fn fft_custom(data: &[f32], target_frequencies: &Vec<f32>) -> Vec<f32> {
    target_frequencies
        .iter()
        .map(|target_f| {
            let forier_data = data
                .into_iter()
                .step_by(6)
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
            ) * f32::powf(*target_f, 0.17)
                / 4.0;
        })
        .collect::<Vec<f32>>()
}

// return a Vec of length "sample_count" with number logarithmically destributed between 0 and 20*e^(6.3) witch is 10 891
// for no other readon than that it looks good
pub fn create_target_frequencies(sample_count: usize) -> Vec<f32> {
    (0..sample_count)
        .map(|i| 60.0 * E.powf(5.1 * i as f32 / sample_count as f32))
        .collect::<Vec<f32>>()
}
