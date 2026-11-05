mod config;
mod graphics;
mod pw_connections;
mod suono;

use crate::pw_connections::connect_to_channels;
use config::load_config_file;
use suono::Suono;

/*
TODO:
    github + README.md          x
    example configs             x
    pw-robustifying             x

    shaders:
    color glow                  ✓
    util funcs                  x
    timeline color hold         x


    low_pri:
    shader fft                  x
    razor copy                  x
    temperature spectrogram     x
    post_processing             x

*/

fn main() {
    let config = load_config_file();
    println!("{config:#?}");

    let mut suono = Suono::init(config);

    let mut frame_counter = 0;

    while !suono.rl.window_should_close() {
        suono.update_screen_dimensions();
        suono.update_audio();
        suono.render();

        if frame_counter % 30 == 0 {
            connect_to_channels();
        }
        frame_counter += 1;
    }
}
