# Retro

This project is a small retro game and graphics experiment.
Its goal is to explore rendering techniques, game mechanics,
and visual aesthetics inspired by classic games of the VGA era.

## Getting Started

1. Clone this repository.
2. Build the project and run it.

## `sillyecs` + `winit` + `wgpu`

The newer example contains an archetype-based ECS with windowing provided
by `winit` instead of `minifb`. Rendering is performed using `wgpu` by targeting an
VGA-sized (320x200) offscreen texture, then linearly sampling to fullscreen. The
VGA upsampling shader provides some contour rendering for less pixel mess.

Other "highlights":

- Archetype-style ECS via [sillyecs](https://github.com/sunsided/sillyecs) to drive simulation and rendering.
- Unbounded simulation runs with vsync-locked rendering passes.
- Hardware instancing is used to render identical entitishopt -s globstares in batched calls.
- Simple dynamic lighting with a singular light source and Blinn-Phong Diffuse and Specular lighting.
- Normal mapping support.
- Model loading from `.obj` via the `tobj`.
- A simple FPS-style camera controller.

## Screenshots

The VGA shader "upsamples" from a 320x200 pixel render target and incorporates
depth information and entity IDs for edge detection and outlining:

![](readme/ecs-vga-shaded.jpg)

Entity information is written to another texture during the first pass rendering:

![](readme/ecs-materials.png)

Rendering can be done to a screen-sized buffer as well:

![](readme/ecs-highres.jpg)

### How to Run

1. Install Rust if you have not already: [https://rustup.rs/](https://rustup.rs/)
2. Build and run the project:
   ```bash
   just run
   ```

## Acknowledgments

- Inspired by retro games and the creative techniques used to build them.
- Shout out to the Rust game development and graphics programming community for libraries and support.
