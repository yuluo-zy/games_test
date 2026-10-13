//! B 0x140dbaed0 deletion and inline normalization; 0x140dbb0d0 presentation.
use tg_iced::gui_runner::{Command, ProgramWithOutcome};
pub use tg_package_integrity::{FileIssues, PackageVerificationError};
pub const STEAM_INTEGRITY_URL: &str =
    "https://help.steampowered.com/en/faqs/view/0C48-FCBD-DA71-93EB";
pub type VerificationResult = Result<(), PackageVerificationError>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Close,
    OpenUrl(String),
    RemoveUnrecognizedPackageFile(String),
}
#[derive(Debug, Clone)]
pub struct Controls {
    pub result: VerificationResult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationStatus {
    pub heading: String,
    pub missing: Vec<String>,
    pub corrupt: Vec<String>,
    pub extra: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Advice {
    pub key: &'static str,
    pub link_label: &'static str,
    pub url: &'static str,
}
/// The external adapter owns file IO. The original performs a single unlink, not re-verification.
pub trait PackageFileRemover {
    fn remove_file(&mut self, package_relative_path: &str) -> std::io::Result<()>;
}
impl Controls {
    pub fn new(result: VerificationResult) -> Self {
        Self { result }
    }
    pub fn normalize_verification_result(&mut self) {
        if matches!(&self.result, Err(PackageVerificationError::Files(f)) if f.missing.is_empty() && f.corrupt.is_empty() && f.extra.is_empty())
        {
            self.result = Ok(());
        }
    }
    /// Failure deliberately leaves the report unchanged (original drops the IO error).
    /// The caller supplies a package-scoped adapter; this method never deletes by itself.
    pub fn remove_unrecognized_package_file(
        &mut self,
        path: &str,
        adapter: &mut impl PackageFileRemover,
    ) {
        if adapter.remove_file(path).is_ok() {
            if let Err(PackageVerificationError::Files(issues)) = &mut self.result {
                issues.extra.retain(|p| p != path);
            }
            self.normalize_verification_result();
        }
    }
    pub fn verification_status(&self) -> VerificationStatus {
        let (heading, issues) = match &self.result {
            Ok(()) => (
                "Crash.Advice.InstallationLooksGood".to_owned(),
                FileIssues::default(),
            ),
            Err(PackageVerificationError::Files(issues)) => {
                ("Crash.Advice.FilesLookIncorrect".to_owned(), issues.clone())
            }
            Err(PackageVerificationError::InvalidManifestChecksum) => (
                "Incorrect checksum of build-info/manifest.json".to_owned(),
                FileIssues::default(),
            ),
            Err(PackageVerificationError::ManifestRead(e)) => (
                format!("Error reading package manifest: {e}"),
                FileIssues::default(),
            ),
            Err(PackageVerificationError::ManifestParse(e)) => (
                format!("Malformed package manifest: {e}"),
                FileIssues::default(),
            ),
            Err(PackageVerificationError::UnsupportedVersion(v)) => (
                format!("Wrong package manifest version. Expected 1, got {v}"),
                FileIssues::default(),
            ),
        };
        VerificationStatus {
            heading,
            missing: issues.missing,
            corrupt: issues.corrupt,
            extra: issues.extra,
        }
    }
    pub fn advice(&self) -> Option<Advice> {
        self.result.as_ref().err().map(|_| Advice {
            key: "Crash.Advice.VerifyIntegrityOrReinstall",
            link_label: "Crash.Advice.SteamFileIntegrityCheck",
            url: STEAM_INTEGRITY_URL,
        })
    }
    pub fn view(&self) -> VerificationStatus {
        self.verification_status()
    }
    pub fn update_with_remover(
        &mut self,
        message: Message,
        adapter: &mut impl PackageFileRemover,
    ) -> Command {
        if let Message::RemoveUnrecognizedPackageFile(path) = message {
            self.remove_unrecognized_package_file(&path, adapter);
            Command::None
        } else {
            self.update(message)
        }
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = ();
    fn update(&mut self, message: Message) -> Command {
        match message {
            Message::Close => Command::CloseWindow,
            Message::OpenUrl(url) => Command::OpenUrl(url),
            Message::RemoveUnrecognizedPackageFile(path) => Command::RemovePackageFile(path),
        }
    }
    fn program_outcome(&self) -> Option<()> {
        None
    }
}
/// Candidate OS adapter confines the GUI's deletion to its selected package root.
/// This scope check is an integration safety wrapper, not an inferred original check.
pub struct ScopedPackageFileRemover {
    root: std::path::PathBuf,
}
impl ScopedPackageFileRemover {
    pub fn new(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(Self {
            root: root.as_ref().canonicalize()?,
        })
    }
}
impl PackageFileRemover for ScopedPackageFileRemover {
    fn remove_file(&mut self, name: &str) -> std::io::Result<()> {
        let path = std::path::Path::new(name);
        if path
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Expected a package-relative file",
            ));
        }
        let target = self.root.join(path).canonicalize()?;
        if target == self.root || !target.starts_with(&self.root) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "File is outside the selected package",
            ));
        }
        std::fs::remove_file(target)
    }
}

#[cfg(feature = "native-gui")]
impl Controls {
    pub fn native_element(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::widget::{button, column, container, row, scrollable};
        use tg_iced::iced::{Alignment, Length};
        use tg_iced::native_text as text;
        let status = self.view();
        let mut body =
            column![text(tg_iced::text(&status.heading).to_owned()).size(20)].spacing(12);
        if let Some(advice) = self.advice() {
            body = body
                .push(text(
                    tg_iced::text(advice.key).replace("{link}", tg_iced::text(advice.link_label)),
                ))
                .push(
                    button(text(tg_iced::text(advice.link_label).to_owned()))
                        .on_press(Message::OpenUrl(advice.url.into())),
                );
        }
        for (key, names) in [
            ("Crash.Advice.MissingOrUnreadableFiles", status.missing),
            ("Crash.Advice.CorruptOrModifiedFiles", status.corrupt),
        ] {
            if !names.is_empty() {
                body = body.push(text(tg_iced::text(key).to_owned()));
                for name in names {
                    body = body.push(text(name));
                }
            }
        }
        if !status.extra.is_empty() {
            body = body.push(text(
                tg_iced::text("Crash.Advice.UnrecognizedFilesClickToDelete").to_owned(),
            ));
            for name in status.extra {
                body = body.push(
                    row![
                        text(name.clone()).width(Length::Fill),
                        button(text("🗑️")).on_press(Message::RemoveUnrecognizedPackageFile(name))
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                );
            }
        }
        container(
            column![
                scrollable(body).height(Length::Fill),
                button(text(tg_iced::text("Ok").to_owned())).on_press(Message::Close)
            ]
            .spacing(20)
            .padding(30),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
#[cfg(feature = "native-gui")]
pub struct NativeControls {
    pub controls: Controls,
    pub remover: ScopedPackageFileRemover,
}
#[cfg(feature = "native-gui")]
impl ProgramWithOutcome for NativeControls {
    type Message = Message;
    type Outcome = ();
    fn update(&mut self, message: Message) -> Command {
        self.controls
            .update_with_remover(message, &mut self.remover)
    }
    fn program_outcome(&self) -> Option<()> {
        None
    }
}
#[cfg(feature = "native-gui")]
impl tg_iced::gui_runner::NativeProgram for NativeControls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        self.controls.native_element()
    }
    fn update_native(&mut self, message: Message) -> tg_iced::iced::Command<Message> {
        let command = self.update(message);
        match command {
            Command::OpenUrl(url) => {
                if let Err(e) = webbrowser::open(&url) {
                    eprintln!("Error opening web browser: {e}");
                }
                tg_iced::iced::Command::none()
            }
            other => tg_iced::gui_runner::platform_command(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Remover {
        fail: bool,
        calls: Vec<String>,
    }
    impl PackageFileRemover for Remover {
        fn remove_file(&mut self, p: &str) -> std::io::Result<()> {
            self.calls.push(p.into());
            if self.fail {
                Err(std::io::Error::other("fixture failure"))
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn failed_deletion_preserves_report_then_success_normalizes_without_reverify() {
        let mut c = Controls::new(Err(PackageVerificationError::Files(FileIssues {
            extra: vec!["a".into(), "a".into()],
            ..Default::default()
        })));
        let before = c.result.clone();
        let mut r = Remover {
            fail: true,
            calls: vec![],
        };
        c.remove_unrecognized_package_file("a", &mut r);
        assert_eq!(c.result, before);
        r.fail = false;
        c.remove_unrecognized_package_file("a", &mut r);
        assert_eq!(c.result, Ok(()));
        assert_eq!(r.calls, ["a", "a"]);
    }
    #[test]
    fn deleting_extra_keeps_missing_and_corrupt_order() {
        let mut c = Controls::new(Err(PackageVerificationError::Files(FileIssues {
            missing: vec!["b".into(), "a".into()],
            corrupt: vec!["c".into()],
            extra: vec!["e".into()],
        })));
        c.remove_unrecognized_package_file(
            "e",
            &mut Remover {
                fail: false,
                calls: vec![],
            },
        );
        let v = c.view();
        assert_eq!(v.missing, ["b", "a"]);
        assert_eq!(v.corrupt, ["c"]);
        assert!(v.extra.is_empty());
        assert!(c.advice().is_some());
    }
}
