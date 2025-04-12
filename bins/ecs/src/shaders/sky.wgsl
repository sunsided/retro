// sky.wgsl

// Define a uniform holding time and the view-projection matrix.
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

fn get_camera_rot() -> mat3x3<f32> {
    return mat3x3<f32>(
        camera.camera_rot_x.xyz,
        camera.camera_rot_y.xyz,
        camera.camera_rot_z.xyz
    );
}

@group(0) @binding(0)
var<uniform> sky: FrameGlobalUniform;

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// Vertex output including a computed world-space ray direction.
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
    // Use UVs that span [0,1] vertically.
    var uvs = array<vec2<f32>, 3>(
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0)
    );
    var out: VertexOutput;
    out.position = vec4(positions[index], 1.0, 1.0);
    out.uv = uvs[index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let f = normalize(get_camera_rot() * vec3(0.0, 0.0, 1.0));
    return vec4((f * 0.5 + 0.5), 1.0);
}

