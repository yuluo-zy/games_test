// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/init.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b9ba40daf070950451b8d534c5f6b3fe1103ff38151f9e0f0fff018ab002ec87
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: init.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 34, 74, 80, 104, 140, 142, 155, 175, 176, 184, 188, 194, 197, 236, 240, 248, 683, 708, 936, 937, 938, 1926, 1935, 1936, 1937, 2318, 3300, 3503, 4223, 4240, 4241
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use dxgi_enumerate_gpus; // namespace candidate
//   package identities: [{"name":"dxgi-enumerate-gpus","source":"local_or_path","version":"0.1.0"}]
// use known_folders; // namespace candidate
//   package identities: [{"name":"known-folders","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nvapi; // namespace candidate
//   package identities: [{"name":"nvapi","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.4"}]
// use nvapi_hi; // namespace candidate
//   package identities: [{"name":"nvapi-hi","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLoader::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":20224,"rva":12421872,"va":5381130992}
// 编译实例 RVAs: 0xbd8af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t1512012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e84c121700","size":5,"soft_fail":false,"target_va":5382644528,"va":5381132511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7507467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8f5718dff","size":5,"soft_fail":false,"target_va":5373629488,"va":5381136950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6237464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8e8d2a0ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5381138195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6049620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8acb0a3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381131103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6049924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e87cafa3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381131423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=14
//     target_aliases=69 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5616292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e85c4daaff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381132319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8a8370000","size":5,"soft_fail":false,"target_va":5381151632,"va":5381137379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=5 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1696201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8c9e11900","size":5,"soft_fail":false,"target_va":5382827728,"va":5381131522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30403914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e84aedcf01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381137073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6775096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8c89e98ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5381131619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30403854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e80eedcf01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381137104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t747173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8a5660b00","size":5,"soft_fail":false,"target_va":5381881088,"va":5381133910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::entry::Entry::load_from
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13472777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e80994cd00","size":5,"soft_fail":false,"target_va":5394605344,"va":5381132562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30405125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e805f2cf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381137166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ash::entry::loaded::LoadingError> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e808dfffff","size":5,"soft_fail":false,"target_va":5381123664,"va":5381132099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e818cdffff","size":5,"soft_fail":false,"target_va":5381124592,"va":5381137619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30405819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8bbf4cf01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381137056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] known_folders::win::get_known_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"known-folders","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13046436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8a412c700","size":5,"soft_fail":false,"target_va":5394177680,"va":5381131239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1568017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e811ed1700","size":5,"soft_fail":false,"target_va":5382699312,"va":5381131290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1567527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e827eb1700","size":5,"soft_fail":false,"target_va":5382698576,"va":5381131044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t1612682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e88a9b1800","size":5,"soft_fail":false,"target_va":5382744080,"va":5381131393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1613076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381130992,"raw_hex":"e8149d1800","size":5,"soft_fail":false,"target_va":5382745328,"va":5381132247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*160(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381130992,"raw_hex":"ff95a0000000","size":6,"soft_fail":false,"va":5381133453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20492,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":683,"record_offset":31296,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":21112,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":20640,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":22368,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":27912,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::try_enumerate_instance_version
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":683,"record_offset":26156,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":39188,"source_line":222,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_major
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":683,"record_offset":26312,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":39190,"source_line":41,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_minor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":683,"record_offset":26340,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":39191,"source_line":45,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_patch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":683,"record_offset":26368,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":39192,"source_line":49,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":25656,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":22876,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":683,"record_offset":30956,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":683,"record_offset":24360,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::flatten
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":26284,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":13639,"source_line":2549,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":26896,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":21196,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":22564,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":20448,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":24276,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":24980,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":24880,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":22336,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":22196,"parent_record_offset":20224,"ancestor_record_offset":20224,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":20224,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":20224,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":20224,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20524,"parent_record_offset":20492,"ancestor_record_offset":20224,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21876,"parent_record_offset":21820,"ancestor_record_offset":20224,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":22444,"parent_record_offset":22412,"ancestor_record_offset":20224,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":20844,"parent_record_offset":20816,"ancestor_record_offset":20224,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":20224,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":20224,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":20224,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":22468,"parent_record_offset":22444,"ancestor_record_offset":20224,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":20904,"parent_record_offset":20872,"ancestor_record_offset":20224,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":20872,"parent_record_offset":20844,"ancestor_record_offset":20224,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20224,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":20780,"parent_record_offset":20752,"ancestor_record_offset":20224,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":21136,"parent_record_offset":21112,"ancestor_record_offset":20224,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":20752,"parent_record_offset":20724,"ancestor_record_offset":20224,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":20224,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":20724,"parent_record_offset":20696,"ancestor_record_offset":20224,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":20668,"parent_record_offset":20640,"ancestor_record_offset":20224,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":22932,"parent_record_offset":22904,"ancestor_record_offset":20224,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":22904,"parent_record_offset":22876,"ancestor_record_offset":20224,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":26760,"parent_record_offset":26736,"ancestor_record_offset":20224,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":22412,"parent_record_offset":22368,"ancestor_record_offset":20224,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":21160,"parent_record_offset":21136,"ancestor_record_offset":20224,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":20816,"parent_record_offset":20780,"ancestor_record_offset":20224,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":29648,"parent_record_offset":29624,"ancestor_record_offset":20224,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":683,"record_offset":26196,"parent_record_offset":26156,"ancestor_record_offset":20224,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":683,"record_offset":26220,"parent_record_offset":26196,"ancestor_record_offset":20224,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":683,"record_offset":26244,"parent_record_offset":26220,"ancestor_record_offset":20224,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":20224,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":21056,"parent_record_offset":21032,"ancestor_record_offset":20224,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":30508,"parent_record_offset":30480,"ancestor_record_offset":20224,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":26920,"parent_record_offset":26896,"ancestor_record_offset":20224,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":21624,"parent_record_offset":21240,"ancestor_record_offset":20224,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":25036,"parent_record_offset":25004,"ancestor_record_offset":20224,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":25064,"parent_record_offset":25004,"ancestor_record_offset":20224,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":21032,"parent_record_offset":20780,"ancestor_record_offset":20224,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=59
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":21340,"parent_record_offset":21304,"ancestor_record_offset":20224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":24324,"parent_record_offset":24300,"ancestor_record_offset":20224,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":24500,"parent_record_offset":24476,"ancestor_record_offset":20224,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":24424,"parent_record_offset":24388,"ancestor_record_offset":20224,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":20596,"parent_record_offset":20572,"ancestor_record_offset":20224,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":22532,"parent_record_offset":22368,"ancestor_record_offset":20224,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":21304,"parent_record_offset":21240,"ancestor_record_offset":20224,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":683,"record_offset":27944,"parent_record_offset":27912,"ancestor_record_offset":20224,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":683,"record_offset":27976,"parent_record_offset":27944,"ancestor_record_offset":20224,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":683,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":20224,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":20224,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":20224,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":24452,"parent_record_offset":24388,"ancestor_record_offset":20224,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":26796,"parent_record_offset":26760,"ancestor_record_offset":20224,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":24920,"parent_record_offset":24880,"ancestor_record_offset":20224,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":24944,"parent_record_offset":24920,"ancestor_record_offset":20224,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":26820,"parent_record_offset":26796,"ancestor_record_offset":20224,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":26852,"parent_record_offset":26760,"ancestor_record_offset":20224,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":25092,"parent_record_offset":25004,"ancestor_record_offset":20224,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":25004,"parent_record_offset":24980,"ancestor_record_offset":20224,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":25116,"parent_record_offset":25092,"ancestor_record_offset":20224,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":21240,"parent_record_offset":21196,"ancestor_record_offset":20224,"inlinee":39185,"source_line":239,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":21660,"parent_record_offset":21624,"ancestor_record_offset":20224,"inlinee":39187,"source_line":242,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":683,"record_offset":21276,"parent_record_offset":21240,"ancestor_record_offset":20224,"inlinee":17230,"source_line":217,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":22244,"parent_record_offset":22220,"ancestor_record_offset":20224,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":20224,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":22268,"parent_record_offset":22244,"ancestor_record_offset":20224,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":21692,"parent_record_offset":21660,"ancestor_record_offset":20224,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":20224,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$10::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31C8
// 首条位置: {"module":140,"inlinee":12744}
// 编译实例 RVAs: 0x3e72b0, 0x593350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10090513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372801712,"raw_hex":"e811f89900","size":5,"soft_fail":false,"target_va":5382892528,"va":5372802010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10090554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372801712,"raw_hex":"e83af89900","size":5,"soft_fail":false,"target_va":5382892704,"va":5372802145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t10092462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372801712,"raw_hex":"e8aeff9900","size":5,"soft_fail":false,"target_va":5382894288,"va":5372801821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10090650","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372801712,"raw_hex":"e99af89900","size":5,"soft_fail":false,"target_va":5382892432,"va":5372801777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$14::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":42936,"rva":12434624,"va":5381143744}
// 编译实例 RVAs: 0xbdbcc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayer::vendor
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":11012,"rva":12417984,"va":5381127104}
// 编译实例 RVAs: 0xbd7bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1772982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127104,"raw_hex":"e8b60d1b00","size":5,"soft_fail":false,"target_va":5382900288,"va":5381127301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30415964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127104,"raw_hex":"e85c1cd001","size":5,"soft_fail":false,"target_va":5411543360,"va":5381127391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":11384,"parent_record_offset":11012,"ancestor_record_offset":11012,"inlinee":20192,"source_line":1442,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":11156,"parent_record_offset":11012,"ancestor_record_offset":11012,"inlinee":9143,"source_line":2397,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":11760,"parent_record_offset":11012,"ancestor_record_offset":11012,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":11272,"parent_record_offset":11240,"ancestor_record_offset":11012,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":11496,"parent_record_offset":11468,"ancestor_record_offset":11012,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":11300,"parent_record_offset":11272,"ancestor_record_offset":11012,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":11012,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":11444,"parent_record_offset":11408,"ancestor_record_offset":11012,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":11240,"parent_record_offset":11212,"ancestor_record_offset":11012,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":683,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":11012,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":683,"record_offset":11528,"parent_record_offset":11468,"ancestor_record_offset":11012,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":683,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":11012,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":683,"record_offset":11592,"parent_record_offset":11408,"ancestor_record_offset":11012,"inlinee":6428,"source_line":26,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":683,"record_offset":11624,"parent_record_offset":11592,"ancestor_record_offset":11012,"inlinee":6429,"source_line":36,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":11012,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":11012,"inlinee":17879,"source_line":432,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":11212,"parent_record_offset":11184,"ancestor_record_offset":11012,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":11184,"parent_record_offset":11156,"ancestor_record_offset":11012,"inlinee":9146,"source_line":1013,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":11812,"parent_record_offset":11784,"ancestor_record_offset":11012,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":11012,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 93, 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x320B
// 首条位置: {"module":155,"inlinee":12811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":155,"record_offset":118440,"parent_record_offset":118416,"ancestor_record_offset":118416,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 93, 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31F2
// 首条位置: {"module":155,"inlinee":12786}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":155,"record_offset":11044,"parent_record_offset":11012,"ancestor_record_offset":11012,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":155,"record_offset":11072,"parent_record_offset":11044,"ancestor_record_offset":11012,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":11100,"parent_record_offset":11072,"ancestor_record_offset":11012,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":11128,"parent_record_offset":11100,"ancestor_record_offset":11012,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 93, 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33F5
// 首条位置: {"module":155,"inlinee":13301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":10900,"parent_record_offset":10868,"ancestor_record_offset":10868,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":155,"record_offset":11072,"parent_record_offset":11044,"ancestor_record_offset":10868,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":10944,"parent_record_offset":10900,"ancestor_record_offset":10868,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":11300,"parent_record_offset":10900,"ancestor_record_offset":10868,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":11100,"parent_record_offset":11072,"ancestor_record_offset":10868,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":11128,"parent_record_offset":11100,"ancestor_record_offset":10868,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":155,"record_offset":11044,"parent_record_offset":11012,"ancestor_record_offset":10868,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":155,"record_offset":11012,"parent_record_offset":10980,"ancestor_record_offset":10868,"inlinee":12786,"source_line":93,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":10980,"parent_record_offset":10900,"ancestor_record_offset":10868,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x344B
// 首条位置: {"module":155,"inlinee":13387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":155,"record_offset":99056,"parent_record_offset":97852,"ancestor_record_offset":97852,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":155,"record_offset":99180,"parent_record_offset":97852,"ancestor_record_offset":97852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":98964,"parent_record_offset":97852,"ancestor_record_offset":97852,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":97900,"parent_record_offset":97852,"ancestor_record_offset":97852,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":99000,"parent_record_offset":97852,"ancestor_record_offset":97852,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":155,"record_offset":99480,"parent_record_offset":99456,"ancestor_record_offset":97852,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":155,"record_offset":99456,"parent_record_offset":99392,"ancestor_record_offset":97852,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":155,"record_offset":99424,"parent_record_offset":99392,"ancestor_record_offset":97852,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":155,"record_offset":99392,"parent_record_offset":99364,"ancestor_record_offset":97852,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":155,"record_offset":99364,"parent_record_offset":99336,"ancestor_record_offset":97852,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":155,"record_offset":99204,"parent_record_offset":99180,"ancestor_record_offset":97852,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":155,"record_offset":98704,"parent_record_offset":98680,"ancestor_record_offset":97852,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":98136,"parent_record_offset":97936,"ancestor_record_offset":97852,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":98264,"parent_record_offset":97936,"ancestor_record_offset":97852,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":99648,"parent_record_offset":99620,"ancestor_record_offset":97852,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":155,"record_offset":99228,"parent_record_offset":99204,"ancestor_record_offset":97852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":98100,"parent_record_offset":97936,"ancestor_record_offset":97852,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":98728,"parent_record_offset":98704,"ancestor_record_offset":97852,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":155,"record_offset":98752,"parent_record_offset":98728,"ancestor_record_offset":97852,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":155,"record_offset":98680,"parent_record_offset":98652,"ancestor_record_offset":97852,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":155,"record_offset":98652,"parent_record_offset":98624,"ancestor_record_offset":97852,"inlinee":12786,"source_line":93,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":155,"record_offset":98232,"parent_record_offset":98208,"ancestor_record_offset":97852,"inlinee":13301,"source_line":93,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":155,"record_offset":98168,"parent_record_offset":97936,"ancestor_record_offset":97852,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":98544,"parent_record_offset":98512,"ancestor_record_offset":97852,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":97936,"parent_record_offset":97900,"ancestor_record_offset":97852,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":98624,"parent_record_offset":98544,"ancestor_record_offset":97852,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":98208,"parent_record_offset":97936,"ancestor_record_offset":97852,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":17656,"rva":3410448,"va":5372119568}
// 编译实例 RVAs: 0x340a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14034aa10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119568,"raw_hex":"e8499f0000","size":5,"soft_fail":false,"target_va":5372160528,"va":5372119746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14034d830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119568,"raw_hex":"e898cd0000","size":5,"soft_fail":false,"target_va":5372172336,"va":5372119699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140351a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t69425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119568,"raw_hex":"e8310f0100","size":5,"soft_fail":false,"target_va":5372189216,"va":5372119786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140352450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t72187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372119568,"raw_hex":"e8fb190100","size":5,"soft_fail":false,"target_va":5372191824,"va":5372119632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":18012,"parent_record_offset":17656,"ancestor_record_offset":17656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":140,"record_offset":17860,"parent_record_offset":17656,"ancestor_record_offset":17656,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":140,"record_offset":17952,"parent_record_offset":17656,"ancestor_record_offset":17656,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":140,"record_offset":18064,"parent_record_offset":18036,"ancestor_record_offset":17656,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":140,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":17656,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$18::clone
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29C8
// 首条位置: {"module":80,"inlinee":10696}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x992A
// 首条位置: {"module":683,"record_offset":11892,"rva":12418288,"va":5381127408}
// 编译实例 RVAs: 0xbd7cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30719657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127408,"raw_hex":"e8a9bed401","size":5,"soft_fail":false,"target_va":5411847152,"va":5381127490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":12044,"parent_record_offset":11892,"ancestor_record_offset":11892,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":11892,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12176,"parent_record_offset":12148,"ancestor_record_offset":11892,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":12116,"parent_record_offset":12076,"ancestor_record_offset":11892,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11892,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12312,"parent_record_offset":12284,"ancestor_record_offset":11892,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":12076,"parent_record_offset":12044,"ancestor_record_offset":11892,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":12228,"parent_record_offset":12200,"ancestor_record_offset":11892,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":12148,"parent_record_offset":12044,"ancestor_record_offset":11892,"inlinee":39173,"source_line":101,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$1::contains_layer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9905
// 首条位置: {"module":683,"inlinee":39173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12176,"parent_record_offset":12148,"ancestor_record_offset":12148,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":12148,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":12148,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":12148,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12312,"parent_record_offset":12284,"ancestor_record_offset":12148,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":12228,"parent_record_offset":12200,"ancestor_record_offset":12148,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x992B
// 首条位置: {"module":683,"record_offset":12376,"rva":12418400,"va":5381127520}
// 编译实例 RVAs: 0xbd7d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30719545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127520,"raw_hex":"e839bed401","size":5,"soft_fail":false,"target_va":5411847152,"va":5381127602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":12528,"parent_record_offset":12376,"ancestor_record_offset":12376,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":12376,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12660,"parent_record_offset":12632,"ancestor_record_offset":12376,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":12600,"parent_record_offset":12560,"ancestor_record_offset":12376,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12768,"parent_record_offset":12740,"ancestor_record_offset":12376,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":12376,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":12560,"parent_record_offset":12528,"ancestor_record_offset":12376,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":12712,"parent_record_offset":12684,"ancestor_record_offset":12376,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":12632,"parent_record_offset":12528,"ancestor_record_offset":12376,"inlinee":39175,"source_line":111,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$1::contains_vendor::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9907
// 首条位置: {"module":683,"inlinee":39175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12660,"parent_record_offset":12632,"ancestor_record_offset":12632,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":12632,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":12740,"parent_record_offset":12712,"ancestor_record_offset":12632,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12768,"parent_record_offset":12740,"ancestor_record_offset":12632,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":12632,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":12712,"parent_record_offset":12684,"ancestor_record_offset":12632,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31F0
// 首条位置: {"module":155,"inlinee":12784}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":75564,"parent_record_offset":75448,"ancestor_record_offset":75448,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":75500,"parent_record_offset":75448,"ancestor_record_offset":75448,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":75472,"parent_record_offset":75448,"ancestor_record_offset":75448,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":75536,"parent_record_offset":75448,"ancestor_record_offset":75448,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x320A
// 首条位置: {"module":142,"inlinee":12810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$20::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2DB7
// 首条位置: {"module":104,"inlinee":11703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":27364,"parent_record_offset":27336,"ancestor_record_offset":27336,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":104,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":27336,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":104,"record_offset":27416,"parent_record_offset":27388,"ancestor_record_offset":27336,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":104,"record_offset":27440,"parent_record_offset":27416,"ancestor_record_offset":27336,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":104,"record_offset":27468,"parent_record_offset":27440,"ancestor_record_offset":27336,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$22::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x208E
// 首条位置: {"module":34,"inlinee":8334}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$79::partial_cmp
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":34,"record_offset":4856,"parent_record_offset":4820,"ancestor_record_offset":4820,"inlinee":8336,"source_line":359,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$70::partial_cmp
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":34,"record_offset":5032,"parent_record_offset":4820,"ancestor_record_offset":4820,"inlinee":8348,"source_line":1820,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$22::partial_cmp
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":4820,"inlinee":8338,"source_line":3762,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":34,"record_offset":4984,"parent_record_offset":4952,"ancestor_record_offset":4820,"inlinee":8346,"source_line":1836,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$14::compare
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":34,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":4820,"inlinee":8344,"source_line":206,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$3::partial_cmp
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":34,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4820,"inlinee":8340,"source_line":37,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$7::partial_compare
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":34,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":4820,"inlinee":8342,"source_line":136,"archive":"_pdb/B/modules/00034.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$24::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2DE8
// 首条位置: {"module":104,"inlinee":11752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstance::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":12860,"rva":12418512,"va":5381127632}
// 编译实例 RVAs: 0xbd7dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1516419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e883231700","size":5,"soft_fail":false,"target_va":5382644528,"va":5381128104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6206872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8684aa1ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5381130739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6046271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8c1bda3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381127754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6047083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e895baa3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381128582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=7
//     target_aliases=69 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5611945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8575eaaff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381127972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e87b480000","size":5,"soft_fail":false,"target_va":5381148928,"va":5381130368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b0c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13470748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e81c8ccd00","size":5,"soft_fail":false,"target_va":5394599040,"va":5381128287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30718891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8abbbd401","size":5,"soft_fail":false,"target_va":5411847168,"va":5381128272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924010
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30720292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e824c1d401","size":5,"soft_fail":false,"target_va":5411848208,"va":5381127911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30410830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e84e08d001","size":5,"soft_fail":false,"target_va":5411540963,"va":5381130128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t753238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8567e0b00","size":5,"soft_fail":false,"target_va":5381881088,"va":5381127845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13470249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8298acd00","size":5,"soft_fail":false,"target_va":5394598384,"va":5381128130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30412686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e88e0fd001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381130189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::full_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13471646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e89e8fcd00","size":5,"soft_fail":false,"target_va":5394601424,"va":5381129773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::pci_identifiers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13472115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e87391cd00","size":5,"soft_fail":false,"target_va":5394601904,"va":5381129784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::info::initialize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13471480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8f88ecd00","size":5,"soft_fail":false,"target_va":5394600512,"va":5381129027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi_hi::gpu::Gpu::enumerate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi-hi","version":"0.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13471715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8e38fcd00","size":5,"soft_fail":false,"target_va":5394600768,"va":5381129048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::instance::InstanceBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t199866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381127632,"raw_hex":"e8ba0c0300","size":5,"soft_fail":false,"target_va":5381328656,"va":5381128785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":17436,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":17492,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":14480,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":14852,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13060,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":14152,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":13032,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":14124,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":14512,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":14396,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":14688,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":39179,"source_line":388,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::InstanceBuilder::debug_markers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":683,"record_offset":16176,"parent_record_offset":12860,"ancestor_record_offset":12860,"inlinee":39183,"source_line":40,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":12860,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12860,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":12860,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13520,"parent_record_offset":13312,"ancestor_record_offset":12860,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":15524,"parent_record_offset":15468,"ancestor_record_offset":12860,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":13252,"parent_record_offset":13224,"ancestor_record_offset":12860,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":15496,"parent_record_offset":15468,"ancestor_record_offset":12860,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":15468,"parent_record_offset":15444,"ancestor_record_offset":12860,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":13312,"parent_record_offset":13280,"ancestor_record_offset":12860,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":13280,"parent_record_offset":13252,"ancestor_record_offset":12860,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":15444,"parent_record_offset":15420,"ancestor_record_offset":12860,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":13348,"parent_record_offset":13312,"ancestor_record_offset":12860,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":14256,"parent_record_offset":14228,"ancestor_record_offset":12860,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":13652,"parent_record_offset":13624,"ancestor_record_offset":12860,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":13196,"parent_record_offset":13156,"ancestor_record_offset":12860,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":13224,"parent_record_offset":13196,"ancestor_record_offset":12860,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":13092,"parent_record_offset":13060,"ancestor_record_offset":12860,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":14792,"parent_record_offset":14688,"ancestor_record_offset":12860,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":12860,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":12860,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":15928,"parent_record_offset":15904,"ancestor_record_offset":12860,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":12860,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":683,"record_offset":14048,"parent_record_offset":13992,"ancestor_record_offset":12860,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":683,"record_offset":13624,"parent_record_offset":13156,"ancestor_record_offset":12860,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":683,"record_offset":13124,"parent_record_offset":13092,"ancestor_record_offset":12860,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":683,"record_offset":13156,"parent_record_offset":13124,"ancestor_record_offset":12860,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] ash::entry::Entry::enumerate_instance_layer_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":683,"record_offset":14724,"parent_record_offset":14688,"ancestor_record_offset":12860,"inlinee":39181,"source_line":269,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":13372,"parent_record_offset":13348,"ancestor_record_offset":12860,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":12860,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":683,"record_offset":13868,"parent_record_offset":13844,"ancestor_record_offset":12860,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":683,"record_offset":13896,"parent_record_offset":13868,"ancestor_record_offset":12860,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":12860,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":13476,"parent_record_offset":13420,"ancestor_record_offset":12860,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":12860,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":13820,"parent_record_offset":13736,"ancestor_record_offset":12860,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13680,"parent_record_offset":13652,"ancestor_record_offset":12860,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13968,"parent_record_offset":13820,"ancestor_record_offset":12860,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":12860,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":12860,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":12860,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":15024,"parent_record_offset":14852,"ancestor_record_offset":12860,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":14568,"parent_record_offset":14536,"ancestor_record_offset":12860,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":14596,"parent_record_offset":14536,"ancestor_record_offset":12860,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":12860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":12860,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":14936,"parent_record_offset":14876,"ancestor_record_offset":12860,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":14964,"parent_record_offset":14876,"ancestor_record_offset":12860,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":13788,"parent_record_offset":13736,"ancestor_record_offset":12860,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":12860,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":14020,"parent_record_offset":13992,"ancestor_record_offset":12860,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":14752,"parent_record_offset":14688,"ancestor_record_offset":12860,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":12860,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14292,"parent_record_offset":14256,"ancestor_record_offset":12860,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":12860,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":12860,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14316,"parent_record_offset":14292,"ancestor_record_offset":12860,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14348,"parent_record_offset":14256,"ancestor_record_offset":12860,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":14624,"parent_record_offset":14536,"ancestor_record_offset":12860,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":12860,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":14648,"parent_record_offset":14624,"ancestor_record_offset":12860,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$3::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":12860,"inlinee":39177,"source_line":151,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$3::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9909
// 首条位置: {"module":683,"inlinee":39177}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":683,"record_offset":13868,"parent_record_offset":13844,"ancestor_record_offset":13844,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":683,"record_offset":13896,"parent_record_offset":13868,"ancestor_record_offset":13844,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":13844,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9911
// 首条位置: {"module":683,"inlinee":39185}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":21624,"parent_record_offset":21240,"ancestor_record_offset":21240,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":21304,"parent_record_offset":21240,"ancestor_record_offset":21240,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":683,"record_offset":21276,"parent_record_offset":21240,"ancestor_record_offset":21240,"inlinee":17230,"source_line":217,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21240,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21876,"parent_record_offset":21820,"ancestor_record_offset":21240,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":21240,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":21240,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":21240,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":21340,"parent_record_offset":21304,"ancestor_record_offset":21240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":21660,"parent_record_offset":21624,"ancestor_record_offset":21240,"inlinee":39187,"source_line":242,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":21692,"parent_record_offset":21660,"ancestor_record_offset":21240,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9913
// 首条位置: {"module":683,"inlinee":39187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":21724,"parent_record_offset":21660,"ancestor_record_offset":21660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":683,"record_offset":21692,"parent_record_offset":21660,"ancestor_record_offset":21660,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":21876,"parent_record_offset":21820,"ancestor_record_offset":21660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21848,"parent_record_offset":21820,"ancestor_record_offset":21660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":21660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":21796,"parent_record_offset":21772,"ancestor_record_offset":21660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":21660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x990B
// 首条位置: {"module":683,"inlinee":39179}
// 编译实例 RVAs: 0xbda7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13460014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138352,"raw_hex":"e82e62cd00","size":5,"soft_fail":false,"target_va":5394598384,"va":5381138365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30404400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138352,"raw_hex":"e830efcf01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381138475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":14792,"parent_record_offset":14688,"ancestor_record_offset":14688,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::enumerate_instance_layer_properties
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":683,"record_offset":14724,"parent_record_offset":14688,"ancestor_record_offset":14688,"inlinee":39181,"source_line":269,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":33016,"parent_record_offset":32744,"ancestor_record_offset":32744,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":14752,"parent_record_offset":14688,"ancestor_record_offset":14688,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":33040,"parent_record_offset":33016,"ancestor_record_offset":32744,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":14688,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::implicit_layers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 395
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x36DA
// 首条位置: {"module":175,"inlinee":14042}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":175,"record_offset":1216,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":10890,"source_line":320,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":2920,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":175,"record_offset":1900,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1160,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1160,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":1160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":1772,"parent_record_offset":1596,"ancestor_record_offset":1160,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":3072,"parent_record_offset":3016,"ancestor_record_offset":1160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":1160,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1160,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1160,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":1596,"parent_record_offset":1564,"ancestor_record_offset":1160,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":1564,"parent_record_offset":1536,"ancestor_record_offset":1160,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1160,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":1632,"parent_record_offset":1596,"ancestor_record_offset":1160,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":175,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":1160,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":175,"record_offset":1384,"parent_record_offset":1352,"ancestor_record_offset":1160,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":175,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":1160,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":175,"record_offset":1352,"parent_record_offset":1320,"ancestor_record_offset":1160,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":175,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1160,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":175,"record_offset":1256,"parent_record_offset":1216,"ancestor_record_offset":1160,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":175,"record_offset":1508,"parent_record_offset":1416,"ancestor_record_offset":1160,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":175,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1160,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":175,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1160,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":175,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":1160,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":175,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1160,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":175,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1160,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":175,"record_offset":1452,"parent_record_offset":1416,"ancestor_record_offset":1160,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":1160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$26::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":22696,"rva":3412800,"va":5372121920}
// 编译实例 RVAs: 0x341340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10770473","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372121920,"raw_hex":"e92958a400","size":5,"soft_fail":false,"target_va":5382892432,"va":5372121954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDeviceInitState::is_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":33076,"rva":11139696,"va":5379848816}
// 编译实例 RVAs: 0xa9fa70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$28::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":2136,"rva":3402944,"va":5372112064}
// 编译实例 RVAs: 0x33ecc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10780329","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372112064,"raw_hex":"e9a97ea400","size":5,"soft_fail":false,"target_va":5382892432,"va":5372112098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$30::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C853
// 首条位置: {"module":3503,"inlinee":116819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$32::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 451
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D3A
// 首条位置: {"module":240,"inlinee":15674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":240,"record_offset":50328,"parent_record_offset":49864,"ancestor_record_offset":49864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":240,"record_offset":49924,"parent_record_offset":49864,"ancestor_record_offset":49864,"inlinee":15676,"source_line":42,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":50264,"parent_record_offset":50236,"ancestor_record_offset":49864,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":50236,"parent_record_offset":50208,"ancestor_record_offset":49864,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":50460,"parent_record_offset":50428,"ancestor_record_offset":49864,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":50208,"parent_record_offset":50180,"ancestor_record_offset":49864,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":50180,"parent_record_offset":50156,"ancestor_record_offset":49864,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":240,"record_offset":49976,"parent_record_offset":49924,"ancestor_record_offset":49864,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":240,"record_offset":50084,"parent_record_offset":49924,"ancestor_record_offset":49864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$19::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":240,"record_offset":50056,"parent_record_offset":49924,"ancestor_record_offset":49864,"inlinee":15678,"source_line":73,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$33::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 451
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31CA
// 首条位置: {"module":140,"inlinee":12746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_supported
// 记录形式: compiled_procedure; instances=1
// 源码行: 510
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":33216,"rva":12429376,"va":5381138496}
// 编译实例 RVAs: 0xbda840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138496,"raw_hex":"e8d2c9ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381138553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138496,"raw_hex":"e839010000","size":5,"soft_fail":false,"target_va":5381138832,"va":5381138514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138496,"raw_hex":"e838060000","size":5,"soft_fail":false,"target_va":5381140256,"va":5381138659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33360,"parent_record_offset":33216,"ancestor_record_offset":33216,"inlinee":39194,"source_line":514,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":33384,"parent_record_offset":33360,"ancestor_record_offset":33216,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33440,"parent_record_offset":33360,"ancestor_record_offset":33216,"inlinee":39196,"source_line":674,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33412,"parent_record_offset":33360,"ancestor_record_offset":33216,"inlinee":39195,"source_line":651,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 514
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x991A
// 首条位置: {"module":683,"inlinee":39194}
// 编译实例 RVAs: 0xbda8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138672,"raw_hex":"e822c9ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381138729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138672,"raw_hex":"e889000000","size":5,"soft_fail":false,"target_va":5381138832,"va":5381138690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":683,"record_offset":33384,"parent_record_offset":33360,"ancestor_record_offset":33360,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33440,"parent_record_offset":33360,"ancestor_record_offset":33360,"inlinee":39196,"source_line":674,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33412,"parent_record_offset":33360,"ancestor_record_offset":33360,"inlinee":39195,"source_line":651,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 记录形式: compiled_procedure; instances=1
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":33732,"rva":12429712,"va":5381138832}
// 编译实例 RVAs: 0xbda990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138832,"raw_hex":"e891c8ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381138874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381138832,"raw_hex":"e80b030000","size":5,"soft_fail":false,"target_va":5381140256,"va":5381139472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$17::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":683,"record_offset":33904,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":39198,"source_line":387,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":33880,"parent_record_offset":33732,"ancestor_record_offset":33732,"inlinee":39195,"source_line":651,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":33952,"parent_record_offset":33928,"ancestor_record_offset":33732,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":33928,"parent_record_offset":33904,"ancestor_record_offset":33732,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":34112,"parent_record_offset":34084,"ancestor_record_offset":33732,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::cmp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":34204,"parent_record_offset":34024,"ancestor_record_offset":33732,"inlinee":10268,"source_line":1836,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34140,"parent_record_offset":34084,"ancestor_record_offset":33732,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34084,"parent_record_offset":34056,"ancestor_record_offset":33732,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":34056,"parent_record_offset":34024,"ancestor_record_offset":33732,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":33732,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$3::partial_cmp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":33976,"parent_record_offset":33952,"ancestor_record_offset":33732,"inlinee":8340,"source_line":37,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$7::partial_compare
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":34000,"parent_record_offset":33976,"ancestor_record_offset":33732,"inlinee":8342,"source_line":136,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$8::compare
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":34024,"parent_record_offset":34000,"ancestor_record_offset":33732,"inlinee":18974,"source_line":167,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=102
// 源码行: 651
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x991B
// 首条位置: {"module":683,"inlinee":39195}
// 编译实例 RVAs: 0xbdaec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140160,"raw_hex":"e86ac3ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381140193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-15553","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381140160,"raw_hex":"e93fc3ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381140236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
// 记录形式: compiled_procedure; instances=1
// 源码行: 655
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":36920,"rva":12431136,"va":5381140256}
// 编译实例 RVAs: 0xbdaf20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7250
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140256,"raw_hex":"e800c3ffff","size":5,"soft_fail":false,"target_va":5381124688,"va":5381140299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 674
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x991C
// 首条位置: {"module":683,"inlinee":39196}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 693
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":18192,"rva":3410896,"va":5372120016}
// 编译实例 RVAs: 0x340bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10524280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e87896a000","size":5,"soft_fail":false,"target_va":5382644528,"va":5372120243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400315e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3208942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e81209cfff","size":5,"soft_fail":false,"target_va":5368911328,"va":5372120265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048bd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1354990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8eeac1400","size":5,"soft_fail":false,"target_va":5373476128,"va":5372121133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ea40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1366596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e844da1400","size":5,"soft_fail":false,"target_va":5373487680,"va":5372121079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d5020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2703489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e881402900","size":5,"soft_fail":false,"target_va":5374824480,"va":5372120986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=2
//     target_aliases=18 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2850077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e81d7d2b00","size":5,"soft_fail":false,"target_va":5374971760,"va":5372121678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2961150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8fe2e2d00","size":5,"soft_fail":false,"target_va":5375081488,"va":5372120333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2960151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8172b2d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372121348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3395936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e860d13300","size":5,"soft_fail":false,"target_va":5375516032,"va":5372120091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d090
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8503508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8d4c08100","size":5,"soft_fail":false,"target_va":5380624528,"va":5372121015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9027846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e806c18900","size":5,"soft_fail":false,"target_va":5381148560,"va":5372120709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39419481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8597e5902","size":5,"soft_fail":false,"target_va":5411540992,"va":5372121506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39419434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e82a7e5902","size":5,"soft_fail":false,"target_va":5411540963,"va":5372121524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2232879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e82f122200","size":5,"soft_fail":false,"target_va":5374353648,"va":5372120764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::enumerate_physical_devices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9179120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372120016,"raw_hex":"e8f00f8c00","size":5,"soft_fail":false,"target_va":5381299712,"va":5372120587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":21276,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":140,"record_offset":20616,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":18904,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":18812,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":19760,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":19532,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":140,"record_offset":18604,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":140,"record_offset":19620,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":21088,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":140,"record_offset":18840,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":140,"record_offset":18876,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":140,"record_offset":19448,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":140,"record_offset":18636,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":140,"record_offset":18520,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":18192,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":18192,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":21684,"parent_record_offset":21660,"ancestor_record_offset":18192,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":18960,"parent_record_offset":18932,"ancestor_record_offset":18192,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":18992,"parent_record_offset":18960,"ancestor_record_offset":18192,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":140,"record_offset":21660,"parent_record_offset":21604,"ancestor_record_offset":18192,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":140,"record_offset":18932,"parent_record_offset":18904,"ancestor_record_offset":18192,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":20148,"parent_record_offset":20112,"ancestor_record_offset":18192,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":21632,"parent_record_offset":21604,"ancestor_record_offset":18192,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":18192,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":20208,"parent_record_offset":20176,"ancestor_record_offset":18192,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":20176,"parent_record_offset":20148,"ancestor_record_offset":18192,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":21580,"parent_record_offset":21556,"ancestor_record_offset":18192,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":20244,"parent_record_offset":20208,"ancestor_record_offset":18192,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":20756,"parent_record_offset":20728,"ancestor_record_offset":18192,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":20728,"parent_record_offset":20700,"ancestor_record_offset":18192,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":20672,"parent_record_offset":20644,"ancestor_record_offset":18192,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":20700,"parent_record_offset":20672,"ancestor_record_offset":18192,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":140,"record_offset":20644,"parent_record_offset":20616,"ancestor_record_offset":18192,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":140,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":18192,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":21148,"parent_record_offset":21112,"ancestor_record_offset":18192,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":18192,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":20512,"parent_record_offset":20488,"ancestor_record_offset":18192,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":20076,"parent_record_offset":20024,"ancestor_record_offset":18192,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":20112,"parent_record_offset":20076,"ancestor_record_offset":18192,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":19644,"parent_record_offset":19620,"ancestor_record_offset":18192,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":18192,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":140,"record_offset":19720,"parent_record_offset":19696,"ancestor_record_offset":18192,"inlinee":11705,"source_line":251,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":140,"record_offset":19696,"parent_record_offset":19644,"ancestor_record_offset":18192,"inlinee":11928,"source_line":233,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":140,"record_offset":20488,"parent_record_offset":20024,"ancestor_record_offset":18192,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":140,"record_offset":19984,"parent_record_offset":19944,"ancestor_record_offset":18192,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":140,"record_offset":20024,"parent_record_offset":19984,"ancestor_record_offset":18192,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":140,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":18192,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":140,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":18192,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":140,"record_offset":21032,"parent_record_offset":21008,"ancestor_record_offset":18192,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":140,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":18192,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":140,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":18192,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":140,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":18192,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":140,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":18192,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":140,"record_offset":20316,"parent_record_offset":20292,"ancestor_record_offset":18192,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":140,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":18192,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":140,"record_offset":22172,"parent_record_offset":22144,"ancestor_record_offset":18192,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":140,"record_offset":18692,"parent_record_offset":18660,"ancestor_record_offset":18192,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":140,"record_offset":18720,"parent_record_offset":18660,"ancestor_record_offset":18192,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":140,"record_offset":21008,"parent_record_offset":20756,"ancestor_record_offset":18192,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":18192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":140,"record_offset":19588,"parent_record_offset":19532,"ancestor_record_offset":18192,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":140,"record_offset":21180,"parent_record_offset":21148,"ancestor_record_offset":18192,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":19068,"parent_record_offset":19040,"ancestor_record_offset":18192,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":18544,"parent_record_offset":18520,"ancestor_record_offset":18192,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":18568,"parent_record_offset":18544,"ancestor_record_offset":18192,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":21208,"parent_record_offset":21148,"ancestor_record_offset":18192,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":21240,"parent_record_offset":21112,"ancestor_record_offset":18192,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":140,"record_offset":18748,"parent_record_offset":18660,"ancestor_record_offset":18192,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":140,"record_offset":18660,"parent_record_offset":18636,"ancestor_record_offset":18192,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":140,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":18192,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::new::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 733
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x379B
// 首条位置: {"module":184,"inlinee":14235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":184,"record_offset":33336,"parent_record_offset":33276,"ancestor_record_offset":33276,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":33308,"parent_record_offset":33276,"ancestor_record_offset":33276,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":33420,"parent_record_offset":33276,"ancestor_record_offset":33276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":33596,"parent_record_offset":33572,"ancestor_record_offset":33276,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":33572,"parent_record_offset":33516,"ancestor_record_offset":33276,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":184,"record_offset":33384,"parent_record_offset":33360,"ancestor_record_offset":33276,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":33544,"parent_record_offset":33516,"ancestor_record_offset":33276,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":33516,"parent_record_offset":33492,"ancestor_record_offset":33276,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":33492,"parent_record_offset":33468,"ancestor_record_offset":33276,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":33360,"parent_record_offset":33336,"ancestor_record_offset":33276,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":33444,"parent_record_offset":33420,"ancestor_record_offset":33276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 744
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":37056,"rva":12431504,"va":5381140624}
// 编译实例 RVAs: 0xbdb090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1503659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8abf11600","size":5,"soft_fail":false,"target_va":5382644528,"va":5381140864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a860
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11079489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8bff056ff","size":5,"soft_fail":false,"target_va":5370062944,"va":5381142428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6060272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e81087a3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381141771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5624712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8782caaff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381140739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087b030
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3540581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e89bf9c9ff","size":5,"soft_fail":false,"target_va":5377601584,"va":5381142160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be2060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e858680000","size":5,"soft_fail":false,"target_va":5381169248,"va":5381142531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30705410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e80287d401","size":5,"soft_fail":false,"target_va":5411847152,"va":5381141737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30705923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e80389d401","size":5,"soft_fail":false,"target_va":5411847184,"va":5381141256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8bf1d0000","size":5,"soft_fail":false,"target_va":5381149488,"va":5381141868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1514120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8881a1700","size":5,"soft_fail":false,"target_va":5382655024,"va":5381140899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e8f8b5ffff","size":5,"soft_fail":false,"target_va":5381123504,"va":5381142451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e814baffff","size":5,"soft_fail":false,"target_va":5381124336,"va":5381142231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] dxgi_enumerate_gpus::enumerate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"dxgi-enumerate-gpus","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13465647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381140624,"raw_hex":"e82f78cd00","size":5,"soft_fail":false,"target_va":5394606528,"va":5381140876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":37236,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":37952,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":37896,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":38060,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":37348,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":37548,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":38500,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":37380,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":37264,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":37056,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":38940,"parent_record_offset":38884,"ancestor_record_offset":37056,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":38912,"parent_record_offset":38884,"ancestor_record_offset":37056,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":37056,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":38860,"parent_record_offset":38836,"ancestor_record_offset":37056,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":38648,"parent_record_offset":38592,"ancestor_record_offset":37056,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":39300,"parent_record_offset":39276,"ancestor_record_offset":37056,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":37056,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":683,"record_offset":38676,"parent_record_offset":38648,"ancestor_record_offset":37056,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":39792,"parent_record_offset":39764,"ancestor_record_offset":37056,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":38620,"parent_record_offset":38592,"ancestor_record_offset":37056,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":37432,"parent_record_offset":37404,"ancestor_record_offset":37056,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":37460,"parent_record_offset":37404,"ancestor_record_offset":37056,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":37572,"parent_record_offset":37548,"ancestor_record_offset":37056,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":37920,"parent_record_offset":37896,"ancestor_record_offset":37056,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":38120,"parent_record_offset":38060,"ancestor_record_offset":37056,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":38148,"parent_record_offset":38060,"ancestor_record_offset":37056,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":38092,"parent_record_offset":38060,"ancestor_record_offset":37056,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":38172,"parent_record_offset":38148,"ancestor_record_offset":37056,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":38028,"parent_record_offset":37952,"ancestor_record_offset":37056,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":38704,"parent_record_offset":38676,"ancestor_record_offset":37056,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":38732,"parent_record_offset":38704,"ancestor_record_offset":37056,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":38528,"parent_record_offset":38500,"ancestor_record_offset":37056,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":37288,"parent_record_offset":37264,"ancestor_record_offset":37056,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":37312,"parent_record_offset":37288,"ancestor_record_offset":37056,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":37488,"parent_record_offset":37404,"ancestor_record_offset":37056,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":37056,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":37512,"parent_record_offset":37488,"ancestor_record_offset":37056,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$7::missing_devices::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":38592,"parent_record_offset":38500,"ancestor_record_offset":37056,"inlinee":39200,"source_line":769,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::missing_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 769
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9920
// 首条位置: {"module":683,"inlinee":39200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":38648,"parent_record_offset":38592,"ancestor_record_offset":38592,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":38620,"parent_record_offset":38592,"ancestor_record_offset":38592,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":683,"record_offset":38676,"parent_record_offset":38648,"ancestor_record_offset":38592,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":38704,"parent_record_offset":38676,"ancestor_record_offset":38592,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":38732,"parent_record_offset":38704,"ancestor_record_offset":38592,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::available_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 791
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":40196,"rva":12433440,"va":5381142560}
// 编译实例 RVAs: 0xbdb820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406084e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6108238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142560,"raw_hex":"e8b2cba2ff","size":5,"soft_fail":false,"target_va":5375034592,"va":5381142825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6061184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142560,"raw_hex":"e88083a3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381142667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b3300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13466155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142560,"raw_hex":"e82b7acd00","size":5,"soft_fail":false,"target_va":5394608896,"va":5381142736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30398173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142560,"raw_hex":"e8ddd6cf01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381142785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":40348,"parent_record_offset":40196,"ancestor_record_offset":40196,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":40784,"parent_record_offset":40760,"ancestor_record_offset":40196,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":40808,"parent_record_offset":40784,"ancestor_record_offset":40196,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":40760,"parent_record_offset":40584,"ancestor_record_offset":40196,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":40528,"parent_record_offset":40500,"ancestor_record_offset":40196,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":40584,"parent_record_offset":40556,"ancestor_record_offset":40196,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":40556,"parent_record_offset":40528,"ancestor_record_offset":40196,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":40620,"parent_record_offset":40584,"ancestor_record_offset":40196,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":40888,"parent_record_offset":40864,"ancestor_record_offset":40196,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":40472,"parent_record_offset":40432,"ancestor_record_offset":40196,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":40500,"parent_record_offset":40472,"ancestor_record_offset":40196,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":40376,"parent_record_offset":40348,"ancestor_record_offset":40196,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":683,"record_offset":40864,"parent_record_offset":40432,"ancestor_record_offset":40196,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":683,"record_offset":40404,"parent_record_offset":40376,"ancestor_record_offset":40196,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":683,"record_offset":40432,"parent_record_offset":40404,"ancestor_record_offset":40196,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":40644,"parent_record_offset":40620,"ancestor_record_offset":40196,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":40196,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":40196,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":40936,"parent_record_offset":40912,"ancestor_record_offset":40196,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":40692,"parent_record_offset":40668,"ancestor_record_offset":40196,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":40716,"parent_record_offset":40692,"ancestor_record_offset":40196,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::available_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 794
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152D1
// 首条位置: {"module":1935,"inlinee":86737}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1935,"record_offset":4416,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":5684,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":4836,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":39898,"source_line":159,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":5364,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":39900,"source_line":191,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":4444,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":15676,"source_line":42,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4344,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":4344,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":4344,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":4344,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":4344,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1935,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":4344,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":4344,"inlinee":39864,"source_line":1555,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1935,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4344,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1935,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4344,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1935,"record_offset":5196,"parent_record_offset":5104,"ancestor_record_offset":4344,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1935,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":4344,"inlinee":39866,"source_line":49,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1935,"record_offset":5300,"parent_record_offset":5196,"ancestor_record_offset":4344,"inlinee":39872,"source_line":53,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":5032,"parent_record_offset":4836,"ancestor_record_offset":4344,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1935,"record_offset":4496,"parent_record_offset":4444,"ancestor_record_offset":4344,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":4604,"parent_record_offset":4444,"ancestor_record_offset":4344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1935,"record_offset":5168,"parent_record_offset":5104,"ancestor_record_offset":4344,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1935,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":4344,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1935,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":4344,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1935,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":4344,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1935,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4344,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$19::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":4576,"parent_record_offset":4444,"ancestor_record_offset":4344,"inlinee":15678,"source_line":73,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":5228,"parent_record_offset":5196,"ancestor_record_offset":4344,"inlinee":39868,"source_line":163,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":5548,"parent_record_offset":5516,"ancestor_record_offset":4344,"inlinee":39876,"source_line":195,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::create_device
// 记录形式: compiled_procedure; instances=1
// 源码行: 813
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":40996,"rva":12433728,"va":5381142848}
// 编译实例 RVAs: 0xbdb940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1501322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e88ae81600","size":5,"soft_fail":false,"target_va":5382644528,"va":5381143201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6061726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e86281a3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381143225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5627043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e85d23aaff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381143070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30398265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e839d7cf01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381143506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t92588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e8ac690100","size":5,"soft_fail":false,"target_va":5381236064,"va":5381143471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t150045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381142848,"raw_hex":"e81d4a0200","size":5,"soft_fail":false,"target_va":5381293056,"va":5381143006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":42228,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":41504,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":41472,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":683,"record_offset":42324,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":41708,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":41172,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":41532,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":41388,"parent_record_offset":40996,"ancestor_record_offset":40996,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":41884,"parent_record_offset":41860,"ancestor_record_offset":40996,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":41860,"parent_record_offset":41804,"ancestor_record_offset":40996,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":41832,"parent_record_offset":41804,"ancestor_record_offset":40996,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":41804,"parent_record_offset":41780,"ancestor_record_offset":40996,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":683,"record_offset":41780,"parent_record_offset":41756,"ancestor_record_offset":40996,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":683,"record_offset":41312,"parent_record_offset":41284,"ancestor_record_offset":40996,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":683,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":40996,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":41588,"parent_record_offset":41556,"ancestor_record_offset":40996,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":683,"record_offset":41616,"parent_record_offset":41556,"ancestor_record_offset":40996,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":41732,"parent_record_offset":41708,"ancestor_record_offset":40996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":41228,"parent_record_offset":41200,"ancestor_record_offset":40996,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":41200,"parent_record_offset":41172,"ancestor_record_offset":40996,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":42264,"parent_record_offset":42228,"ancestor_record_offset":40996,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":41412,"parent_record_offset":41388,"ancestor_record_offset":40996,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":42288,"parent_record_offset":42264,"ancestor_record_offset":40996,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":41436,"parent_record_offset":41412,"ancestor_record_offset":40996,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":41644,"parent_record_offset":41556,"ancestor_record_offset":40996,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":40996,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":41668,"parent_record_offset":41644,"ancestor_record_offset":40996,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$7::create_device::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":683,"record_offset":41260,"parent_record_offset":41172,"ancestor_record_offset":40996,"inlinee":39202,"source_line":821,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":683,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":40996,"inlinee":10735,"source_line":437,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::create_device::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 821
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9922
// 首条位置: {"module":683,"inlinee":39202}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] uuid::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":683,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":41260,"inlinee":10735,"source_line":437,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":683,"record_offset":41312,"parent_record_offset":41284,"ancestor_record_offset":41260,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":683,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":41260,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::order_and_filter_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 834
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":42584,"rva":12434480,"va":5381143600}
// 编译实例 RVAs: 0xbdbc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b5460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13473800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381143600,"raw_hex":"e80898cd00","size":5,"soft_fail":false,"target_va":5394617440,"va":5381143635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13489317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381143600,"raw_hex":"e8a5d4cd00","size":5,"soft_fail":false,"target_va":5394633008,"va":5381143686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13497761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381143600,"raw_hex":"e8a1f5cd00","size":5,"soft_fail":false,"target_va":5394641488,"va":5381143722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":42740,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":23377,"source_line":352,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":42768,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":42832,"parent_record_offset":42796,"ancestor_record_offset":42584,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":683,"record_offset":42888,"parent_record_offset":42796,"ancestor_record_offset":42584,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":683,"record_offset":42796,"parent_record_offset":42768,"ancestor_record_offset":42584,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 838
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F8
// 首条位置: {"module":1937,"inlinee":86776}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=92
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1937,"record_offset":692,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":39195,"source_line":651,"archive":"_pdb/B/modules/01937.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 857
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F0
// 首条位置: {"module":1936,"inlinee":86768}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1936,"record_offset":9216,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1936,"record_offset":8492,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":39196,"source_line":674,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$32::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1936,"record_offset":8528,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":15674,"source_line":451,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1936,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":8436,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1936,"record_offset":8900,"parent_record_offset":8872,"ancestor_record_offset":8436,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1936,"record_offset":9108,"parent_record_offset":9080,"ancestor_record_offset":8436,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1936,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":8436,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1936,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":8436,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1936,"record_offset":8644,"parent_record_offset":8592,"ancestor_record_offset":8436,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1936,"record_offset":8752,"parent_record_offset":8592,"ancestor_record_offset":8436,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1936,"record_offset":9260,"parent_record_offset":9216,"ancestor_record_offset":8436,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1936,"record_offset":9288,"parent_record_offset":9216,"ancestor_record_offset":8436,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1936,"record_offset":8592,"parent_record_offset":8528,"ancestor_record_offset":8436,"inlinee":15676,"source_line":42,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$19::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1936,"record_offset":8724,"parent_record_offset":8592,"ancestor_record_offset":8436,"inlinee":15678,"source_line":73,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: init.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::AvailableGraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$75::fmt<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "core::fmt::impl$75::fmt", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "tiny_glade::gui::settings::ui_settings_video_display::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["alloc::string::String", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::string::String", "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::string::String", "core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "tiny_glade::gui::settings::ui_settings_video_display::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "slice2$", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<rhapsody::raw::init::AvailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "alloc::slice::impl$9::to_owned", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>", "constructor": "alloc::vec::Vec::retain", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by_key::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "i64", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>", "alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >", "tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::choose_pivot<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::choose_pivot", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::AvailableGraphicsDeviceInitState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsDevicesForSurface
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >", "constructor": "rhapsody::raw::init::GraphicsDevicesForSurface::new", "arguments": ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsDevicesForSurface>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsDevicesForSurface"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInitError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 42
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::GraphicsInitError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::GraphicsInitError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::init::GraphicsInstance", "enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::sys::backtrace::__rust_begin_short_backtrace", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::Builder::spawn", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::impl$0::spawn_unchecked_::closure_env$1", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::sync_channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::sync_channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Slot", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<usize>", "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::SyncSender", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "std::thread::impl$0::spawn_unchecked_::MaybeDangling", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstance
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::init::GraphicsInstance", "enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::sys::backtrace::__rust_begin_short_backtrace", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::Builder::spawn", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::impl$0::spawn_unchecked_::closure_env$1", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::sync_channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::sync_channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Slot", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<usize>", "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::SyncSender", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "std::thread::impl$0::spawn_unchecked_::MaybeDangling", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::init::GraphicsInstanceLayer,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ash::vk::definitions::LayerProperties", "rhapsody::raw::init::GraphicsInstanceLayer", "tuple$<>", "rhapsody::raw::init::impl$4::implicit_layers::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::LayerProperties,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > > >", "constructor": "alloc::vec::into_iter::impl$5::fold", "arguments": ["ash::vk::definitions::LayerProperties", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsInstanceLayer>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure$1::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>", "tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure$1::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure$1::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure$1::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayerAllowlist
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,rhapsody::raw::init::GraphicsInstanceLayerAllowlist>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "rhapsody::raw::init::GraphicsInstanceLayerAllowlist"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds>", "constructor": "tuple$", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlist", "tg_gpu_selector::Workarounds"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> >", "constructor": "core::option::Option", "arguments": ["tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsInstanceLayerAllowlist>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlist"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "ref$", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$5::dedup::closure_env$0", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "alloc::vec::Vec::dedup_by", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global", "alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>", "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::MapAccess::next_value<ron::de::CommaSeparated,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "serde::de::MapAccess::next_value", "arguments": ["ron::de::CommaSeparated", "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)", "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::choose_pivot<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::shared::pivot::choose_pivot", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global", "tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLoader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::UnavailableGraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::UnavailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::impl$4::new::LocationToTry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::impl$4::new::LocationToTry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: init.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: init.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 22; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)
// 候选参数类型: ["ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>", "ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<winit::monitor::VideoMode>,ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>", "ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::render::render_config::LastKnownGraphicsDevice>,ref$<country_core::render::render_config::LastKnownGraphicsDevice>)
// 候选参数类型: ["ref$<country_core::render::render_config::LastKnownGraphicsDevice>", "ref$<country_core::render::render_config::LastKnownGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tiny_glade::gui::settings::RefreshRateMillihertz>,ref$<tiny_glade::gui::settings::RefreshRateMillihertz>)
// 候选参数类型: ["ref$<tiny_glade::gui::settings::RefreshRateMillihertz>", "ref$<tiny_glade::gui::settings::RefreshRateMillihertz>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 信息 ID: f5a0976496154eae61b878c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5964c8 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598dbd -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_supported @ 0xbda852 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible @ 0xbda902 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::impl$2::view @ 0x21bc6d7 -> 0xbda990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::device_driver_advice @ 0x21be62d -> 0xbda990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
// 信息 ID: 215b6df610b7e6977240a404
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=683, inline_record=33440; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible; module=683, inline_record=33692; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1936, inline_record=8492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
// 信息 ID: ada5294638c26a1034545d58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598dab -> 0xbda8f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=683, inline_record=33360; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
// 信息 ID: ba05e4ac398f017b971aacad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 95 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0x1a53b20 -> 0xbdaec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0x1a541f8 -> 0xbdaec0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=683, inline_record=33412; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible; module=683, inline_record=33664; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::driver_support; module=683, inline_record=33880; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=692; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=720; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=796; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=824; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=1028; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
// 信息 ID: 9106f7003939326e3b1fbfac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_supported @ 0xbda8e3 -> 0xbdaf20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_supported
// 信息 ID: 6deffb58de34addaed2c5b1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x595c04 -> 0xbda840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDeviceInitState::is_error
// 信息 ID: 3e0c09e7dacf100eddc4e151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x2482de -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x790c41 -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x7b5cc6 -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e205a3 -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x2267bce -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x226a184 -> 0xa9fa70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::available_devices
// 信息 ID: 63cb09dbabe08c1ead9363c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5953b7 -> 0xbdb820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::create_device
// 信息 ID: 949a7eb24f72f2241e03342d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5966b0 -> 0xbdb940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices
// 信息 ID: ea670bd2ca6c72e174091457
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5953e6 -> 0xbdb090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 信息 ID: 93d25979f953fc5aa293b4a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::init::GraphicsDevicesForSurface::new -> ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x594fc1 -> 0x340bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::order_and_filter_devices
// 信息 ID: d9cc50998e7b27bc3e79f088
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x595410 -> 0xbdbc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstance::new
// 信息 ID: dddc4a0e5fc72edd9e79f212
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > @ 0xd99bf -> 0xbd7dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayer::vendor
// 信息 ID: 89e4c2c1ce63710d3b504ce4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::vulkan_config::Controls::new @ 0xdf36fc -> 0xbd7bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
// 信息 ID: a5b4c3b60323e5e2791e26d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc12d99c00e7307ec4b5db0b: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] e98aff1265803b4c26f26dec: iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure$1::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1a569d6 -> 0xbd7cf0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=20880; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=21772; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=22952; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=23300; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
// 信息 ID: f651c8fff2cb1ba5d40eb5bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 82810ea9d2375ca9b2c82431: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] e98aff1265803b4c26f26dec: iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<enum2$<core::option::Option<alloc::string::String> >,tg_gpu_selector::vulkan_config::VendorLayers> >,tg_gpu_selector::vulkan_config::impl$3::allowlist_view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1a58066 -> 0xbd7d60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=22120; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=23644; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
// 信息 ID: 388b45a6475965ab34c35eac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x3e79e6 -> 0xbda7b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=683, inline_record=14688; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLoader::new
// 信息 ID: 312918e54241962823ba46dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > @ 0xd9976 -> 0xbd8af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x3e7619 -> 0xbd8af0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize
// 信息 ID: 5313d77dd3159708b9be3f9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >; module=155, inline_record=53320; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::render::render_config::_::impl$0::deserialize::__Visitor>; module=155, inline_record=118416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: bc9e04a8ff83614230344aab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<rhapsody::raw::init::_::impl$0::deserialize::__Field> >; module=155, inline_record=11012; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=98652; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<rhapsody::raw::init::_::impl$0::deserialize::__FieldVisitor>; module=188, inline_record=32452; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<rhapsody::raw::init::_::impl$0::deserialize::__FieldVisitor>; module=188, inline_record=32876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8804a6ed0d72942368555e2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<rhapsody::raw::init::_::impl$0::deserialize::__Field> >; module=155, inline_record=10868; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=98232; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=98388; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=98512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: d2f99dfc0a3a207df8f3c077
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=75448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 0c6efe5e136839982d6e52bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=155, inline_record=97852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::serialize
// 信息 ID: 30a559580e45914220aff397
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=142, inline_record=27180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: aa8220f7f295e89a1bdef6eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::init::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,rhapsody::raw::init::GraphicsInstanceLayerAllowlist> @ 0x34e972 -> 0x340a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$10::fmt
// 信息 ID: 7cc21c7731affb8078749f79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >; module=140, inline_record=396; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >; module=936, inline_record=420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$14::from
// 信息 ID: 64fe76acd2def32a8d0eefc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5974d6 -> 0xbdbcc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$18::clone
// 信息 ID: 350e36a46d3862302bee547e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=80, inline_record=3088; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vulkan_config_main::run_vulkan_configuration_app; module=176, inline_record=67084; 不是独立调用边。
// [PDB 内联宿主] country_core::render::render_config::impl$38::clone; module=194, inline_record=3124; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=9768; 不是独立调用边。
// [PDB 内联宿主] country_core::game_config::impl$11::clone; module=240, inline_record=14432; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=20768; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=937, inline_record=16540; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::vulkan_config::impl$2::update; module=938, inline_record=6672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$1::contains_layer::closure$0
// 信息 ID: 700ad190a68b9b9c44029379
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$1::contains_layer", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer; module=683, inline_record=12148; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=21052; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=21888; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=23068; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=23412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$1::contains_vendor::closure$0
// 信息 ID: 7a2b423059adf8eb2e1106d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$1::contains_vendor", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor; module=683, inline_record=12632; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=22232; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=23756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$20::eq
// 信息 ID: ad1b25e9f71e91f9ce2f0bd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=104, inline_record=27336; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=104, inline_record=27868; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>; module=2318, inline_record=6988; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>; module=2318, inline_record=7604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$22::partial_cmp
// 信息 ID: 76060d1eba0ca4fa810d1000
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 397666899118c3873d44ca17: bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=34, inline_record=4820; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=34, inline_record=5128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=34, inline_record=5408; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=74, inline_record=6720; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=74, inline_record=7196; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=74, inline_record=8464; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=236, inline_record=3532; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=236, inline_record=4048; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$24::clone
// 信息 ID: 08913266e3901581530bb027
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>; module=104, inline_record=45296; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$11::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>; module=2318, inline_record=14640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$26::fmt
// 信息 ID: f402689e4cd325a4801fe3bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$28::fmt
// 信息 ID: 1b9284dde8fb7e62971fa81c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$30::eq
// 信息 ID: e56e979090a53ce75c6af34c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::impl$2::view; module=3503, inline_record=6168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$32::clone
// 信息 ID: 22a69f1777bd69906f19a3c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=49864; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>; module=936, inline_record=1292; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1936, inline_record=8528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$33::fmt
// 信息 ID: 1c27bde46a503f4f92ef32bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$75::fmt<rhapsody::raw::init::AvailableGraphicsDevice>; module=140, inline_record=1276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$3::new::closure$1
// 信息 ID: 7dbdf3f8ecf633c2e3e120b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::init::impl$3::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=683, inline_record=13844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::implicit_layers::closure$0
// 信息 ID: 401a3d36377dfd0a29161da6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::implicit_layers", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::init::GraphicsInstanceLayer,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::LayerProperties,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::LayerProperties,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ash::vk::definitions::LayerProperties>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::LayerProperties,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::LayerProperties,rhapsody::raw::init::GraphicsInstanceLayer,tuple$<>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > > >; module=175, inline_record=1160; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ash::vk::definitions::LayerProperties>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>; module=1926, inline_record=492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::new::closure$0
// 信息 ID: 3b3ee8377cb9c835ddaa3bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLoader::new; module=683, inline_record=21240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::new::closure$0::closure$0
// 信息 ID: b4827d3d193612ac43167abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLoader::new; module=683, inline_record=21660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::available_devices::closure$0
// 信息 ID: 8c9901f40bb22d3e12294f03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::available_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::available_devices::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::available_devices::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=4344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::create_device::closure$0
// 信息 ID: dd0ba5009767fa0f493f8b96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::create_device", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsDevicesForSurface::create_device; module=683, inline_record=41260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::missing_devices::closure$0
// 信息 ID: 6dc6ddd07fdee430dd779d65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::missing_devices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices; module=683, inline_record=38592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::new::closure$2
// 信息 ID: a093232fcdb85a4550d00115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::init::impl$7::new", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::new::closure_env$2 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::new::closure_env$2 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; module=184, inline_record=33276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$0
// 信息 ID: 65990f44620087775fec5e74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::order_and_filter_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::stable::driftsort_main<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::stable::drift::sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::shared::pivot::median3_rec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::shared::pivot::choose_pivot<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 46 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=640; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=984; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=1088; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=2056; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=2184; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=2384; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; module=1937, inline_record=2504; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$1
// 信息 ID: 9a4a6b63709bbdf9f4192d38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::init::impl$7::order_and_filter_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1 <- alloc::vec::Vec::retain<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1 <- alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1936, inline_record=8436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
