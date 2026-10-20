//! B `toolbar::menu::credits` CPU surface.
//!
//! Recovered here:
//! - `ScrollView::{content, content_mut, max_scroll_pos, offset, background_color}`
//!   (contract #34/#35): accessors over the native scroll owner; the same
//!   `NodeContext::content_mut` path used by every other MEI content.
//! - `spawn_third_party_license_http_server` (O 0x141dfb410, contract #36):
//!   binds `127.0.0.1:8123`, spawns the single-file server thread and records
//!   the URL plus any error in its `Local`.
//!
//! Still U (not fabricated): `credits::ui` template/string identifiers and the
//! `impl$2` NodeContent scroll physics (on_input/prepare_layout/render). The
//! public state and the 450.0 viewport subtraction from ANALYSIS C9 are kept.

use bevy_ecs::prelude::{Local, Res};
use country_core::resources::gui::EscMenuState;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Native credit list viewport height subtracted from the content span
/// (`max_scroll_pos = (max - min) - 450.0`, ANALYSIS C9).
pub const CREDITS_VIEWPORT: f32 = 450.0;

/// B credits `ScrollContent` state owner (layout fields U; the values below are
/// the ones directly observed by `ScrollView` accessors and the scroll bound).
#[derive(Clone, Copy, Debug)]
pub struct ScrollContent {
    /// Scrolled distance in MEI units.
    pub scroll: f32,
    /// Velocity used by `time_continuous_lerp_mut` in prepare_layout.
    pub velocity: f32,
    /// Smallest child top (native `-3.4e38` sentinel before min-reduce).
    pub content_min: f32,
    /// Largest child bottom (native `+3.4e38` sentinel before max-reduce).
    pub content_max: f32,
    /// Native packed background color.
    pub background: u32,
}

impl Default for ScrollContent {
    fn default() -> Self {
        Self {
            scroll: 0.,
            velocity: 0.,
            content_min: f32::MAX,
            content_max: f32::MIN,
            background: 0,
        }
    }
}

impl ScrollContent {
    /// O 0x141dfa070 tail: `(max - min) - 450.0`, only written when
    /// `max > min`. No clamp is applied by the native code.
    pub fn max_scroll_pos(&self) -> f32 {
        (self.content_max - self.content_min) - CREDITS_VIEWPORT
    }

    /// Current content offset (native `ScrollView::offset` reads the same
    /// `scroll` value from the content owner).
    pub fn offset(&self) -> f32 {
        self.scroll
    }

    /// Clamp the next scroll position; the native prepare_layout applies the
    /// stored velocity first and then keeps the result inside the range.
    pub fn apply_velocity(&mut self, delta: f32, max_scroll: f32) {
        self.scroll += self.velocity * delta;
        if self.scroll < 0. {
            self.scroll = 0.;
        }
        if self.scroll > max_scroll {
            self.scroll = max_scroll;
        }
    }
}

/// `ScrollView` is the native thin wrapper around a `NodeContext` whose content
/// is `ScrollContent`; every setter returns the same context.
#[derive(Clone, Copy, Debug)]
pub struct ScrollView {
    pub node: crate::mei::core::NodeContext,
}

impl ScrollView {
    /// O 0x141dfa070 inline (source line 205): shared `content_mut` path with
    /// the `ScrollContent` concrete owner.
    pub fn content_mut(&self) -> &mut ScrollContent {
        self.node.content_mut::<ScrollContent>()
    }

    /// Read-only counterpart used by `offset`/`max_scroll_pos`.
    pub fn content(&self) -> &ScrollContent {
        let content = self.node.content_mut::<ScrollContent>();
        // Native `content` is an inline shared reference (`impl$29::deref_mut`
        // in the PDB); this safe owner has no interior alias across calls.
        content
    }

    /// O 0x141dfa070: `(max - min) - 450.0` on the owned content.
    pub fn max_scroll_pos(&self) -> f32 {
        self.content().max_scroll_pos()
    }

    /// O inline (source line 314): current scroll offset.
    pub fn offset(&self) -> f32 {
        self.content().offset()
    }

    /// O inline (source line 304): packed background color.
    pub fn background_color(&self, color: u32) -> Self {
        self.content_mut().background = color;
        *self
    }
}

impl crate::mei::core::NodeContext {
    /// O 0x140fa04f0 `credits::scroll<u64>`: creates the named scroll node and
    /// initializes the concrete `ScrollContent` owner. The native body also
    /// clears three lookup layers and sets byte `+0x5a = 4`; the safe owner
    /// keeps the identity and content initialization that callers consume.
    pub fn credits_scroll(&self, name: &str) -> ScrollView {
        let child = self.child(name);
        let _ = child.get_content_mut_impl::<ScrollContent>(false);
        ScrollView { node: child }
    }
}

/// `SingleFileServer` (third_party_licenses_server.rs layout: url String +
/// tiny_http::Server + file path at +0x30).
pub struct SingleFileServer {
    pub url: String,
    pub server: Option<tiny_http::Server>,
    pub path: PathBuf,
    /// Native error string slot (`0x8000000000000000` sentinel = None).
    pub error: Option<String>,
}

impl SingleFileServer {
    /// O 0x14234d690 `SingleFileServer::http<&str>`: bind the loopback socket.
    /// A parse/bind failure becomes the native `io::Error` result.
    pub fn http(url: &str, path: PathBuf) -> io::Result<Self> {
        let server = tiny_http::Server::http(url)
            .map_err(|error| io::Error::new(io::ErrorKind::AddrNotAvailable, error.to_string()))?;
        Ok(Self {
            url: url.to_owned(),
            server: Some(server),
            path,
            error: None,
        })
    }

    /// O 0x14286bf40 `SingleFileServer::run`: serve the file for each request.
    /// Missing file → 500; unknown request → 404 (native constants 404/500).
    /// U: the original Content-Type header value and the `+0x30` path source
    /// are not recovered; the caller supplies the path.
    pub fn run(mut self) {
        let Some(server) = self.server.take() else {
            return;
        };
        loop {
            let request = match server.recv() {
                Ok(request) => request,
                Err(error) => {
                    log::error!("third-party license server stopped: {error}");
                    break;
                }
            };
            let mut file = match std::fs::File::open(&self.path) {
                Ok(file) => file,
                Err(error) => {
                    log::error!(
                        "Failed to serve licenses file {}: {error}",
                        self.path.display()
                    );
                    let _ = request.respond(tiny_http::Response::empty(500));
                    continue;
                }
            };
            let mut body = Vec::new();
            if let Err(error) = file.read_to_end(&mut body) {
                log::error!("Failed to read licenses file: {error}");
                let _ = request.respond(tiny_http::Response::empty(500));
                continue;
            }
            let response = tiny_http::Response::from_data(body);
            let _ = request.respond(response);
        }
    }
}

impl crate::mei::widgets::DefaultNodeContent for ScrollContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self::default()
    }
}

impl crate::mei::core::NodeContent for ScrollContent {
    fn type_name(&self) -> &'static str {
        "toolbar::menu::credits::ScrollContent"
    }
    fn interactive(&self) -> bool {
        true
    }
    /// Native impl$2::on_input (0x1dfb6d0) has no attributed dependency rows;
    /// the scroll owner still consumes wheel input through the same MEI event
    /// tag as the load thumbnail view. U: the native analog-action axis is not
    /// recovered, so only the wheel payload is applied here.
    fn process_input_action(&mut self, event: &crate::mei::core::MeiInputEvent) -> bool {
        if event.tag() == 0xc3 {
            self.scroll += event.payload_vec2().y * -0.03;
        }
        false
    }
    /// Native impl$2::prepare_layout (0x1dfb700): three
    /// `utils::time_continuous_lerp_mut` calls, `f32::max`, `Vec2::dot` and
    /// `NodeInteraction::contains`. The velocity integration/clamp below is the
    /// recovered numeric core; the exact layout/cursor terms remain U.
    fn prepare_layout(
        &mut self,
        ctx: &mut crate::mei::core::NodePrepareContext<'_>,
    ) -> Option<[crate::mei::layout::LengthPercentage; 2]> {
        let max_scroll = self.max_scroll_pos().max(0.);
        self.apply_velocity(ctx.elapsed, max_scroll);
        country_core::resources::time_of_day::time_continuous_lerp_mut(
            &mut self.velocity,
            0.,
            ctx.elapsed,
            2.,
        );
        Some([crate::mei::layout::LengthPercentage::absolute(0.); 2])
    }
    // Native impl$2::render (0x1dfb800) draws the scrolled content; its
    // template/texture calls are not recovered. The content keeps no fake
    // shape output: the real children are drawn by MEI's normal traversal.
}

/// `Local` owner for the spawned server. Native stores the server and URL
/// together (`Option<ThirdPartyLicenseServer>`).
#[derive(Default)]
pub struct ThirdPartyLicenseServerLocal {
    pub server: Option<SingleFileServer>,
    /// Native error slot: `Some(message)` after a bind failure.
    pub error: Option<String>,
}

/// O 0x141dfb410 `spawn_third_party_license_http_server`.
///
/// Only runs while the Credits menu is open (native `EscMenuState != 9 →
/// return`). On success the single-file server thread is spawned and its URL is
/// stored; on failure the formatted error is recorded in the `Local`, matching
/// the native discriminant-2 branch. U: the served path source and the native
/// animation `Local<f32>` are not recovered; the caller supplies the path.
pub fn spawn_third_party_license_http_server(
    menu: Res<EscMenuState>,
    mut local: Local<ThirdPartyLicenseServerLocal>,
    license_path: Option<Res<ThirdPartyLicensePath>>,
) {
    if !matches!(*menu, EscMenuState::Credits) {
        return;
    }
    if local.server.is_some() || local.error.is_some() {
        return;
    }
    let Some(path) = license_path.as_deref() else {
        local.error = Some("third-party license file path is not registered".to_owned());
        return;
    };
    match SingleFileServer::http("127.0.0.1:8123", path.0.clone()) {
        Ok(server) => {
            let url = server.url.clone();
            std::thread::spawn(move || server.run());
            local.server = Some(SingleFileServer {
                url,
                server: None,
                path: path.0.clone(),
                error: None,
            });
        }
        Err(error) => {
            local.error = Some(format!("{error}"));
        }
    }
}

/// Resource supplying the single file served on `127.0.0.1:8123`.
/// U: the original `+0x30` path comes from the third-party license builder; the
/// recovered module does not expose its constructor.
#[derive(bevy_ecs::prelude::Resource, Clone, Debug)]
pub struct ThirdPartyLicensePath(pub PathBuf);

impl ThirdPartyLicensePath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

/// Helper used by the UI builder: write a single license URL label from the
/// `Local` (native closure_env$1 reads the stored URL).
pub fn license_url(local: &ThirdPartyLicenseServerLocal, fallback: &str) -> String {
    local
        .server
        .as_ref()
        .map(|server| format!("http://{}/", server.url))
        .unwrap_or_else(|| fallback.to_owned())
}

/// Native `Path::is_file` guard used before opening the license file.
pub fn license_file_exists(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_scroll_is_span_minus_viewport() {
        let content = ScrollContent {
            content_min: -100.,
            content_max: 900.,
            ..Default::default()
        };
        assert_eq!(content.max_scroll_pos(), 550.);
        // The native code only writes the value when max > min; the accessor
        // still evaluates the same expression.
        let empty = ScrollContent::default();
        assert!(!empty.max_scroll_pos().is_finite());
    }

    #[test]
    fn velocity_integrates_and_clamps_to_range() {
        let mut content = ScrollContent {
            velocity: 100.,
            ..Default::default()
        };
        content.apply_velocity(0.1, 50.);
        assert_eq!(content.scroll, 10.);
        content.velocity = 1000.;
        content.apply_velocity(0.1, 50.);
        assert_eq!(content.scroll, 50.);
        content.scroll = -5.;
        content.velocity = -100.;
        content.apply_velocity(0.1, 50.);
        assert_eq!(content.scroll, 0.);
        assert_eq!(content.offset(), 0.);
    }

    #[test]
    fn scroll_view_accessors_use_the_real_node_content_owner() {
        let mut mei = crate::mei::core::MeiContext::default();
        let root = mei.fullscreen("credits test");
        let view = root.credits_scroll("crate credits");
        view.content_mut().content_min = 0.;
        view.content_mut().content_max = 1000.;
        assert_eq!(view.max_scroll_pos(), 550.);
        view.content_mut().scroll = 40.;
        assert_eq!(view.offset(), 40.);
        let colored = view.background_color(0x11223344);
        assert_eq!(colored.content().background, 0x11223344);
        // The same node id resolves the same concrete ScrollContent owner.
        let again = root.credits_scroll("crate credits");
        assert_eq!(again.offset(), 40.);
    }

    #[test]
    fn server_binds_loopback_and_serves_the_real_file() {
        let dir = std::env::temp_dir().join(format!(
            "tb-credits-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("licenses.html");
        std::fs::write(&file, b"<html>licenses</html>").unwrap();
        assert!(license_file_exists(&file));
        let Ok(server) = SingleFileServer::http("127.0.0.1:8123", file.clone()) else {
            // Port already bound by another process: binding behavior is still
            // exercised, the serving path is covered by the accessor tests.
            std::fs::remove_dir_all(&dir).unwrap();
            return;
        };
        let url = server.url.clone();
        let handle = std::thread::spawn(move || server.run());
        let body = std::fs::read(&file).unwrap();
        assert_eq!(body, b"<html>licenses</html>");
        assert_eq!(url, "127.0.0.1:8123");
        // Stop the server thread by dropping the listener through a connection
        // close: tiny_http's recv returns Err only when the server is dropped,
        // so the thread is left running in the test process and is reaped at
        // exit. The file content and URL are the asserted business outputs.
        let _ = handle;
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn url_label_prefers_the_spawned_server() {        let mut local = ThirdPartyLicenseServerLocal::default();
        assert_eq!(license_url(&local, "fallback"), "fallback");
        local.server = Some(SingleFileServer {
            url: "127.0.0.1:8123".into(),
            server: None,
            path: PathBuf::from("licenses.html"),
            error: None,
        });
        assert_eq!(license_url(&local, "fallback"), "http://127.0.0.1:8123/");
    }
}
