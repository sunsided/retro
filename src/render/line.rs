use crate::{FrameBuffer, HEIGHT, WIDTH};

/// Draws a line on a framebuffer using Bresenham's line algorithm.
///
/// # Parameters
/// - `buffer`: A mutable reference to the framebuffer where the line will be drawn.
/// - `x0`, `y0`: The starting point of the line (coordinates in pixels).
/// - `x1`, `y1`: The ending point of the line (coordinates in pixels).
/// - `color`: The color of the line, represented as a 32-bit integer.
///
/// # Functionality
/// This function calculates a line between the `(x0, y0)` and `(x1, y1)` coordinates
/// using Bresenham's line-drawing algorithm. It then modifies the provided framebuffer
/// by setting the pixels along the line to the specified color. The framebuffer is assumed
/// to be a 1-dimensional array of pixels with a width of `WIDTH` and height of `HEIGHT`.
///
/// # Important Notes
/// - The function clamps drawing to the bounds of the framebuffer (`WIDTH` by `HEIGHT`);
///   pixels outside these dimensions are ignored.
/// - Ensure that the framebuffer has been properly initialized before calling this function.
///
/// # Example
/// ```rust
/// use retro::render::line::draw_line;
/// use retro::{HEIGHT, WIDTH};
///
/// let mut buffer = [0; WIDTH * HEIGHT];
/// draw_line(&mut buffer, 10, 10, 100, 50, 0xFF00FF); // Draw a purple line
/// ```
pub const fn draw_line(
    buffer: &mut FrameBuffer,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: u32,
) {
    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < WIDTH as isize && y0 < HEIGHT as isize {
            buffer[y0 as usize * WIDTH + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
