// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/set_flag_connection_n_chimney_smoke.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b8a9ea637af70ad8363582018bb7a4f779e19dcbc3744b378797b3d2d8b220bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: set_flag_connection_n_chimney_smoke.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3759
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::set_flag_connection_n_chimney_smoke::set_flag_connection_and_chimney_smoke
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3759,"record_offset":260,"rva":36276592,"va":5404985712}
// 编译实例 RVAs: 0x2298970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408392d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_flag_connection_n_chimney_smoke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27654596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e83c065afe","size":5,"soft_fail":false,"target_va":5377331920,"va":5404986511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140839340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_flag_connection_n_chimney_smoke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27654068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e84c085afe","size":5,"soft_fail":false,"target_va":5377332032,"va":5404986095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-30629678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e8d2a02cfe","size":5,"soft_fail":false,"target_va":5374356528,"va":5404986201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6555119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e8ef056400","size":5,"soft_fail":false,"target_va":5411541776,"va":5404986652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25682230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e8ca1e78fe","size":5,"soft_fail":false,"target_va":5379303920,"va":5404986145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25673443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404985712,"raw_hex":"e81d4178fe","size":5,"soft_fail":false,"target_va":5379312624,"va":5404986062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3759,"record_offset":1840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3759,"record_offset":1500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3759,"record_offset":448,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3759,"record_offset":1148,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3759,"record_offset":1896,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3759,"record_offset":1340,"parent_record_offset":1308,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3759,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3759,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3759,"record_offset":1308,"parent_record_offset":1264,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3759,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3759,"record_offset":1264,"parent_record_offset":1220,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3759,"record_offset":768,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3759,"record_offset":1464,"parent_record_offset":1176,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3759,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":260,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3759,"record_offset":1220,"parent_record_offset":1176,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3759,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3759,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3759,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":260,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3759,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":260,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3759,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":260,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3759,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3759,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":260,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3759,"record_offset":724,"parent_record_offset":696,"ancestor_record_offset":260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3759,"record_offset":584,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3759,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":260,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3759,"record_offset":1632,"parent_record_offset":1552,"ancestor_record_offset":260,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3759,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3759,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3759,"record_offset":1428,"parent_record_offset":1264,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3759,"record_offset":640,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3759,"record_offset":992,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3759,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":260,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3759,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3759,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3759,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3759,"record_offset":1708,"parent_record_offset":1656,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03759.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3759,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03759.jsonl.gz"}
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
// 完整结构及原始类型记录: set_flag_connection_n_chimney_smoke.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: set_flag_connection_n_chimney_smoke.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: set_flag_connection_n_chimney_smoke.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_color::set_flag_connection_n_chimney_smoke::set_flag_connection_and_chimney_smoke
// 信息 ID: b92ca6c5a8658d63382e3316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 126e923900f9bf441740b259: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > @ 0x1c8b0d8 -> 0x2298970 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x1cabc76 -> 0x2298970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
