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

// Edge-preserving fragment shader

fn luminance(c: vec4<f32>) -> f32 {
    return dot(c.rgb, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fs_edge_preserving_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Return black if UV is out of bounds
    if (in.uv.x < 0.0 || in.uv.x > 1.0 || in.uv.y < 0.0 || in.uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // Determine texel size from the retro texture dimensions
    let texSize: vec2<f32> = vec2<f32>(textureDimensions(myTexture, 0));
    let pixelSize: vec2<f32> = 1.0 / texSize;

    // Sample the center and immediate neighbors
    let center = textureSample(myTexture, mySampler, in.uv);
    let left   = textureSample(myTexture, mySampler, in.uv - vec2(pixelSize.x, 0.0));
    let right  = textureSample(myTexture, mySampler, in.uv + vec2(pixelSize.x, 0.0));
    let up     = textureSample(myTexture, mySampler, in.uv - vec2(0.0, pixelSize.y));
    let down   = textureSample(myTexture, mySampler, in.uv + vec2(0.0, pixelSize.y));

    let lumCenter = luminance(center);
    let lumLeft   = luminance(left);
    let lumRight  = luminance(right);
    let lumUp     = luminance(up);
    let lumDown   = luminance(down);

    // Calculate a simple gradient metric
    let gradient = abs(lumLeft - lumCenter) +
                   abs(lumRight - lumCenter) +
                   abs(lumUp - lumCenter) +
                   abs(lumDown - lumCenter);

    // Choose a threshold (tweak this value with testing)
    let threshold: f32 = 0.1;

    // If a strong edge is detected, keep the pixel sharp
    if (gradient > threshold) {
        return center;
    } else {
        // For smooth regions, blend center and neighbors to improve clarity
        let avgColor = (center + left + right + up + down) / 5.0;
        return avgColor;
    }
}

