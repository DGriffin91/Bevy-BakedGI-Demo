use std::{num::NonZeroU8, ops::RangeInclusive};

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, FilterMode, Sampler,
            SamplerBindingType, SamplerDescriptor, ShaderStages, TextureSampleType,
            TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};

#[derive(Debug, Clone, Copy, AsStd140)]
pub struct MaterialSetProp {
    pub scale: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub blend: f32,
}

#[derive(Debug, Clone, Copy, AsStd140)]
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

#[derive(Debug, Clone)]
pub struct MaterialTexture {
    pub texture_handle: Option<Handle<Image>>,
    pub path: String,
    pub name: String,
}

impl MaterialTexture {
    pub fn new(asset_server: &Res<AssetServer>, path: &str, name: &str) -> Self {
        MaterialTexture {
            texture_handle: Some(asset_server.load(path)),
            path: String::from(path),
            name: String::from(name),
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "4ee9c361-1124-4113-890e-197d82b00123"]
pub struct CustomMaterial {
    pub material_properties: MaterialProperties,
    pub textures: [MaterialTexture; 5],
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

fn get_custom_sampler(render_device: &mut Res<RenderDevice>) -> Sampler {
    let mut sampler_descriptor = SamplerDescriptor::default();

    sampler_descriptor.address_mode_u = AddressMode::Repeat;
    sampler_descriptor.address_mode_v = AddressMode::Repeat;
    sampler_descriptor.mipmap_filter = FilterMode::Linear;
    sampler_descriptor.mag_filter = FilterMode::Linear;
    sampler_descriptor.min_filter = FilterMode::Linear;
    sampler_descriptor.anisotropy_clamp = NonZeroU8::new(16);

    render_device.create_sampler(&sampler_descriptor)
}

impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<MaterialPipeline<Self>>,
        SRes<RenderAssets<Image>>,
    );
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, material_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let material_properties = &material.material_properties;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: material_properties.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        //TODO don't allocate?
        let mut entries = Vec::new();
        let mut samplers = Vec::new();

        for _ in &material.textures {
            samplers.push(get_custom_sampler(render_device));
        }

        entries.push(BindGroupEntry {
            binding: entries.len() as u32,
            resource: buffer.as_entire_binding(),
        });

        for (texture, sampler) in material.textures.iter().zip(&samplers) {
            let (texture_view, _sampler) = if let Some(result) = material_pipeline
                .mesh_pipeline
                .get_image_texture(gpu_images, &texture.texture_handle)
            {
                result
            } else {
                return Err(PrepareAssetError::RetryNextUpdate(material.clone()));
            };

            entries.push(BindGroupEntry {
                binding: entries.len() as u32,
                resource: BindingResource::TextureView(texture_view),
            });

            entries.push(BindGroupEntry {
                binding: entries.len() as u32,
                resource: BindingResource::Sampler(sampler),
            });
        }

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &entries,
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for CustomMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        let r = Some(asset_server.load("shaders/custom_material.wgsl"));
        asset_server.watch_for_changes().unwrap();
        r
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        let texture_quantity = 5;

        //TODO don't allocate?
        let mut entries = Vec::new();

        entries.push(BindGroupLayoutEntry {
            binding: entries.len() as u32,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(
                    0, //TODO is it fine the leave this as 0? (was 148)
                ),
            },
            count: None,
        });

        for _ in 0..texture_quantity {
            entries.push(BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            });
            entries.push(BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            });
        }

        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &entries,
            label: None,
        })
    }
}
