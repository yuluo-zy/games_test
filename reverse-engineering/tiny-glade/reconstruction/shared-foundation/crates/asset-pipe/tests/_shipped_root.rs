/// Resolve the shipped B release root. The original Windows layout is the
/// default; this machine keeps a macOS copy for local shipped verification.
pub fn root() -> std::path::PathBuf {
    let windows = std::path::Path::new("D:/game/ljxsj_92385/Tiny Glade");
    if windows.exists() {
        windows.to_path_buf()
    } else {
        std::path::Path::new("/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade").to_path_buf()
    }
}
