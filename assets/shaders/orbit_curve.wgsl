#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_view_bindings::view

struct OrbitUniform {
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> orbit_uniform: OrbitUniform;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mesh_functions::get_world_from_local(input.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(input.position, 1.0));
    out.clip_position = view.clip_from_world * world_position;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return orbit_uniform.color;
}

