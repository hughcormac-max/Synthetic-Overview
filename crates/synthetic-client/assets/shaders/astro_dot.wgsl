#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_view_bindings::view

struct AstroDotUniform {
    color: vec4<f32>,
    size_px: f32,
};

@group(2) @binding(0)
var<uniform> dot_uniform: AstroDotUniform;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) offset: vec2<f32>,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let model = mesh_functions::get_world_from_local(input.instance_index);
    let world_center = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    let clip_center = view.clip_from_world * vec4<f32>(world_center, 1.0);

    let viewport_size = max(view.viewport.zw, vec2<f32>(1.0, 1.0));
    let pixel_scale = 2.0 * dot_uniform.size_px / viewport_size;

    out.clip_position = vec4<f32>(
        clip_center.x + input.position.x * pixel_scale.x * clip_center.w,
        clip_center.y + input.position.y * pixel_scale.y * clip_center.w,
        clip_center.z,
        clip_center.w
    );
    out.offset = input.position.xy;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist_sq = in.offset.x * in.offset.x + in.offset.y * in.offset.y;
    if dist_sq > 1.0 {
        discard;
    }
    return dot_uniform.color;
}

