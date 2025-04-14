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

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_position: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.uv = input.uv;
    out.world_position = input.position;

    // Apply no model transform for now
    out.clip_position = vec4<f32>(input.position, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let time = frame.day * 6.2831853; // 2π * day

    // Basic 2D sine waves for wave movement
    let wave1 = sin((input.uv.x + time) * 10.0) * 0.05;
    let wave2 = cos((input.uv.y - time * 0.8) * 15.0) * 0.05;
    let wave = wave1 + wave2;

    // Fake normal calculation for specular lighting
    let normal = normalize(vec3<f32>(-wave1 * 20.0, 1.0, -wave2 * 20.0));

    let light_dir = normalize(vec3<f32>(0.2, 1.0, 0.3));
    let view_dir = normalize(vec3<f32>(0.0, 1.0, 0.0));
    let half_vec = normalize(view_dir + light_dir);

    let diffuse = max(dot(normal, light_dir), 0.0);
    let specular = pow(max(dot(normal, half_vec), 0.0), 64.0);

    let water_color = vec3<f32>(0.0, 0.3, 0.5);
    let result = water_color * diffuse + vec3<f32>(1.0) * specular;

    return vec4<f32>(vec3(1.0), 1.0);
}
