// sky.wgsl

struct FrameGlobalUniform {
    time: f32,
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Generate camera ray in world space
    let ndc = in.uv * 2.0 - vec2(1.0);
    let clip = vec4(ndc, 0.0, 1.0); // z = 0 for near plane

    let view_ray = camera.inverse_proj * clip;
    let ray_dir_view = normalize(view_ray.xyz); // no divide by w needed
    let ray_dir_world = normalize(get_camera_rot() * ray_dir_view);

    // Compute sky vertical factor
    let vertical = clamp((ray_dir_world.y + 1.0) * 0.5, 0.0, 1.0);

    // Day/Night Cycle
    let night_color = vec3(0.03, 0.03, 0.2);
    let day_color = vec3(0.4, 0.75, 1.0);
    let cycle = 0.5 * (1.0 + cos((sky.time - 12.0) * 6.2831853 / 24.0));
    let blended_sky = mix(night_color, day_color, cycle);

    // Time-dependent horizon falloff
    var effective_threshold: f32;
    const low_level = 0.1;
    const high_level = 0.5;
    if (sky.time < 6.0) {
        effective_threshold = mix(high_level, low_level, smoothstep(0.0, 6.0, sky.time));
    } else if (sky.time > 18.0) {
        effective_threshold = mix(low_level, high_level, smoothstep(18.0, 24.0, sky.time));
    } else {
        effective_threshold = low_level;
    }
    let horizon = smoothstep(effective_threshold, 1.0, vertical);

    // Sun direction (moving on tilted arc over world sphere)
    // Time = 0 → sun at -X horizon, Time = 12 → overhead, Time = 24 → full loop
    let sun_angle = ((sky.time + 18.0) * 6.2831853 / 24.0);
    let tilt = radians(145.0);  // orbital inclination

    // Construct sun_dir in world space: rotates around Z axis, tilted toward Y
    let sun_dir = normalize(vec3(
        cos(sun_angle),
        sin(sun_angle) * sin(tilt),
        sin(sun_angle) * cos(tilt)
    ));

    // Only glow when sun is above the horizon (Y > 0)
    let sun_visibility = max(sun_dir.y, 0);

    // Angular alignment between current pixel and sun direction
    let cos_angle = dot(ray_dir_world, sun_dir);
    let glow_size = 0.95;  // higher = smaller sun disc
    let glow = pow(clamp((cos_angle - glow_size) / (1.0 - glow_size), 0.0, 1.0), 12.0);

    // Final sun glow intensity modulated by whether sun is above horizon
    let sun_glow = glow * sun_visibility;

    // Sunset factor: 1 at horizon, 0 at zenith
    let sunset_factor = 1.0 - clamp(sun_dir.y, 0.0, 1.0);
    let sunset_strength = sunset_factor * cycle;

    let sun_color_tinted = mix(vec3(1.2, 1.05, 0.85), vec3(1.3, 0.5, 0.2), sunset_strength);
    let horizon_tint = mix(blended_sky, vec3(1.0, 0.4, 0.2), sunset_strength * 0.5);

    let horizon_distance = 1.0 - vertical;  // 0 at zenith, 1 at horizon
    let reddish_glow = vec3(1.0, 0.25, 0.1) * horizon_distance * sunset_strength * 0.5;

    // Final sky with sun
    let sky_color = mix(blended_sky, horizon_tint, horizon);
    let color = sky_color * horizon + (sun_color_tinted + reddish_glow) * sun_glow;


    return vec4(color, 1.0);
}
