use crate::config::*;
use ffi::rlSetShader;
use raylib::ffi::ShaderUniformDataType::{
    SHADER_UNIFORM_FLOAT, SHADER_UNIFORM_INT, SHADER_UNIFORM_UINT, SHADER_UNIFORM_VEC2,
};
use raylib::prelude::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::LazyLock;

pub struct Graphic {
    //config
    texture: RenderTexture2D,
    shader: ffi::Shader,
    position: GraphicPosition,
    //state
    width: f32,
    height: f32,
    topleft: (f32, f32),
}

impl Graphic {
    pub fn init(
        config: &GraphicConfig,
        sample_count: usize,
        timeline_length: usize,
        gain: f32,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Graphic {
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

        let shader = load_shader(config, rl, thread);

        unsafe {
            buffer_uniforms(shader, config, sample_count, timeline_length, gain);
        }

        let texture = rl
            .load_render_texture(thread, width as u32, height as u32)
            .expect("amogus");

        return Graphic {
            texture: texture,
            shader: shader,
            position: config.position,
            width: width,
            height: height,
            topleft: topleft,
            //color_idx: config.color_scheme.color_idx,
            //color_count: config.color_scheme.color_count,
        };
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, _gain: f32, timeline_offset: i32) {
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
            let timeline_offset_loc = ffi::GetShaderLocation(self.shader, TIMELINE_OFFSET.as_ptr());
            ffi::SetShaderValue(
                self.shader,
                timeline_offset_loc,
                &timeline_offset as *const _ as *const c_void,
                SHADER_UNIFORM_INT as i32,
            );
        }

        d.draw_texture_ex(&self.texture, self.topleft, 0.0, 1.0, Color::BLACK);
    }

    pub fn update(
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

fn load_shader(
    config: &GraphicConfig,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> ffi::Shader {
    let fs_path = match (config.graphics_type, config.style) {
        (GraphicType::Spectrogram, GraphicStyle::Graph) => {
            Some("/home/johannes/rust/suono/shaders/spectrogram_graph.glsl")
        }
        (GraphicType::Spectrogram, GraphicStyle::Lines) => {
            Some("/home/johannes/rust/suono/shaders/spectrogram_lines.glsl")
        }
        (GraphicType::Spectrogram, GraphicStyle::Full) => {
            Some("/home/johannes/rust/suono/shaders/spectrogram_full.glsl")
        }
        (GraphicType::Spectrogram, GraphicStyle::Dots) => {
            Some("/home/johannes/rust/suono/shaders/spectrogram_dots.glsl")
        }
        (GraphicType::Waveform, GraphicStyle::Graph) => {
            Some("/home/johannes/rust/suono/shaders/waveform_graph.glsl")
        }
        (GraphicType::Waveform, GraphicStyle::Lines) => {
            Some("/home/johannes/rust/suono/shaders/waveform_lines.glsl")
        }
        (GraphicType::Waveform, GraphicStyle::Dots) => {
            Some("/home/johannes/rust/suono/shaders/waveform_dots.glsl")
        }
        (GraphicType::Waveform, GraphicStyle::DotsSingle) => {
            Some("/home/johannes/rust/suono/shaders/waveform_dots_single.glsl")
        }
        (GraphicType::Timeline, GraphicStyle::Lines) => {
            Some("/home/johannes/rust/suono/shaders/timeline_lines.glsl")
        }
        (GraphicType::Timeline, GraphicStyle::Graph) => {
            Some("/home/johannes/rust/suono/shaders/timeline_graph.glsl")
        }
        (GraphicType::Timeline, GraphicStyle::Full) => {
            Some("/home/johannes/rust/suono/shaders/timeline_full.glsl")
        }

        _ => Some("/home/johannes/rust/suono/shaders/spectrogram_lines.glsl"),
    };
    unsafe { rl.load_shader(thread, None, fs_path).unwrap() }
}

unsafe fn buffer_uniforms(
    shader: ffi::Shader,
    config: &GraphicConfig,
    sample_count: usize,
    timeline_length: usize,
    gain: f32,
) {
    unsafe {
        let should_glow = config.color_scheme.glow as i32;
        let should_glow_loc = ffi::GetShaderLocation(shader, SHOULD_GLOW.as_ptr());
        ffi::SetShaderValue(
            shader,
            should_glow_loc,
            &should_glow as *const _ as *const c_void,
            SHADER_UNIFORM_INT as i32,
        );

        let color_idx_loc = ffi::GetShaderLocation(shader, COLOR_IDX.as_ptr());
        ffi::SetShaderValue(
            shader,
            color_idx_loc,
            &config.color_scheme.color_idx as *const _ as *const c_void,
            SHADER_UNIFORM_INT as i32,
        );

        let color_count_loc = ffi::GetShaderLocation(shader, COLOR_COUNT.as_ptr());
        ffi::SetShaderValue(
            shader,
            color_count_loc,
            &config.color_scheme.color_count as *const _ as *const c_void,
            SHADER_UNIFORM_INT as i32,
        );

        let line_width_loc = ffi::GetShaderLocation(shader, LINE_WIDTH.as_ptr());
        ffi::SetShaderValue(
            shader,
            line_width_loc,
            &config.line_width as *const _ as *const c_void,
            SHADER_UNIFORM_FLOAT as i32,
        );

        let sample_count_loc = ffi::GetShaderLocation(shader, SAMPLE_COUNT.as_ptr());
        ffi::SetShaderValue(
            shader,
            sample_count_loc,
            &sample_count as *const _ as *const c_void,
            SHADER_UNIFORM_UINT as i32,
        );
        let timeline_length_loc = ffi::GetShaderLocation(shader, TIMELINE_LENGTH.as_ptr());
        ffi::SetShaderValue(
            shader,
            timeline_length_loc,
            &timeline_length as *const _ as *const c_void,
            SHADER_UNIFORM_UINT as i32,
        );
        let gain_loc = ffi::GetShaderLocation(shader, GAIN.as_ptr());
        ffi::SetShaderValue(
            shader,
            gain_loc,
            &gain as *const _ as *const c_void,
            SHADER_UNIFORM_FLOAT as i32,
        );
        let timeline_offset_loc = ffi::GetShaderLocation(shader, TIMELINE_OFFSET.as_ptr());
        let zero = 0i32;
        ffi::SetShaderValue(
            shader,
            timeline_offset_loc,
            &zero as *const _ as *const c_void,
            SHADER_UNIFORM_INT as i32,
        );
    }
}

static SHOULD_GLOW: LazyLock<CString> =
    LazyLock::new(|| CString::new("should_glow").unwrap_or(CString::default()));
static TEX_DIM: LazyLock<CString> =
    LazyLock::new(|| CString::new("Tex_dim").unwrap_or(CString::default()));
static COLOR_IDX: LazyLock<CString> =
    LazyLock::new(|| CString::new("color_offset").unwrap_or(CString::default()));
static COLOR_COUNT: LazyLock<CString> =
    LazyLock::new(|| CString::new("color_count").unwrap_or(CString::default()));
static LINE_WIDTH: LazyLock<CString> =
    LazyLock::new(|| CString::new("line_width").unwrap_or(CString::default()));
static SAMPLE_COUNT: LazyLock<CString> =
    LazyLock::new(|| CString::new("sample_count").unwrap_or(CString::default()));
static TIMELINE_LENGTH: LazyLock<CString> =
    LazyLock::new(|| CString::new("timeline_length").unwrap_or(CString::default()));
static GAIN: LazyLock<CString> =
    LazyLock::new(|| CString::new("gain").unwrap_or(CString::default()));
static TIMELINE_OFFSET: LazyLock<CString> =
    LazyLock::new(|| CString::new("timeline_offset").unwrap_or(CString::default()));
