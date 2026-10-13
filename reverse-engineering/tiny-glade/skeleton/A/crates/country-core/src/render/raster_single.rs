// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/raster_single.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 04897a09681f731630af6693a90eefd1a2c0cea21947fd0266dc7a04e648b501
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: raster_single.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 487
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":14976,"rva":6948240,"va":5375657360}
// 编译实例 RVAs: 0x6a0590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3033435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8a5b6d1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375658454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2909997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8d398d3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375659704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1442113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8bffee9ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375658908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406441b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-379222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8aa36faff","size":5,"soft_fail":false,"target_va":5375279536,"va":5375658753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140644950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-378297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8473afaff","size":5,"soft_fail":false,"target_va":5375281488,"va":5375659780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t301033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8e9970400","size":5,"soft_fail":false,"target_va":5375963408,"va":5375662370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t307144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8c8af0400","size":5,"soft_fail":false,"target_va":5375967552,"va":5375660403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t564522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e82a9d0800","size":5,"soft_fail":false,"target_va":5376222944,"va":5375658417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7650255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8cfbb7400","size":5,"soft_fail":false,"target_va":5383308144,"va":5375657884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7689606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e886557500","size":5,"soft_fail":false,"target_va":5383348608,"va":5375658997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7690455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8d7587500","size":5,"soft_fail":false,"target_va":5383349424,"va":5375658964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7691348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8545c7500","size":5,"soft_fail":false,"target_va":5383350624,"va":5375659271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfe090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7721464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8f8d17500","size":5,"soft_fail":false,"target_va":5383381136,"va":5375659667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27934655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8bf3faa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375658700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1870440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8688a1c00","size":5,"soft_fail":false,"target_va":5377529888,"va":5375659443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27318369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e861d8a001","size":5,"soft_fail":false,"target_va":5402978688,"va":5375660314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8b6e5ffff","size":5,"soft_fail":false,"target_va":5375653520,"va":5375660245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27322180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e844e7a001","size":5,"soft_fail":false,"target_va":5402980128,"va":5375657943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e843f1ffff","size":5,"soft_fail":false,"target_va":5375653712,"va":5375657480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27323087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8cfeaa001","size":5,"soft_fail":false,"target_va":5402980656,"va":5375657564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t44254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8deac0000","size":5,"soft_fail":false,"target_va":5375704400,"va":5375660141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e822a40000","size":5,"soft_fail":false,"target_va":5375700688,"va":5375658665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-262767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e891fdfbff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375658298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-261984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8a000fcff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375660219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t698776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e898a90a00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375658131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t702698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8eab80a00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375657857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t698692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e844a90a00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375657447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t683555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8236e0a00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375658200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1707794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e8120f1a00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375658425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1286935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657360,"raw_hex":"e817a31300","size":5,"soft_fail":false,"target_va":5376946416,"va":5375659476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":20688,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":18164,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":17980,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::final_post::final_post::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/final_post.rs
//     evidence={"module":487,"record_offset":18696,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":27429,"source_line":99,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":16912,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":15324,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":15900,"parent_record_offset":14976,"ancestor_record_offset":14976,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":14976,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":19364,"parent_record_offset":19308,"ancestor_record_offset":14976,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":487,"record_offset":19060,"parent_record_offset":19036,"ancestor_record_offset":14976,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":487,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":14976,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":19336,"parent_record_offset":19308,"ancestor_record_offset":14976,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":14976,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":14976,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":18524,"parent_record_offset":18496,"ancestor_record_offset":14976,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":14976,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17360,"parent_record_offset":17332,"ancestor_record_offset":14976,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17272,"parent_record_offset":17240,"ancestor_record_offset":14976,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17304,"parent_record_offset":17272,"ancestor_record_offset":14976,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17476,"parent_record_offset":17304,"ancestor_record_offset":14976,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":17588,"parent_record_offset":17564,"ancestor_record_offset":14976,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":18012,"parent_record_offset":17980,"ancestor_record_offset":14976,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":487,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":14976,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":487,"record_offset":15536,"parent_record_offset":15440,"ancestor_record_offset":14976,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":18980,"parent_record_offset":18696,"ancestor_record_offset":14976,"inlinee":9308,"source_line":1257,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":17164,"parent_record_offset":17136,"ancestor_record_offset":14976,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":16496,"parent_record_offset":16036,"ancestor_record_offset":14976,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":14976,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":17212,"parent_record_offset":17032,"ancestor_record_offset":14976,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":487,"record_offset":15848,"parent_record_offset":15820,"ancestor_record_offset":14976,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":17540,"parent_record_offset":17240,"ancestor_record_offset":14976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":14976,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":14976,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":16456,"parent_record_offset":16424,"ancestor_record_offset":14976,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":14976,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":487,"record_offset":17940,"parent_record_offset":16992,"ancestor_record_offset":14976,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":15472,"parent_record_offset":15440,"ancestor_record_offset":14976,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":15500,"parent_record_offset":15472,"ancestor_record_offset":14976,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":14976,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":16552,"parent_record_offset":16528,"ancestor_record_offset":14976,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":16200,"parent_record_offset":16092,"ancestor_record_offset":14976,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":487,"record_offset":15440,"parent_record_offset":15412,"ancestor_record_offset":14976,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":487,"record_offset":15932,"parent_record_offset":15900,"ancestor_record_offset":14976,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":487,"record_offset":16064,"parent_record_offset":16036,"ancestor_record_offset":14976,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":487,"record_offset":16036,"parent_record_offset":16004,"ancestor_record_offset":14976,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":14976,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":14976,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":14976,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":16528,"parent_record_offset":16004,"ancestor_record_offset":14976,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15636,"parent_record_offset":15604,"ancestor_record_offset":14976,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15796,"parent_record_offset":15636,"ancestor_record_offset":14976,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15604,"parent_record_offset":15580,"ancestor_record_offset":14976,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15356,"parent_record_offset":15324,"ancestor_record_offset":14976,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":14976,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15412,"parent_record_offset":15380,"ancestor_record_offset":14976,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":14976,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":16092,"parent_record_offset":16064,"ancestor_record_offset":14976,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":16340,"parent_record_offset":16092,"ancestor_record_offset":14976,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":16312,"parent_record_offset":16092,"ancestor_record_offset":14976,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":16144,"parent_record_offset":16092,"ancestor_record_offset":14976,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":18848,"parent_record_offset":18696,"ancestor_record_offset":14976,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":18204,"parent_record_offset":18164,"ancestor_record_offset":14976,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":18088,"parent_record_offset":18060,"ancestor_record_offset":14976,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":18060,"parent_record_offset":18036,"ancestor_record_offset":14976,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":14976,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":487,"record_offset":19036,"parent_record_offset":18696,"ancestor_record_offset":14976,"inlinee":22121,"source_line":81,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":16952,"parent_record_offset":16912,"ancestor_record_offset":14976,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":17076,"parent_record_offset":17032,"ancestor_record_offset":14976,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":17240,"parent_record_offset":17032,"ancestor_record_offset":14976,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":17512,"parent_record_offset":17240,"ancestor_record_offset":14976,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":18116,"parent_record_offset":18088,"ancestor_record_offset":14976,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":487,"record_offset":18232,"parent_record_offset":18204,"ancestor_record_offset":14976,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":18288,"parent_record_offset":18260,"ancestor_record_offset":14976,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":18376,"parent_record_offset":18344,"ancestor_record_offset":14976,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":18496,"parent_record_offset":18344,"ancestor_record_offset":14976,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":18260,"parent_record_offset":18232,"ancestor_record_offset":14976,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":17104,"parent_record_offset":17076,"ancestor_record_offset":14976,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":487,"record_offset":17136,"parent_record_offset":17104,"ancestor_record_offset":14976,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":17032,"parent_record_offset":16992,"ancestor_record_offset":14976,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":16992,"parent_record_offset":16952,"ancestor_record_offset":14976,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":21140,"rva":6953648,"va":5375662768}
// 编译实例 RVAs: 0x6a1ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3038650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e846a2d1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375663669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1447172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8fceae9ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375663967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406441b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-384521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8f721faff","size":5,"soft_fail":false,"target_va":5375279536,"va":5375664052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t298802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8328f0400","size":5,"soft_fail":false,"target_va":5375963408,"va":5375664601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t303105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e801a00400","size":5,"soft_fail":false,"target_va":5375967552,"va":5375664442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t559306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8ca880800","size":5,"soft_fail":false,"target_va":5376222944,"va":5375663633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7644934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e806a77400","size":5,"soft_fail":false,"target_va":5383308144,"va":5375663205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27929448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8682baa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375663907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27314358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8b6c8a001","size":5,"soft_fail":false,"target_va":5402978688,"va":5375664325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8e7d5ffff","size":5,"soft_fail":false,"target_va":5375653520,"va":5375664292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27316866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e882d2a001","size":5,"soft_fail":false,"target_va":5402980128,"va":5375663257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e855dcffff","size":5,"soft_fail":false,"target_va":5375653712,"va":5375662838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27317736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8e8d5a001","size":5,"soft_fail":false,"target_va":5402980656,"va":5375662915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t40162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8e29c0000","size":5,"soft_fail":false,"target_va":5375704400,"va":5375664233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8c48f0000","size":5,"soft_fail":false,"target_va":5375700688,"va":5375663879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-268053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8ebe8fbff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375663584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-266038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8caf0fbff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375664273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t693483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8eb940a00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375663424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t697370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e81aa40a00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375663185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t693321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e849940a00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375662818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t678262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e876590a00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375663493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1702579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375662768,"raw_hex":"e8b3fa1900","size":5,"soft_fail":false,"target_va":5377366224,"va":5375663640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":24284,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":24100,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
//     evidence={"module":487,"record_offset":24816,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":27431,"source_line":227,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":23068,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":21480,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":22056,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23452,"parent_record_offset":23424,"ancestor_record_offset":21140,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":24644,"parent_record_offset":24616,"ancestor_record_offset":21140,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23504,"parent_record_offset":23480,"ancestor_record_offset":21140,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23480,"parent_record_offset":23452,"ancestor_record_offset":21140,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23392,"parent_record_offset":23364,"ancestor_record_offset":21140,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23424,"parent_record_offset":23392,"ancestor_record_offset":21140,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23596,"parent_record_offset":23424,"ancestor_record_offset":21140,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":23708,"parent_record_offset":23684,"ancestor_record_offset":21140,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":24132,"parent_record_offset":24100,"ancestor_record_offset":21140,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":487,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":21140,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":487,"record_offset":21692,"parent_record_offset":21596,"ancestor_record_offset":21140,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":23288,"parent_record_offset":23260,"ancestor_record_offset":21140,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":22652,"parent_record_offset":22192,"ancestor_record_offset":21140,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":23336,"parent_record_offset":23164,"ancestor_record_offset":21140,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":487,"record_offset":22004,"parent_record_offset":21976,"ancestor_record_offset":21140,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":23660,"parent_record_offset":23364,"ancestor_record_offset":21140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":22428,"parent_record_offset":22404,"ancestor_record_offset":21140,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":22404,"parent_record_offset":22380,"ancestor_record_offset":21140,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":22612,"parent_record_offset":22580,"ancestor_record_offset":21140,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":23552,"parent_record_offset":23528,"ancestor_record_offset":21140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":487,"record_offset":24060,"parent_record_offset":23132,"ancestor_record_offset":21140,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":21628,"parent_record_offset":21596,"ancestor_record_offset":21140,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":21656,"parent_record_offset":21628,"ancestor_record_offset":21140,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":22380,"parent_record_offset":22356,"ancestor_record_offset":21140,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":22708,"parent_record_offset":22684,"ancestor_record_offset":21140,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":22356,"parent_record_offset":22248,"ancestor_record_offset":21140,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":487,"record_offset":21596,"parent_record_offset":21568,"ancestor_record_offset":21140,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":487,"record_offset":22088,"parent_record_offset":22056,"ancestor_record_offset":21140,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":487,"record_offset":22220,"parent_record_offset":22192,"ancestor_record_offset":21140,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":487,"record_offset":22192,"parent_record_offset":22160,"ancestor_record_offset":21140,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":22136,"parent_record_offset":22112,"ancestor_record_offset":21140,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":22160,"parent_record_offset":22136,"ancestor_record_offset":21140,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":22112,"parent_record_offset":22088,"ancestor_record_offset":21140,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":487,"record_offset":22684,"parent_record_offset":22160,"ancestor_record_offset":21140,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21792,"parent_record_offset":21760,"ancestor_record_offset":21140,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21952,"parent_record_offset":21792,"ancestor_record_offset":21140,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":21140,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21512,"parent_record_offset":21480,"ancestor_record_offset":21140,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21976,"parent_record_offset":21952,"ancestor_record_offset":21140,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21568,"parent_record_offset":21536,"ancestor_record_offset":21140,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":21140,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":22248,"parent_record_offset":22220,"ancestor_record_offset":21140,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":22496,"parent_record_offset":22248,"ancestor_record_offset":21140,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":22468,"parent_record_offset":22248,"ancestor_record_offset":21140,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":487,"record_offset":22300,"parent_record_offset":22248,"ancestor_record_offset":21140,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":24868,"parent_record_offset":24816,"ancestor_record_offset":21140,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":24324,"parent_record_offset":24284,"ancestor_record_offset":21140,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":24208,"parent_record_offset":24180,"ancestor_record_offset":21140,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":21140,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":21140,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":21140,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":23100,"parent_record_offset":23068,"ancestor_record_offset":21140,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":23200,"parent_record_offset":23164,"ancestor_record_offset":21140,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":23364,"parent_record_offset":23164,"ancestor_record_offset":21140,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":487,"record_offset":23632,"parent_record_offset":23364,"ancestor_record_offset":21140,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":25044,"parent_record_offset":24816,"ancestor_record_offset":21140,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":25068,"parent_record_offset":25044,"ancestor_record_offset":21140,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":24960,"parent_record_offset":24816,"ancestor_record_offset":21140,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":24984,"parent_record_offset":24960,"ancestor_record_offset":21140,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":24236,"parent_record_offset":24208,"ancestor_record_offset":21140,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":487,"record_offset":24352,"parent_record_offset":24324,"ancestor_record_offset":21140,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":21140,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":24496,"parent_record_offset":24464,"ancestor_record_offset":21140,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":24616,"parent_record_offset":24464,"ancestor_record_offset":21140,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":24380,"parent_record_offset":24352,"ancestor_record_offset":21140,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":23228,"parent_record_offset":23200,"ancestor_record_offset":21140,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":487,"record_offset":23260,"parent_record_offset":23228,"ancestor_record_offset":21140,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":23164,"parent_record_offset":23132,"ancestor_record_offset":21140,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":23132,"parent_record_offset":23100,"ancestor_record_offset":21140,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: raster_single.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: raster_single.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: raster_single.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1>
// 信息 ID: 3134275d906fb3841fff78dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::render::pass_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>", "1"]; 字段布局 = unknown。
// [类型关系] country_core::render::raster_single::raster_single_object -> ["1", "array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>", "country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::final_post::final_post @ 0x554a31 -> 0x6a0590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>
// 信息 ID: 1d419f34294234fcadc513ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::render::pass_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>", "1"]; 字段布局 = unknown。
// [类型关系] country_core::render::raster_single::raster_single_object -> ["1", "array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>", "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass @ 0x688ca8 -> 0x6a1ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
