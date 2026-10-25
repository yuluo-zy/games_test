//! Inspect/export the production heap ABI for an existing shader package.
use shader_compiler::{
    bindless::lower_pipeline, shader_pipeline::compiled::CompiledShaderPipeline,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = args
        .next()
        .ok_or("expected shader package and output directory")?;
    let output = std::path::PathBuf::from(args.next().ok_or("expected output directory")?);
    let pipeline = CompiledShaderPipeline::decode(&std::fs::read(input)?)?;
    let (abi, stages) = lower_pipeline(&pipeline)?;
    std::fs::create_dir_all(&output)?;
    for (index, stage) in stages.iter().enumerate() {
        std::fs::write(output.join(format!("{index}.spv")), stage.spirv_bytes())?;
    }
    std::fs::write(output.join("abi.txt"), format!("{abi:#?}\n"))?;
    Ok(())
}
