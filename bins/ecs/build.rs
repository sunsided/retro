use sillyecs::EcsCode;
use std::fs::File;
use std::io::BufReader;

fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=ecs.yaml");

    let file = File::open("ecs.yaml").expect("Failed to open ecs.yaml");
    let reader = BufReader::new(file);
    EcsCode::generate(reader)?.write_files_to("generated")?;
    Ok(())
}
