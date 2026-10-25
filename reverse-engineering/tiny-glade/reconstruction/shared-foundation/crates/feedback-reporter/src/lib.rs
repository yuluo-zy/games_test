//! B crates/feedback-reporter/src/lib.rs.
//! The original game combines Bevy events, ZIP attachments and blocking HTTP.
//! Field names are reconstructed consumer-facing names, not claimed Rust ABI.

use anyhow::{Result, anyhow};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use relative_path::{RelativePath, RelativePathBuf};
use std::fs;
use std::io::{Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use zip::write::FileOptions;

/// Observed in B report_player_feedback's obfstr constant.
pub const REPORT_ENDPOINT: &str =
    "http://masham.pouncelight.games:30399/report/9bd72b15-f3cf-40df-89ef-192ef65eba8a";
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
pub const WAIT_FOR_FILES_FRAMES: u32 = 600;

/// Files selected by the game's save producer, rather than a recursive walk.
#[derive(Debug, Clone)]
pub struct PlayerFeedbackReportSaveFiles {
    pub root_dir: PathBuf,
    pub files: Vec<RelativePathBuf>,
    /// Original pending consumer requires two files before sending. The exact
    /// author field names are unknown; callers supply the producer's two paths.
    pub ready_files: [PathBuf; 2],
}

#[derive(Debug, Clone)]
pub struct FeedbackBenchmarkInfo {
    pub benchmark_file: PathBuf,
    pub gpu_utilization_file: PathBuf,
}

#[derive(Event, Debug, Clone)]
pub struct ReportPlayerFeedbackCmd {
    pub text: String,
    pub meta: String,
    pub screenshot: Option<Vec<u8>>,
    pub save_files: Option<PlayerFeedbackReportSaveFiles>,
    pub benchmark: Option<FeedbackBenchmarkInfo>,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerFeedbackReportSendingFinished {
    pub error: Option<String>,
}

#[derive(Default)]
pub struct PendingFeedback {
    pub report: Option<ReportPlayerFeedbackCmd>,
    pub waited_frames: u32,
}

impl PendingFeedback {
    pub fn replace(&mut self, report: ReportPlayerFeedbackCmd) {
        self.report = Some(report);
        self.waited_frames = 0;
    }

    /// Native B 0x140690a2a..0x140690aae: wait while either file is absent
    /// and counter <=600. The counter increments on the waiting branch.
    /// No wall-clock conversion or upload retry is introduced.
    pub fn take_ready(&mut self) -> Option<ReportPlayerFeedbackCmd> {
        let report = self.report.as_ref()?;
        if let Some(files) = &report.save_files {
            let ready = files.ready_files[0].is_file() && files.ready_files[1].is_file();
            if !ready && self.waited_frames <= WAIT_FOR_FILES_FRAMES {
                self.waited_frames = self.waited_frames.wrapping_add(1);
                return None;
            }
        }
        self.waited_frames = self.waited_frames.wrapping_add(1);
        self.report.take()
    }
}

/// Packing projection of original B zip_dir at RVA 0x1002650.
/// The archive prefix is the input directory's UTF-8 filename; selected
/// relative paths retain order, missing paths are skipped, files are read
/// completely, and a selected directory follows the original unimplemented
/// branch. The original zip crate supplies encoding/compression.
pub fn zip_dir<W: Write + Seek>(
    save: &PlayerFeedbackReportSaveFiles,
    writer: W,
    compression: zip::CompressionMethod,
) -> Result<W> {
    let dirname = save.root_dir.file_name().unwrap().to_str().unwrap();
    let prefix = RelativePath::new(dirname);
    let mut archive = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(compression)
        .unix_permissions(0o755);
    let mut contents = Vec::new();
    for relative in &save.files {
        let path = relative.to_path(&save.root_dir);
        // Native B zip_dir @ 0x141002650 stats the entry first and skips only
        // on a NotFound metadata error (Windows ERROR_FILE_NOT_FOUND == 2).
        // Any other metadata error falls through to is_file() and panics like
        // a non-file path, matching the decompiled stat -> is_file flow.
        if let Err(error) = fs::metadata(&path)
            && error.kind() == std::io::ErrorKind::NotFound
        {
            continue;
        }
        if !path.is_file() {
            panic!("not yet implemented");
        }
        let name = prefix.join(relative);
        log::trace!(
            "feedback save: adding file {} as {} ...",
            path.display(),
            name
        );
        archive.start_file(name.as_str(), options)?;
        fs::File::open(path)?.read_to_end(&mut contents)?;
        archive.write_all(&contents)?;
        contents.clear();
    }
    Ok(archive.finish()?)
}

/// Native send_report's benchmark ZIP projection: benchmark.puffin, then
/// info.txt. Missing/invalid GPU text becomes an Err(...) diagnostic; missing
/// benchmark data fails the attachment, which send_report logs and omits.
pub fn zip_benchmark_data(info: &FeedbackBenchmarkInfo) -> Result<Vec<u8>> {
    let benchmark = fs::read(&info.benchmark_file)?;
    let diagnostic = match fs::read_to_string(&info.gpu_utilization_file) {
        Ok(text) => format!("GPU_UTILIZATION: Ok\n{text}"),
        Err(error) => format!("GPU_UTILIZATION: Err({error})"),
    };
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    archive.start_file("benchmark.puffin", options)?;
    archive.write_all(&benchmark)?;
    archive.start_file("info.txt", options)?;
    archive.write_all(diagnostic.as_bytes())?;
    Ok(archive.finish()?.into_inner())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPart {
    pub name: &'static str,
    pub bytes: Vec<u8>,
    pub mime: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedReport {
    pub parts: Vec<ReportPart>,
}

/// Recover the game-owned attachment and multipart assembly policy.
/// save packing failures propagate; benchmark packing failures only omit that
/// attachment. Binary parts do not receive invented upload filenames.
pub fn prepare_report(report: &ReportPlayerFeedbackCmd) -> Result<PreparedReport> {
    let mut parts = vec![
        ReportPart {
            name: "text",
            bytes: report.text.as_bytes().to_vec(),
            mime: None,
        },
        ReportPart {
            name: "meta",
            bytes: report.meta.as_bytes().to_vec(),
            mime: None,
        },
    ];
    if let Some(image) = &report.screenshot {
        parts.push(ReportPart {
            name: "screenshot",
            bytes: image.clone(),
            mime: Some("image/jpg"),
        });
    }
    if let Some(save) = &report.save_files {
        let archive = zip_dir(
            save,
            Cursor::new(Vec::new()),
            zip::CompressionMethod::Deflated,
        )?;
        parts.push(ReportPart {
            name: "save",
            bytes: archive.into_inner(),
            mime: Some("application/zip"),
        });
    }
    if let Some(benchmark) = &report.benchmark {
        match zip_benchmark_data(benchmark) {
            Ok(bytes) => parts.push(ReportPart {
                name: "benchmark",
                bytes,
                mime: Some("application/zip"),
            }),
            Err(error) => log::error!("Failed zipping benchmark data: {error}"),
        }
    }
    Ok(PreparedReport { parts })
}

/// Injectable HTTP boundary. A successful status ignores response-body errors,
/// matching the original branch before Response::text.
#[derive(Debug)]
pub struct ReportResponse {
    pub status: u16,
    pub body: std::result::Result<String, String>,
}

pub trait ReportTransport: Send + Sync + 'static {
    fn post(&self, report: PreparedReport) -> std::result::Result<ReportResponse, String>;
}

#[derive(Resource, Clone)]
pub struct FeedbackTransport(pub Arc<dyn ReportTransport>);

impl Default for FeedbackTransport {
    fn default() -> Self {
        Self(Arc::new(HttpReportTransport {
            endpoint: REPORT_ENDPOINT.into(),
        }))
    }
}

/// Explicit HTTP transport. Construction is offline; post performs the call.
/// Tests and fixture Apps inject their own transport and never contact the
/// observed game service.
pub struct HttpReportTransport {
    pub endpoint: String,
}

impl ReportTransport for HttpReportTransport {
    fn post(&self, report: PreparedReport) -> std::result::Result<ReportResponse, String> {
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .unwrap();
        let mut form = reqwest::blocking::multipart::Form::new();
        for part in report.parts {
            let mut value = if part.mime.is_some() {
                reqwest::blocking::multipart::Part::bytes(part.bytes)
            } else {
                reqwest::blocking::multipart::Part::text(String::from_utf8(part.bytes).unwrap())
            };
            if let Some(mime) = part.mime {
                value = value.mime_str(mime).unwrap();
            }
            form = form.part(part.name, value);
        }
        let response = client
            .post(&self.endpoint)
            .multipart(form)
            .send()
            .map_err(|e| e.to_string())?;
        let status = response.status().as_u16();
        let body = if (200..300).contains(&status) {
            Ok(String::new())
        } else {
            response.text().map_err(|e| e.to_string())
        };
        Ok(ReportResponse { status, body })
    }
}

pub fn send_report(
    report: &ReportPlayerFeedbackCmd,
    transport: &dyn ReportTransport,
) -> Result<()> {
    let prepared = prepare_report(report)?;
    let response = transport.post(prepared).map_err(|error| {
        log::error!("Error sending feedback report: {error}");
        anyhow!("Connection failed")
    })?;
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    let text = response.body.map_err(|error| {
        log::error!("Error while receiving feedback service response: {error}");
        anyhow!("Error while waiting for a response")
    })?;
    Err(anyhow!(text))
}

/// The original plugin registers both events and an Update system.
/// The reconstruction's HTTP boundary is supplied as a Resource, so test Apps
/// can exercise the game state machine without external messages.
pub struct FeedbackReporterPlugin;
impl Plugin for FeedbackReporterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FeedbackTransport>()
            .add_event::<ReportPlayerFeedbackCmd>()
            .add_event::<PlayerFeedbackReportSendingFinished>()
            .add_systems(Update, report_player_feedback);
    }
}

pub fn report_player_feedback(
    mut reports: EventReader<ReportPlayerFeedbackCmd>,
    mut finished: EventWriter<PlayerFeedbackReportSendingFinished>,
    mut pending: Local<PendingFeedback>,
    transport: Res<FeedbackTransport>,
) {
    for report in reports.read() {
        pending.replace(report.clone());
    }
    let Some(report) = pending.take_ready() else {
        return;
    };
    let error = match send_report(&report, transport.0.as_ref()) {
        Ok(()) => {
            log::info!("Feedback successfully sent");
            None
        }
        Err(error) => {
            log::error!("Error sending feedback: {error}");
            Some(error.to_string())
        }
    };
    finished.write(PlayerFeedbackReportSendingFinished { error });
}

/// Convenience constructor for fixture/consumer paths; no file is created.
pub fn selected_save_files(
    root: &Path,
    files: Vec<RelativePathBuf>,
    ready_files: [PathBuf; 2],
) -> PlayerFeedbackReportSaveFiles {
    PlayerFeedbackReportSaveFiles {
        root_dir: root.into(),
        files,
        ready_files,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "tg-feedback-{}-{}",
                std::process::id(),
                FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn save(&self) -> PlayerFeedbackReportSaveFiles {
            PlayerFeedbackReportSaveFiles {
                root_dir: self.0.clone(),
                files: vec![],
                ready_files: [self.0.join("ready-a"), self.0.join("ready-b")],
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn command() -> ReportPlayerFeedbackCmd {
        ReportPlayerFeedbackCmd {
            text: "message".into(),
            meta: "metadata".into(),
            screenshot: None,
            save_files: None,
            benchmark: None,
        }
    }
    fn unpack(bytes: Vec<u8>) -> Vec<(String, Vec<u8>, zip::CompressionMethod, u32)> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
        (0..archive.len())
            .map(|index| {
                let mut file = archive.by_index(index).unwrap();
                let name = file.name().to_string();
                let compression = file.compression();
                let mode = file.unix_mode().unwrap();
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();
                (name, bytes, compression, mode)
            })
            .collect()
    }

    #[test]
    fn save_archive_preserves_selection_order_prefix_and_missing_skip() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("nested")).unwrap();
        fs::write(fixture.0.join("nested/世界.bin"), [0, 255, 1, 128]).unwrap();
        fs::write(fixture.0.join("empty"), []).unwrap();
        fs::write(fixture.0.join("unselected"), b"must not appear").unwrap();
        let mut save = fixture.save();
        save.files = ["nested/世界.bin", "missing", "empty"]
            .map(RelativePathBuf::from)
            .into();
        let bytes = zip_dir(
            &save,
            Cursor::new(Vec::new()),
            zip::CompressionMethod::Deflated,
        )
        .unwrap()
        .into_inner();
        let entries = unpack(bytes);
        let prefix = fixture.0.file_name().unwrap().to_str().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, format!("{prefix}/nested/世界.bin"));
        assert_eq!(entries[0].1, [0, 255, 1, 128]);
        assert_eq!(entries[1].0, format!("{prefix}/empty"));
        assert!(entries[1].1.is_empty());
        assert_eq!(entries[0].2, zip::CompressionMethod::Deflated);
        assert_eq!(entries[0].3 & 0o777, 0o755);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn selected_directory_matches_original_unimplemented_branch() {
        let fixture = Fixture::new();
        fs::create_dir(fixture.0.join("dir")).unwrap();
        let mut save = fixture.save();
        save.files.push("dir".into());
        let _ = zip_dir(
            &save,
            Cursor::new(Vec::new()),
            zip::CompressionMethod::Stored,
        );
    }

    #[test]
    fn non_notfound_metadata_error_panics_not_skips() {
        // Native B 0x141002650 skips only on NotFound; a metadata error with
        // any other kind falls through to is_file() and panics. A NUL byte in
        // the path yields ErrorKind::InvalidInput on all platforms.
        let fixture = Fixture::new();
        let mut save = fixture.save();
        save.files.push(RelativePathBuf::from("bad\0path"));
        let panicked = std::panic::catch_unwind(|| {
            let _ = zip_dir(
                &save,
                Cursor::new(Vec::new()),
                zip::CompressionMethod::Stored,
            );
        })
        .is_err();
        assert!(panicked);
    }

    #[test]
    fn benchmark_zip_has_fixed_names_and_read_failure_diagnostic() {
        let fixture = Fixture::new();
        let benchmark = fixture.0.join("trace");
        let gpu = fixture.0.join("gpu");
        fs::write(&benchmark, [1, 2, 3]).unwrap();
        fs::write(&gpu, "42%\n").unwrap();
        let info = FeedbackBenchmarkInfo {
            benchmark_file: benchmark,
            gpu_utilization_file: gpu.clone(),
        };
        let entries = unpack(zip_benchmark_data(&info).unwrap());
        assert_eq!(
            entries.iter().map(|e| e.0.as_str()).collect::<Vec<_>>(),
            ["benchmark.puffin", "info.txt"]
        );
        assert_eq!(entries[0].1, [1, 2, 3]);
        assert_eq!(entries[1].1, b"GPU_UTILIZATION: Ok\n42%\n");
        fs::write(gpu, [0xff]).unwrap();
        let entries = unpack(zip_benchmark_data(&info).unwrap());
        assert!(
            String::from_utf8(entries[1].1.clone())
                .unwrap()
                .starts_with("GPU_UTILIZATION: Err(")
        );
    }

    #[test]
    fn multipart_policy_omits_failed_benchmark_and_keeps_empty_screenshot() {
        let fixture = Fixture::new();
        let mut cmd = command();
        cmd.screenshot = Some(Vec::new());
        cmd.save_files = Some(fixture.save());
        cmd.benchmark = Some(FeedbackBenchmarkInfo {
            benchmark_file: fixture.0.join("absent"),
            gpu_utilization_file: fixture.0.join("gpu"),
        });
        let parts = prepare_report(&cmd).unwrap().parts;
        assert_eq!(
            parts.iter().map(|p| p.name).collect::<Vec<_>>(),
            ["text", "meta", "screenshot", "save"]
        );
        assert_eq!(parts[2].mime, Some("image/jpg"));
        assert!(parts[2].bytes.is_empty());
        assert_eq!(parts[3].mime, Some("application/zip"));
    }

    struct Stub {
        responses: Mutex<Vec<std::result::Result<ReportResponse, String>>>,
        sent: Mutex<Vec<PreparedReport>>,
    }
    impl ReportTransport for Stub {
        fn post(&self, report: PreparedReport) -> std::result::Result<ReportResponse, String> {
            self.sent.lock().unwrap().push(report);
            self.responses.lock().unwrap().remove(0)
        }
    }
    fn stub(response: std::result::Result<ReportResponse, String>) -> Stub {
        Stub {
            responses: Mutex::new(vec![response]),
            sent: Mutex::new(Vec::new()),
        }
    }

    #[test]
    fn response_contract_covers_success_status_and_both_failure_boundaries() {
        for status in [200, 202, 299] {
            assert!(
                send_report(
                    &command(),
                    &stub(Ok(ReportResponse {
                        status,
                        body: Err("ignored".into())
                    }))
                )
                .is_ok()
            );
        }
        let error =
            send_report(&command(), &stub(Err("fixture socket failure".into()))).unwrap_err();
        assert_eq!(error.to_string(), "Connection failed");
        let error = send_report(
            &command(),
            &stub(Ok(ReportResponse {
                status: 500,
                body: Ok("service reason".into()),
            })),
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "service reason");
        let error = send_report(
            &command(),
            &stub(Ok(ReportResponse {
                status: 300,
                body: Err("fixture body failure".into()),
            })),
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "Error while waiting for a response");
    }

    #[test]
    fn missing_save_files_wait_through_600_then_release_and_replacement_resets() {
        let fixture = Fixture::new();
        let mut pending = PendingFeedback::default();
        let mut report = command();
        report.save_files = Some(fixture.save());
        pending.replace(report.clone());
        for count in 0..=600 {
            assert_eq!(pending.waited_frames, count);
            assert!(pending.take_ready().is_none());
        }
        assert!(pending.take_ready().is_some());
        assert!(pending.report.is_none());
        pending.replace(report);
        assert_eq!(pending.waited_frames, 0);
        fs::write(fixture.0.join("ready-a"), []).unwrap();
        assert!(pending.take_ready().is_none());
        fs::write(fixture.0.join("ready-b"), []).unwrap();
        assert!(pending.take_ready().is_some());
    }

    #[test]
    fn bevy_update_sends_latest_event_once_and_emits_finished() {
        let transport = Arc::new(stub(Ok(ReportResponse {
            status: 204,
            body: Ok(String::new()),
        })));
        let mut app = App::new();
        app.insert_resource(FeedbackTransport(transport.clone()))
            .add_plugins(FeedbackReporterPlugin);
        let first = command();
        let mut last = command();
        last.text = "last".into();
        app.world_mut().send_event(first);
        app.world_mut().send_event(last);
        app.update();
        app.update();
        let sent = transport.sent.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].parts[0].bytes, b"last");
        let events = app
            .world()
            .resource::<Events<PlayerFeedbackReportSendingFinished>>();
        // Event retention can expire after two updates, so verify one additional
        // request with a reader captured before the next update.
        let mut reader = events.get_cursor_current();
        drop(sent);
        transport
            .responses
            .lock()
            .unwrap()
            .push(Err("fixture failure".into()));
        app.world_mut().send_event(command());
        app.update();
        let events = app
            .world()
            .resource::<Events<PlayerFeedbackReportSendingFinished>>();
        let results = reader.read(events).collect::<Vec<_>>();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].error.as_deref(), Some("Connection failed"));
    }
    #[test]
    fn standalone_plugin_installs_transport_and_keeps_preinserted_provider() {
        let mut app = App::new();
        app.add_plugins(FeedbackReporterPlugin);
        assert!(app.world().contains_resource::<FeedbackTransport>());
        // No report events: initializing/updating does not send any request.
        app.update();
        let transport = Arc::new(stub(Ok(ReportResponse {
            status: 204,
            body: Ok(String::new()),
        })));
        let expected: Arc<dyn ReportTransport> = transport;
        let mut app = App::new();
        app.insert_resource(FeedbackTransport(expected.clone()))
            .add_plugins(FeedbackReporterPlugin);
        let installed = &app.world().resource::<FeedbackTransport>().0;
        assert!(Arc::ptr_eq(installed, &expected));
    }
}
