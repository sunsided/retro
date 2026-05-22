use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use sillyecs_build::EcsCode;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs};

fn main() -> eyre::Result<()> {
    compile_shaders()?;
    generate_ecs_code()?;
    copy_resources()?;
    Ok(())
}

fn compile_shaders() -> eyre::Result<()> {
    const SHADER_DIR: &str = "src/shaders";
    let shader_paths = find_shader_paths(SHADER_DIR);
    if shader_paths.is_empty() {
        eprintln!("❌ No WGSL shaders found in directory: {SHADER_DIR}");
        std::process::exit(1);
    }
    for path in shader_paths {
        println!("cargo:rerun-if-changed={path}");
        let src = fs::read_to_string(&path)?;
        naga::front::wgsl::parse_str(&src)
            .unwrap_or_else(|_| panic!("WGSL failed to parse: {path}"));
    }

    Ok(())
}

fn find_shader_paths<P>(dir: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut shader_paths = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "wgsl") {
                shader_paths.push(path.to_string_lossy().into_owned());
            }
        }
    }

    shader_paths
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

    let paths_to_copy = vec!["../../resources/"];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;
    Ok(())
}
