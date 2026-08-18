<p align="center">
  <img
    width="400"
    src="https://github.com/johan-05/suono/assets/suono_logo.png"
    alt="Starship – Cross-shell prompt"
  />
</p>
# Suono - A modern GPU-rendered audio vizualizer
## Installation
Clone the repo:
```sh
git clone https://github.com/johan-05/suono
```
Then simply cd into it and run it:
```sh
cd suono
cargo run --release
```

## configuration
Configuration happens in `XDG_CONFIG_HOME/suono.conf`

The [default config-file](https://github.com/johan-05/suono/assets/defaul_config_file) is a great place to start. <br />
The basic construction of the file is of one **`[global]´** section and up to four **`graphic`** sections

### Global
Settings that affect all graphics
```conf
[global]
background_color = 11711A                      # Color annotated in Hex format without the "#"
background_image = /home/USER/path/to/img.png  # Use absolute paths, not relative ones
sample_count = 200                             # Dictates how many frequencies a Spectrogram samples
sample_interpolation_scalar = 0.75             # Adjusts the delay effect of Spectrograms
timeline_length = 800                          # Sets the amount of samples the Timeline shows
```

### Graphics
Suono supports rendering up to four graphics simultaneously. Each one is market with a **`[Graphic]`** annotation <br />and crutially a **`type`** field which makes it into a **`Spectrogram`**, a **`Waveform`** or a **`Timeline`**
A graphic-element can be configured with the following attributes:

| Field     | Options                         |
| ----------| ------------------------------- |


