//! Rebuild the application's bundled presentation shaders from the current HLSL.
//! cargo run -p shader-compiler --example build_presentation -- <dxc> <work-dir>
use shader_compiler::{
    DxcStage,
    build::{BuildSpec, build_pipeline},
};
use shader_pipeline::shader_defines::ShaderDefines;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut args = std::env::args_os().skip(1);
    let dxc = PathBuf::from(args.next().ok_or("missing DXC executable")?);
    let work = PathBuf::from(args.next().ok_or("missing exclusive work directory")?);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let output = root.join("crates/tiny-glade/shaders/compiled");
    std::fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    for name in ["resolve_to_swapchain", "resolve_msaa_to_swapchain"] {
        for depth in ["0", "1"] {
            let spec = BuildSpec {
                source_root: root.join("shaders"),
                logical_root: "shaders".into(),
                source_path: format!("blit/{name}.raster.hlsl"),
                defines: ShaderDefines(vec![("SHADER_VARIANT_DEPTH_ONLY".into(), depth.into())]),
                stages: vec![
                    (DxcStage::Vertex, "vs_main".into()),
                    (DxcStage::Fragment, "ps_main".into()),
                ],
            };
            let built = build_pipeline(&dxc, &spec, &work.join(format!("{name}-{depth}")))?;
            let path = output.join(format!("{name}-{depth}.bin"));
            std::fs::write(&path, built.encoded).map_err(|e| e.to_string())?;
            println!("{}", path.display());
        }
    }
    Ok(())
}
