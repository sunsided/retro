use crate::render::cube::{Vec3, project, rotate_y};
use crate::render::line::draw_line;
use crate::{Color, FrameBuffer};
use std::f32::consts::PI;

pub fn draw_wireframe_sphere(buffer: &mut FrameBuffer, frame: usize, color: Color) {
    let angle = (frame as f32) * PI / 180.0;
    let lat_segments = 12;
    let lon_segments = 24;
    let radius = 1.0;

    for i in 0..lat_segments {
        let theta1 = PI * (i as f32) / (lat_segments as f32);
        let theta2 = PI * ((i + 1) as f32) / (lat_segments as f32);

        for j in 0..lon_segments {
            let phi1 = 2.0 * PI * (j as f32) / (lon_segments as f32);
            let phi2 = 2.0 * PI * ((j + 1) as f32) / (lon_segments as f32);

            let p1 = Vec3(
                radius * theta1.sin() * phi1.cos(),
                radius * theta1.cos(),
                radius * theta1.sin() * phi1.sin(),
            );
            let p2 = Vec3(
                radius * theta1.sin() * phi2.cos(),
                radius * theta1.cos(),
                radius * theta1.sin() * phi2.sin(),
            );
            let p3 = Vec3(
                radius * theta2.sin() * phi1.cos(),
                radius * theta2.cos(),
                radius * theta2.sin() * phi1.sin(),
            );

            let (x1, y1) = project(rotate_y(p1, angle));
            let (x2, y2) = project(rotate_y(p2, angle));
            let (x3, y3) = project(rotate_y(p3, angle));

            draw_line(buffer, x1, y1, x2, y2, color);
            draw_line(buffer, x1, y1, x3, y3, color);
        }
    }
}
