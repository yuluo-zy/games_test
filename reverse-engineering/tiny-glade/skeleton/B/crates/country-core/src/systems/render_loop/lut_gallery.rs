// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/lut_gallery.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: da0eeb39f94c8c72b823701ed0aca16e52ae0f4cfa01d9ccef52da00540580b6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lut_gallery.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 489, 1596, 1611, 1734
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":489,"record_offset":15300,"rva":9110720,"va":5377819840}
// 编译实例 RVAs: 0x8b04c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2738382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377819840,"raw_hex":"e83237d6ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377819865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33721004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377819840,"raw_hex":"e8ac8a0202","size":5,"soft_fail":false,"target_va":5411540992,"va":5377819983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13169288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377819840,"raw_hex":"e888f2c800","size":5,"soft_fail":false,"target_va":5390989248,"va":5377819955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140b67880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2847547","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377819840,"raw_hex":"e93b732b00","size":5,"soft_fail":false,"target_va":5380667520,"va":5377819968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":489,"record_offset":15796,"parent_record_offset":15300,"ancestor_record_offset":15300,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":15452,"parent_record_offset":15300,"ancestor_record_offset":15300,"inlinee":27143,"source_line":203,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":15608,"parent_record_offset":15588,"ancestor_record_offset":15300,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":15632,"parent_record_offset":15608,"ancestor_record_offset":15300,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":15300,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":489,"record_offset":15512,"parent_record_offset":15476,"ancestor_record_offset":15300,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":489,"record_offset":15536,"parent_record_offset":15512,"ancestor_record_offset":15300,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":489,"record_offset":15564,"parent_record_offset":15536,"ancestor_record_offset":15300,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":489,"record_offset":15732,"parent_record_offset":15476,"ancestor_record_offset":15300,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":489,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15300,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":489,"record_offset":15756,"parent_record_offset":15732,"ancestor_record_offset":15300,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":489,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":15300,"inlinee":12273,"source_line":214,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":489,"record_offset":15680,"parent_record_offset":15476,"ancestor_record_offset":15300,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::startup
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A16
// 首条位置: {"module":1596,"inlinee":27158}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1611,"record_offset":2307868,"parent_record_offset":2307836,"ancestor_record_offset":2307836,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_shader_into_library
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":1611,"record_offset":2307924,"parent_record_offset":2307836,"ancestor_record_offset":2307836,"inlinee":33477,"source_line":220,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1611,"record_offset":2307892,"parent_record_offset":2307868,"ancestor_record_offset":2307836,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":2307960,"parent_record_offset":2307924,"ancestor_record_offset":2307836,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":489,"record_offset":15828,"rva":9110880,"va":5377820000}
// 编译实例 RVAs: 0x8b0560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8221576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8788c82ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5377822371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8204022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e80ad182ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377820129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7668324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e89cfd8aff","size":5,"soft_fail":false,"target_va":5370158128,"va":5377826447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2742072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8c828d6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377823571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-164692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8ac7cfdff","size":5,"soft_fail":false,"target_va":5377659696,"va":5377824383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8bf06feff","size":5,"soft_fail":false,"target_va":5377696704,"va":5377826044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-123348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e82c1efeff","size":5,"soft_fail":false,"target_va":5377701312,"va":5377824655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fd6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t313102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e80ec70400","size":5,"soft_fail":false,"target_va":5378135744,"va":5377822637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t357909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e815760500","size":5,"soft_fail":false,"target_va":5378180624,"va":5377822710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t763512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e878a60b00","size":5,"soft_fail":false,"target_va":5378586704,"va":5377823187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t763937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e821a80b00","size":5,"soft_fail":false,"target_va":5378586944,"va":5377823002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t764259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e863a90b00","size":5,"soft_fail":false,"target_va":5378587184,"va":5377822920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3365041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8b1583300","size":5,"soft_fail":false,"target_va":5381187408,"va":5377822362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141304fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10830668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e84c43a500","size":5,"soft_fail":false,"target_va":5388652464,"va":5377821791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14126213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8858cd700","size":5,"soft_fail":false,"target_va":5391949088,"va":5377822870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33465546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8caa4fe01","size":5,"soft_fail":false,"target_va":5411285568,"va":5377820017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34024819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8732d0702","size":5,"soft_fail":false,"target_va":5411847152,"va":5377822328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34024509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e83d2c0702","size":5,"soft_fail":false,"target_va":5411847168,"va":5377822654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5018818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8c2944c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5377821385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t33717750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8f67d0202","size":5,"soft_fail":false,"target_va":5411541776,"va":5377824021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t553206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8f6700800","size":5,"soft_fail":false,"target_va":5378374304,"va":5377821093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t350058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e86a570500","size":5,"soft_fail":false,"target_va":5378173584,"va":5377823521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t347242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e86a4c0500","size":5,"soft_fail":false,"target_va":5378169824,"va":5377822577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-299535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e8f16dfbff","size":5,"soft_fail":false,"target_va":5377522032,"va":5377821562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-299202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e83e6ffbff","size":5,"soft_fail":false,"target_va":5377524592,"va":5377823789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33668425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e849bd0102","size":5,"soft_fail":false,"target_va":5411492368,"va":5377823938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33670813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e89dc60102","size":5,"soft_fail":false,"target_va":5411494800,"va":5377823982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4003224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e898153d00","size":5,"soft_fail":false,"target_va":5381823600,"va":5377820371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4003609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e819173d00","size":5,"soft_fail":false,"target_va":5381824016,"va":5377820402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3296381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e87d4c3200","size":5,"soft_fail":false,"target_va":5381117840,"va":5377821454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33715551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e85f750202","size":5,"soft_fail":false,"target_va":5411539504,"va":5377823948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33716279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e837780202","size":5,"soft_fail":false,"target_va":5411540288,"va":5377824004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33713054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e89e6b0202","size":5,"soft_fail":false,"target_va":5411537104,"va":5377824045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4111710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377820000,"raw_hex":"e85ebd3e00","size":5,"soft_fail":false,"target_va":5381934464,"va":5377822749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":489,"record_offset":25524,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":489,"record_offset":17784,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":489,"record_offset":19064,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":489,"record_offset":24404,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::enumerate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":24348,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":27149,"source_line":958,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":24264,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":17756,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":22572,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":26556,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":18052,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":489,"record_offset":26128,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":489,"record_offset":26260,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":489,"record_offset":25656,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":489,"record_offset":23740,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":23628,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":27145,"source_line":82,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::LoadMenuThumbnailMetrics::positions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":489,"record_offset":24060,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":27148,"source_line":238,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":489,"record_offset":23920,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":27146,"source_line":94,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":489,"record_offset":16076,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":489,"record_offset":17812,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":489,"record_offset":17968,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":489,"record_offset":26816,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":489,"record_offset":26732,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":27108,"parent_record_offset":27080,"ancestor_record_offset":15828,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":27080,"parent_record_offset":27024,"ancestor_record_offset":15828,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":489,"record_offset":23652,"parent_record_offset":23628,"ancestor_record_offset":15828,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":489,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":15828,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":15828,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":27052,"parent_record_offset":27024,"ancestor_record_offset":15828,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":27024,"parent_record_offset":27000,"ancestor_record_offset":15828,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":15828,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":16560,"parent_record_offset":16536,"ancestor_record_offset":15828,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":27000,"parent_record_offset":26976,"ancestor_record_offset":15828,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":16512,"parent_record_offset":16488,"ancestor_record_offset":15828,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":16488,"parent_record_offset":16448,"ancestor_record_offset":15828,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":24112,"parent_record_offset":24088,"ancestor_record_offset":15828,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":24172,"parent_record_offset":24060,"ancestor_record_offset":15828,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":16448,"parent_record_offset":16424,"ancestor_record_offset":15828,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":489,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":15828,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":489,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":15828,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":489,"record_offset":24512,"parent_record_offset":24472,"ancestor_record_offset":15828,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":489,"record_offset":25552,"parent_record_offset":25524,"ancestor_record_offset":15828,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":489,"record_offset":19112,"parent_record_offset":19064,"ancestor_record_offset":15828,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":489,"record_offset":20116,"parent_record_offset":20072,"ancestor_record_offset":15828,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":489,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":15828,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":489,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":15828,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":489,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":15828,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":489,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":15828,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":489,"record_offset":16704,"parent_record_offset":16168,"ancestor_record_offset":15828,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":489,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":15828,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":489,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":15828,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":489,"record_offset":16788,"parent_record_offset":16728,"ancestor_record_offset":15828,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":489,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":489,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":15828,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":489,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":15828,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":489,"record_offset":16132,"parent_record_offset":16104,"ancestor_record_offset":15828,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":489,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":15828,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":489,"record_offset":16168,"parent_record_offset":16104,"ancestor_record_offset":15828,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":489,"record_offset":19160,"parent_record_offset":19112,"ancestor_record_offset":15828,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":489,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":15828,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":489,"record_offset":25956,"parent_record_offset":25928,"ancestor_record_offset":15828,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":489,"record_offset":26008,"parent_record_offset":25984,"ancestor_record_offset":15828,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":489,"record_offset":25984,"parent_record_offset":25928,"ancestor_record_offset":15828,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":489,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":15828,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":20000,"parent_record_offset":19976,"ancestor_record_offset":15828,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":489,"record_offset":23772,"parent_record_offset":23740,"ancestor_record_offset":15828,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":19400,"parent_record_offset":19376,"ancestor_record_offset":15828,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":15828,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":489,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":15828,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":489,"record_offset":24736,"parent_record_offset":24704,"ancestor_record_offset":15828,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":489,"record_offset":24704,"parent_record_offset":24472,"ancestor_record_offset":15828,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":489,"record_offset":25084,"parent_record_offset":25052,"ancestor_record_offset":15828,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::Enumerate::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":489,"record_offset":24372,"parent_record_offset":24348,"ancestor_record_offset":15828,"inlinee":27151,"source_line":23,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":489,"record_offset":25136,"parent_record_offset":25084,"ancestor_record_offset":15828,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":489,"record_offset":24288,"parent_record_offset":24264,"ancestor_record_offset":15828,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":489,"record_offset":24440,"parent_record_offset":24404,"ancestor_record_offset":15828,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":489,"record_offset":24312,"parent_record_offset":24288,"ancestor_record_offset":15828,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":489,"record_offset":24472,"parent_record_offset":24440,"ancestor_record_offset":15828,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":489,"record_offset":24820,"parent_record_offset":24796,"ancestor_record_offset":15828,"inlinee":11883,"source_line":74,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":489,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":15828,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":24088,"parent_record_offset":24060,"ancestor_record_offset":15828,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":489,"record_offset":24200,"parent_record_offset":24172,"ancestor_record_offset":15828,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":489,"record_offset":24916,"parent_record_offset":24892,"ancestor_record_offset":15828,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":489,"record_offset":27412,"parent_record_offset":27384,"ancestor_record_offset":15828,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":19636,"parent_record_offset":19608,"ancestor_record_offset":15828,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":19608,"parent_record_offset":19580,"ancestor_record_offset":15828,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":489,"record_offset":16336,"parent_record_offset":16272,"ancestor_record_offset":15828,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":489,"record_offset":25296,"parent_record_offset":25272,"ancestor_record_offset":15828,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":24768,"parent_record_offset":24736,"ancestor_record_offset":15828,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":16360,"parent_record_offset":16336,"ancestor_record_offset":15828,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":24992,"parent_record_offset":24736,"ancestor_record_offset":15828,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":25016,"parent_record_offset":24992,"ancestor_record_offset":15828,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":16400,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":15828,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":20072,"parent_record_offset":19112,"ancestor_record_offset":15828,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":24892,"parent_record_offset":24868,"ancestor_record_offset":15828,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":16888,"parent_record_offset":16076,"ancestor_record_offset":15828,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":24868,"parent_record_offset":24844,"ancestor_record_offset":15828,"inlinee":11885,"source_line":2240,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":24844,"parent_record_offset":24820,"ancestor_record_offset":15828,"inlinee":11884,"source_line":2374,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":16976,"parent_record_offset":16948,"ancestor_record_offset":15828,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":489,"record_offset":25900,"parent_record_offset":25868,"ancestor_record_offset":15828,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":21492,"parent_record_offset":20116,"ancestor_record_offset":15828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":24228,"parent_record_offset":24172,"ancestor_record_offset":15828,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":15828,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":489,"record_offset":24572,"parent_record_offset":24512,"ancestor_record_offset":15828,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":489,"record_offset":24600,"parent_record_offset":24512,"ancestor_record_offset":15828,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":489,"record_offset":24544,"parent_record_offset":24512,"ancestor_record_offset":15828,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":15828,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":24940,"parent_record_offset":24916,"ancestor_record_offset":15828,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":25580,"parent_record_offset":25552,"ancestor_record_offset":15828,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":25608,"parent_record_offset":25580,"ancestor_record_offset":15828,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":489,"record_offset":16272,"parent_record_offset":16244,"ancestor_record_offset":15828,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":489,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":15828,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":489,"record_offset":16308,"parent_record_offset":16272,"ancestor_record_offset":15828,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":489,"record_offset":16628,"parent_record_offset":16448,"ancestor_record_offset":15828,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":489,"record_offset":25208,"parent_record_offset":25184,"ancestor_record_offset":15828,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":489,"record_offset":25160,"parent_record_offset":25136,"ancestor_record_offset":15828,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":489,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":15828,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":15828,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":20332,"parent_record_offset":20208,"ancestor_record_offset":15828,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":21584,"parent_record_offset":21548,"ancestor_record_offset":15828,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":20248,"parent_record_offset":20208,"ancestor_record_offset":15828,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":18160,"parent_record_offset":18100,"ancestor_record_offset":15828,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":17876,"parent_record_offset":17840,"ancestor_record_offset":15828,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":15828,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":15828,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":15828,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":489,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":15828,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":489,"record_offset":19976,"parent_record_offset":19952,"ancestor_record_offset":15828,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":489,"record_offset":25704,"parent_record_offset":25656,"ancestor_record_offset":15828,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":489,"record_offset":23848,"parent_record_offset":23820,"ancestor_record_offset":15828,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":489,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":15828,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":489,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":15828,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":25344,"parent_record_offset":25320,"ancestor_record_offset":15828,"inlinee":27154,"source_line":78,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":25368,"parent_record_offset":25344,"ancestor_record_offset":15828,"inlinee":27145,"source_line":82,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/lut_gallery.rs
//     evidence={"module":489,"record_offset":25320,"parent_record_offset":25296,"ancestor_record_offset":15828,"inlinee":27153,"source_line":100,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":489,"record_offset":19928,"parent_record_offset":19904,"ancestor_record_offset":15828,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":489,"record_offset":19544,"parent_record_offset":19516,"ancestor_record_offset":15828,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":489,"record_offset":19580,"parent_record_offset":19544,"ancestor_record_offset":15828,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":489,"record_offset":19516,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":489,"record_offset":19376,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":489,"record_offset":19680,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":489,"record_offset":19460,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":489,"record_offset":19196,"parent_record_offset":19160,"ancestor_record_offset":15828,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":489,"record_offset":19220,"parent_record_offset":19196,"ancestor_record_offset":15828,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":489,"record_offset":19904,"parent_record_offset":19796,"ancestor_record_offset":15828,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19844,"parent_record_offset":19820,"ancestor_record_offset":15828,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19764,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":15828,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":15828,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19248,"parent_record_offset":19220,"ancestor_record_offset":15828,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":15828,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19348,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":489,"record_offset":19796,"parent_record_offset":19296,"ancestor_record_offset":15828,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":489,"record_offset":18076,"parent_record_offset":18052,"ancestor_record_offset":15828,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":489,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":15828,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":489,"record_offset":17840,"parent_record_offset":17812,"ancestor_record_offset":15828,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":489,"record_offset":18100,"parent_record_offset":18076,"ancestor_record_offset":15828,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":489,"record_offset":17932,"parent_record_offset":17840,"ancestor_record_offset":15828,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":489,"record_offset":18132,"parent_record_offset":18100,"ancestor_record_offset":15828,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":489,"record_offset":26840,"parent_record_offset":26816,"ancestor_record_offset":15828,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":489,"record_offset":26756,"parent_record_offset":26732,"ancestor_record_offset":15828,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":489,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":15828,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":489,"record_offset":25736,"parent_record_offset":25704,"ancestor_record_offset":15828,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":489,"record_offset":25800,"parent_record_offset":25768,"ancestor_record_offset":15828,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":489,"record_offset":25928,"parent_record_offset":25868,"ancestor_record_offset":15828,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":489,"record_offset":25768,"parent_record_offset":25736,"ancestor_record_offset":15828,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":489,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":15828,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":489,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":15828,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":489,"record_offset":20172,"parent_record_offset":20116,"ancestor_record_offset":15828,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":489,"record_offset":20392,"parent_record_offset":20172,"ancestor_record_offset":15828,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":489,"record_offset":21520,"parent_record_offset":21492,"ancestor_record_offset":15828,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":489,"record_offset":20208,"parent_record_offset":20172,"ancestor_record_offset":15828,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":489,"record_offset":21548,"parent_record_offset":21520,"ancestor_record_offset":15828,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":489,"record_offset":20304,"parent_record_offset":20208,"ancestor_record_offset":15828,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":489,"record_offset":25868,"parent_record_offset":25832,"ancestor_record_offset":15828,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":489,"record_offset":25832,"parent_record_offset":25800,"ancestor_record_offset":15828,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A11
// 首条位置: {"module":489,"inlinee":27153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::grading_lut::GradingLut::asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":25344,"parent_record_offset":25320,"ancestor_record_offset":25320,"inlinee":27154,"source_line":78,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":489,"record_offset":25392,"parent_record_offset":25368,"ancestor_record_offset":25320,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":489,"record_offset":25440,"parent_record_offset":25416,"ancestor_record_offset":25320,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":489,"record_offset":25416,"parent_record_offset":25392,"ancestor_record_offset":25320,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":489,"record_offset":25368,"parent_record_offset":25344,"ancestor_record_offset":25320,"inlinee":27145,"source_line":82,"archive":"_pdb/B/modules/00489.jsonl.gz"}
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
// 完整结构及原始类型记录: lut_gallery.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::lut_gallery::LutGalleryPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>", "constructor": "core::iter::sources::once::Once", "arguments": ["country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lut_gallery.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/lut_gallery.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: lut_gallery.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":489,"record_offset":26128,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00489.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":489,"record_offset":26260,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00489.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"db5b56159295e7815cb797b3","relative_path":"_lut_gallery_thumbnail.raster.hlsl_b903e43ffb3da915.bin","sha256":"760bbda00525a069899111d341ead4a020962effbbfff042af72eeceea8d87e4","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"lut_gallery","prefix_candidates":["lut_gallery"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c2d9aec423a679b7a540623e","relative_path":"_lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin","sha256":"3b81b9ed238de4de5f2166f175118bf803c14669e25cff89f89bc7cbb9559696","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"lut_gallery","prefix_candidates":["lut_gallery"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"$Globals","storage_class":"Uniform","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[51,51,51,51,29,29,30,30,30,28]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"icon_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"grading_lut_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"framebuffer_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_center","type_id":29,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"thumbnail_size","type_id":29,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_hovered","type_id":30,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"is_active","type_id":30,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":8,"name":"show_icon","type_id":30,"decorations":[{"decoration":"Offset","operands":[56]}]},{"index":9,"name":"lut_amount","type_id":28,"decorations":[{"decoration":"Offset","operands":[60]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,41]},"decorations":[],"pointee":{"id":41,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _lut_gallery_thumbnail.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"$Globals","storage_class":"Uniform","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[51,51,51,51,29,29,30,30,30,28]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"icon_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"grading_lut_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"framebuffer_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_center","type_id":29,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"thumbnail_size","type_id":29,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_hovered","type_id":30,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"is_active","type_id":30,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":8,"name":"show_icon","type_id":30,"decorations":[{"decoration":"Offset","operands":[56]}]},{"index":9,"name":"lut_amount","type_id":28,"decorations":[{"decoration":"Offset","operands":[60]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,41]},"decorations":[],"pointee":{"id":41,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"$Globals","storage_class":"Uniform","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[71,71,71,71,38,38,39,39,39,37]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"icon_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"grading_lut_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"framebuffer_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_center","type_id":38,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"thumbnail_size","type_id":38,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_hovered","type_id":39,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"is_active","type_id":39,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":8,"name":"show_icon","type_id":39,"decorations":[{"decoration":"Offset","operands":[56]}]},{"index":9,"name":"lut_amount","type_id":37,"decorations":[{"decoration":"Offset","operands":[60]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"input_tex","storage_class":"UniformConstant","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,25]},"decorations":[],"pointee":{"id":25,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"icon_tex","storage_class":"UniformConstant","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,25]},"decorations":[],"pointee":{"id":25,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"grading_lut_tex","storage_class":"UniformConstant","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.3d.image","record":{"opcode":"OpTypeImage","operands":[37,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.SV_TARGET","storage_class":"Output","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1454945,"position_among_mapped_render_targets":15,"mapped_render_target_count":25,"previous_callsite":1454909,"next_callsite":1454981,"previous_target_files":["crates/country-core/src/systems/render_loop/session_selection/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lut_gallery.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 230; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::impl$0::build
// 信息 ID: 0ebd4db388798ca79c70c5e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// 信息 ID: 19a64e9dffc7b175a4d11c8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163361 -> 0x8b0560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
// 信息 ID: 579aee54ff7ebb654332905c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=489, inline_record=25320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::startup
// 信息 ID: f2e646af465c85eec76dced5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=2307836; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=3521156; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> >; module=1734, inline_record=21072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
