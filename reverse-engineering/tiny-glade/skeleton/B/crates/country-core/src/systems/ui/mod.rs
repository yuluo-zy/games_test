// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ui/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7265a15b4596abdfc30e0f83e138745c71c6199a762c1676112a5cf529e03126
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 61, 80, 135, 241, 333, 334, 335, 342, 507, 511, 563, 584, 1181, 1182, 1184, 1195, 1205, 1210, 1224, 1243, 1244, 2755, 2759, 2761, 2762, 2809, 2815, 2839, 2843, 2844, 2846, 2847, 3741, 3915, 3916
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: [{"name":"append-only-vec","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8","version":"0.1.2"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$2::eq
// 记录形式: inline_instance, inlinee_declaration; instances=308
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25BE
// 首条位置: {"module":61,"inlinee":9662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$4::hash
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D82
// 首条位置: {"module":241,"inlinee":15746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":241,"record_offset":61740,"parent_record_offset":61716,"ancestor_record_offset":61716,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":241,"record_offset":61764,"parent_record_offset":61740,"ancestor_record_offset":61716,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":241,"record_offset":61840,"parent_record_offset":61816,"ancestor_record_offset":61716,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3741,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":1724,"inlinee":9051,"source_line":255,"archive":"_pdb/B/modules/03741.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3741,"record_offset":1876,"parent_record_offset":1844,"ancestor_record_offset":1724,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/03741.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3741,"record_offset":1960,"parent_record_offset":1876,"ancestor_record_offset":1724,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/03741.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":241,"record_offset":61788,"parent_record_offset":61764,"ancestor_record_offset":61716,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3741,"record_offset":1932,"parent_record_offset":1876,"ancestor_record_offset":1724,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03741.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":241,"record_offset":61816,"parent_record_offset":61764,"ancestor_record_offset":61716,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::impl$0::add_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":75952,"rva":9722976,"va":5378432096}
// 编译实例 RVAs: 0x945c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9553468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8c4396eff","size":5,"soft_fail":false,"target_va":5368879232,"va":5378432695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8831484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8043e79ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378432279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8816338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e82e7979ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378432445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b218a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1948662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8f6bb1d00","size":5,"soft_fail":false,"target_va":5380380832,"va":5378432165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b67280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2233707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e86b152200","size":5,"soft_fail":false,"target_va":5380665984,"va":5378432272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141453000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11588423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e847d3b000","size":5,"soft_fail":false,"target_va":5390020608,"va":5378432180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14175a380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14763767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8f746e100","size":5,"soft_fail":false,"target_va":5393195904,"va":5378432132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33414574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8aeddfd01","size":5,"soft_fail":false,"target_va":5411847200,"va":5378432621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12532571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e85b3bbf00","size":5,"soft_fail":false,"target_va":5390964944,"va":5378432368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12361166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e8ce9dbc00","size":5,"soft_fail":false,"target_va":5390793680,"va":5378432509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33104443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378432096,"raw_hex":"e83b22f901","size":5,"soft_fail":false,"target_va":5411537104,"va":5378432656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":76688,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":76172,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":511,"record_offset":76100,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":511,"record_offset":76200,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":28895,"source_line":55,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::impl$67::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":511,"record_offset":76660,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":15725,"source_line":395,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":511,"record_offset":78112,"parent_record_offset":75952,"ancestor_record_offset":75952,"inlinee":29265,"source_line":18,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":77156,"parent_record_offset":77132,"ancestor_record_offset":75952,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":77204,"parent_record_offset":77180,"ancestor_record_offset":75952,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":77180,"parent_record_offset":77156,"ancestor_record_offset":75952,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":77132,"parent_record_offset":77108,"ancestor_record_offset":75952,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":77108,"parent_record_offset":77076,"ancestor_record_offset":75952,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":77076,"parent_record_offset":77052,"ancestor_record_offset":75952,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":511,"record_offset":78160,"parent_record_offset":78136,"ancestor_record_offset":75952,"inlinee":29268,"source_line":177,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$3::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":511,"record_offset":78136,"parent_record_offset":78112,"ancestor_record_offset":75952,"inlinee":29267,"source_line":217,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":511,"record_offset":76712,"parent_record_offset":76688,"ancestor_record_offset":75952,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":511,"record_offset":76140,"parent_record_offset":76100,"ancestor_record_offset":75952,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":77348,"parent_record_offset":77324,"ancestor_record_offset":75952,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":511,"record_offset":77324,"parent_record_offset":77300,"ancestor_record_offset":75952,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":511,"record_offset":76824,"parent_record_offset":76800,"ancestor_record_offset":75952,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":76872,"parent_record_offset":76848,"ancestor_record_offset":75952,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":76848,"parent_record_offset":76824,"ancestor_record_offset":75952,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":77052,"parent_record_offset":77028,"ancestor_record_offset":75952,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":511,"record_offset":76740,"parent_record_offset":76712,"ancestor_record_offset":75952,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":511,"record_offset":76800,"parent_record_offset":76772,"ancestor_record_offset":75952,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":511,"record_offset":76488,"parent_record_offset":76460,"ancestor_record_offset":75952,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":511,"record_offset":76540,"parent_record_offset":76516,"ancestor_record_offset":75952,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":511,"record_offset":76516,"parent_record_offset":76460,"ancestor_record_offset":75952,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":511,"record_offset":76564,"parent_record_offset":76540,"ancestor_record_offset":75952,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":76964,"parent_record_offset":76900,"ancestor_record_offset":75952,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":76772,"parent_record_offset":76740,"ancestor_record_offset":75952,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":76988,"parent_record_offset":76964,"ancestor_record_offset":75952,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":77276,"parent_record_offset":76740,"ancestor_record_offset":75952,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":77028,"parent_record_offset":76848,"ancestor_record_offset":75952,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":511,"record_offset":76432,"parent_record_offset":76404,"ancestor_record_offset":75952,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":76588,"parent_record_offset":76564,"ancestor_record_offset":75952,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":76900,"parent_record_offset":76872,"ancestor_record_offset":75952,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":76936,"parent_record_offset":76900,"ancestor_record_offset":75952,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":511,"record_offset":76240,"parent_record_offset":76200,"ancestor_record_offset":75952,"inlinee":7395,"source_line":249,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":511,"record_offset":76272,"parent_record_offset":76240,"ancestor_record_offset":75952,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":76336,"parent_record_offset":76304,"ancestor_record_offset":75952,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":76460,"parent_record_offset":76404,"ancestor_record_offset":75952,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":511,"record_offset":76304,"parent_record_offset":76272,"ancestor_record_offset":75952,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":76404,"parent_record_offset":76368,"ancestor_record_offset":75952,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":511,"record_offset":76368,"parent_record_offset":76336,"ancestor_record_offset":75952,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifier_config_hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70B2
// 首条位置: {"module":507,"inlinee":28850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":39448,"parent_record_offset":39424,"ancestor_record_offset":39424,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00507.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui::GlobalId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::ui::GlobalId>", "constructor": "ref$", "arguments": ["country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<country_core::systems::ui::GlobalId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot> >", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "enum2$<toolbar::mei::core::IsRoot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::iter_subtree<country_core::systems::ui::GlobalId,toolbar::mei::core::update_nav::closure_env$0>", "constructor": "toolbar::mei::core::iter_subtree", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::update_nav::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "country_core::input::action_set::DigitalActionSetActionType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,u32>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,u32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::next<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "hashbrown::map::impl$44::next", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,toolbar::mei::core::Node>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::core::Node,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>", "constructor": "tuple$", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "country_core::systems::ui::GlobalId", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::ui::GlobalId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<country_core::systems::ui::GlobalId,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["country_core::systems::ui::GlobalId", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<toolbar::mei::layout::LayoutTemp,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["toolbar::mei::layout::LayoutTemp", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>", "constructor": "std::collections::hash::map::ValuesMut", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::core::Node"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref_mut$<toolbar::mei::core::Node>,std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref_mut$<toolbar::mei::core::Node>", "std::collections::hash::map::ValuesMut<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >", "constructor": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::layout::LayoutTemp", "tuple$<>", "toolbar::mei::core::impl$17::perform_layout::closure_env$1", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<country_core::systems::ui::GlobalId>,country_core::systems::ui::GlobalId,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > > >", "constructor": "core::iter::adapters::copied::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "country_core::systems::ui::GlobalId", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::ui::GlobalId", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$0::add_ui
// 信息 ID: 788819d28fb6700f7f8fff9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f98b -> 0x945c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$2::eq
// 信息 ID: 2a69ae223e035956a68a977f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 277 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=80, inline_record=5664; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content_mut<toolbar::mei::widgets::FrameContent>; module=135, inline_record=2132; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>; module=135, inline_record=3604; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::content<tiny_glade::gui::gui_tod::TodCircleContent>; module=135, inline_record=6940; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$26::tab_switcher<ref$<str$> >; module=135, inline_record=8516; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> >; module=135, inline_record=15708; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> >; module=135, inline_record=18184; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=135, inline_record=21052; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::impl$4::hash
// 信息 ID: cad23dd5d019926e809f18a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >; module=241, inline_record=61716; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >; module=1182, inline_record=2480; 不是独立调用边。
// [PDB 内联宿主] utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,country_core::input::action_set::DigitalActionSetActionType> >; module=1184, inline_record=4068; 不是独立调用边。
// [PDB 内联宿主] utils::calculate_hash<tuple$<country_core::systems::ui::GlobalId,u32> >; module=1184, inline_record=6024; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$26::label_with_id<u64>; module=3741, inline_record=1724; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$26::image<ref$<str$> >; module=3741, inline_record=5664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifier_config_hash
// 信息 ID: bb1aba8492ae9307615ba7ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete; module=507, inline_record=39424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::signifiers::signifier_create::signifier_create; module=584, inline_record=47148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
