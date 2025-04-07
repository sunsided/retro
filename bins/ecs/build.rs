use std::env;
use sillyecs::EcsCode;
use std::fs::File;
use std::io::BufReader;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

fn main() -> eyre::Result<()> {
    generate_ecs_code()?;
    copy_resources()?;
    Ok(())
}

fn generate_ecs_code() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=ecs.yaml");

    let file = File::open("ecs.yaml").expect("Failed to open ecs.yaml");
    let reader = BufReader::new(file);
    EcsCode::generate(reader)?.write_files_to("generated")?;
    Ok(())
}

fn copy_resources() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=../../resources/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;

    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("../../resources/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;
    Ok(())
}
