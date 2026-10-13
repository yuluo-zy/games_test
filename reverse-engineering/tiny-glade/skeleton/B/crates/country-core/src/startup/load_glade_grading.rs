// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/load_glade_grading.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 82b2a1ab1f017705d4dd2f4fed9be8837bb60a6ded53da712bf4d3c7c927a908
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_glade_grading.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 493
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_grading::load_glade_grading
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":65636,"rva":9173440,"va":5377882560}
// 编译实例 RVAs: 0x8bf9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4964316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8dcbf4b00","size":5,"soft_fail":false,"target_va":5382847888,"va":5377883567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2980399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8d185d2ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5377884874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2801206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8ca41d5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377882689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2801249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e89f41d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377882748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4944601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8d9724b00","size":5,"soft_fail":false,"target_va":5382827728,"va":5377883122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_grading.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11821439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e87f61b400","size":5,"soft_fail":false,"target_va":5389704896,"va":5377883452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33656724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8948f0102","size":5,"soft_fail":false,"target_va":5411540963,"va":5377884234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33658571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8cb960102","size":5,"soft_fail":false,"target_va":5411542880,"va":5377884304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4822290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e812954900","size":5,"soft_fail":false,"target_va":5382705472,"va":5377883177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4860961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377882560,"raw_hex":"e8212c4a00","size":5,"soft_fail":false,"target_va":5382744080,"va":5377883114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377882560,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5377883272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":493,"record_offset":66028,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":493,"record_offset":65972,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::clone::Clone::clone_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":493,"record_offset":71124,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":8396,"source_line":174,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":493,"record_offset":71552,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":66644,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":493,"record_offset":69744,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":493,"record_offset":69232,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":27119,"source_line":36,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":493,"record_offset":70328,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":493,"record_offset":66980,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":26641,"source_line":269,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::is_halloween2024
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":493,"record_offset":65832,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":27361,"source_line":219,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":493,"record_offset":67264,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67232,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67092,"parent_record_offset":65636,"ancestor_record_offset":65636,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":66456,"parent_record_offset":66424,"ancestor_record_offset":65636,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":66488,"parent_record_offset":66456,"ancestor_record_offset":65636,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":66916,"parent_record_offset":66884,"ancestor_record_offset":65636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":66424,"parent_record_offset":66376,"ancestor_record_offset":65636,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":66884,"parent_record_offset":66804,"ancestor_record_offset":65636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":493,"record_offset":68432,"parent_record_offset":68404,"ancestor_record_offset":65636,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66300,"parent_record_offset":66264,"ancestor_record_offset":65636,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66848,"parent_record_offset":66804,"ancestor_record_offset":65636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66804,"parent_record_offset":66772,"ancestor_record_offset":65636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66376,"parent_record_offset":66336,"ancestor_record_offset":65636,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66336,"parent_record_offset":66300,"ancestor_record_offset":65636,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":66772,"parent_record_offset":66740,"ancestor_record_offset":65636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":66208,"parent_record_offset":66172,"ancestor_record_offset":65636,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":66172,"parent_record_offset":66136,"ancestor_record_offset":65636,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":66100,"parent_record_offset":66064,"ancestor_record_offset":65636,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":66136,"parent_record_offset":66100,"ancestor_record_offset":65636,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":66064,"parent_record_offset":66028,"ancestor_record_offset":65636,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":493,"record_offset":69268,"parent_record_offset":69232,"ancestor_record_offset":65636,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":66264,"parent_record_offset":66208,"ancestor_record_offset":65636,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":493,"record_offset":65996,"parent_record_offset":65972,"ancestor_record_offset":65636,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":68460,"parent_record_offset":68432,"ancestor_record_offset":65636,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":493,"record_offset":66580,"parent_record_offset":66548,"ancestor_record_offset":65636,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":493,"record_offset":71600,"parent_record_offset":71576,"ancestor_record_offset":65636,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":493,"record_offset":71840,"parent_record_offset":71624,"ancestor_record_offset":65636,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":493,"record_offset":71576,"parent_record_offset":71552,"ancestor_record_offset":65636,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":493,"record_offset":71864,"parent_record_offset":71840,"ancestor_record_offset":65636,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":493,"record_offset":71624,"parent_record_offset":71600,"ancestor_record_offset":65636,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":493,"record_offset":68484,"parent_record_offset":68460,"ancestor_record_offset":65636,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":493,"record_offset":71888,"parent_record_offset":71864,"ancestor_record_offset":65636,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":66548,"parent_record_offset":66208,"ancestor_record_offset":65636,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":66676,"parent_record_offset":66644,"ancestor_record_offset":65636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":68232,"parent_record_offset":68204,"ancestor_record_offset":65636,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":68280,"parent_record_offset":68256,"ancestor_record_offset":65636,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":68256,"parent_record_offset":68232,"ancestor_record_offset":65636,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":493,"record_offset":71796,"parent_record_offset":71768,"ancestor_record_offset":65636,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":493,"record_offset":65908,"parent_record_offset":65880,"ancestor_record_offset":65636,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":71768,"parent_record_offset":71740,"ancestor_record_offset":65636,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65636,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65636,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":71656,"parent_record_offset":71624,"ancestor_record_offset":65636,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":71740,"parent_record_offset":71656,"ancestor_record_offset":65636,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":493,"record_offset":71184,"parent_record_offset":71152,"ancestor_record_offset":65636,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":493,"record_offset":71152,"parent_record_offset":71124,"ancestor_record_offset":65636,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$1::iter_non_default_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":493,"record_offset":71912,"parent_record_offset":71888,"ancestor_record_offset":65636,"inlinee":26595,"source_line":110,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":493,"record_offset":71228,"parent_record_offset":71152,"ancestor_record_offset":65636,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":493,"record_offset":67028,"parent_record_offset":67004,"ancestor_record_offset":65636,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":493,"record_offset":67004,"parent_record_offset":66980,"ancestor_record_offset":65636,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$3::is_halloween2024::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":493,"record_offset":65940,"parent_record_offset":65856,"ancestor_record_offset":65636,"inlinee":27363,"source_line":222,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":493,"record_offset":67140,"parent_record_offset":67116,"ancestor_record_offset":65636,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":493,"record_offset":67188,"parent_record_offset":67164,"ancestor_record_offset":65636,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":493,"record_offset":67164,"parent_record_offset":67140,"ancestor_record_offset":65636,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":493,"record_offset":68204,"parent_record_offset":68176,"ancestor_record_offset":65636,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":493,"record_offset":68176,"parent_record_offset":68148,"ancestor_record_offset":65636,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67056,"parent_record_offset":67004,"ancestor_record_offset":65636,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67116,"parent_record_offset":67092,"ancestor_record_offset":65636,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67320,"parent_record_offset":67296,"ancestor_record_offset":65636,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":493,"record_offset":67296,"parent_record_offset":67264,"ancestor_record_offset":65636,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00493.jsonl.gz"}
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
// 完整结构及原始类型记录: load_glade_grading.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_glade_grading.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_glade_grading.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_grading::load_glade_grading
// 信息 ID: 47a7551873682968a391ac64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c1db00b8934df51f17603d18: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > > @ 0x14d0260 -> 0x8bf9c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x17cbaee -> 0x8bf9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
