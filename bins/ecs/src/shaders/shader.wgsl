// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    // Clip-space is a coordinate space where vertex coordinates are transformed
    // during the graphics pipeline process. It is defined in normalized device coordinates (NDC),
    // typically with x, y, and z coordinates ranging from -1.0 to 1.0. Clip-space positions
    // are calculated by multiplying the vertex position by the combined Model-View-Projection (MVP) matrix.
    //
    // After this transformation, primitives outside the range [-1.0, 1.0] in clip-space
    // are clipped, effectively removing them from the rendering process. Once clipping is
    // finished, the coordinates are divided by the w component, resulting in normalized device
    // coordinates (NDC), which are then mapped to the screen space for rasterization.
    //
    // In this shader, the clip_position provides clip-space coordinates, which are essential
    // for rendering geometry accurately on the screen.
    //
    // The @builtin(position) bit tells WGPU that this is the value we want to use as
    // the vertex's clip coordinates. This is analogous to GLSL's gl_Position variable.
    //
    // Something to note about @builtin(position), in the fragment shader, this value
    // is in framebuffer space (opens new window). This means that if your window is 800x600,
    // the x and y of clip_position would be between 0-800 and 0-600, respectively,
    // with the y = 0 being the top of the screen. This can be useful if you want to
    // know the pixel coordinates of a given fragment, but if you want the position coordinates,
    // you'll have to pass them in separately.
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    // We need to reassemble the matrix before we can use it.
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;

    // We'll apply the model_matrix before we apply camera_uniform.view_proj.
    // We do this because the camera_uniform.view_proj changes the coordinate system
    // from world space to camera space. Our model_matrix is a world space transformation,
    // so we don't want to be in camera space when using it.
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

// The @location(0) bit tells WGPU to store the vec4 value returned
// by this function in the first color target.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

// The @location(0) bit tells WGPU to store the vec4 value returned
// by this function in the first color target.
@fragment
fn fs_textured_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
