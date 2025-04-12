struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,

    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,

    @location(6) floor_index: u32,
    @location(7) wall_index: u32,
    @location(8) top_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) floor_index: u32,
    @location(2) @interpolate(flat) wall_index: u32,
    @location(3) @interpolate(flat) top_index: u32,
};

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(2) @binding(0)
var<uniform> light: Light;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(
        input.model_0,
        input.model_1,
        input.model_2,
        input.model_3,
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * model * vec4<f32>(input.position, 1.0);
    out.uv = input.uv;
    out.floor_index = input.floor_index;
    out.wall_index = input.wall_index;
    out.top_index = input.top_index;
    return out;
}

@group(3) @binding(0) var tile_texture: texture_2d<f32>;
@group(3) @binding(1) var tile_sampler: sampler;

// const TILES_PER_ROW: u32 = 16u;
const TILES_PER_ROW: u32 = 1u;
const TILE_SIZE: f32 = 1.0 / f32(TILES_PER_ROW);

fn atlas_uv(index: u32, uv: vec2<f32>) -> vec2<f32> {
    let tx = f32(index % TILES_PER_ROW);
    let ty = f32(index / TILES_PER_ROW);
    return vec2<f32>(
        (tx + uv.x) * TILE_SIZE,
        (ty + uv.y) * TILE_SIZE,
    );
}

@fragment
fn fs_main(
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) floor_index: u32,
    @location(2) @interpolate(flat) wall_index: u32,
    @location(3) @interpolate(flat) top_index: u32
) -> @location(0) vec4<f32> {
    let floor_color = textureSample(tile_texture, tile_sampler, atlas_uv(floor_index, uv));
    let wall_color = textureSample(tile_texture, tile_sampler, atlas_uv(wall_index, uv));
    let top_color = textureSample(tile_texture, tile_sampler, atlas_uv(top_index, uv));

    // Blend in layer order: floor → wall → top
    let mixed = mix(floor_color, wall_color, wall_color.a);
    return mix(mixed, top_color, top_color.a);
    // return vec4(1.0, 0.0, 0.0, 1.0);
}
