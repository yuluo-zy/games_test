//! B game GPU choice and Vulkan layer/workaround UI policy.
pub mod gpu_selection;
pub mod vulkan_config;
use tg_iced::gui_runner::{GuiBackend, GuiError, Options, run_gui};
pub fn run_gpu_selection_gui(
    controls: gpu_selection::Controls,
    options: Options,
    backend: &mut impl GuiBackend<gpu_selection::Controls>,
) -> Result<Option<usize>, GuiError> {
    run_gui(controls, options, backend)
}
pub fn run_vulkan_configuration_gui(
    controls: vulkan_config::Controls,
    options: Options,
    backend: &mut impl GuiBackend<vulkan_config::Controls>,
) -> Result<Option<vulkan_config::Configuration>, GuiError> {
    run_gui(controls, options, backend)
}

#[cfg(feature = "native-gui")]
pub fn run_gpu_selection_gui_native(
    controls: gpu_selection::Controls,
    options: Options,
) -> Result<Option<usize>, GuiError> {
    tg_iced::gui_runner::run_native_gui(controls, options)
}
#[cfg(feature = "native-gui")]
pub fn run_vulkan_configuration_gui_native(
    controls: vulkan_config::Controls,
    options: Options,
) -> Result<Option<vulkan_config::Configuration>, GuiError> {
    tg_iced::gui_runner::run_native_gui(controls, options)
}

#[cfg(test)]
mod original_contract_tests {
    use super::*;
    use tg_iced::gui_runner::ProgramWithOutcome;
    include!("fixtures/message_cases.rs");
    #[test]
    fn original_gpu_message_leaf_fields() {
        for &(variant, value, tag, index, more) in GPU_CASES {
            let mut c = gpu_selection::Controls::new(vec![], std::path::PathBuf::new());
            let m = match variant {
                0 => gpu_selection::Message::Select(value as usize),
                1 => gpu_selection::Message::ShowMoreInfo,
                3 => gpu_selection::Message::Close,
                5 => gpu_selection::Message::Noop,
                _ => unreachable!(),
            };
            c.update(m);
            assert_eq!(c.more_info, more);
            assert_eq!(
                c.program_outcome(),
                if tag == 1 { Some(index as usize) } else { None }
            );
        }
    }
    #[test]
    fn original_six_workaround_fields() {
        for &(index, seed, repeat, expected) in CONFIG_CASES {
            let w = vulkan_config::Workarounds {
                sync_before_each_pass: seed,
                sync_before_each_draw: seed,
                sync_before_each_dispatch: seed,
                serial_recording: seed,
                skip_loading_screen: seed,
                vendor_crash_breadcrumbs: seed,
            };
            let mut c = vulkan_config::Controls::new(
                vec![],
                vulkan_config::Configuration {
                    allowlist: vulkan_config::Allowlist {
                        items: vec![],
                        policy_word: 0,
                    },
                    workarounds: w,
                },
            );
            for _ in 0..repeat {
                c.update(vulkan_config::Message::ToggleWorkaround(index));
            }
            for (i, value) in expected.into_iter().enumerate() {
                assert_eq!(c.configuration.workarounds.get(i), Some(value));
            }
        }
    }
}
