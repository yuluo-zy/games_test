use crash_reporter_gui::{Controls, CrashInfo, InitialScreen, NativeControls};
use tg_iced::gui_runner::{Options, capture_native_gui, write_native_capture};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tg_iced::load_gui_locale()?;
    let path = std::env::args().nth(1).expect("capture output path");
    let root = std::env::args().nth(2).expect("fixture root");
    let controls = NativeControls {
        controls: Controls::new(
            CrashInfo {
                heading: "Tiny Glade".into(),
                diagnostic_text: "Fixture diagnostic".into(),
                log_path: None,
                recovery_succeeded: true,
            },
            InitialScreen::Main,
            || panic!("The presentation probe must not send any report"),
        ),
        remover: tg_integrity_report::controls::ScopedPackageFileRemover::new(root)?,
    };
    let image = capture_native_gui(controls, Options::new("Tiny Glade crash probe", 720, 640))?;
    write_native_capture(&image, path)?;
    println!("captured {}x{}", image.size.width, image.size.height);
    Ok(())
}
