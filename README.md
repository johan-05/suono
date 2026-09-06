<p align="center">
  <img
    width="900"
    src="https://raw.githubusercontent.com/johan-05/suono/main/assets/suono_logo.png"
    alt="Suono - Audio visualized"
  />
</p>

# Suono &nbsp; - &nbsp; A modern GPU-rendered audio vizualizer
## Installation
Clone the repo:
```sh
git clone https://github.com/johan-05/suono
```
Then simply cd into it and build it:
```sh
cd suono
cargo build --release
```

## configuration
Configuration happens in `XDG_CONFIG_HOME/suono.conf`, usually located at `~/.config/suono.conf`

The [config templates](https://github.com/johan-05/suono/assets) are a great place to take inspiration.

The basic structure of the file is of one `[global]` section and up to four `[graphic]` sections

### Global
Settings that affect all graphics
```conf
[global]
background_color = 11711A                      # Color annotated in Hex format without the "#"
background_image = /home/USER/path/to/img.png  # Use absolute paths, not relative ones
sample_count = 200                             # Dictates how many frequencies a Spectrogram samples
sample_interpolation_scalar = 0.75             # Adjusts the delay effect of Spectrograms
timeline_length = 400                          # Sets the amount of samples the Timeline shows
line_width = 1.8                               # Sets the thickness of lines
gain = 1.0                                     # Adjusts audio gain
```

### Graphics
Suono supports rendering up to four graphics simultaneously. Each one is market with a **`[Graphic]`** annotation <br />and crutially a **`type`** field which makes it into a **`Spectrogram`**, a **`Waveform`** or a **`Timeline`** <br />
A graphic-element can be configured with the following attributes:

| Field     | Options                         |
| ----------| ------------------------------- |
| type      | `Spectrogram`, `Waveform` or `Timeline` |
| position  | `full`, `left`, `right`, `top`, `bottom`, `topleft`,<br/> `topright`, `bottomleft` or `bottomright` |
| background_color | `none` Background is same as window background <br/> `RRGGBB` Color in Hex format |
| style  | `lines`, `graph`, `dots`, `dots_single` |
| color_scheme | `[RRGGBB, RRGGBB, RRGGBB]` |
| color_blend | `true` or `false` |

Here is an example of what a graphics-element config could look like:
```conf
[graphic]
type = spectrogram
position = full
background_color = none
style = lines
color_scheme = [02A797, 865991, CB2080]
color_blend = true
```

### Dependencies
Suono is currently only compatible with linux and pipewire. Make shure you have `pw-link` and `cargo` installed. Suono uses raylib for graphics which works best with OpenGL 3.3 but also works with older versions.

