use crate::render::line::draw_line;
use crate::{Color, FrameBuffer, HEIGHT, WIDTH};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Vec3(f32, f32, f32);

pub fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3(v.0 * cos + v.2 * sin, v.1, -v.0 * sin + v.2 * cos)
}

pub fn project(v: Vec3) -> (usize, usize) {
    let scale = 100.0 / (v.2 + 3.0); // Simple perspective
    let x = (v.0 * scale + WIDTH as f32 / 2.0).round() as isize;
    let y = (v.1 * scale + HEIGHT as f32 / 2.0).round() as isize;
    (
        x.clamp(0, (WIDTH - 1) as isize) as usize,
        y.clamp(0, (HEIGHT - 1) as isize) as usize,
    )
}

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
