use tg_iced::{
    dialog::{Controls, MessageDialog},
    gui_runner::{Options, capture_native_gui, write_native_capture},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tg_iced::load_gui_locale()?;
    let path = std::env::args().nth(1).expect("capture output path");
    let controls = Controls::new(
        MessageDialog::new(640, 480)
            .set_title("Tiny Glade")
            .set_description("Dialog fixture"),
    );
    let image = capture_native_gui(controls, Options::new("Tiny Glade dialog probe", 640, 480))?;
    write_native_capture(&image, path)?;
    println!("captured {}x{}", image.size.width, image.size.height);
    Ok(())
}
