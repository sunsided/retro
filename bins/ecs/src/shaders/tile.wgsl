// Tile shader with normal mapping and lighting support

struct FrameGlobalUniform {
    time_of_day: f32,
    day: f32,
    star_visibility: f32,
    sun_dir: vec3<f32>,
    sun_visibility: f32,
    sun_color: vec3<f32>,
    sky_color: vec3<f32>,
    ambient_light: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> frame: FrameGlobalUniform;

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> light: Light;

@group(3) @binding(0) var tile_texture: texture_2d<f32>;
@group(3) @binding(1) var tile_sampler: sampler;
@group(3) @binding(2) var tile_normal: texture_2d<f32>;
@group(3) @binding(3) var tile_normal_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,

    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,

    @location(6) tile_index: u32,
    @location(7) deco_index: u32,

    @location(8) entity_id: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) @interpolate(flat) tile_index: u32,
    @location(3) @interpolate(flat) deco_index: u32,
    @location(4) @interpolate(flat) entity_id: u32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(
        input.model_0,
        input.model_1,
        input.model_2,
        input.model_3,
    );

    let world_position = (model * vec4<f32>(input.position, 1.0)).xyz;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.tex_coords = input.uv;
    out.world_position = world_position;
    out.tile_index = input.tile_index;
    out.deco_index = input.deco_index;
    out.entity_id = input.entity_id;
    return out;
}

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

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @location(1) entity_id: u32,
};

fn blinn_phong(
    normal: vec3<f32>,
    light_dir: vec3<f32>,
    light_color: vec3<f32>,
    view_dir: vec3<f32>,
    intensity: f32
) -> vec3<f32> {
    let diffuse = max(dot(normal, light_dir), 0.0);
    let half_dir = normalize(view_dir + light_dir);
    let specular = pow(max(dot(normal, half_dir), 0.0), 32.0);
    return (diffuse + specular) * light_color * intensity;
}

@fragment
fn fs_main(
    in: VertexOutput
) -> FragmentOutput {
    let tile_uv = atlas_uv(in.tile_index, in.tex_coords);
    let tile_color = textureSample(tile_texture, tile_sampler, tile_uv);
    let tile_normal = vec3<f32>(0.0, 1.0, 0.0); // assume flat upward-facing tile

    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let light_dir = normalize(light.position - in.world_position);
    let sun_dir = normalize(frame.sun_dir);

    let dyn_light = blinn_phong(tile_normal, light_dir, light.color, view_dir, 1.0);
    let sun_light = blinn_phong(tile_normal, sun_dir, frame.sun_color, view_dir, frame.sun_visibility);

    let result = (frame.ambient_light + dyn_light + sun_light) * tile_color.rgb;

    var out: FragmentOutput;
    out.color = vec4<f32>(result, tile_color.a);
    out.entity_id = 0u;
    return out;
}
