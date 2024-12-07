#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tile_size: f32;
@group(2) @binding(1) var<uniform> half_max_width: f32;
@group(2) @binding(2) var<uniform> half_max_height: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let intensity = 0.01;

    let x_dist = 1.0 - (abs(half_max_width - mesh.world_position.x) % tile_size) / tile_size;
    let y_dist = 1.0 - (abs(half_max_height - mesh.world_position.y) % tile_size) / tile_size;

    let dist = max(x_dist, y_dist);

    let val = smoothstep(0.88, 1.0, dist);

    return vec4f(intensity, intensity, intensity, val);
}

