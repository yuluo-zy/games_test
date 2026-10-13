//! B installation report consumer; package hashing is reused from tg-package-integrity.
pub mod controls;
pub use controls::Controls;
use tg_iced::gui_runner::{GuiBackend, GuiError, Options, run_gui};
pub fn run_integrity_report_gui(
    controls: Controls,
    options: Options,
    backend: &mut impl GuiBackend<Controls>,
) -> Result<(), GuiError> {
    run_gui(controls, options, backend).map(|_| ())
}

#[cfg(feature = "native-gui")]
pub fn run_integrity_report_gui_native(
    controls: Controls,
    package_root: impl AsRef<std::path::Path>,
    options: Options,
) -> Result<(), GuiError> {
    let remover = controls::ScopedPackageFileRemover::new(package_root)
        .map_err(|e| GuiError::Backend(e.to_string()))?;
    tg_iced::gui_runner::run_native_gui(controls::NativeControls { controls, remover }, options)
        .map(|_| ())
}
