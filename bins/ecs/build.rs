use ecs_builder::build;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    println!("cargo:rerun-if-changed=ecs.yaml");

    let file = File::open("ecs.yaml").expect("Failed to open ecs.yaml");
    let reader = BufReader::new(file);
    let code = build(reader).expect("Failed to build ECS");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Write components to a file
    let mut components_file = File::create(format!("{out_dir}/components.gen.rs"))
        .expect("Failed to create components.gen.rs");
    components_file
        .write_all(code.components.as_bytes())
        .expect("Failed to write to components.gen.rs");

    // Write archetypes to a file
    let mut archetypes_file = File::create(format!("{out_dir}/archetypes.gen.rs"))
        .expect("Failed to create archetypes.gen.rs");
    archetypes_file
        .write_all(code.archetypes.as_bytes())
        .expect("Failed to write to archetypes.gen.rs");

    // Write world to a file
    let mut archetypes_file =
        File::create(format!("{out_dir}/world.gen.rs")).expect("Failed to create world.gen.rs");
    archetypes_file
        .write_all(code.world.as_bytes())
        .expect("Failed to write to world.gen.rs");
}
