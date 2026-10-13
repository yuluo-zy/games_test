// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a6b218e1e94cbb261bb868098786f6e66e85952536ab0c7640aff7c307d089d7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 251
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":16068,"rva":3940080,"va":5372649200}
// 编译实例 RVAs: 0x3c1ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e81495c9ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5372649751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-223460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e81c97fcff","size":5,"soft_fail":false,"target_va":5372426160,"va":5372649615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403c0b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e886ebffff","size":5,"soft_fail":false,"target_va":5372644160,"va":5372649397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e8d1860100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372649658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3634507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e84b753700","size":5,"soft_fail":false,"target_va":5376283792,"va":5372649280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30329726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e87ecbce01","size":5,"soft_fail":false,"target_va":5402979152,"va":5372649421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4704198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e8c6c74700","size":5,"soft_fail":false,"target_va":5377353776,"va":5372649573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3591657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649200,"raw_hex":"e8e9cd3600","size":5,"soft_fail":false,"target_va":5376240912,"va":5372649250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":251,"record_offset":16504,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":16532,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":16364,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":16248,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":16588,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":251,"record_offset":16560,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":251,"record_offset":16420,"parent_record_offset":16068,"ancestor_record_offset":16068,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":16068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":16796,"parent_record_offset":16740,"ancestor_record_offset":16068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":16768,"parent_record_offset":16740,"ancestor_record_offset":16068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":16068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":251,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":16068,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":16068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":16300,"parent_record_offset":16272,"ancestor_record_offset":16068,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":16328,"parent_record_offset":16272,"ancestor_record_offset":16068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":251,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":16068,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":251,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":16068,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":17328,"rva":3940656,"va":5372649776}
// 编译实例 RVAs: 0x3c2130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a6e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2208748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8144cdeff","size":5,"soft_fail":false,"target_va":5370441296,"va":5372650039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cc830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2054719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8c1a5e0ff","size":5,"soft_fail":false,"target_va":5370595376,"va":5372650090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-199222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8caf5fcff","size":5,"soft_fail":false,"target_va":5372451200,"va":5372650417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t99522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8c2840100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372650169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t99483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e89b840100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372650224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072edf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3591146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8eacb3600","size":5,"soft_fail":false,"target_va":5376241136,"va":5372649985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072ef10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3591107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8c3cb3600","size":5,"soft_fail":false,"target_va":5376241424,"va":5372650312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30943240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e80828d801","size":5,"soft_fail":false,"target_va":5403593360,"va":5372650115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30327769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8d9c3ce01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372650370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30328792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e8d8c7ce01","size":5,"soft_fail":false,"target_va":5402979152,"va":5372650355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1560984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e898d11700","size":5,"soft_fail":false,"target_va":5374211136,"va":5372650147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1562918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e826d91700","size":5,"soft_fail":false,"target_va":5374213216,"va":5372650293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3691917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372649776,"raw_hex":"e88d553800","size":5,"soft_fail":false,"target_va":5376341760,"va":5372649838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":17496,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":251,"record_offset":17656,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":251,"record_offset":17608,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":251,"record_offset":18332,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":251,"record_offset":17580,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":8307,"source_line":61,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":251,"record_offset":17524,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":251,"record_offset":17552,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":17948,"parent_record_offset":17924,"ancestor_record_offset":17328,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":17972,"parent_record_offset":17948,"ancestor_record_offset":17328,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":18284,"parent_record_offset":18260,"ancestor_record_offset":17328,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":17328,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":18260,"parent_record_offset":18204,"ancestor_record_offset":17328,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":18232,"parent_record_offset":18204,"ancestor_record_offset":17328,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":17896,"parent_record_offset":17868,"ancestor_record_offset":17328,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":17328,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":18044,"parent_record_offset":17868,"ancestor_record_offset":17328,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":18204,"parent_record_offset":18176,"ancestor_record_offset":17328,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":17804,"parent_record_offset":17772,"ancestor_record_offset":17328,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":17772,"parent_record_offset":17740,"ancestor_record_offset":17328,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":17708,"parent_record_offset":17656,"ancestor_record_offset":17328,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":17740,"parent_record_offset":17708,"ancestor_record_offset":17328,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":17840,"parent_record_offset":17804,"ancestor_record_offset":17328,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":251,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":17328,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":251,"record_offset":18080,"parent_record_offset":17804,"ancestor_record_offset":17328,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":18152,"parent_record_offset":17656,"ancestor_record_offset":17328,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":17328,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":18364,"rva":3941392,"va":5372650512}
// 编译实例 RVAs: 0x3c2410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e4140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3007908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e85c1ad2ff","size":5,"soft_fail":false,"target_va":5369643328,"va":5372651231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t98654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e85e810100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372651053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t498532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e8649b0700","size":5,"soft_fail":false,"target_va":5373149840,"va":5372651303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2519967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e89f732600","size":5,"soft_fail":false,"target_va":5375171056,"va":5372651084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4879026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e8b2724a00","size":5,"soft_fail":false,"target_va":5377529888,"va":5372650857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30294921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e88943ce01","size":5,"soft_fail":false,"target_va":5402945568,"va":5372650642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hc7947b08ac6d0c76 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30297610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e80a4ece01","size":5,"soft_fail":false,"target_va":5402949248,"va":5372651633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t30294905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e87943ce01","size":5,"soft_fail":false,"target_va":5402945776,"va":5372650866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30298764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e88c52ce01","size":5,"soft_fail":false,"target_va":5402949904,"va":5372651135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30327087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e82fc1ce01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372651788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$image..error..ImageError$GT$::ha2b3edfb87e784f9 (.llvm.3360697089587412842)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1115357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e823fbeeff","size":5,"soft_fail":false,"target_va":5371536656,"va":5372652008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30328586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e80ac7ce01","size":5,"soft_fail":false,"target_va":5402980304,"va":5372651713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30328561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e8f1c6ce01","size":5,"soft_fail":false,"target_va":5402980320,"va":5372651754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::free_functions::save_buffer_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3404092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e83cf13300","size":5,"soft_fail":false,"target_va":5376055456,"va":5372651359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4744050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e872634800","size":5,"soft_fail":false,"target_va":5377395552,"va":5372651497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4757625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e879984800","size":5,"soft_fail":false,"target_va":5377408752,"va":5372651122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4789199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e8cf134900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372650572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4785847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372650512,"raw_hex":"e8b7064900","size":5,"soft_fail":false,"target_va":5377436944,"va":5372651092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":19068,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":20580,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":18720,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":18624,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":251,"record_offset":19920,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":18596,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":21136,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":19236,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":19760,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":251,"record_offset":18652,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20004,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":7581,"source_line":674,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::save
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20324,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":9745,"source_line":996,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":19676,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":18568,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":19536,"parent_record_offset":18364,"ancestor_record_offset":18364,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":19476,"parent_record_offset":19452,"ancestor_record_offset":18364,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":19452,"parent_record_offset":19396,"ancestor_record_offset":18364,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":18364,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":19424,"parent_record_offset":19396,"ancestor_record_offset":18364,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":19396,"parent_record_offset":19368,"ancestor_record_offset":18364,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":19944,"parent_record_offset":19920,"ancestor_record_offset":18364,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":251,"record_offset":20652,"parent_record_offset":20616,"ancestor_record_offset":18364,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":20616,"parent_record_offset":20580,"ancestor_record_offset":18364,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":251,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":18364,"inlinee":13703,"source_line":208,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":20376,"parent_record_offset":20352,"ancestor_record_offset":18364,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":251,"record_offset":19968,"parent_record_offset":19944,"ancestor_record_offset":18364,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":251,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":18364,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":251,"record_offset":20152,"parent_record_offset":20128,"ancestor_record_offset":18364,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":251,"record_offset":20796,"parent_record_offset":20768,"ancestor_record_offset":18364,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":20080,"parent_record_offset":20052,"ancestor_record_offset":18364,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":19092,"parent_record_offset":19068,"ancestor_record_offset":18364,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":251,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":18364,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":19260,"parent_record_offset":19236,"ancestor_record_offset":18364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":18364,"inlinee":13704,"source_line":1515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":18792,"parent_record_offset":18768,"ancestor_record_offset":18364,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$1::copied::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":18816,"parent_record_offset":18792,"ancestor_record_offset":18364,"inlinee":13706,"source_line":1519,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":251,"record_offset":19856,"parent_record_offset":19816,"ancestor_record_offset":18364,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":251,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":18364,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":18364,"inlinee":7582,"source_line":802,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20052,"parent_record_offset":20028,"ancestor_record_offset":18364,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":18364,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":251,"record_offset":20220,"parent_record_offset":20196,"ancestor_record_offset":18364,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":251,"record_offset":20352,"parent_record_offset":20324,"ancestor_record_offset":18364,"inlinee":7594,"source_line":1111,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":18364,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":18364,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":18364,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":19728,"parent_record_offset":19676,"ancestor_record_offset":18364,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":18364,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":19560,"parent_record_offset":19536,"ancestor_record_offset":18364,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":18364,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":18364,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17CC
// 首条位置: {"module":21,"inlinee":6092}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":21,"record_offset":8400,"parent_record_offset":8332,"ancestor_record_offset":8332,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17CB
// 首条位置: {"module":21,"inlinee":6091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
//     evidence={"module":21,"record_offset":8332,"parent_record_offset":8264,"ancestor_record_offset":8264,"inlinee":6092,"source_line":127,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":21,"record_offset":8400,"parent_record_offset":8332,"ancestor_record_offset":8264,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00021.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::panic_reporter::panic_screenshot::GraphicsResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>", "constructor": "core::option::Option", "arguments": ["tiny_glade::panic_reporter::panic_screenshot::GraphicsResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_screenshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 信息 ID: 7196f19eca6a419bb120f340
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fffb2 -> 0x3c2130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 信息 ID: 45a7565dfb18f764da4214d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cdcf7 -> 0x3c1ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 信息 ID: 6b3f27e9f0894a8ad76aad66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278ce1 -> 0x3c2410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 信息 ID: 469b14fa934469b278729512
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_screenshot::dump_screenshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<array$<u8,3>,2>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=21, inline_record=8264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 信息 ID: ea36a0b0722e54149492bdfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=21, inline_record=8332; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=21, inline_record=8508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
