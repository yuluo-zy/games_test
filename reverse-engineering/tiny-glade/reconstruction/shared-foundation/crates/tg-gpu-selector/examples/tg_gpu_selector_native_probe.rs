use tg_gpu_selector::{
    gpu_selection::{Controls, Device, DriverSupport, Vendor},
    vulkan_config::{self, Allowlist, Configuration, Layer, Workarounds},
};
use tg_iced::gui_runner::{Options, capture_native_gui, write_native_capture};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tg_iced::load_gui_locale()?;
    let path = std::env::args().nth(1).expect("capture output path");
    let config = std::env::args().nth(2).as_deref() == Some("config");
    let image = if config {
        capture_native_gui(
            vulkan_config::Controls::new(
                vec![Layer {
                    name: "Fixture layer".into(),
                    vendor: Some("Fixture vendor".into()),
                    description: "Layer description".into(),
                }],
                Configuration {
                    allowlist: Allowlist {
                        items: vec![],
                        policy_word: 1,
                    },
                    workarounds: Workarounds::default(),
                },
            ),
            Options::new("Tiny Glade Vulkan probe", 720, 640),
        )?
    } else {
        capture_native_gui(
            Controls::new(
                vec![Device {
                    name: "Fixture graphics device".into(),
                    vendor: Vendor::Nvidia,
                    driver_version: "1".into(),
                    driver_support: DriverSupport::Unsupported {
                        minimum: "2".into(),
                    },
                    init_error: None,
                }],
                std::path::PathBuf::from("fixture-log"),
            )
            .with_reason(tg_gpu_selector::gpu_selection::SelectionReason::PrimaryUnsupported),
            Options::new("Tiny Glade GPU probe", 720, 640),
        )?
    };
    write_native_capture(&image, path)?;
    println!("captured {}x{}", image.size.width, image.size.height);
    Ok(())
}
