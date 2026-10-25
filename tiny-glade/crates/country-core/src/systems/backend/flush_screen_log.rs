//! B's thread-safe producer buffer and main-thread screen-log consumer.
use std::sync::Mutex;

use bevy_ecs::system::ResMut;

use crate::resources::gui::DebugGui;

enum LogType {
    Error,
    Warning,
}

static SCREEN_LOG_BUFFER: Mutex<Vec<(String, LogType)>> = Mutex::new(Vec::new());

/// B 0x1408c10a0: copy and enqueue under the mutex, preserving duplicates.
pub fn log_error_to_screen(message: &str) {
    SCREEN_LOG_BUFFER
        .lock()
        .unwrap()
        .push((message.to_owned(), LogType::Error));
}

/// B 0x1408c1310.
pub fn log_warning_to_screen(message: &str) {
    SCREEN_LOG_BUFFER
        .lock()
        .unwrap()
        .push((message.to_owned(), LogType::Warning));
}

/// B 0x1408c0d90 drains in insertion order while holding the lock. An empty
/// buffer does not dereference ResMut mutably or change DebugGui's change tick.
pub fn flush_error_log_to_screen(mut debug_gui: ResMut<DebugGui>) {
    let mut buffer = SCREEN_LOG_BUFFER.lock().unwrap();
    for (message, severity) in buffer.drain(..) {
        let color = match severity {
            LogType::Error => egui::Color32::RED,
            LogType::Warning => egui::Color32::YELLOW,
        };
        debug_gui.log_to_screen(&message, color);
    }
}
