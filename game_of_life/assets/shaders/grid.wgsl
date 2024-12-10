//
// Mostly based on this tutorial:
// https://www.youtube.com/watch?v=peQaxcQ89SA
//

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tile_size: f32;

const tau = 6.283185307179586;
const intensity = .01;
const thiccness = .07;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let coords = mesh.world_position / tile_size;

    let line = cos(coords * tau);
    let alpha = smoothstep(1.0 - thiccness, 1.0, max(line.x, line.y));

    return vec4f(intensity, intensity, intensity, alpha);
}

