// Tile shader with normal mapping and lighting support

struct FrameGlobalUniform {
    time_of_day: f32,
    day: f32,
    star_visibility: f32,
    star_rotational_speed: f32,
    sun_dir: vec3<f32>,
    sun_visibility: f32,
    sun_color: vec3<f32>,
    sky_color: vec3<f32>,
    ambient_light: vec3<f32>,
    window_aspect: f32,
    retro_aspect: f32,
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
    out.entity_id = in.entity_id;
    return out;
}

const TAU: f32 = 6.2831853;

fn rot(a: f32) -> mat2x2<f32> {
    let c = cos(a);
    let s = sin(a);
    return mat2x2<f32>(
        vec2<f32>( c, s),
        vec2<f32>(-s, c)
    );
}

fn hash21(n: vec2<f32>) -> f32 {
    return fract(cos(dot(n, vec2<f32>(5.9898, 4.1414))) * 65899.89956);
}

fn noise(n: vec2<f32>) -> f32 {
    let d = vec2<f32>(0.0, 1.0);
    let b = floor(n);
    let f = smoothstep(vec2<f32>(0.0), vec2<f32>(1.0), fract(n));

    let h00 = hash21(b);
    let h10 = hash21(b + d.yx);
    let h01 = hash21(b + d.xy);
    let h11 = hash21(b + d.yy);

    let x1 = mix(h00, h10, f.x);
    let x2 = mix(h01, h11, f.x);
    return mix(x1, x2, f.y);
}

fn mix_noise(p: vec2<f32>) -> vec2<f32> {
    let epsilon = 0.968785675;
    let noise_x = noise(vec2<f32>(p.x + epsilon, p.y)) - noise(vec2<f32>(p.x - epsilon, p.y));
    let noise_y = noise(vec2<f32>(p.x, p.y + epsilon)) - noise(vec2<f32>(p.x, p.y - epsilon));
    return vec2<f32>(noise_x, noise_y);
}

fn fbm_fancy(p_: vec2<f32>, time: f32) -> f32 {
    var p = p_;
    var amplitude = 3.0;
    var total = 0.0;
    var pp = p;

    for (var i = 1.3232; i < 7.45; i = i + 1.0) {
        var p_offset = p + time * 0.05;
        pp = pp + time * 0.09;

        var n = mix_noise(i * p_offset * 0.3244243 + time * 0.131321);
        let angle = time * 0.5 - (0.03456 * p.x + 0.0342322 * p.y) * 50.0;
        let r = rot(angle);
        n = r * n;

        p = p + n * 0.5;
        total = total + (sin(noise(p) * 8.5) * 0.55 + 0.4566) / amplitude;

        p = mix(pp, p, 0.5);

        amplitude = amplitude * 1.3;
        p = p * 2.007556;
        pp = pp * 1.6895367;
    }

    return total;
}

// https://www.shadertoy.com/view/lslXRS
@fragment
fn fs_lava(in: VertexOutput) -> FragmentOutput {
    // periodic day to avoid jumps
    let t = sin(fract(frame.day) * TAU );

    let uv = in.world_position.xz / 1.0; // adjust for world scale
    let fbm_val = fbm_fancy(uv, t);

    let base_color = vec3<f32>(0.212, 0.08, 0.03) / max(fbm_val, 0.0001);
    let final_color = pow(base_color, vec3<f32>(1.5)); // gamma correction / stylization

    var out: FragmentOutput;
    out.color = vec4<f32>(final_color, 1.0);
    out.entity_id = in.entity_id;
    return out;
}

// https://www.shadertoy.com/view/lslXRS
@fragment
fn fs_water(in: VertexOutput) -> FragmentOutput {
    // periodic day to avoid jumps
    let t = sin(fract(frame.day) * TAU * 2) / 2;

    let uv = in.world_position.xz / 0.5; // adjust for world scale
    let fbm_val = fbm_fancy(uv, t);

    let base_color = vec3<f32>(0.0, 0.08, 0.212) / max(fbm_val, 0.0001);
    let color = pow(base_color, vec3<f32>(2.5)); // gamma correction / stylization
    let final_color = clamp(color, vec3<f32>(0.0), vec3<f32>(0.5));

    var out: FragmentOutput;
    out.color = vec4<f32>(final_color, 1.0);
    out.entity_id = in.entity_id;
    return out;
}

const PI: f32 = 3.1415926;
const BLADES_SPACING: f32 = 0.004;
const JITTER_MAX: f32 = 0.004;
const LOOKUP_DIST: i32 = 5;

fn hash12(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

fn hash22(p: vec2<f32>) -> vec2<f32> {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.xx + p3.yz) * p3.zy);
}

fn hash32(p: vec2<f32>) -> vec3<f32> {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yxz + 19.19);
    return fract((p3.xxy + p3.yzz) * p3.zyx);
}

fn get_grass_color(x: f32) -> vec3<f32> {
    let a = vec3<f32>(0.2, 0.4, 0.3);
    let b = vec3<f32>(0.3, 0.5, 0.2);
    let c = vec3<f32>(0.2, 0.4, 0.2);
    let d = vec3<f32>(0.66, 0.77, 0.33);
    return a + b * cos(2.0 * PI * (c * x + d));
}

fn get_grass_blade(position: vec2<f32>, grass_pos: vec2<f32>) -> vec4<f32> {
    let grass_vec3 = hash32(grass_pos * 123512.41) * 2.0 - vec3<f32>(1.0);
    let dir2 = normalize(grass_vec3.xy);
    let height = grass_vec3.z * 0.2 + 0.2;

    let blade_len = hash12(grass_pos * 102348.7) * 0.01 + 0.012;

    let gv = position - grass_pos;
    let gx = dot(dir2, gv);
    let gy = dot(vec2<f32>(-dir2.y, dir2.x), gv);
    let gxn = gx / blade_len;

    if gxn >= 0.0 && gxn <= 1.0 && abs(gy) <= 0.0008 * (1.0 - gxn * gxn) {
        let color = get_grass_color(hash12(grass_pos * 2631.6)) * (0.2 + 0.8 * gxn);
        return vec4<f32>(color, 0.2 + height * gxn); // z stored in alpha
    }

    return vec4<f32>(0.0);
}

fn get_grass_pixel(position: vec2<f32>) -> vec4<f32> {
    let xcount = i32(1.0 / BLADES_SPACING);
    let ycount = i32(1.0 / BLADES_SPACING);

    let ox = i32(position.x * f32(xcount));
    let oy = i32(position.y * f32(ycount));

    var max_z = 0.0;
    var final_color = vec4<f32>(0.0);

    for (var i = -LOOKUP_DIST; i < LOOKUP_DIST; i = i + 1) {
        for (var j = -LOOKUP_DIST; j < LOOKUP_DIST; j = j + 1) {
            let upos = vec2<f32>(f32(ox + i), f32(oy + j));
            let jittered = upos * BLADES_SPACING + hash22(upos) * JITTER_MAX;

            let col = get_grass_blade(position, jittered);
            if col.a > max_z {
                max_z = col.a;
                final_color = vec4<f32>(col.rgb, 1.0);
            }
        }
    }

    return final_color;
}

// https://www.shadertoy.com/view/WsSGWd
@fragment
fn fs_grass(in: VertexOutput) -> FragmentOutput {
    let uv = in.world_position.xz;

    // Normalize UV to 0..1 across a patch
    let scaled_uv = fract(uv * 0.1); // or wrap based on tile size

    let grass_color = get_grass_pixel(scaled_uv);

    var out: FragmentOutput;
    out.color = grass_color;
    out.entity_id = in.entity_id;
    return out;
}
