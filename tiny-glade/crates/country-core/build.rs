use std::{env, fs, path::{Path, PathBuf}, process::Command};

fn llvm_tool(name: &str) -> PathBuf {
    if let Some(root) = env::var_os("LLVM_BIN") { return PathBuf::from(root).join(name); }
    let installed = Path::new("C:/Program Files/LLVM/bin").join(name);
    if installed.exists() { installed } else { PathBuf::from(name) }
}

fn main() {
    println!("cargo:rerun-if-env-changed=FMOD_LIB_DIR");
    println!("cargo:rerun-if-env-changed=TG_INSTALL_ROOT");
    println!("cargo:rerun-if-env-changed=LLVM_BIN");
    if env::var_os("CARGO_FEATURE_NATIVE_AUDIO").is_none()
        || env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") { return; }
    if let Some(directory) = env::var_os("FMOD_LIB_DIR") {
        // An installed FMOD SDK can supply its original import libraries.
        println!("cargo:rustc-link-search=native={}", PathBuf::from(directory).display());
        return;
    }
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let installation = env::var_os("TG_INSTALL_ROOT").map(PathBuf::from)
        .unwrap_or_else(|| root.ancestors().nth(3).unwrap().join("ljxsj_92385/Tiny Glade"));
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // The shipped DLLs contain the C ABI used by the unchanged locked libfmod.
    // Generate linker metadata only; do not replace or modify either DLL.
    for library in ["fmod", "fmodstudio"] {
        let dll = installation.join(format!("{library}.dll"));
        println!("cargo:rerun-if-changed={}", dll.display());
        let exports = Command::new(llvm_tool("llvm-readobj.exe"))
            .arg("--coff-exports").arg(&dll).output().expect("LLVM llvm-readobj is required, or set FMOD_LIB_DIR to the FMOD SDK lib directory");
        assert!(exports.status.success(), "Cannot read {}: {}", dll.display(), String::from_utf8_lossy(&exports.stderr));
        let mut definition = format!("LIBRARY {library}.dll\nEXPORTS\n");
        let mut count = 0;
        for line in String::from_utf8_lossy(&exports.stdout).lines() {
            if let Some(name) = line.trim().strip_prefix("Name: ").filter(|name| name.starts_with("FMOD_")) {
                definition.push_str(name);
                definition.push('\n');
                count += 1;
            }
        }
        assert!(count > 0, "No FMOD C exports in {}", dll.display());
        let def = output.join(format!("{library}.def"));
        fs::write(&def, definition).unwrap();
        let result = Command::new(llvm_tool("llvm-dlltool.exe"))
            .args(["-m", "i386:x86-64", "-d"]).arg(&def)
            .arg("-l").arg(output.join(format!("{library}_vc.lib")))
            .output().expect("LLVM llvm-dlltool is required, or set FMOD_LIB_DIR");
        assert!(result.status.success(), "FMOD import library generation failed: {}", String::from_utf8_lossy(&result.stderr));
        // Windows resolves static imports before main. Keep the original DLL
        // beside local executables and test binaries, inside Cargo's target.
        let profile = output.ancestors().nth(3).expect("Cargo OUT_DIR layout");
        for directory in [profile.to_path_buf(), profile.join("deps")] {
            fs::create_dir_all(&directory).unwrap();
            let destination = directory.join(format!("{library}.dll"));
            let original = fs::read(&dll).unwrap();
            if fs::read(&destination).ok().as_deref() != Some(original.as_slice()) {
                fs::write(&destination, original).unwrap();
            }
        }
    }
    println!("cargo:rustc-link-search=native={}", output.display());
}
