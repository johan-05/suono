mod config;
mod graphics;
mod spectrogram;
mod suono;
mod timeline;
mod waveform;
use std::time::Duration;

use config::load_config_file;

use suono::Suono;

#[allow(unused_variables)]
#[allow(dead_code)]

/*
TODO:
    Background img              ✓
    github + README.md          ✓
    build.rs                    x
    color blending              ✓
    PW compatability            x
    optimizing+parallelizing    x
    steal from cavalier         x

    maybe:
    modulate sampeling rate     x
    color glow                  x
    shaders                     x
*/

fn main() -> ! {
    let config = load_config_file();
    println!("{config:#?}");
    let mut suono = Suono::init(config);

    loop {
        suono.update_screen_dimensions();
        suono.update_audio_data();
        suono.render();
        std::thread::sleep(Duration::from_millis(14));
    }
}
