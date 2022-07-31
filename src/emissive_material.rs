use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "4ee9c361-1124-4113-890e-197d82b00321"]
pub struct EmissiveMaterial {
    #[uniform(0)]
    pub emissive: Color,
    #[texture(1)]
    #[sampler(2)]
    pub emissive_texture: Option<Handle<Image>>,
}

impl Material for EmissiveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/emissive_material.wgsl".into()
    }
}
