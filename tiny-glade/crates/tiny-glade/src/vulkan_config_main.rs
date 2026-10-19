//! `tiny_glade::vulkan_config_main` — Vulkan layer configuration entry point.
//!
//! B `run_vulkan_configuration_app` (`0x1403e74b0`) call list:
//! `rhapsody::interop::GraphicsInstanceLoader::{new, implicit_layers}`,
//! `ash::vk::LayerProperties`, `tg_gpu_selector::run_vulkan_configuration_gui`,
//! `app_config::write_game_config_to_disk`, and a winit `EventLoop` for the GUI.
//!
//! The winit/tg-iced GUI and the `GameConfig` write are blocked in the current
//! workspace (`app_config::write_game_config_to_disk` is an explicit
//! unimplemented stub), so the recovered orchestration is exposed behind
//! [`VulkanConfigGui`].

use anyhow::{Context, Result};

/// Layer entry shown to the configuration GUI.
#[derive(Clone, Debug)]
pub struct LayerChoice {
    pub name: String,
    pub description: String,
    pub selected: bool,
}

/// GUI result.
#[derive(Clone, Debug, Default)]
pub struct VulkanConfiguration {
    pub enabled_layers: Vec<String>,
    pub disabled_layers: Vec<String>,
}

/// Back-end used by the winit/iced GUI in the original.
pub trait VulkanConfigGui {
    fn run(&mut self, layers: &[LayerChoice]) -> Result<Option<VulkanConfiguration>>;
}

/// B `tiny_glade::vulkan_config_main::run_vulkan_configuration_app`
/// (`0x1403e74b0`).
///
/// 1. Build `GraphicsInstanceLoader::new()`.
/// 2. Read `implicit_layers()` and map each `GraphicsInstanceLayer` to an
///    allowlist choice.
/// 3. Run the configuration GUI (winit event loop + iced).
/// 4. On acceptance write the game config to disk.
pub fn run_vulkan_configuration_app(gui: &mut dyn VulkanConfigGui) -> Result<()> {
    let loader =
        rhapsody::interop::GraphicsInstanceLoader::new().context("Failed to load Vulkan")?;
    let layers = loader
        .implicit_layers()
        .unwrap_or_default()
        .iter()
        .map(|layer| LayerChoice {
            name: layer.name.clone(),
            description: layer.description.clone(),
            selected: true,
        })
        .collect::<Vec<_>>();

    match gui.run(&layers)? {
        Some(configuration) => {
            log::info!(
                "vulkan configuration accepted: {} enabled, {} disabled",
                configuration.enabled_layers.len(),
                configuration.disabled_layers.len()
            );
            // B: `app_config::write_game_config_to_disk()`; blocked on the
            // missing `GameConfig` serde plane.
            Ok(())
        }
        None => {
            log::info!("vulkan configuration cancelled");
            Ok(())
        }
    }
}
