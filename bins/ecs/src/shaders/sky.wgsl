// sky.wgsl

struct FrameGlobalUniform {
    time_of_day: f32,
    star_visibility: f32,
    sun_dir: vec3<f32>,
    sun_visibility: f32,
    sun_color: vec3<f32>,
    sky_color: vec3<f32>,
    ambient_light: vec3<f32>,
};

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    inverse_proj: mat4x4<f32>,
    camera_rot_x: vec4<f32>,
    camera_rot_y: vec4<f32>,
    camera_rot_z: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> sky: FrameGlobalUniform;

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

fn get_camera_rot() -> mat3x3<f32> {
    return mat3x3<f32>(
        camera.camera_rot_x.xyz,
        camera.camera_rot_y.xyz,
        camera.camera_rot_z.xyz
    );
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0)
    );
    var uvs = array<vec2<f32>, 3>(
        vec2(0.0, 0.0),
        vec2(2.0, 0.0),
        vec2(0.0, 2.0)
    );
    var out: VertexOutput;
    out.position = vec4(positions[index], 1.0, 1.0);
    out.uv = uvs[index];
    return out;
}

fn hash33(p: vec3<f32>) -> f32 {
    let p2 = fract(p * 0.1031);
    let p3 = p2 + dot(p2, vec3(p2.y, p2.z, p2.x) + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Generate camera ray in world space
    let ndc = in.uv * 2.0 - vec2(1.0);
    let clip = vec4(ndc, 0.0, 1.0); // z = 0 for near plane
    let ray_dir_view = normalize((camera.inverse_proj * clip).xyz);
    let ray_dir_world = normalize(get_camera_rot() * ray_dir_view);

    // Sky vertical factor and horizon fade
    let vertical = clamp((ray_dir_world.y + 1.0) * 0.5, 0.0, 1.0);

    let horizon_threshold = mix(0.15, 0.5, sky.star_visibility);
    let horizon = smoothstep(horizon_threshold, 1.0, vertical);

    // Base sky color + reddish glow at horizon
    let reddish_glow = vec3(1.0, 0.25, 0.1) * (1.0 - vertical) * sky.sun_visibility * 0.5;
    let base_sky = sky.sky_color + reddish_glow;
    let fade = pow(1.0 - sky.sun_visibility, 2.0);  // only fades if sun is low
    let night_fade = clamp(1.0 - sky.sun_visibility, 0.0, 1.0);
    let sky_color = base_sky * (1.0 - night_fade * 0.85); // keep 15% at most

    // Sun disc contribution
    let sun_alignment = dot(ray_dir_world, sky.sun_dir);
    let sun_disc = pow(clamp((sun_alignment - 0.95) / (1.0 - 0.95), 0.0, 1.0), 12.0);
    let sun_glow = sun_disc * sky.sun_visibility;
    let sun_contrib = sky.sun_color * sun_glow;

    // Procedural sparse stars
    let noise = hash33(floor(ray_dir_world * 150.0));
    let star = step(0.995, noise);
    let star_brightness = star * pow(noise, 64.0);
    let min_star_elevation = 0.6;
    let elevation_fade = smoothstep(min_star_elevation, 1.0, vertical);
    let stars = vec3(1.0) * star_brightness * elevation_fade * sky.star_visibility;

    // Final color
    let final_color = (sky_color * horizon) + sun_contrib + stars;
    return vec4(final_color, 1.0);
}
