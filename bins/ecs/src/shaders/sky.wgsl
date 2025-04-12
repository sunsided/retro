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
    let color = blended_sky * horizon;

    return vec4(color, 1.0);
}
