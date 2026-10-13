// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 53ac0ab41c4400a9c0a0cf08291dc833842955622a1237ab2fa195d717c177d3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bin_lights.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 534
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
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":534,"record_offset":45820,"rva":10385264,"va":5379094384}
// 编译实例 RVAs: 0x9e7770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9478348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8345f6fff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379094455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4014417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8afbec2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379095900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4014443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e895bec2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379095942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1381592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e828ebeaff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379097731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-516343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8091ff8ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379094546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-508954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8e63bf8ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5379094917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8f559ffff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379095254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6d00
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2028626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e852f41e00","size":5,"soft_fail":false,"target_va":5381123328,"va":5379094697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2075072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8c0a91f00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379095275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fe130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12675080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e80868c100","size":5,"soft_fail":false,"target_va":5391769904,"va":5379094819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12695315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e813b7c100","size":5,"soft_fail":false,"target_va":5391790752,"va":5379095432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160d680
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12737856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8405dc200","size":5,"soft_fail":false,"target_va":5391832704,"va":5379094843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32752537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e899c3f301","size":5,"soft_fail":false,"target_va":5411847152,"va":5379094610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32752225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e861c2f301","size":5,"soft_fail":false,"target_va":5411847168,"va":5379094938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32443272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8880bef01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379097686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32444119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8d70eef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379097652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32384788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e81427ee01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379097640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-500893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8635bf8ff","size":5,"soft_fail":false,"target_va":5378595520,"va":5379096408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-498684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e80464f8ff","size":5,"soft_fail":false,"target_va":5378596192,"va":5379094871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::clear_rg_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-40271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8b162ffff","size":5,"soft_fail":false,"target_va":5379054512,"va":5379094778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2020748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e88cd51e00","size":5,"soft_fail":false,"target_va":5381115424,"va":5379094671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2104314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379094384,"raw_hex":"e8fa1b2000","size":5,"soft_fail":false,"target_va":5381199056,"va":5379094737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379094384,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379094630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":47084,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":534,"record_offset":46844,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":534,"record_offset":47408,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":26675,"source_line":448,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":534,"record_offset":48208,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":534,"record_offset":47332,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":534,"record_offset":47120,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":534,"record_offset":46008,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":534,"record_offset":47204,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":48520,"parent_record_offset":48496,"ancestor_record_offset":45820,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":48544,"parent_record_offset":48520,"ancestor_record_offset":45820,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":48820,"parent_record_offset":48796,"ancestor_record_offset":45820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":48496,"parent_record_offset":48468,"ancestor_record_offset":45820,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":48796,"parent_record_offset":48740,"ancestor_record_offset":45820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":46460,"parent_record_offset":46436,"ancestor_record_offset":45820,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48408,"parent_record_offset":48380,"ancestor_record_offset":45820,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48768,"parent_record_offset":48740,"ancestor_record_offset":45820,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48740,"parent_record_offset":48716,"ancestor_record_offset":45820,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":46508,"parent_record_offset":46484,"ancestor_record_offset":45820,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":46484,"parent_record_offset":46460,"ancestor_record_offset":45820,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48468,"parent_record_offset":48436,"ancestor_record_offset":45820,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48436,"parent_record_offset":48408,"ancestor_record_offset":45820,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":48716,"parent_record_offset":48692,"ancestor_record_offset":45820,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":48344,"parent_record_offset":48316,"ancestor_record_offset":45820,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":48316,"parent_record_offset":48288,"ancestor_record_offset":45820,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":48260,"parent_record_offset":48208,"ancestor_record_offset":45820,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":48288,"parent_record_offset":48260,"ancestor_record_offset":45820,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":46436,"parent_record_offset":46412,"ancestor_record_offset":45820,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":46412,"parent_record_offset":46372,"ancestor_record_offset":45820,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":48380,"parent_record_offset":48344,"ancestor_record_offset":45820,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":46372,"parent_record_offset":46348,"ancestor_record_offset":45820,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":534,"record_offset":46760,"parent_record_offset":46736,"ancestor_record_offset":45820,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":534,"record_offset":46736,"parent_record_offset":46712,"ancestor_record_offset":45820,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":534,"record_offset":46684,"parent_record_offset":46652,"ancestor_record_offset":45820,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":534,"record_offset":46628,"parent_record_offset":46100,"ancestor_record_offset":45820,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":534,"record_offset":46652,"parent_record_offset":46628,"ancestor_record_offset":45820,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":534,"record_offset":46128,"parent_record_offset":46100,"ancestor_record_offset":45820,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":534,"record_offset":46712,"parent_record_offset":46652,"ancestor_record_offset":45820,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":534,"record_offset":46176,"parent_record_offset":46152,"ancestor_record_offset":45820,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":534,"record_offset":46152,"parent_record_offset":46128,"ancestor_record_offset":45820,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":534,"record_offset":46348,"parent_record_offset":46324,"ancestor_record_offset":45820,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":534,"record_offset":46064,"parent_record_offset":46036,"ancestor_record_offset":45820,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":534,"record_offset":46036,"parent_record_offset":46008,"ancestor_record_offset":45820,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":534,"record_offset":46100,"parent_record_offset":46036,"ancestor_record_offset":45820,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":534,"record_offset":46912,"parent_record_offset":46872,"ancestor_record_offset":45820,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":534,"record_offset":47272,"parent_record_offset":47204,"ancestor_record_offset":45820,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":48620,"parent_record_offset":48596,"ancestor_record_offset":45820,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":534,"record_offset":46264,"parent_record_offset":46204,"ancestor_record_offset":45820,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":47376,"parent_record_offset":47332,"ancestor_record_offset":45820,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":46284,"parent_record_offset":46264,"ancestor_record_offset":45820,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":46324,"parent_record_offset":46152,"ancestor_record_offset":45820,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":46812,"parent_record_offset":46008,"ancestor_record_offset":45820,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":48596,"parent_record_offset":48344,"ancestor_record_offset":45820,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":47512,"parent_record_offset":47440,"ancestor_record_offset":45820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":534,"record_offset":46996,"parent_record_offset":46968,"ancestor_record_offset":45820,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":534,"record_offset":47024,"parent_record_offset":46996,"ancestor_record_offset":45820,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":534,"record_offset":46204,"parent_record_offset":46176,"ancestor_record_offset":45820,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":534,"record_offset":46576,"parent_record_offset":46552,"ancestor_record_offset":45820,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":534,"record_offset":46236,"parent_record_offset":46204,"ancestor_record_offset":45820,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":534,"record_offset":46552,"parent_record_offset":46372,"ancestor_record_offset":45820,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":534,"record_offset":46940,"parent_record_offset":46912,"ancestor_record_offset":45820,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":47144,"parent_record_offset":47120,"ancestor_record_offset":45820,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":47168,"parent_record_offset":47144,"ancestor_record_offset":45820,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":534,"record_offset":46872,"parent_record_offset":46844,"ancestor_record_offset":45820,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":534,"record_offset":47440,"parent_record_offset":47408,"ancestor_record_offset":45820,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":534,"record_offset":47244,"parent_record_offset":47204,"ancestor_record_offset":45820,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":534,"record_offset":47300,"parent_record_offset":47204,"ancestor_record_offset":45820,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00534.jsonl.gz"}
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
// 完整结构及原始类型记录: bin_lights.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::bin_lights::BinLights
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["country_core::systems::render_loop::compute_passes::bin_lights::BinLights"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bin_lights.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: bin_lights.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":534,"record_offset":47332,"parent_record_offset":45820,"ancestor_record_offset":45820,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00534.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"f9ed7751baaa8e383afe69d7","relative_path":"_light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin","sha256":"ede0ca4305e9fa777cdc964fcdb792ffd273fe5ace5d8be588b0d43987cd6bab","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"bin_lights","prefix_candidates":["bin_lights","light_binning_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"e6ce409b9ddb9e19248ee284","relative_path":"_light_binning_bin_lights_to_cells.cs.hlsl_cs_main_8faee3cc8c252c04.bin","sha256":"a8b34992bac0f7b649f43057d2115f79980c17f6c2ad122d5c7d2cb426e140cb","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"bin_lights","prefix_candidates":["bin_lights","light_binning_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"2644de89792425cfedf008d1","relative_path":"_light_binning_cell_prefix_merge.cs.hlsl_cs_main_0.bin","sha256":"949303f9a7aab44e7be152df16f43e9ecdc169031199ae529a06ecfe79b9b5d9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"bin_lights","prefix_candidates":["bin_lights","light_binning_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"4ab4007090ee6165399bba2b","relative_path":"_light_binning_cell_prefix_sum.cs.hlsl_cs_main_0.bin","sha256":"864d44b3740e445fcbdd3d148082911b4d7de0c521125db7b4941a54e814e505","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"bin_lights","prefix_candidates":["bin_lights","light_binning_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"7dbd80e472cc0f665cad2b12","relative_path":"_light_binning_cell_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin","sha256":"33dbd1ad39f7ae1df78456277e8ded7f3621c4e42b5b8098536e402d44f6c134","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"bin_lights","prefix_candidates":["bin_lights","light_binning_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _light_binning_cell_prefix_merge.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"__alias_inout_buf","storage_class":"Uniform","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[15]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"subtotals_buf","storage_class":"Uniform","type":{"id":22,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.StructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[15]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,19]},"decorations":[],"pointee":{"id":19,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[17,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _light_binning_cell_prefix_sum.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[512,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"__alias_inout_buf","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.RWByteAddressBuffer","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"__alias_subtotals_buf","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,23]},"decorations":[],"pointee":{"id":23,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[21,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,23]},"decorations":[],"pointee":{"id":23,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[21,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _light_binning_cell_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"push_constants","storage_class":"PushConstant","type":{"id":28,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,11]},"decorations":[],"pointee":{"id":11,"name":"type.ConstantBuffer.PushConstants","record":{"opcode":"OpTypeStruct","operands":[18]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"range_end","type_id":18,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"__alias_inout_buf","storage_class":"Uniform","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.RWByteAddressBuffer","record":{"opcode":"OpTypeStruct","operands":[16]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":16,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,32]},"decorations":[],"pointee":{"id":32,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[18,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_8faee3cc8c252c04.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"lantern_buf","storage_class":"Uniform","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,22]},"decorations":[],"pointee":{"id":22,"name":"type.StructuredBuffer.LanternPacked","record":{"opcode":"OpTypeStruct","operands":[29]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":29,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"$Globals","storage_class":"Uniform","type":{"id":49,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"lantern_count","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"__alias_indirection_grid","storage_class":"Uniform","type":{"id":50,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,27]},"decorations":[],"pointee":{"id":27,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[30]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":51,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"frame_constants","storage_class":"Uniform","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,22]},"decorations":[],"pointee":{"id":22,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[73,73,73,73,73,73,73,73,40,38,38,40,74,40,38,41,38,41,40,41,40,38,40,40,40,73,33]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":73,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":73,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":73,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":73,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":38,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":38,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":74,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":38,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":41,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":38,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":41,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":41,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":38,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":40,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":40,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":73,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":33,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"lantern_buf","storage_class":"Uniform","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.StructuredBuffer.LanternPacked","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"$Globals","storage_class":"Uniform","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,28]},"decorations":[],"pointee":{"id":28,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[40]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"lantern_count","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"__alias_indirection_grid","storage_class":"Uniform","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,30]},"decorations":[],"pointee":{"id":30,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"__alias_light_indices","storage_class":"Uniform","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,30]},"decorations":[],"pointee":{"id":30,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163457,"position_among_mapped_render_targets":45,"mapped_render_target_count":59,"previous_callsite":5163436,"next_callsite":5163493,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bin_lights.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// 信息 ID: 6d28fd6b14995b0d2d006e23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec9c1 -> 0x9e7770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
