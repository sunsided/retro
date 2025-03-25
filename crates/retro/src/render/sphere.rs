use crate::render::cube::{Vec3, project, rotate_y};
use crate::render::line::draw_line;
use crate::{Color, FrameBuffer};
use std::f32::consts::PI;

/// Draws a rotating wireframe sphere onto the framebuffer.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the framebuffer where the sphere will be rendered.
/// * `frame` - The current frame number, used to calculate the rotation angle of the sphere.
/// * `color` - The color of the sphere's wireframe.
pub fn draw_wireframe_sphere(buffer: &mut FrameBuffer, frame: usize, color: Color) {
    let angle = (frame as f32) * PI / 180.0;
    let lat_segments = 12; // Number of latitude segments (horizontal slices of the sphere).
    let lon_segments = 24; // Number of longitude segments (vertical slices of the sphere).
    let radius = 1.0;

    // Iterate through latitude segments to create the sphere's grid.
    for i in 0..lat_segments {
        // Angles for the current and next latitude segment.
        let theta1 = PI * (i as f32) / (lat_segments as f32);
        let theta2 = PI * ((i + 1) as f32) / (lat_segments as f32);

        // Iterate through longitude segments to create the sphere's grid.
        for j in 0..lon_segments {
            // Angles for the current and next longitude segment.
            let phi1 = 2.0 * PI * (j as f32) / (lon_segments as f32);
            let phi2 = 2.0 * PI * ((j + 1) as f32) / (lon_segments as f32);

            // Calculate the 3D points for the current grid on the sphere.
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

            // Rotate and project the 3D points onto the 2D screen.
            let (x1, y1) = project(rotate_y(p1, angle));
            let (x2, y2) = project(rotate_y(p2, angle));
            let (x3, y3) = project(rotate_y(p3, angle));

            // Draw lines connecting the points on the 2D screen to form the sphere's wireframe.
            draw_line(buffer, x1, y1, x2, y2, color);
            draw_line(buffer, x1, y1, x3, y3, color);
        }
    }
}
