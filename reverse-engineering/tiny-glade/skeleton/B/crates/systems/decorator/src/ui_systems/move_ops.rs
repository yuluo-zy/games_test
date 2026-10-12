// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/ui_systems/move_ops.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fd31e5d9152c284d26527db6a79cca83d67c42aa349bb2fc48b8b8f3463f7011
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: move_ops.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1523
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1523,"record_offset":832,"rva":19398160,"va":5388107280}
// 编译实例 RVAs: 0x127fe10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13026086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e8da3c39ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388107569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13026206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e8623c39ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388107705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23433251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e823906501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388107736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5267322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e886a0afff","size":5,"soft_fail":false,"target_va":5382840208,"va":5388107525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10364626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e82ed961ff","size":5,"soft_fail":false,"target_va":5377743024,"va":5388107645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7680022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e8eacf8aff","size":5,"soft_fail":false,"target_va":5380427664,"va":5388107681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6221016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107280,"raw_hex":"e82813a1ff","size":5,"soft_fail":false,"target_va":5381886416,"va":5388107427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1523,"record_offset":1356,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1523,"record_offset":1440,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1523,"record_offset":1656,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1523,"record_offset":1008,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1523,"record_offset":1200,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1523,"record_offset":1276,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":832,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":832,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":832,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":832,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":832,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":1808,"parent_record_offset":1752,"ancestor_record_offset":832,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1523,"record_offset":1468,"parent_record_offset":1440,"ancestor_record_offset":832,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1523,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":832,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":832,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":832,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":832,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1523,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":832,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1523,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":832,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1523,"record_offset":1076,"parent_record_offset":1036,"ancestor_record_offset":832,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1523,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":832,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1523,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1523,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":832,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1523,"record_offset":1100,"parent_record_offset":1036,"ancestor_record_offset":832,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1523,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":832,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1523,"record_offset":1136,"parent_record_offset":1100,"ancestor_record_offset":832,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1523,"record_offset":2116,"rva":19398688,"va":5388107808}
// 编译实例 RVAs: 0x1280020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13026386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8ae3b39ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388107869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13026749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8433a39ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388108248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23432819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8738e6501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388108168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23432768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8408e6501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388108190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10363989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8abdb61ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388108000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10363900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e804dc61ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388108103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10364709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8dbd861ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388107984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10364174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e8f2da61ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388108041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10364157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388107808,"raw_hex":"e803db61ff","size":5,"soft_fail":false,"target_va":5377743920,"va":5388108072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":2296,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2980,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1523,"record_offset":2444,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1523,"record_offset":3064,"parent_record_offset":2116,"ancestor_record_offset":2116,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":2116,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":2116,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2116,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":2116,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1523,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2116,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1523,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2116,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2116,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":2764,"parent_record_offset":2732,"ancestor_record_offset":2116,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1523,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":2116,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2640,"parent_record_offset":2612,"ancestor_record_offset":2116,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2116,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2612,"parent_record_offset":2584,"ancestor_record_offset":2116,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":2116,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":2116,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1523,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2116,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1523,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2116,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1523,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":2116,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1523,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2116,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1523,"record_offset":2676,"parent_record_offset":2640,"ancestor_record_offset":2116,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1523,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2116,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1523,"record_offset":2892,"parent_record_offset":2640,"ancestor_record_offset":2116,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1523,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":2116,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::move_ops::calculate_move_dst
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1523,"record_offset":3180,"rva":19399152,"va":5388108272}
// 编译实例 RVAs: 0x12801f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: move_ops.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13432159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e8a10a33ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5388108490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23433037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e84d8f6501","size":5,"soft_fail":false,"target_va":5411541808,"va":5388108766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23432993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e8218f6501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388108778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8798228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e8ecbf79ff","size":5,"soft_fail":false,"target_va":5379310224,"va":5388108447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7690989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e813a58aff","size":5,"soft_fail":false,"target_va":5380417680,"va":5388108664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6218498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388108272,"raw_hex":"e8fe1ca1ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5388108685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1523,"record_offset":3328,"parent_record_offset":3180,"ancestor_record_offset":3180,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1523,"record_offset":3364,"parent_record_offset":3180,"ancestor_record_offset":3180,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1523,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3180,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1523,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3180,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1523,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":3180,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1523,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":3180,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1523,"record_offset":4260,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1523,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":3180,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1523,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":3180,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1523,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3180,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1523,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3180,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1523,"record_offset":3844,"parent_record_offset":3816,"ancestor_record_offset":3180,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1523,"record_offset":3876,"parent_record_offset":3844,"ancestor_record_offset":3180,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1523,"record_offset":3816,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1523,"record_offset":3676,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1523,"record_offset":4288,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1523,"record_offset":3760,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1523,"record_offset":3648,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1523,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":3180,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1523,"record_offset":3428,"parent_record_offset":3392,"ancestor_record_offset":3180,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1523,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3180,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1523,"record_offset":4144,"parent_record_offset":3976,"ancestor_record_offset":3180,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1523,"record_offset":3488,"parent_record_offset":3456,"ancestor_record_offset":3180,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":4084,"parent_record_offset":4000,"ancestor_record_offset":3180,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":4028,"parent_record_offset":4000,"ancestor_record_offset":3180,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":4380,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3180,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":3180,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":3516,"parent_record_offset":3456,"ancestor_record_offset":3180,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3180,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":3620,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1523,"record_offset":3976,"parent_record_offset":3564,"ancestor_record_offset":3180,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01523.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1523,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":3180,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01523.jsonl.gz"}
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
// 完整结构及原始类型记录: move_ops.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: move_ops.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: move_ops.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::calculate_move_dst
// 信息 ID: 0b202b537753aacd7ec3bffb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x7a7a7f -> 0x12801f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac6d2 -> 0x12801f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier
// 信息 ID: a42747909cf0ebe835867af4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6e09 -> 0x127fe10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure$0
// 信息 ID: 52f89a4f5496032eafce986f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::move_ops::show_decorator_remove_signifier", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x127fdc2 -> 0x1280020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
