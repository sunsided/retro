// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// A single big triangle, generated entirely in the vertex shader.
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var out: VertexOutput;

    // Three positions, each in clip space. This covers more than [-1..1]^2 so it fills the screen.
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0),
    );

    // Matching UVs. You can map [0..1] fully, or go beyond to do fancy scrolling or letterboxing.
    var uvs = array<vec2<f32>, 3>(
        vec2(0.0, 1.0),
        vec2(2.0, 1.0),
        vec2(0.0, -1.0),
    );

    out.clip_position = vec4<f32>(positions[i], 0.0, 1.0);
    out.uv = uvs[i];
    return out;
}

// Fragment shader

@group(0) @binding(0)
var myTexture: texture_2d<f32>;

@group(0) @binding(1)
var mySampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Example: sample the texture with nearest or linear filter, etc.
    // If your UV coordinates go out of [0..1], you can clamp or return black.

    let color = textureSample(myTexture, mySampler, in.uv);
    return color;
}
