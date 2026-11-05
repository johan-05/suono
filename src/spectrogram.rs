/*use crate::config::*;
use crate::graphics::Component;
use ffi::rlSetShader;
use itertools::Itertools;
use raylib::ffi::ShaderUniformDataType::{SHADER_UNIFORM_INT, SHADER_UNIFORM_VEC2};
use raylib::prelude::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::LazyLock;

#[allow(dead_code)]
pub struct Spectrogram {
    //config
    texture: RenderTexture2D,
    shader: ffi::Shader,
    background_color: Color,
    position: GraphicPosition,
    style: GraphicStyle,
    color_scheme: ColorScheme,
    //state
    width: f32,
    height: f32,
    topleft: (f32, f32),
}

//#[allow(dead_code)]
impl Spectrogram {
    pub fn init(
        config: GraphicConfig,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Box<dyn Component> {
        let (width, height) = match config.position {
            GraphicPosition::Full => (1280.0, 720.0),
            GraphicPosition::Top | GraphicPosition::Bottom => (1280.0, 360.0),
            GraphicPosition::Left | GraphicPosition::Right => (640.0, 720.0),
            _ => (640.0, 360.0),
        };

        let topleft = match config.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0.0, 0.0),
            GraphicPosition::Right | GraphicPosition::TopRight => (640.0, 0.0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0.0, 360.0),
            GraphicPosition::BottomRight => (640.0, 360.0),
        };

        let shader = unsafe {
            match config.style {
                GraphicStyle::Graph => rl
                    .load_shader(
                        thread,
                        None,
                        Some("/home/johannes/rust/suono/shaders/spectrogram_graph.glsl"),
                    )
                    .unwrap(),
                GraphicStyle::Dots => rl
                    .load_shader(
                        thread,
                        None,
                        Some("/home/johannes/rust/suono/shaders/spectrogram_dots.glsl"),
                    )
                    .unwrap(),
                _ => rl
                    .load_shader(
                        thread,
                        None,
                        Some("/home/johannes/rust/suono/shaders/spectrogram_lines.glsl"),
                    )
                    .unwrap(),
            }
        };

        let should_glow = config.color_scheme.glow as i32;
        unsafe {
            let should_glow_loc = ffi::GetShaderLocation(shader, SHOULD_GLOW.as_ptr());
            ffi::SetShaderValue(
                shader,
                should_glow_loc,
                &should_glow as *const _ as *const c_void,
                SHADER_UNIFORM_INT as i32,
            );
        }

        let texture = rl
            .load_render_texture(thread, width as u32, height as u32)
            .expect("amogus");

        return Box::new(Spectrogram {
            texture: texture,
            shader: shader,
            background_color: config.background_color,
            position: config.position,
            style: config.style,
            color_scheme: config.color_scheme,
            width: width,
            height: height,
            topleft: topleft,
        });
    }
}

impl Component for Spectrogram {
    fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        _fft_results: &Vec<f32>,
        _decoded_audio: &[f32],
        _audio_history: &Vec<f32>,
        _gain: f32,
    ) {
        unsafe { rlSetShader(self.shader.id, self.shader.locs) };

        let texture_dim = vec![self.width, self.height];

        unsafe {
            let texture_dim_loc = ffi::GetShaderLocation(self.shader, TEX_DIM.as_ptr());

            ffi::SetShaderValue(
                self.shader,
                texture_dim_loc,
                texture_dim.as_ptr() as *const c_void,
                SHADER_UNIFORM_VEC2 as i32,
            );
        }

        d.draw_texture_ex(
            &self.texture,
            self.topleft,
            0.0,
            1.0,
            //self.width / 1280.0,
            Color::BLACK,
        );
    }

    fn update(
        &mut self,
        new_width: f32,
        new_height: f32,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        (self.width, self.height) = match self.position {
            GraphicPosition::Full => (new_width, new_height),
            GraphicPosition::Top | GraphicPosition::Bottom => (new_width, new_height / 2.0),
            GraphicPosition::Left | GraphicPosition::Right => (new_width / 2.0, new_height),
            _ => (new_width / 2.0, new_height / 2.0),
        };

        self.topleft = match self.position {
            GraphicPosition::Full
            | GraphicPosition::TopLeft
            | GraphicPosition::Top
            | GraphicPosition::Left => (0.0, 0.0),
            GraphicPosition::Right | GraphicPosition::TopRight => (self.width, 0.0),
            GraphicPosition::BottomLeft | GraphicPosition::Bottom => (0.0, self.height),
            GraphicPosition::BottomRight => (self.width, self.height),
        };

        let texture = rl
            .load_render_texture(thread, self.width as u32, self.height as u32)
            .expect("amogus");
        self.texture = texture;
    }
}

static SHOULD_GLOW: LazyLock<CString> =
    LazyLock::new(|| CString::new("Should_glow").unwrap_or(CString::default()));
static TEX_DIM: LazyLock<CString> =
    LazyLock::new(|| CString::new("Tex_dim").unwrap_or(CString::default()));
*/
