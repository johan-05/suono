mod config;
use config::load_config_file;

mod suono;
use suono::Suono;

mod graphics;

fn main() {
    let config = load_config_file();
    println!("{:?}", config);

    let mut suono = Suono::init(config);

    while !suono.rl.window_should_close() {
        suono.update_screen_dimensions();

        suono.update_audio_data();

        suono.render()
    }
}

/*fn fft_normalized(data: &[(f32, f32)], target_frequencies: &Vec<f32>) -> Vec<f32> {
    target_frequencies
        .par_iter()
        .map(|target_f| {
            let forier_data = data
                .into_iter()
                .map(|d| {
                    d.1 * E.powc(Complex {
                        re: 0.0,
                        im: -2.0 * PI * d.0 * target_f,
                    })
                })
                .sum::<Complex<f32>>();
            let mass_center = forier_data * (1.0 / data.len() as f32);
            f32::sqrt(mass_center.norm())
        })
        .collect::<Vec<f32>>()
}*/
