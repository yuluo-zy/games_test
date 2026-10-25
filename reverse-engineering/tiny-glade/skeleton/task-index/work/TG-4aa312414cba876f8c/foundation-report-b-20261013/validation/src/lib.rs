use tg_integrity_report::{controls, Controls, run_integrity_report_gui};
use tg_iced::gui_runner::{Options, GuiError, GuiBackend};
include!("../../report_consumer.rs");
#[cfg(test)] mod tests {
use super::*;
struct Recorder { calls: usize, observed: Option<Options>, fail: bool }
impl GuiBackend<Controls> for Recorder { fn run(&mut self, controls: &mut Controls, options: &Options)->Result<(),GuiError>{ self.calls+=1; self.observed=Some(options.clone()); assert!(matches!(controls.result, Err(tg_package_integrity::PackageVerificationError::ManifestRead(_)))); if self.fail {Err(GuiError::BackendUnavailable)} else {Ok(())} } }
#[test] fn failed_manifest_is_presented_once_and_backend_error_propagates(){
let path=std::env::temp_dir().join(format!("tg-no-manifest-{}",std::process::id()));
let mut recorder=Recorder {calls:0,observed:None,fail:false};
assert_eq!(verify_package_integrity_and_show_report(&path,&mut recorder),Ok(()));
assert_eq!(recorder.calls,1);assert_eq!(recorder.observed,Some(Options::new("Tiny Glade file verification report",1000,800)));
recorder.fail=true;assert_eq!(verify_package_integrity_and_show_report(&path,&mut recorder),Err(GuiError::BackendUnavailable));assert_eq!(recorder.calls,2);
}
#[test] fn native_success_size_policy(){assert_eq!(verification_report_options(&Ok(())),Options::new("Tiny Glade file verification report",500,300));}
}
