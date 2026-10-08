use garden_presentation::{catalog::ArtCatalog, manifest::AssetManifest};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let strict = match args.as_slice() {
        [] => false,
        [flag] if flag == "--strict" => true,
        _ => return Err("usage: art-check [--strict]".into()),
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let theme = ArtCatalog::parse(&std::fs::read_to_string(
        root.join("themes/warm-stone/theme.json"),
    )?)?;
    let manifest = AssetManifest::parse(&std::fs::read_to_string(
        root.join("manifests/models.json"),
    )?)?;
    let summary = manifest.check(&root, strict)?;
    println!(
        "theme={} revision={} | {:?}",
        theme.theme_id(),
        theme.revision(),
        summary
    );
    println!(
        "Header/count checks only; full glTF validation, in-engine axes/UV review and Android profiling remain required."
    );
    Ok(())
}
