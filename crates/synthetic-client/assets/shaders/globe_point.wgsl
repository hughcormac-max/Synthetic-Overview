#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_view_bindings::view

struct GlobeUniform {
    base_color: vec4<f32>,
    atmosphere_color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> globe_uniform: GlobeUniform;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // World space transformations
    let model = mesh_functions::get_world_from_local(input.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(input.position, 1.0)).xyz;
    let world_normal = mesh_functions::mesh_normal_local_to_world(input.normal, input.instance_index);

    // Exact perspective view vector from vertex to camera
    let camera_position = view.world_position;
    let view_vector = normalize(camera_position - world_position);

    // Backface culling dot product test
    let facing = dot(world_normal, view_vector);

    if facing <= 0.0 {
        // Move vertex behind far clip plane to cull
        out.clip_position = vec4<f32>(0.0, 0.0, 2.0, 1.0);
        out.color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        return out;
    }

    // Project visible vertex to clip space via view matrix
    out.clip_position = view.clip_from_world * vec4<f32>(world_position, 1.0);

    // Limb darkening and illumination curve
    let intensity = clamp(facing, 0.2, 1.0);
    out.color = vec4<f32>(globe_uniform.base_color.rgb * intensity, globe_uniform.base_color.a);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
