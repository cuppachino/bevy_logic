use bevy::{ prelude::*, render::render_resource::{ AsBindGroup, ShaderRef } };

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LogicGateMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl From<Color> for LogicGateMaterial {
    fn from(value: Color) -> Self {
        Self {
            color: value,
            ..Default::default()
        }
    }
}

impl Default for LogicGateMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            color_texture: None,
            alpha_mode: AlphaMode::Blend,
        }
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for LogicGateMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/logic_gate_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
