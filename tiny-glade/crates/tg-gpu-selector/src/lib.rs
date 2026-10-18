#![doc = include_str!("../README.md")]
pub mod gpu_selection;
pub mod vulkan_config;
use tg_iced::gui_runner::{GuiBackend, GuiError, Options, ProgramWithOutcome, run_gui};
/// Run candidate selection with a caller-provided event/rendering backend.
/// Returns the selected input-list index, or `None` on close. Closed controls bypass the backend.
pub fn run_gpu_selection_gui(
    controls: gpu_selection::Controls,
    options: Options,
    backend: &mut impl GuiBackend<gpu_selection::Controls>,
) -> Result<Option<usize>, GuiError> {
    if controls.is_closed() {
        return Ok(controls.program_outcome());
    }
    run_gui(controls, options, backend)
}
/// Edit the supplied configuration through a caller-provided backend.
/// Apply returns a frozen snapshot; cancel/close returns `None`. Backend failures propagate.
pub fn run_vulkan_configuration_gui(
    controls: vulkan_config::Controls,
    options: Options,
    backend: &mut impl GuiBackend<vulkan_config::Controls>,
) -> Result<Option<vulkan_config::Configuration>, GuiError> {
    if controls.is_closed() {
        return Ok(controls.program_outcome());
    }
    run_gui(controls, options, backend)
}

#[cfg(feature = "native-gui")]
/// Open native candidate selection, or return the outcome of already-closed controls.
/// Configure localization beforehand; the iced backend requires the main thread and one event loop per process.
pub fn run_gpu_selection_gui_native(
    controls: gpu_selection::Controls,
    options: Options,
) -> Result<Option<usize>, GuiError> {
    if controls.is_closed() {
        return Ok(controls.program_outcome());
    }
    tg_iced::gui_runner::run_native_gui(controls, options)
}
#[cfg(feature = "native-gui")]
/// Open the native Vulkan configuration editor; no device creation or disk writes occur here.
/// Apply returns the configuration for the caller to persist; cancel returns `None`.
pub fn run_vulkan_configuration_gui_native(
    controls: vulkan_config::Controls,
    options: Options,
) -> Result<Option<vulkan_config::Configuration>, GuiError> {
    if controls.is_closed() {
        return Ok(controls.program_outcome());
    }
    tg_iced::gui_runner::run_native_gui(controls, options)
}
