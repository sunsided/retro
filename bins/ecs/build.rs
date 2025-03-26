use ecs_builder::build;
use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("cargo:rerun-if-changed=ecs.yaml");

    let file = File::open("ecs.yaml").expect("Failed to open ecs.yaml");
    let reader = BufReader::new(file);
    build(reader)
}
