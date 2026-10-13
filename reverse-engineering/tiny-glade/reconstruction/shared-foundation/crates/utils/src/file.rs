use std::{fs,io,path::{Path,PathBuf},time::SystemTime};
use crate::local_time::system_time_to_local_time;
pub fn date_time_string_for_filename_at(now:SystemTime)->String {
    let now=system_time_to_local_time(now);
    format!("{:04}-{:02}-{:02} {:02}-{:02}-{:02}",now.year(),u8::from(now.month()),now.day(),now.hour(),now.minute(),now.second())
}
pub fn date_time_string_for_filename()->String {date_time_string_for_filename_at(SystemTime::now())}
/// B preserves the whole file name (including its extension), appends the date
/// and `.json`, and keeps the directory. It panics for absent/non-UTF8 names.
pub fn generate_backup_file_path_at(path:&Path,now:SystemTime)->PathBuf {
    let name=path.file_name().and_then(|x|x.to_str()).expect("file must have a UTF-8 file name");
    path.with_file_name(format!("{name}-{}.json",date_time_string_for_filename_at(now)))
}
pub fn generate_backup_file_path(path:&Path)->PathBuf {generate_backup_file_path_at(path,SystemTime::now())}
pub fn copy_file_to_backup(path:&Path)->io::Result<()> {
    let backup=generate_backup_file_path(path);fs::copy(path,backup).map(|_|())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn backup_filename_retains_old_extension_and_parent() {
        assert_eq!(generate_backup_file_path_at(Path::new("saves/a.ron"),SystemTime::UNIX_EPOCH),PathBuf::from("saves/a.ron-1970-01-01 00-00-00.json"));
    }
}
