// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/raster_single.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 04897a09681f731630af6693a90eefd1a2c0cea21947fd0266dc7a04e648b501
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: raster_single.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 531
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":531,"record_offset":43856,"rva":10282880,"va":5378992000}
// 编译实例 RVAs: 0x9ce780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9392161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8dfaf70ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378992956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3913250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8de49c4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378994749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1301459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e82d24ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378998158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=22
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1294541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8333fecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378995848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-918646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e88afbf1ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378993265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fd6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-859100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e824e4f2ff","size":5,"soft_fail":false,"target_va":5378135744,"va":5378994839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8f898f3ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378993427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-407540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e80cc8f9ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5378993503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-406475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e835ccf9ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5378993654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2194463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e81f7c2100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378992940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12954745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e879acc500","size":5,"soft_fail":false,"target_va":5391949088,"va":5378994338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162f890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12977772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e86c06c600","size":5,"soft_fail":false,"target_va":5391972496,"va":5378994719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32677322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8ca9df201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378992641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32854235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8db50f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378992928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3845697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e841ae3a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5378994506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32546075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e81b9df001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378995504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e805f0ffff","size":5,"soft_fail":false,"target_va":5378988032,"va":5378992118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32551160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8f8b0f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5378992195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-821696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e84076f3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378995275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-823358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8c26ff3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378993177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1470876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8648ee9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378992903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1470763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8d58ee9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378995350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2392809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8e9822400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378992722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2398034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e852972400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378992601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2391027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8f37b2400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378995448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2392635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e83b822400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378992064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2125034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8ea6c2000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378992801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9829520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e890fc9500","size":5,"soft_fail":false,"target_va":5388825296,"va":5378995771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32541553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e8718bf001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378995546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2939920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378992000,"raw_hex":"e810dc2c00","size":5,"soft_fail":false,"target_va":5381934464,"va":5378994539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":50392,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":46852,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":46636,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::final_post::final_post::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/final_post.rs
//     evidence={"module":531,"record_offset":47324,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":30353,"source_line":120,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":45628,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":44204,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":44652,"parent_record_offset":43856,"ancestor_record_offset":43856,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":531,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":43856,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":531,"record_offset":48816,"parent_record_offset":48760,"ancestor_record_offset":43856,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":531,"record_offset":48488,"parent_record_offset":48464,"ancestor_record_offset":43856,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":531,"record_offset":48536,"parent_record_offset":48512,"ancestor_record_offset":43856,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":48788,"parent_record_offset":48760,"ancestor_record_offset":43856,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":48760,"parent_record_offset":48736,"ancestor_record_offset":43856,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":48736,"parent_record_offset":48712,"ancestor_record_offset":43856,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":46016,"parent_record_offset":45988,"ancestor_record_offset":43856,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":47152,"parent_record_offset":47124,"ancestor_record_offset":43856,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":46068,"parent_record_offset":46044,"ancestor_record_offset":43856,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":46044,"parent_record_offset":46016,"ancestor_record_offset":43856,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":45956,"parent_record_offset":45924,"ancestor_record_offset":43856,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":45988,"parent_record_offset":45956,"ancestor_record_offset":43856,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":46160,"parent_record_offset":45988,"ancestor_record_offset":43856,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":46272,"parent_record_offset":46248,"ancestor_record_offset":43856,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":531,"record_offset":46676,"parent_record_offset":46636,"ancestor_record_offset":43856,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":531,"record_offset":46092,"parent_record_offset":46068,"ancestor_record_offset":43856,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":531,"record_offset":44352,"parent_record_offset":44320,"ancestor_record_offset":43856,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":531,"record_offset":45792,"parent_record_offset":45752,"ancestor_record_offset":43856,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":48408,"parent_record_offset":47324,"ancestor_record_offset":43856,"inlinee":7274,"source_line":1303,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":47656,"parent_record_offset":47616,"ancestor_record_offset":43856,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":48432,"parent_record_offset":48408,"ancestor_record_offset":43856,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":48512,"parent_record_offset":48488,"ancestor_record_offset":43856,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":44600,"parent_record_offset":44572,"ancestor_record_offset":43856,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":45892,"parent_record_offset":45752,"ancestor_record_offset":43856,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":46224,"parent_record_offset":45924,"ancestor_record_offset":43856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":45064,"parent_record_offset":45040,"ancestor_record_offset":43856,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":45040,"parent_record_offset":45016,"ancestor_record_offset":43856,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":45248,"parent_record_offset":45220,"ancestor_record_offset":43856,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":46116,"parent_record_offset":46092,"ancestor_record_offset":43856,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":44992,"parent_record_offset":44968,"ancestor_record_offset":43856,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":45016,"parent_record_offset":44992,"ancestor_record_offset":43856,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":45320,"parent_record_offset":45292,"ancestor_record_offset":43856,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":44968,"parent_record_offset":44852,"ancestor_record_offset":43856,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":44320,"parent_record_offset":44292,"ancestor_record_offset":43856,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":44680,"parent_record_offset":44652,"ancestor_record_offset":43856,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":44820,"parent_record_offset":44788,"ancestor_record_offset":43856,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":43856,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":44728,"parent_record_offset":44704,"ancestor_record_offset":43856,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":44752,"parent_record_offset":44728,"ancestor_record_offset":43856,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":44704,"parent_record_offset":44680,"ancestor_record_offset":43856,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":45292,"parent_record_offset":44752,"ancestor_record_offset":43856,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44292,"parent_record_offset":44260,"ancestor_record_offset":43856,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44260,"parent_record_offset":44236,"ancestor_record_offset":43856,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44452,"parent_record_offset":44420,"ancestor_record_offset":43856,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44548,"parent_record_offset":44452,"ancestor_record_offset":43856,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":43856,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44236,"parent_record_offset":44204,"ancestor_record_offset":43856,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":44572,"parent_record_offset":44548,"ancestor_record_offset":43856,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":44852,"parent_record_offset":44820,"ancestor_record_offset":43856,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":45136,"parent_record_offset":44852,"ancestor_record_offset":43856,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":45108,"parent_record_offset":44852,"ancestor_record_offset":43856,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":44912,"parent_record_offset":44852,"ancestor_record_offset":43856,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":47484,"parent_record_offset":47324,"ancestor_record_offset":43856,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":47616,"parent_record_offset":47324,"ancestor_record_offset":43856,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":46900,"parent_record_offset":46852,"ancestor_record_offset":43856,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":46772,"parent_record_offset":46740,"ancestor_record_offset":43856,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":46740,"parent_record_offset":46708,"ancestor_record_offset":43856,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":46708,"parent_record_offset":46676,"ancestor_record_offset":43856,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":531,"record_offset":48464,"parent_record_offset":47324,"ancestor_record_offset":43856,"inlinee":27145,"source_line":82,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":45668,"parent_record_offset":45628,"ancestor_record_offset":43856,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":50464,"parent_record_offset":50428,"ancestor_record_offset":43856,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":51016,"parent_record_offset":50744,"ancestor_record_offset":43856,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":531,"record_offset":45820,"parent_record_offset":45792,"ancestor_record_offset":43856,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":45924,"parent_record_offset":45752,"ancestor_record_offset":43856,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":46196,"parent_record_offset":45924,"ancestor_record_offset":43856,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":531,"record_offset":46804,"parent_record_offset":46772,"ancestor_record_offset":43856,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":531,"record_offset":46932,"parent_record_offset":46900,"ancestor_record_offset":43856,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":46996,"parent_record_offset":46964,"ancestor_record_offset":43856,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":47124,"parent_record_offset":47064,"ancestor_record_offset":43856,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":46964,"parent_record_offset":46932,"ancestor_record_offset":43856,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":531,"record_offset":45848,"parent_record_offset":45820,"ancestor_record_offset":43856,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":45752,"parent_record_offset":45708,"ancestor_record_offset":43856,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":45708,"parent_record_offset":45668,"ancestor_record_offset":43856,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":531,"record_offset":52092,"rva":10289312,"va":5378998432}
// 编译实例 RVAs: 0x9d00a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9398482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e82e9770ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378999277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1303587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8dd1becff","size":5,"soft_fail":false,"target_va":5377696704,"va":5379000286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1299076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e87c2decff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379000383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-924993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8bfe2f1ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378999612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-413720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8e8aff9ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5378999683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2188142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e86e632100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378999261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12949525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e81598c500","size":5,"soft_fail":false,"target_va":5391949088,"va":5378999558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32670088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e88881f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378999875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32847640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e81837f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378999523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32541646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8ce8bf001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378999933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8fed6ffff","size":5,"soft_fail":false,"target_va":5378988032,"va":5378998525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32544760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8f897f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5378998595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-826152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8d864f3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378999731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-829676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e81457f3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378999495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1477204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8ac75e9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378999231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1475177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8977de9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378999764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2386467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8236a2400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378999064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2391660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e86c7e2400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378998975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2386649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e8d96a2400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378999826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2386210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e822692400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378998489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2118699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e82b542000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378999136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9825159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e887eb9500","size":5,"soft_fail":false,"target_va":5388825296,"va":5379000132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32537178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378998432,"raw_hex":"e85a7af001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378999921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":55692,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":55064,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":54848,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/final_post.rs
//     evidence={"module":531,"record_offset":55536,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":30355,"source_line":244,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":53880,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":52448,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":52904,"parent_record_offset":52092,"ancestor_record_offset":52092,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54200,"parent_record_offset":54172,"ancestor_record_offset":52092,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":55364,"parent_record_offset":55336,"ancestor_record_offset":52092,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54252,"parent_record_offset":54228,"ancestor_record_offset":52092,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54228,"parent_record_offset":54200,"ancestor_record_offset":52092,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54148,"parent_record_offset":54120,"ancestor_record_offset":52092,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54172,"parent_record_offset":54148,"ancestor_record_offset":52092,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54344,"parent_record_offset":54172,"ancestor_record_offset":52092,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":54456,"parent_record_offset":54432,"ancestor_record_offset":52092,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":531,"record_offset":54888,"parent_record_offset":54848,"ancestor_record_offset":52092,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":531,"record_offset":54276,"parent_record_offset":54252,"ancestor_record_offset":52092,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":531,"record_offset":52604,"parent_record_offset":52572,"ancestor_record_offset":52092,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":531,"record_offset":54016,"parent_record_offset":53976,"ancestor_record_offset":52092,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":55656,"parent_record_offset":55616,"ancestor_record_offset":52092,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":52852,"parent_record_offset":52824,"ancestor_record_offset":52092,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":54804,"parent_record_offset":53976,"ancestor_record_offset":52092,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":54408,"parent_record_offset":54120,"ancestor_record_offset":52092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":53316,"parent_record_offset":53292,"ancestor_record_offset":52092,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":53292,"parent_record_offset":53268,"ancestor_record_offset":52092,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":53500,"parent_record_offset":53472,"ancestor_record_offset":52092,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":54300,"parent_record_offset":54276,"ancestor_record_offset":52092,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":53244,"parent_record_offset":53220,"ancestor_record_offset":52092,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":53268,"parent_record_offset":53244,"ancestor_record_offset":52092,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":53572,"parent_record_offset":53544,"ancestor_record_offset":52092,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":53220,"parent_record_offset":53104,"ancestor_record_offset":52092,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":52572,"parent_record_offset":52544,"ancestor_record_offset":52092,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":52932,"parent_record_offset":52904,"ancestor_record_offset":52092,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":53072,"parent_record_offset":53040,"ancestor_record_offset":52092,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":53040,"parent_record_offset":53004,"ancestor_record_offset":52092,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":52980,"parent_record_offset":52956,"ancestor_record_offset":52092,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":53004,"parent_record_offset":52980,"ancestor_record_offset":52092,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":52956,"parent_record_offset":52932,"ancestor_record_offset":52092,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":53544,"parent_record_offset":53004,"ancestor_record_offset":52092,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52544,"parent_record_offset":52512,"ancestor_record_offset":52092,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52512,"parent_record_offset":52484,"ancestor_record_offset":52092,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52704,"parent_record_offset":52672,"ancestor_record_offset":52092,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52800,"parent_record_offset":52704,"ancestor_record_offset":52092,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52672,"parent_record_offset":52648,"ancestor_record_offset":52092,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52484,"parent_record_offset":52448,"ancestor_record_offset":52092,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":52824,"parent_record_offset":52800,"ancestor_record_offset":52092,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":53104,"parent_record_offset":53072,"ancestor_record_offset":52092,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":53388,"parent_record_offset":53104,"ancestor_record_offset":52092,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":53360,"parent_record_offset":53104,"ancestor_record_offset":52092,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":53164,"parent_record_offset":53104,"ancestor_record_offset":52092,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":55572,"parent_record_offset":55536,"ancestor_record_offset":52092,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":55616,"parent_record_offset":55536,"ancestor_record_offset":52092,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":55112,"parent_record_offset":55064,"ancestor_record_offset":52092,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":54984,"parent_record_offset":54952,"ancestor_record_offset":52092,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":54952,"parent_record_offset":54920,"ancestor_record_offset":52092,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":54920,"parent_record_offset":54888,"ancestor_record_offset":52092,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":53912,"parent_record_offset":53880,"ancestor_record_offset":52092,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":55756,"parent_record_offset":55724,"ancestor_record_offset":52092,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":56256,"parent_record_offset":55996,"ancestor_record_offset":52092,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":531,"record_offset":54044,"parent_record_offset":54016,"ancestor_record_offset":52092,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":54120,"parent_record_offset":53976,"ancestor_record_offset":52092,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":54380,"parent_record_offset":54120,"ancestor_record_offset":52092,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":531,"record_offset":55016,"parent_record_offset":54984,"ancestor_record_offset":52092,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":531,"record_offset":55144,"parent_record_offset":55112,"ancestor_record_offset":52092,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":55208,"parent_record_offset":55176,"ancestor_record_offset":52092,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":55336,"parent_record_offset":55276,"ancestor_record_offset":52092,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":55176,"parent_record_offset":55144,"ancestor_record_offset":52092,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":531,"record_offset":54072,"parent_record_offset":54044,"ancestor_record_offset":52092,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":53976,"parent_record_offset":53944,"ancestor_record_offset":52092,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":53944,"parent_record_offset":53912,"ancestor_record_offset":52092,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00531.jsonl.gz"}
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
// 首条位置: {"module":531,"record_offset":38948,"rva":10280640,"va":5378989760}
// 编译实例 RVAs: 0x9cdec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9389831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8f9b870ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378990626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1295059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e82d3decff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378991758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1290548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8cc4eecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378991855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-916401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e84f04f2ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378991020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-810316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8b4a2f3ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378990935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2196793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e839852100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378990610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32678609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8d1a2f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378991354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: raster_single.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32856288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8e058f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378990875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32550167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e817adf001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378991412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8def8ffff","size":5,"soft_fail":false,"target_va":5378988032,"va":5378989853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32553432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8d8b9f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5378989923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-817624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e82886f3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378991203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-821028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8dc78f3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378990847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1468532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e88c97e9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378990559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1466656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8e09ee9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378991243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2395139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8038c2400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378990392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2400332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e84ca02400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378990303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2395170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8228c2400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378991305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2394882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8028b2400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378989817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2127371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e80b762000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378990464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9833687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8d70c9600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378991604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32545699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378989760,"raw_hex":"e8a39bf001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378991400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":42700,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":41908,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":41692,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
//     evidence={"module":531,"record_offset":42380,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":30351,"source_line":279,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":40724,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":39292,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":39748,"parent_record_offset":38948,"ancestor_record_offset":38948,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41044,"parent_record_offset":41016,"ancestor_record_offset":38948,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":42208,"parent_record_offset":42180,"ancestor_record_offset":38948,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41096,"parent_record_offset":41072,"ancestor_record_offset":38948,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41072,"parent_record_offset":41044,"ancestor_record_offset":38948,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":40992,"parent_record_offset":40964,"ancestor_record_offset":38948,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41016,"parent_record_offset":40992,"ancestor_record_offset":38948,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41188,"parent_record_offset":41016,"ancestor_record_offset":38948,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":41300,"parent_record_offset":41276,"ancestor_record_offset":38948,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":531,"record_offset":41732,"parent_record_offset":41692,"ancestor_record_offset":38948,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":531,"record_offset":41120,"parent_record_offset":41096,"ancestor_record_offset":38948,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":531,"record_offset":39448,"parent_record_offset":39416,"ancestor_record_offset":38948,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":531,"record_offset":40860,"parent_record_offset":40820,"ancestor_record_offset":38948,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":39696,"parent_record_offset":39668,"ancestor_record_offset":38948,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":41648,"parent_record_offset":40820,"ancestor_record_offset":38948,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":41252,"parent_record_offset":40964,"ancestor_record_offset":38948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":38948,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":38948,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":40344,"parent_record_offset":40316,"ancestor_record_offset":38948,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":41144,"parent_record_offset":41120,"ancestor_record_offset":38948,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":40088,"parent_record_offset":40064,"ancestor_record_offset":38948,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":40112,"parent_record_offset":40088,"ancestor_record_offset":38948,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":40416,"parent_record_offset":40388,"ancestor_record_offset":38948,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":40064,"parent_record_offset":39948,"ancestor_record_offset":38948,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":39416,"parent_record_offset":39388,"ancestor_record_offset":38948,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":39776,"parent_record_offset":39748,"ancestor_record_offset":38948,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":39916,"parent_record_offset":39884,"ancestor_record_offset":38948,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":39884,"parent_record_offset":39848,"ancestor_record_offset":38948,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":39824,"parent_record_offset":39800,"ancestor_record_offset":38948,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":39848,"parent_record_offset":39824,"ancestor_record_offset":38948,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":39800,"parent_record_offset":39776,"ancestor_record_offset":38948,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":40388,"parent_record_offset":39848,"ancestor_record_offset":38948,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39388,"parent_record_offset":39356,"ancestor_record_offset":38948,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39356,"parent_record_offset":39328,"ancestor_record_offset":38948,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39548,"parent_record_offset":39516,"ancestor_record_offset":38948,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39644,"parent_record_offset":39548,"ancestor_record_offset":38948,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":38948,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39328,"parent_record_offset":39292,"ancestor_record_offset":38948,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":38948,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":39948,"parent_record_offset":39916,"ancestor_record_offset":38948,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":40232,"parent_record_offset":39948,"ancestor_record_offset":38948,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":40204,"parent_record_offset":39948,"ancestor_record_offset":38948,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":40008,"parent_record_offset":39948,"ancestor_record_offset":38948,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":42432,"parent_record_offset":42380,"ancestor_record_offset":38948,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":41956,"parent_record_offset":41908,"ancestor_record_offset":38948,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":41828,"parent_record_offset":41796,"ancestor_record_offset":38948,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":41796,"parent_record_offset":41764,"ancestor_record_offset":38948,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":41764,"parent_record_offset":41732,"ancestor_record_offset":38948,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":42572,"parent_record_offset":42548,"ancestor_record_offset":38948,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":40756,"parent_record_offset":40724,"ancestor_record_offset":38948,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":42764,"parent_record_offset":42732,"ancestor_record_offset":38948,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":43264,"parent_record_offset":43004,"ancestor_record_offset":38948,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":531,"record_offset":40888,"parent_record_offset":40860,"ancestor_record_offset":38948,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":40964,"parent_record_offset":40820,"ancestor_record_offset":38948,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":41224,"parent_record_offset":40964,"ancestor_record_offset":38948,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42608,"parent_record_offset":42380,"ancestor_record_offset":38948,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":38948,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42524,"parent_record_offset":42380,"ancestor_record_offset":38948,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42548,"parent_record_offset":42524,"ancestor_record_offset":38948,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":531,"record_offset":41860,"parent_record_offset":41828,"ancestor_record_offset":38948,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":531,"record_offset":41988,"parent_record_offset":41956,"ancestor_record_offset":38948,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":42052,"parent_record_offset":42020,"ancestor_record_offset":38948,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":42180,"parent_record_offset":42120,"ancestor_record_offset":38948,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":42020,"parent_record_offset":41988,"ancestor_record_offset":38948,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":531,"record_offset":40916,"parent_record_offset":40888,"ancestor_record_offset":38948,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":40820,"parent_record_offset":40788,"ancestor_record_offset":38948,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":40788,"parent_record_offset":40756,"ancestor_record_offset":38948,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00531.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>

// ------------------------------------------------------------------
// 函数: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1>
// 信息 ID: 7ef35f7a0ee1b224ae40e2cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::render::pass_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>", "1"]; 字段布局 = unknown。
// [类型关系] country_core::render::raster_single::raster_single_object -> ["1", "array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>", "country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::final_post::final_post @ 0x90d597 -> 0x9ce780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0>
// 信息 ID: f312995c0142c9a964237f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::render::pass_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>", "1"]; 字段布局 = unknown。
// [类型关系] country_core::render::raster_single::raster_single_object -> ["1", "array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>", "country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain @ 0x90d96f -> 0x9d00a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>
// 信息 ID: 7f5f4fa3d058d645250c6bab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::render::pass_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>", "1"]; 字段布局 = unknown。
// [类型关系] country_core::render::raster_single::raster_single_object -> ["1", "array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>", "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass @ 0x9c7051 -> 0x9cdec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
