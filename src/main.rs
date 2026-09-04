mod config;
mod graphics;
mod pw_connections;
mod spectrogram;
mod suono;
mod timeline;
mod waveform;

use config::load_config_file;

use pw_connections::connect_to_channels;
use suono::Suono;

#[allow(unused_variables)]
#[allow(dead_code)]

/*
TODO:
    Background img              ✓
    github + README.md          ✓
    build.rs                    ✓
    color blending              ✓
    PW compatability            x
    optimizing+parallelizing    ✓
    steal from cavalier         x
    timeline simplification     ✓
    waveform simplification     x
    spectrogram simplification  x

    maybe:
    razor copy                  x
    temperature spectrogram     x
    color glow                  x
    shaders                     x
*/

fn main() {
    //unimplemented!("amogus");
    let config = load_config_file();
    println!("{config:#?}");
    let mut suono = Suono::init(config);

    while !suono.rl.window_should_close() {
        suono.update_screen_dimensions();
        suono.update_audio_data();
        suono.render();
        connect_to_channels();
        // panic!()
    }
}
