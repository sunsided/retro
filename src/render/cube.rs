use crate::render::line::draw_line;
use crate::{Color, FrameBuffer, HEIGHT, WIDTH};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
/// A 3D vector used to represent points or directions in 3D space.
pub struct Vec3(pub f32, pub f32, pub f32);

/// Rotates a 3D vector around the Y-axis by a specified angle.
///
/// # Parameters
/// - `v`: The vector to rotate.
/// - `angle`: The angle to rotate by, in radians.
///
/// # Returns
/// The rotated vector.
pub fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3(v.0 * cos + v.2 * sin, v.1, -v.0 * sin + v.2 * cos)
}

/// Projects a 3D vector onto the 2D screen using a simple perspective transformation.
///
/// # Parameters
/// - `v`: The 3D vector to project.
///
/// # Returns
/// A tuple containing the projected 2D screen coordinates as `(x, y)`.
pub fn project(v: Vec3) -> (usize, usize) {
    let scale = 100.0 / (v.2 + 3.0); // Simple perspective
    let x = (v.0 * scale + WIDTH as f32 / 2.0).round() as isize;
    let y = (v.1 * scale + HEIGHT as f32 / 2.0).round() as isize;
    (
        x.clamp(0, (WIDTH - 1) as isize) as usize,
        y.clamp(0, (HEIGHT - 1) as isize) as usize,
    )
}

/// Draws a rotating wireframe cube onto a framebuffer.
///
/// # Parameters
/// - `buffer`: A mutable reference to the framebuffer where the cube will be drawn.
/// - `frame`: The current frame count, which determines the rotation angle.
/// - `color`: The color of the cube's edges.
///
/// # Functionality
/// This function creates a cube represented by 8 vertices in 3D space. It rotates
/// these vertices around the Y-axis based on the `frame` parameter, projects them
/// onto a 2D screen, and renders the edges of the cube by connecting the appropriate
/// vertices using the `draw_line` function.
pub fn draw_wireframe_cube(buffer: &mut FrameBuffer, frame: usize, color: Color) {
    let angle = (frame as f32) * PI / 180.0;

    let vertices = [
        Vec3(-1.0, -1.0, -1.0),
        Vec3(1.0, -1.0, -1.0),
        Vec3(1.0, 1.0, -1.0),
        Vec3(-1.0, 1.0, -1.0),
        Vec3(-1.0, -1.0, 1.0),
        Vec3(1.0, -1.0, 1.0),
        Vec3(1.0, 1.0, 1.0),
        Vec3(-1.0, 1.0, 1.0),
    ];

    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // back face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // front face
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // connections
    ];

    let transformed: Vec<(usize, usize)> = vertices
        .iter()
        .map(|&v| project(rotate_y(v, angle)))
        .collect();

    for &(i, j) in &edges {
        let (x0, y0) = transformed[i];
        let (x1, y1) = transformed[j];
        draw_line(buffer, x0, y0, x1, y1, color);
    }
}
