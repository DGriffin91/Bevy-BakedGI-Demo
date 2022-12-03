use std::{num::NonZeroU8, ops::RangeInclusive};

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{
            AddressMode, AsBindGroup, FilterMode, SamplerDescriptor, ShaderRef, ShaderType,
        },
        texture::ImageSampler,
    },
};

use bevy_egui::egui;

#[derive(ShaderType, Debug, Clone, Copy)]
pub struct MaterialSetProp {
    pub scale: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub blend: f32,
}

fn log_slider<Num: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut Num,
    range: RangeInclusive<Num>,
    text: &str,
) {
    ui.add(egui::Slider::new(value, range).logarithmic(true).text(text));
}

impl MaterialSetProp {
    pub fn build_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.label(label);
        log_slider(ui, &mut self.scale, 0.0..=100.0, "scale");
        log_slider(ui, &mut self.contrast, 0.0..=10.0, "contrast");
        log_slider(ui, &mut self.brightness, 0.0..=40.0, "brightness");
        log_slider(ui, &mut self.blend, 0.0..=1.0, "blend");
    }
}

#[derive(ShaderType, Debug, Clone, Copy)]
pub struct MaterialProperties {
    pub lightmap: MaterialSetProp,
    pub base_a: MaterialSetProp,
    pub base_b: MaterialSetProp,
    pub vary_a: MaterialSetProp,
    pub vary_b: MaterialSetProp,
    pub reflection: MaterialSetProp,
    pub walls: MaterialSetProp,
    pub reflection_mask: MaterialSetProp,
    pub mist: MaterialSetProp,
    pub directional_light_blend: f32,
    //pub directional_light_color: Vec3,
}

impl MaterialProperties {
    pub fn build_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Debug Print").clicked() {
            dbg!(&self);
        }
        self.lightmap.build_ui(ui, "lightmap");
        self.base_a.build_ui(ui, "base_a");
        self.base_b.build_ui(ui, "base_b");
        self.vary_a.build_ui(ui, "vary_a");
        self.vary_b.build_ui(ui, "vary_b");
        self.reflection.build_ui(ui, "reflection");
        self.reflection_mask.build_ui(ui, "reflection_mask");
        self.walls.build_ui(ui, "walls");
        self.mist.build_ui(ui, "mist");
        ui.label("-------------");
        ui.add(
            egui::Slider::new(&mut self.directional_light_blend, 0.0..=5.0)
                .text("directional_light_blend"),
        );
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "4ee9c361-1124-4113-890e-197d82b00123"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub material_properties: MaterialProperties,
    #[texture(1)]
    #[sampler(2)]
    pub lightmap: Option<Handle<Image>>,
    pub lightmap_path: String,
    #[texture(3)]
    #[sampler(4)]
    pub base: Option<Handle<Image>>,
    pub base_path: String,
    #[texture(5)]
    #[sampler(6)]
    pub vary: Option<Handle<Image>>,
    pub vary_path: String,
    #[texture(7)]
    #[sampler(8)]
    pub reflection: Option<Handle<Image>>,
    pub reflection_path: String,
    #[texture(9)]
    #[sampler(10)]
    pub walls: Option<Handle<Image>>,
    pub walls_path: String,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

fn load_button(
    ui: &mut egui::Ui,
    com: &mut Commands,
    ass: &AssetServer,
    name: &str,
    path: &mut String,
    image_handle: &mut Option<Handle<Image>>,
) {
    ui.label(name);
    ui.horizontal(|ui| {
        ui.text_edit_singleline(path);
        if ui.button("LOAD").clicked() {
            *image_handle = Some(load_mark(com, ass, &*path));
        }
    });
}

impl CustomMaterial {
    pub fn build_ui(&mut self, ui: &mut egui::Ui, com: &mut Commands, ass: &Res<AssetServer>) {
        self.material_properties.build_ui(ui);
        ui.label("CustomMaterial");
        load_button(
            ui,
            com,
            ass,
            "lightmap",
            &mut self.lightmap_path,
            &mut self.lightmap,
        );
        load_button(ui, com, ass, "base", &mut self.base_path, &mut self.base);
        load_button(ui, com, ass, "vary", &mut self.vary_path, &mut self.vary);
        load_button(
            ui,
            com,
            ass,
            "reflection",
            &mut self.reflection_path,
            &mut self.reflection,
        );
        load_button(ui, com, ass, "walls", &mut self.walls_path, &mut self.walls);
    }
}

pub fn load_mark(com: &mut Commands, ass: &AssetServer, path: &str) -> Handle<Image> {
    let handle = ass.load(path);
    com.spawn(NeedsTextureSetup(handle.clone()));
    handle
}

#[derive(Component)]
pub struct NeedsTextureSetup(Handle<Image>);

pub fn set_texture_settings(
    mut com: Commands,
    to_be_converted: Query<(Entity, &NeedsTextureSetup)>,
    mut images: ResMut<Assets<Image>>,
) {
    for (entity, needs_setup) in to_be_converted.iter() {
        if let Some(img) = images.get_mut(&needs_setup.0) {
            img.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                anisotropy_clamp: NonZeroU8::new(16),
                ..default()
            });
            com.entity(entity).despawn();
        }
    }
}
