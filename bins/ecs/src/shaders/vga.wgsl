// Vertex Shader

struct RetroUniform {
    window_aspect: f32,
    retro_aspect: f32,
};

@group(0) @binding(0)
var<uniform> aspect: RetroUniform;

@group(0) @binding(1)
var myTexture: texture_2d<f32>;

@group(0) @binding(2)
var mySampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Generate a single big triangle. We'll scale it to maintain the retro aspect ratio.
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var out: VertexOutput;

    // Positions in clip space:
    var base_positions = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0),
        vec2( 1.0, -1.0),
        vec2( 1.0,  1.0),
        vec2(-1.0, -1.0),
        vec2( 1.0,  1.0),
        vec2(-1.0,  1.0),
    );

    // Matching UVs that can go beyond [0..1] if you want the clamp/fade logic:
    // Typically you'd just do [0,1], [1,1], [1,0] etc.
    var base_uv = array<vec2<f32>, 6>(
        vec2(0.0, 1.0),
        vec2(1.0, 1.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(1.0, 0.0),
        vec2(0.0, 0.0),
    );

    let pos = base_positions[i];
    let uv  = base_uv[i];

    // Determine how to scale to preserve 320:200 = 1.6 in a window_aspect of, say, 3.555
    var scale = vec2<f32>(1.0, 1.0);
    if aspect.window_aspect > aspect.retro_aspect {
        // Window is "too wide" => scale X
        scale.x = aspect.retro_aspect / aspect.window_aspect;
    } else {
        // Window is narrower => scale Y
        scale.y = aspect.window_aspect / aspect.retro_aspect;
    }

    // Scale the triangle so it doesn't fill horizontally if the aspect ratio is too wide.
    let scaled_pos = pos * scale;

    out.clip_position = vec4<f32>(scaled_pos, 0.0, 1.0);
    out.uv = uv;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // If outside [0..1], return black
    if (in.uv.x < 0.0 || in.uv.x > 1.0 || in.uv.y < 0.0 || in.uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // Otherwise sample the retro texture
    return textureSample(myTexture, mySampler, in.uv);
}
