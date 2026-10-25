/// B game consumer policy; original Rust ABI is not claimed.
pub fn verification_report_options(result: &controls::VerificationResult) -> Options {
    let (width, height) = if result.is_ok() { (500, 300) } else { (1000, 800) };
    Options::new("Tiny Glade file verification report", width, height)
}

/// Executes package verification once and presents both success and failure.
/// Backend owns the original iced/winit window and event loop boundary.
pub fn verify_package_integrity_and_show_report(
    package_root: impl AsRef<std::path::Path>,
    backend: &mut impl GuiBackend<Controls>,
) -> Result<(), GuiError> {
    let result = tg_package_integrity::verify_package_manifest(package_root);
    let options = verification_report_options(&result);
    run_integrity_report_gui(Controls::new(result), options, backend)
}
