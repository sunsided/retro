// Vertex shader

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

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,

    // Normal rotation matrix
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
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
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
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

    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = normal_matrix * model.normal;

    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
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
    let object_color = vec4<f32>(0.3, 0.2, 0.1, 1.0);

    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    // Diffuse lighting
    let light_dir = normalize(light.position - in.world_position);
    let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;

    // Specular lighting (Blinn-Phong)
    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);
    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;

    let result = (ambient_color + diffuse_color) * object_color.xyz;
    return vec4<f32>(result, object_color.a);
}

// The @location(0) bit tells WGPU to store the vec4 value returned
// by this function in the first color target.
@fragment
fn fs_textured_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    // Diffuse lighting
    let light_dir = normalize(light.position - in.world_position);
    let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;

    // Specular lighting (Blinn-Phong)
    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);
    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;

    let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;
    return vec4<f32>(result, object_color.a);
}
