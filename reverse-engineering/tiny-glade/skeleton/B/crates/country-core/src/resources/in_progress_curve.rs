// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/in_progress_curve.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b387d959777d099a703f3e21905c2b8746a4446260535fb8f23fc32ee16ae3fb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: in_progress_curve.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 551, 554, 567, 578
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":23132,"rva":11354528,"va":5380063648}
// 编译实例 RVAs: 0xad41a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5253711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8b1d5afff","size":5,"soft_fail":false,"target_va":5374813424,"va":5380067130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4982349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8b3f9b3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380063832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4982544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8f0f8b3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380064043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088c190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2395768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e88871dbff","size":5,"soft_fail":false,"target_va":5377671568,"va":5380067331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141468880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10044364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8cc439900","size":5,"soft_fail":false,"target_va":5390108800,"va":5380064431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10045415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8e7479900","size":5,"soft_fail":false,"target_va":5390111792,"va":5380066372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31782972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e83cf8e401","size":5,"soft_fail":false,"target_va":5411847168,"va":5380064191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31781742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e86ef3e401","size":5,"soft_fail":false,"target_va":5411847184,"va":5380065437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31280545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8a14ddd01","size":5,"soft_fail":false,"target_va":5411347040,"va":5380066490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t31474303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e87f42e001","size":5,"soft_fail":false,"target_va":5411540963,"va":5380066655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31474789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e86544e001","size":5,"soft_fail":false,"target_va":5411541225,"va":5380066431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31476689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8d14be001","size":5,"soft_fail":false,"target_va":5411543104,"va":5380066410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31476463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8ef4ae001","size":5,"soft_fail":false,"target_va":5411543088,"va":5380066620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063648,"raw_hex":"e8cbdcffff","size":5,"soft_fail":false,"target_va":5380055008,"va":5380064016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":34184,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":27024,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":567,"record_offset":23324,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":26528,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":567,"record_offset":27240,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":567,"record_offset":28300,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":567,"record_offset":23440,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::Rdp::stitch_with
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":29048,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":32447,"source_line":22,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":23132,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":24108,"parent_record_offset":24084,"ancestor_record_offset":23132,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":24996,"parent_record_offset":24972,"ancestor_record_offset":23132,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":24060,"parent_record_offset":23884,"ancestor_record_offset":23132,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":24972,"parent_record_offset":24916,"ancestor_record_offset":23132,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23628,"parent_record_offset":23604,"ancestor_record_offset":23132,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30584,"parent_record_offset":30556,"ancestor_record_offset":23132,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23824,"parent_record_offset":23796,"ancestor_record_offset":23132,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":24944,"parent_record_offset":24916,"ancestor_record_offset":23132,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":24916,"parent_record_offset":24892,"ancestor_record_offset":23132,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30644,"parent_record_offset":30612,"ancestor_record_offset":23132,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":23132,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23652,"parent_record_offset":23628,"ancestor_record_offset":23132,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30612,"parent_record_offset":30584,"ancestor_record_offset":23132,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23884,"parent_record_offset":23852,"ancestor_record_offset":23132,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23852,"parent_record_offset":23824,"ancestor_record_offset":23132,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":24892,"parent_record_offset":24868,"ancestor_record_offset":23132,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":23920,"parent_record_offset":23884,"ancestor_record_offset":23132,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":23756,"parent_record_offset":23720,"ancestor_record_offset":23132,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":23720,"parent_record_offset":23536,"ancestor_record_offset":23132,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":29328,"parent_record_offset":29048,"ancestor_record_offset":23132,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":25144,"parent_record_offset":25108,"ancestor_record_offset":23132,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30512,"parent_record_offset":30484,"ancestor_record_offset":23132,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30016,"parent_record_offset":29944,"ancestor_record_offset":23132,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":23604,"parent_record_offset":23580,"ancestor_record_offset":23132,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":23580,"parent_record_offset":23536,"ancestor_record_offset":23132,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":31668,"parent_record_offset":31636,"ancestor_record_offset":23132,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29988,"parent_record_offset":29944,"ancestor_record_offset":23132,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29944,"parent_record_offset":29048,"ancestor_record_offset":23132,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30556,"parent_record_offset":30512,"ancestor_record_offset":23132,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30168,"parent_record_offset":29944,"ancestor_record_offset":23132,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":23796,"parent_record_offset":23756,"ancestor_record_offset":23132,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":23500,"parent_record_offset":23440,"ancestor_record_offset":23132,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29168,"parent_record_offset":29048,"ancestor_record_offset":23132,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":31604,"parent_record_offset":29048,"ancestor_record_offset":23132,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":32260,"parent_record_offset":32216,"ancestor_record_offset":23132,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30456,"parent_record_offset":29048,"ancestor_record_offset":23132,"inlinee":10301,"source_line":3726,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":23536,"parent_record_offset":23500,"ancestor_record_offset":23132,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32360,"parent_record_offset":32260,"ancestor_record_offset":23132,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32712,"parent_record_offset":31668,"ancestor_record_offset":23132,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32540,"parent_record_offset":32512,"ancestor_record_offset":23132,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":31636,"parent_record_offset":31604,"ancestor_record_offset":23132,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":30484,"parent_record_offset":30456,"ancestor_record_offset":23132,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":23132,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":23132,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":24184,"parent_record_offset":24160,"ancestor_record_offset":23132,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":23132,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":23132,"inlinee":23354,"source_line":147,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":31900,"parent_record_offset":31868,"ancestor_record_offset":23132,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":32040,"parent_record_offset":31932,"ancestor_record_offset":23132,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":31868,"parent_record_offset":31668,"ancestor_record_offset":23132,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":32216,"parent_record_offset":32040,"ancestor_record_offset":23132,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":23992,"parent_record_offset":23968,"ancestor_record_offset":23132,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23132,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":32400,"parent_record_offset":31932,"ancestor_record_offset":23132,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":26420,"parent_record_offset":26392,"ancestor_record_offset":23132,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":30668,"parent_record_offset":30644,"ancestor_record_offset":23132,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":32160,"parent_record_offset":32116,"ancestor_record_offset":23132,"inlinee":32453,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$16::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":567,"record_offset":32116,"parent_record_offset":32072,"ancestor_record_offset":23132,"inlinee":32451,"source_line":35,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":567,"record_offset":29252,"parent_record_offset":29224,"ancestor_record_offset":23132,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":24160,"parent_record_offset":23756,"ancestor_record_offset":23132,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":24844,"parent_record_offset":23440,"ancestor_record_offset":23132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":30128,"parent_record_offset":29944,"ancestor_record_offset":23132,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":23400,"parent_record_offset":23372,"ancestor_record_offset":23132,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":32312,"parent_record_offset":32260,"ancestor_record_offset":23132,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":23132,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27164,"parent_record_offset":27140,"ancestor_record_offset":23132,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27052,"parent_record_offset":27024,"ancestor_record_offset":23132,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":29224,"parent_record_offset":29196,"ancestor_record_offset":23132,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27140,"parent_record_offset":27116,"ancestor_record_offset":23132,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":29280,"parent_record_offset":29224,"ancestor_record_offset":23132,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27116,"parent_record_offset":27080,"ancestor_record_offset":23132,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":27080,"parent_record_offset":27052,"ancestor_record_offset":23132,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":31932,"parent_record_offset":31900,"ancestor_record_offset":23132,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":23132,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::stitch_with::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":32072,"parent_record_offset":32040,"ancestor_record_offset":23132,"inlinee":32449,"source_line":44,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BA6
// 首条位置: {"module":554,"inlinee":31654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":50628,"parent_record_offset":48980,"ancestor_record_offset":48980,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":52748,"parent_record_offset":48980,"ancestor_record_offset":48980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":554,"record_offset":49088,"parent_record_offset":48980,"ancestor_record_offset":48980,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":554,"record_offset":51248,"parent_record_offset":48980,"ancestor_record_offset":48980,"inlinee":31662,"source_line":6,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":49624,"parent_record_offset":49600,"ancestor_record_offset":48980,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":49648,"parent_record_offset":49624,"ancestor_record_offset":48980,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":50576,"parent_record_offset":50552,"ancestor_record_offset":48980,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":49600,"parent_record_offset":49424,"ancestor_record_offset":48980,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":50552,"parent_record_offset":50464,"ancestor_record_offset":48980,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":49364,"parent_record_offset":49336,"ancestor_record_offset":48980,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":50492,"parent_record_offset":50464,"ancestor_record_offset":48980,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":50464,"parent_record_offset":50440,"ancestor_record_offset":48980,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":49424,"parent_record_offset":49392,"ancestor_record_offset":48980,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":49392,"parent_record_offset":49364,"ancestor_record_offset":48980,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":50440,"parent_record_offset":50416,"ancestor_record_offset":48980,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":49460,"parent_record_offset":49424,"ancestor_record_offset":48980,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":49296,"parent_record_offset":49260,"ancestor_record_offset":48980,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":49260,"parent_record_offset":49188,"ancestor_record_offset":48980,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":49232,"parent_record_offset":49188,"ancestor_record_offset":48980,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":49336,"parent_record_offset":49296,"ancestor_record_offset":48980,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":49152,"parent_record_offset":49088,"ancestor_record_offset":48980,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":49188,"parent_record_offset":49152,"ancestor_record_offset":48980,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":49484,"parent_record_offset":49460,"ancestor_record_offset":48980,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":49508,"parent_record_offset":49484,"ancestor_record_offset":48980,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":49724,"parent_record_offset":49700,"ancestor_record_offset":48980,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":49532,"parent_record_offset":49508,"ancestor_record_offset":48980,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":49556,"parent_record_offset":49532,"ancestor_record_offset":48980,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":50520,"parent_record_offset":50492,"ancestor_record_offset":48980,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":49700,"parent_record_offset":49296,"ancestor_record_offset":48980,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":50392,"parent_record_offset":49088,"ancestor_record_offset":48980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":11280,"rva":11350128,"va":5380059248}
// 编译实例 RVAs: 0xad3070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5248797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8e3e8afff","size":5,"soft_fail":false,"target_va":5374813424,"va":5380062216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4978133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e82b0ab4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380059616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4979001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8c706b4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380060500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10041934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e84e3a9900","size":5,"soft_fail":false,"target_va":5390101424,"va":5380059485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10050777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8d95c9900","size":5,"soft_fail":false,"target_va":5390111792,"va":5380061010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146a830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10057350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e886769900","size":5,"soft_fail":false,"target_va":5390116912,"va":5380059557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31787295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e81f09e501","size":5,"soft_fail":false,"target_va":5411847168,"va":5380059868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t713544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e848e30a00","size":5,"soft_fail":false,"target_va":5380775408,"va":5380061859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31286245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8e563dd01","size":5,"soft_fail":false,"target_va":5411347040,"va":5380060790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31481135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e82f5de001","size":5,"soft_fail":false,"target_va":5411540963,"va":5380059823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31480136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e84859e001","size":5,"soft_fail":false,"target_va":5411541776,"va":5380061635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31481342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8fe5de001","size":5,"soft_fail":false,"target_va":5411542064,"va":5380060717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31480638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e83e5be001","size":5,"soft_fail":false,"target_va":5411542296,"va":5380061653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31481211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e87b5de001","size":5,"soft_fail":false,"target_va":5411541904,"va":5380060688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31481162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e84a5de001","size":5,"soft_fail":false,"target_va":5411542880,"va":5380061713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31482335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e8df61e001","size":5,"soft_fail":false,"target_va":5411543088,"va":5380060748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380059248,"raw_hex":"e805ebffff","size":5,"soft_fail":false,"target_va":5380055008,"va":5380060374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":19044,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12084,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":11584,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15276,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$21::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15936,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":10301,"source_line":3726,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":11844,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":17752,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":567,"record_offset":11544,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":14872,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":11516,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":11692,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":567,"record_offset":16424,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":567,"record_offset":11488,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":567,"record_offset":12680,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":567,"record_offset":11872,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":567,"record_offset":12708,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":11280,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":11280,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":14224,"parent_record_offset":14200,"ancestor_record_offset":11280,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":12272,"parent_record_offset":12240,"ancestor_record_offset":11280,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":14200,"parent_record_offset":14144,"ancestor_record_offset":11280,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12876,"parent_record_offset":12852,"ancestor_record_offset":11280,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":11280,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":11280,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":11280,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":14172,"parent_record_offset":14144,"ancestor_record_offset":11280,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":14144,"parent_record_offset":14120,"ancestor_record_offset":11280,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":15472,"parent_record_offset":15440,"ancestor_record_offset":11280,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":11280,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12900,"parent_record_offset":12876,"ancestor_record_offset":11280,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":15440,"parent_record_offset":15412,"ancestor_record_offset":11280,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12240,"parent_record_offset":12208,"ancestor_record_offset":11280,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":12208,"parent_record_offset":12180,"ancestor_record_offset":11280,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":11280,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":13168,"parent_record_offset":13124,"ancestor_record_offset":11280,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":12996,"parent_record_offset":12968,"ancestor_record_offset":11280,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":12968,"parent_record_offset":12792,"ancestor_record_offset":11280,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":16000,"parent_record_offset":15964,"ancestor_record_offset":11280,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15784,"parent_record_offset":15340,"ancestor_record_offset":11280,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":11280,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12828,"parent_record_offset":12792,"ancestor_record_offset":11280,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15340,"parent_record_offset":15308,"ancestor_record_offset":11280,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12372,"parent_record_offset":12116,"ancestor_record_offset":11280,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":16244,"parent_record_offset":16000,"ancestor_record_offset":11280,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15384,"parent_record_offset":15340,"ancestor_record_offset":11280,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":17920,"parent_record_offset":17872,"ancestor_record_offset":11280,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12152,"parent_record_offset":12116,"ancestor_record_offset":11280,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12764,"parent_record_offset":12708,"ancestor_record_offset":11280,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":17792,"parent_record_offset":17752,"ancestor_record_offset":11280,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":15676,"parent_record_offset":15648,"ancestor_record_offset":11280,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":11280,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":15740,"parent_record_offset":15676,"ancestor_record_offset":11280,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":15896,"parent_record_offset":15340,"ancestor_record_offset":11280,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":15308,"parent_record_offset":15276,"ancestor_record_offset":11280,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":11280,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":567,"record_offset":12116,"parent_record_offset":12084,"ancestor_record_offset":11280,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":567,"record_offset":17832,"parent_record_offset":17792,"ancestor_record_offset":11280,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":567,"record_offset":17872,"parent_record_offset":17832,"ancestor_record_offset":11280,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":11280,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":11280,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":567,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":11280,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":567,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":11280,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":567,"record_offset":12484,"parent_record_offset":12448,"ancestor_record_offset":11280,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":12024,"parent_record_offset":11872,"ancestor_record_offset":11280,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":567,"record_offset":12052,"parent_record_offset":11872,"ancestor_record_offset":11280,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":11280,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":567,"record_offset":12516,"parent_record_offset":12448,"ancestor_record_offset":11280,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":567,"record_offset":12448,"parent_record_offset":12416,"ancestor_record_offset":11280,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":567,"record_offset":12416,"parent_record_offset":12372,"ancestor_record_offset":11280,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::repeat_n::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\repeat_n.rs
//     evidence={"module":567,"record_offset":15608,"parent_record_offset":15568,"ancestor_record_offset":11280,"inlinee":32446,"source_line":140,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":15568,"parent_record_offset":15540,"ancestor_record_offset":11280,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":15540,"parent_record_offset":15340,"ancestor_record_offset":11280,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":15648,"parent_record_offset":15568,"ancestor_record_offset":11280,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":11280,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":11280,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":11280,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":15004,"parent_record_offset":14972,"ancestor_record_offset":11280,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":11280,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":11744,"parent_record_offset":11716,"ancestor_record_offset":11280,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":567,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":11280,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":567,"record_offset":11912,"parent_record_offset":11872,"ancestor_record_offset":11280,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":13408,"parent_record_offset":12996,"ancestor_record_offset":11280,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":11280,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":14072,"parent_record_offset":12708,"ancestor_record_offset":11280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":12612,"parent_record_offset":12372,"ancestor_record_offset":11280,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":16348,"parent_record_offset":16324,"ancestor_record_offset":11280,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":11280,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":12580,"parent_record_offset":12372,"ancestor_record_offset":11280,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":567,"record_offset":16452,"parent_record_offset":16424,"ancestor_record_offset":11280,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":11616,"parent_record_offset":11584,"ancestor_record_offset":11280,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":11648,"parent_record_offset":11616,"ancestor_record_offset":11280,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":11280,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":15208,"parent_record_offset":15172,"ancestor_record_offset":11280,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":11280,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":567,"record_offset":16276,"parent_record_offset":15964,"ancestor_record_offset":11280,"inlinee":6415,"source_line":137,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":16300,"parent_record_offset":16276,"ancestor_record_offset":11280,"inlinee":6416,"source_line":88,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":11772,"parent_record_offset":11716,"ancestor_record_offset":11280,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":11280,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":567,"record_offset":11804,"parent_record_offset":11772,"ancestor_record_offset":11280,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BA7
// 首条位置: {"module":554,"inlinee":31655}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":53252,"parent_record_offset":53172,"ancestor_record_offset":53172,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::end_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":554,"record_offset":53196,"parent_record_offset":53172,"ancestor_record_offset":53172,"inlinee":27911,"source_line":1074,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":53220,"parent_record_offset":53196,"ancestor_record_offset":53172,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B44
// 首条位置: {"module":551,"inlinee":31556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":49228,"parent_record_offset":49144,"ancestor_record_offset":49144,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::start_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":49168,"parent_record_offset":49144,"ancestor_record_offset":49144,"inlinee":31557,"source_line":1070,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":49196,"parent_record_offset":49168,"ancestor_record_offset":49144,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::try_add_raw_input_points
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":21108,"rva":11353936,"va":5380063056}
// 编译实例 RVAs: 0xad3f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4981700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e83cfcb3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380063183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4981965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e833fbb3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380063464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10037473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e8e1289900","size":5,"soft_fail":false,"target_va":5390100576,"va":5380063098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31783733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e835fbe401","size":5,"soft_fail":false,"target_va":5411847168,"va":5380063430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31283514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e83a59dd01","size":5,"soft_fail":false,"target_va":5411347040,"va":5380063521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31477409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380063056,"raw_hex":"e8a14ee001","size":5,"soft_fail":false,"target_va":5411540963,"va":5380063549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":21320,"parent_record_offset":21108,"ancestor_record_offset":21108,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":21952,"parent_record_offset":21108,"ancestor_record_offset":21108,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":567,"record_offset":21288,"parent_record_offset":21108,"ancestor_record_offset":21108,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":21596,"parent_record_offset":21572,"ancestor_record_offset":21108,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":21620,"parent_record_offset":21596,"ancestor_record_offset":21108,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":22744,"parent_record_offset":22720,"ancestor_record_offset":21108,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":21572,"parent_record_offset":21480,"ancestor_record_offset":21108,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":22720,"parent_record_offset":22632,"ancestor_record_offset":21108,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22268,"parent_record_offset":22240,"ancestor_record_offset":21108,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":21108,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22364,"parent_record_offset":22336,"ancestor_record_offset":21108,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22660,"parent_record_offset":22632,"ancestor_record_offset":21108,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":21108,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22336,"parent_record_offset":22300,"ancestor_record_offset":21108,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22300,"parent_record_offset":22268,"ancestor_record_offset":21108,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":21480,"parent_record_offset":21448,"ancestor_record_offset":21108,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":21448,"parent_record_offset":21420,"ancestor_record_offset":21108,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":21108,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":21516,"parent_record_offset":21480,"ancestor_record_offset":21108,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":22196,"parent_record_offset":21984,"ancestor_record_offset":21108,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":21672,"parent_record_offset":21352,"ancestor_record_offset":21108,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":22456,"parent_record_offset":22196,"ancestor_record_offset":21108,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":22240,"parent_record_offset":22196,"ancestor_record_offset":21108,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":21392,"parent_record_offset":21352,"ancestor_record_offset":21108,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":567,"record_offset":22028,"parent_record_offset":21984,"ancestor_record_offset":21108,"inlinee":10233,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":567,"record_offset":22512,"parent_record_offset":22488,"ancestor_record_offset":21108,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":567,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":21108,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":567,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":21108,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":21984,"parent_record_offset":21952,"ancestor_record_offset":21108,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":567,"record_offset":21352,"parent_record_offset":21320,"ancestor_record_offset":21108,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":21108,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":567,"record_offset":21808,"parent_record_offset":21768,"ancestor_record_offset":21108,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":567,"record_offset":21768,"parent_record_offset":21728,"ancestor_record_offset":21108,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":567,"record_offset":21728,"parent_record_offset":21672,"ancestor_record_offset":21108,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":567,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":21108,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":22688,"parent_record_offset":22660,"ancestor_record_offset":21108,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":21108,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":22488,"parent_record_offset":21984,"ancestor_record_offset":21108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":21884,"parent_record_offset":21672,"ancestor_record_offset":21108,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":22124,"parent_record_offset":22100,"ancestor_record_offset":21108,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":22100,"parent_record_offset":22076,"ancestor_record_offset":21108,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":21852,"parent_record_offset":21672,"ancestor_record_offset":21108,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EE4
// 首条位置: {"module":578,"inlinee":32484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":147668,"parent_record_offset":147644,"ancestor_record_offset":147644,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BAC
// 首条位置: {"module":554,"inlinee":31660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":54888,"parent_record_offset":54856,"ancestor_record_offset":54856,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":54916,"parent_record_offset":54856,"ancestor_record_offset":54856,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":54940,"parent_record_offset":54916,"ancestor_record_offset":54856,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BB0
// 首条位置: {"module":554,"inlinee":31664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":554,"record_offset":62656,"parent_record_offset":62632,"ancestor_record_offset":62632,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00554.jsonl.gz"}
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
// 完整结构及原始类型记录: in_progress_curve.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::CurveSimplificationOptions
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::InProgressCurve
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::in_progress_curve::InProgressCurve>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::in_progress_curve::InProgressCurve"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: in_progress_curve.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: in_progress_curve.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>", "bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 信息 ID: 2b3df436464a93514339a6de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=54856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 信息 ID: f14626effc21dfdedb7403cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7e5c8 -> 0xad41a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7e771 -> 0xad41a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 信息 ID: 3e9601684e6e29de71c4ba3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=551, inline_record=49144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 信息 ID: f2d54e847ff0a10b7ecde331
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=62632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 信息 ID: 7ebdbd3064469894eca316ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=53172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=57312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::new
// 信息 ID: d3d6a4ce74609a18870978d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7d951 -> 0xad3070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0xaaf400 -> 0xad3070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 信息 ID: 596726b40003d25a714e6d7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=578, inline_record=147644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::try_add_raw_input_points
// 信息 ID: aff15ddc5c131ccbb5bce1a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7dcf8 -> 0xad3f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7df05 -> 0xad3f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7d4aa -> 0xad3f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::impl$3::clone
// 信息 ID: 12f8de09f536c0afb6a0c4d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=48980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
