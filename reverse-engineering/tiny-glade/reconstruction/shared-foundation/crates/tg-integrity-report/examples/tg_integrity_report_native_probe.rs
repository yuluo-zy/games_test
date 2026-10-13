use tg_iced::gui_runner::{Options, capture_native_gui, write_native_capture};
use tg_integrity_report::{
    Controls,
    controls::{NativeControls, ScopedPackageFileRemover},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tg_iced::load_gui_locale()?;
    let path = std::env::args().nth(1).expect("capture output path");
    let root = std::env::args().nth(2).expect("fixture root");
    let controls = NativeControls {
        controls: Controls::new(Err(tg_package_integrity::PackageVerificationError::Files(
            tg_package_integrity::FileIssues {
                missing: vec!["missing-fixture".into()],
                corrupt: vec!["corrupt-fixture".into()],
                extra: vec!["extra-fixture".into()],
            },
        ))),
        remover: ScopedPackageFileRemover::new(root)?,
    };
    let image = capture_native_gui(
        controls,
        Options::new("Tiny Glade integrity probe", 640, 480),
    )?;
    write_native_capture(&image, path)?;
    println!("captured {}x{}", image.size.width, image.size.height);
    Ok(())
}
