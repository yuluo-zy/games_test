/// Resolve the shipped B release root. The original Windows layout is the
/// default; this machine keeps a macOS copy for local shipped verification.
pub fn root() -> std::path::PathBuf {
    if let Some(root) = std::env::var_os("TINY_GLADE_ROOT") {
        return std::path::PathBuf::from(root);
    }
    let windows = std::path::Path::new("D:/game/ljxsj_92385/Tiny Glade");
    if windows.exists() {
        return windows.to_path_buf();
    }
    let mac = std::path::Path::new("/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade");
    if mac.exists() {
        return mac.to_path_buf();
    }
    // Workspace-relative copy (see `.gitignore`: `/Tiny Glade/`, linked as `ljxsj_92385/Tiny Glade`).
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade")
}
