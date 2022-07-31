#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct EmissiveMaterial {
    emissive: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: EmissiveMaterial;
@group(1) @binding(1)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(2)
var emissive_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(pow(textureSample(emissive_texture, emissive_sampler, in.uv).rgb, vec3<f32>(1.0)), 1.0);
}
