// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d05da395c8beceeffbcc035feac5229706c28497b569bb1ba51a72ad03ec4d1f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: main_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 388
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
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":41528,"rva":5295536,"va":5374004656}
// 编译实例 RVAs: 0x50cdb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1380001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e85ff1eaff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374005020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1709073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e811141a00","size":5,"soft_fail":false,"target_va":5375714368,"va":5374005290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1957105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8f1dc1d00","size":5,"soft_fail":false,"target_va":5375963408,"va":5374006298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1961336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e878ed1d00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374006211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2217964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8ecd72100","size":5,"soft_fail":false,"target_va":5376222944,"va":5374004975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29588097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8817ac301","size":5,"soft_fail":false,"target_va":5403593360,"va":5374005258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28972730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8ba16ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374006145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1695464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8e8de1900","size":5,"soft_fail":false,"target_va":5375700688,"va":5374005219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1390641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e831381500","size":5,"soft_fail":false,"target_va":5375395536,"va":5374004890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1392828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8bc401500","size":5,"soft_fail":false,"target_va":5375398240,"va":5374005407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t914064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e890f20d00","size":5,"soft_fail":false,"target_va":5374918976,"va":5374004907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2336997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8e5a82300","size":5,"soft_fail":false,"target_va":5376341760,"va":5374004758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3361237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374004656,"raw_hex":"e8d5493300","size":5,"soft_fail":false,"target_va":5377366224,"va":5374004982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374004656,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5374005357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":42632,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":41732,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":388,"record_offset":41880,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":41992,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":42668,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":21989,"source_line":115,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":388,"record_offset":42072,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":41760,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":388,"record_offset":42700,"parent_record_offset":41528,"ancestor_record_offset":41528,"inlinee":21991,"source_line":20,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":42456,"parent_record_offset":42428,"ancestor_record_offset":41528,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":42508,"parent_record_offset":42484,"ancestor_record_offset":41528,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":42484,"parent_record_offset":42428,"ancestor_record_offset":41528,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":388,"record_offset":41792,"parent_record_offset":41760,"ancestor_record_offset":41528,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":388,"record_offset":42532,"parent_record_offset":42508,"ancestor_record_offset":41528,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":42384,"parent_record_offset":42360,"ancestor_record_offset":41528,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":41952,"parent_record_offset":41928,"ancestor_record_offset":41528,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":42036,"parent_record_offset":41992,"ancestor_record_offset":41528,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":42556,"parent_record_offset":42532,"ancestor_record_offset":41528,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":41904,"parent_record_offset":41880,"ancestor_record_offset":41528,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":388,"record_offset":41928,"parent_record_offset":41904,"ancestor_record_offset":41528,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":42096,"parent_record_offset":42072,"ancestor_record_offset":41528,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":42136,"parent_record_offset":42096,"ancestor_record_offset":41528,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":41840,"parent_record_offset":41816,"ancestor_record_offset":41528,"inlinee":21982,"source_line":132,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":41528,"inlinee":21981,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":42776,"parent_record_offset":42752,"ancestor_record_offset":41528,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":42812,"parent_record_offset":42700,"ancestor_record_offset":41528,"inlinee":21994,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":41528,"inlinee":21995,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":42728,"parent_record_offset":42700,"ancestor_record_offset":41528,"inlinee":21992,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":41528,"inlinee":21993,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":388,"record_offset":42164,"parent_record_offset":42136,"ancestor_record_offset":41528,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":42220,"parent_record_offset":42192,"ancestor_record_offset":41528,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":42308,"parent_record_offset":42276,"ancestor_record_offset":41528,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":42428,"parent_record_offset":42276,"ancestor_record_offset":41528,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":42192,"parent_record_offset":42164,"ancestor_record_offset":41528,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":42332,"parent_record_offset":42308,"ancestor_record_offset":41528,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":388,"record_offset":42360,"parent_record_offset":42332,"ancestor_record_offset":41528,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":42276,"parent_record_offset":42248,"ancestor_record_offset":41528,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":42248,"parent_record_offset":42220,"ancestor_record_offset":41528,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":28916,"rva":5288464,"va":5373997584}
// 编译实例 RVAs: 0x50b210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1373585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e86f0aebff","size":5,"soft_fail":false,"target_va":5372625024,"va":5373998604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1715489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8212d1a00","size":5,"soft_fail":false,"target_va":5375714368,"va":5373998874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1962610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e872f21d00","size":5,"soft_fail":false,"target_va":5375963408,"va":5374000793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1966952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e868031e00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374000595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2224380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8fcf02100","size":5,"soft_fail":false,"target_va":5376222944,"va":5373998559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9310087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8870f8e00","size":5,"soft_fail":false,"target_va":5383308144,"va":5373998052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29594513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e89193c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373998842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28978207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e81f2cba01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374000476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28978441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8092dba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374000434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e86ad8ffff","size":5,"soft_fail":false,"target_va":5373990272,"va":5374000401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28982019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8033bba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373998104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8cde3ffff","size":5,"soft_fail":false,"target_va":5373990464,"va":5373997678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28982903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8773eba01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373997748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1701880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8f8f71900","size":5,"soft_fail":false,"target_va":5375700688,"va":5373998803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1397087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e85f511500","size":5,"soft_fail":false,"target_va":5375395536,"va":5373998444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1399244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8cc591500","size":5,"soft_fail":false,"target_va":5375398240,"va":5373998991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t920496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8b00b0e00","size":5,"soft_fail":false,"target_va":5374918976,"va":5373998475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2358637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e86dfd2300","size":5,"soft_fail":false,"target_va":5376356912,"va":5373998270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2362523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e89b0c2400","size":5,"soft_fail":false,"target_va":5376360560,"va":5373998032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2358505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8e9fc2300","size":5,"soft_fail":false,"target_va":5376356144,"va":5373997634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2343433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e809c22300","size":5,"soft_fail":false,"target_va":5376341760,"va":5373998322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3367653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373997584,"raw_hex":"e8e5623300","size":5,"soft_fail":false,"target_va":5377366224,"va":5373998566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373997584,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5373998941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":388,"record_offset":31852,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":32604,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":31964,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":32640,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":21989,"source_line":115,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":388,"record_offset":32044,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":31728,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":388,"record_offset":32672,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":21991,"source_line":20,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":30696,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":29108,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":29684,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31080,"parent_record_offset":31052,"ancestor_record_offset":28916,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32428,"parent_record_offset":32400,"ancestor_record_offset":28916,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":28916,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31108,"parent_record_offset":31080,"ancestor_record_offset":28916,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31020,"parent_record_offset":30992,"ancestor_record_offset":28916,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31052,"parent_record_offset":31020,"ancestor_record_offset":28916,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31224,"parent_record_offset":31052,"ancestor_record_offset":28916,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":31336,"parent_record_offset":31312,"ancestor_record_offset":28916,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":388,"record_offset":31760,"parent_record_offset":31728,"ancestor_record_offset":28916,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":388,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":28916,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":388,"record_offset":29320,"parent_record_offset":29224,"ancestor_record_offset":28916,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":30916,"parent_record_offset":30888,"ancestor_record_offset":28916,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":30280,"parent_record_offset":29820,"ancestor_record_offset":28916,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":30964,"parent_record_offset":30792,"ancestor_record_offset":28916,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":388,"record_offset":29632,"parent_record_offset":29604,"ancestor_record_offset":28916,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":31288,"parent_record_offset":30992,"ancestor_record_offset":28916,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":31924,"parent_record_offset":31900,"ancestor_record_offset":28916,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":30056,"parent_record_offset":30032,"ancestor_record_offset":28916,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":30032,"parent_record_offset":30008,"ancestor_record_offset":28916,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":30240,"parent_record_offset":30208,"ancestor_record_offset":28916,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":31180,"parent_record_offset":31156,"ancestor_record_offset":28916,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":388,"record_offset":31688,"parent_record_offset":30760,"ancestor_record_offset":28916,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":29256,"parent_record_offset":29224,"ancestor_record_offset":28916,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":31876,"parent_record_offset":31852,"ancestor_record_offset":28916,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":29284,"parent_record_offset":29256,"ancestor_record_offset":28916,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":388,"record_offset":31900,"parent_record_offset":31876,"ancestor_record_offset":28916,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":30008,"parent_record_offset":29984,"ancestor_record_offset":28916,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":30336,"parent_record_offset":30312,"ancestor_record_offset":28916,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":29984,"parent_record_offset":29876,"ancestor_record_offset":28916,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":388,"record_offset":29224,"parent_record_offset":29196,"ancestor_record_offset":28916,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":388,"record_offset":29716,"parent_record_offset":29684,"ancestor_record_offset":28916,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":388,"record_offset":29848,"parent_record_offset":29820,"ancestor_record_offset":28916,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":388,"record_offset":29820,"parent_record_offset":29788,"ancestor_record_offset":28916,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":29764,"parent_record_offset":29740,"ancestor_record_offset":28916,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":29788,"parent_record_offset":29764,"ancestor_record_offset":28916,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":29740,"parent_record_offset":29716,"ancestor_record_offset":28916,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":30312,"parent_record_offset":29788,"ancestor_record_offset":28916,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29420,"parent_record_offset":29388,"ancestor_record_offset":28916,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29580,"parent_record_offset":29420,"ancestor_record_offset":28916,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29388,"parent_record_offset":29364,"ancestor_record_offset":28916,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29140,"parent_record_offset":29108,"ancestor_record_offset":28916,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29604,"parent_record_offset":29580,"ancestor_record_offset":28916,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29196,"parent_record_offset":29164,"ancestor_record_offset":28916,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":29164,"parent_record_offset":29140,"ancestor_record_offset":28916,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":29876,"parent_record_offset":29848,"ancestor_record_offset":28916,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":30124,"parent_record_offset":29876,"ancestor_record_offset":28916,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":30096,"parent_record_offset":29876,"ancestor_record_offset":28916,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":29928,"parent_record_offset":29876,"ancestor_record_offset":28916,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":32068,"parent_record_offset":32044,"ancestor_record_offset":28916,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":32108,"parent_record_offset":32068,"ancestor_record_offset":28916,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":31808,"parent_record_offset":31784,"ancestor_record_offset":28916,"inlinee":21982,"source_line":132,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":31784,"parent_record_offset":31760,"ancestor_record_offset":28916,"inlinee":21981,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":28916,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":30728,"parent_record_offset":30696,"ancestor_record_offset":28916,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":30828,"parent_record_offset":30792,"ancestor_record_offset":28916,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":30992,"parent_record_offset":30792,"ancestor_record_offset":28916,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":31260,"parent_record_offset":30992,"ancestor_record_offset":28916,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32784,"parent_record_offset":32672,"ancestor_record_offset":28916,"inlinee":21994,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32808,"parent_record_offset":32784,"ancestor_record_offset":28916,"inlinee":21995,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32700,"parent_record_offset":32672,"ancestor_record_offset":28916,"inlinee":21992,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":28916,"inlinee":21993,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":388,"record_offset":32136,"parent_record_offset":32108,"ancestor_record_offset":28916,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32192,"parent_record_offset":32164,"ancestor_record_offset":28916,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":32280,"parent_record_offset":32248,"ancestor_record_offset":28916,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32400,"parent_record_offset":32248,"ancestor_record_offset":28916,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32164,"parent_record_offset":32136,"ancestor_record_offset":28916,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":30856,"parent_record_offset":30828,"ancestor_record_offset":28916,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":388,"record_offset":30888,"parent_record_offset":30856,"ancestor_record_offset":28916,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":30792,"parent_record_offset":30760,"ancestor_record_offset":28916,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":30760,"parent_record_offset":30728,"ancestor_record_offset":28916,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":35220,"rva":5292000,"va":5374001120}
// 编译实例 RVAs: 0x50bfe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1377121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e89ffceaff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374002140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1711953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8511f1a00","size":5,"soft_fail":false,"target_va":5375714368,"va":5374002410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1959074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8a2e41d00","size":5,"soft_fail":false,"target_va":5375963408,"va":5374004329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1963416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e898f51d00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374004131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2220844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e82ce32100","size":5,"soft_fail":false,"target_va":5376222944,"va":5374002095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9306551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8b7018e00","size":5,"soft_fail":false,"target_va":5383308144,"va":5374001588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29590977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8c185c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5374002378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28974671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e84f1eba01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374004012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28974905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8391fba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374003970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e89acaffff","size":5,"soft_fail":false,"target_va":5373990272,"va":5374003937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28978483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8332dba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374001640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8fdd5ffff","size":5,"soft_fail":false,"target_va":5373990464,"va":5374001214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28979367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8a730ba01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374001284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1698344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e828ea1900","size":5,"soft_fail":false,"target_va":5375700688,"va":5374002339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1393551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e88f431500","size":5,"soft_fail":false,"target_va":5375395536,"va":5374001980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1395708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8fc4b1500","size":5,"soft_fail":false,"target_va":5375398240,"va":5374002527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t916960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8e0fd0d00","size":5,"soft_fail":false,"target_va":5374918976,"va":5374002011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2355101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e89def2300","size":5,"soft_fail":false,"target_va":5376356912,"va":5374001806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2358987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e8cbfe2300","size":5,"soft_fail":false,"target_va":5376360560,"va":5374001568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2354969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e819ef2300","size":5,"soft_fail":false,"target_va":5376356144,"va":5374001170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2339897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e839b42300","size":5,"soft_fail":false,"target_va":5376341760,"va":5374001858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3364117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374001120,"raw_hex":"e815553300","size":5,"soft_fail":false,"target_va":5377366224,"va":5374002102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374001120,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5374002477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":388,"record_offset":38160,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":38912,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":38272,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":38948,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":21989,"source_line":115,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":388,"record_offset":38352,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":38036,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":388,"record_offset":38980,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":21991,"source_line":20,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":37004,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":35416,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":35992,"parent_record_offset":35220,"ancestor_record_offset":35220,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37388,"parent_record_offset":37360,"ancestor_record_offset":35220,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":38736,"parent_record_offset":38708,"ancestor_record_offset":35220,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37440,"parent_record_offset":37416,"ancestor_record_offset":35220,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37416,"parent_record_offset":37388,"ancestor_record_offset":35220,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37328,"parent_record_offset":37300,"ancestor_record_offset":35220,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37360,"parent_record_offset":37328,"ancestor_record_offset":35220,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37532,"parent_record_offset":37360,"ancestor_record_offset":35220,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":37644,"parent_record_offset":37620,"ancestor_record_offset":35220,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":388,"record_offset":38068,"parent_record_offset":38036,"ancestor_record_offset":35220,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":388,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":35220,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":388,"record_offset":35628,"parent_record_offset":35532,"ancestor_record_offset":35220,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":37224,"parent_record_offset":37196,"ancestor_record_offset":35220,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":36588,"parent_record_offset":36128,"ancestor_record_offset":35220,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":37272,"parent_record_offset":37100,"ancestor_record_offset":35220,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":388,"record_offset":35940,"parent_record_offset":35912,"ancestor_record_offset":35220,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":37596,"parent_record_offset":37300,"ancestor_record_offset":35220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":38232,"parent_record_offset":38208,"ancestor_record_offset":35220,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":36364,"parent_record_offset":36340,"ancestor_record_offset":35220,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":36340,"parent_record_offset":36316,"ancestor_record_offset":35220,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":388,"record_offset":36548,"parent_record_offset":36516,"ancestor_record_offset":35220,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":35220,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":388,"record_offset":37996,"parent_record_offset":37068,"ancestor_record_offset":35220,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":35564,"parent_record_offset":35532,"ancestor_record_offset":35220,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":38184,"parent_record_offset":38160,"ancestor_record_offset":35220,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":35220,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":388,"record_offset":38208,"parent_record_offset":38184,"ancestor_record_offset":35220,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":36316,"parent_record_offset":36292,"ancestor_record_offset":35220,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":36644,"parent_record_offset":36620,"ancestor_record_offset":35220,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":388,"record_offset":36292,"parent_record_offset":36184,"ancestor_record_offset":35220,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":388,"record_offset":35532,"parent_record_offset":35504,"ancestor_record_offset":35220,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":388,"record_offset":36024,"parent_record_offset":35992,"ancestor_record_offset":35220,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":388,"record_offset":36156,"parent_record_offset":36128,"ancestor_record_offset":35220,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":388,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":35220,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":36072,"parent_record_offset":36048,"ancestor_record_offset":35220,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":36096,"parent_record_offset":36072,"ancestor_record_offset":35220,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":36048,"parent_record_offset":36024,"ancestor_record_offset":35220,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":388,"record_offset":36620,"parent_record_offset":36096,"ancestor_record_offset":35220,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35728,"parent_record_offset":35696,"ancestor_record_offset":35220,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35888,"parent_record_offset":35728,"ancestor_record_offset":35220,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35696,"parent_record_offset":35672,"ancestor_record_offset":35220,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35448,"parent_record_offset":35416,"ancestor_record_offset":35220,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35912,"parent_record_offset":35888,"ancestor_record_offset":35220,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35504,"parent_record_offset":35472,"ancestor_record_offset":35220,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":35472,"parent_record_offset":35448,"ancestor_record_offset":35220,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":36184,"parent_record_offset":36156,"ancestor_record_offset":35220,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":36432,"parent_record_offset":36184,"ancestor_record_offset":35220,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":36404,"parent_record_offset":36184,"ancestor_record_offset":35220,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":388,"record_offset":36236,"parent_record_offset":36184,"ancestor_record_offset":35220,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":38376,"parent_record_offset":38352,"ancestor_record_offset":35220,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":38416,"parent_record_offset":38376,"ancestor_record_offset":35220,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":38116,"parent_record_offset":38092,"ancestor_record_offset":35220,"inlinee":21982,"source_line":132,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":38092,"parent_record_offset":38068,"ancestor_record_offset":35220,"inlinee":21981,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":39056,"parent_record_offset":39032,"ancestor_record_offset":35220,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":37036,"parent_record_offset":37004,"ancestor_record_offset":35220,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":37136,"parent_record_offset":37100,"ancestor_record_offset":35220,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":37300,"parent_record_offset":37100,"ancestor_record_offset":35220,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":388,"record_offset":37568,"parent_record_offset":37300,"ancestor_record_offset":35220,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":39092,"parent_record_offset":38980,"ancestor_record_offset":35220,"inlinee":21994,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":39116,"parent_record_offset":39092,"ancestor_record_offset":35220,"inlinee":21995,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":39008,"parent_record_offset":38980,"ancestor_record_offset":35220,"inlinee":21992,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":39032,"parent_record_offset":39008,"ancestor_record_offset":35220,"inlinee":21993,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":388,"record_offset":38444,"parent_record_offset":38416,"ancestor_record_offset":35220,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":38500,"parent_record_offset":38472,"ancestor_record_offset":35220,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":38588,"parent_record_offset":38556,"ancestor_record_offset":35220,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":38708,"parent_record_offset":38556,"ancestor_record_offset":35220,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":38472,"parent_record_offset":38444,"ancestor_record_offset":35220,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":37164,"parent_record_offset":37136,"ancestor_record_offset":35220,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":388,"record_offset":37196,"parent_record_offset":37164,"ancestor_record_offset":35220,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":37100,"parent_record_offset":37068,"ancestor_record_offset":35220,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":37068,"parent_record_offset":37036,"ancestor_record_offset":35220,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55E7
// 首条位置: {"module":388,"inlinee":21991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32784,"parent_record_offset":32672,"ancestor_record_offset":32672,"inlinee":21994,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32700,"parent_record_offset":32672,"ancestor_record_offset":32672,"inlinee":21992,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":32672,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32808,"parent_record_offset":32784,"ancestor_record_offset":32672,"inlinee":21995,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":32672,"inlinee":21993,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
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
// 完整结构及原始类型记录: main_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: main_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: main_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138958,"position_among_mapped_render_targets":28,"mapped_render_target_count":70,"previous_callsite":3138490,"next_callsite":3139015,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139162,"position_among_mapped_render_targets":33,"mapped_render_target_count":70,"previous_callsite":3139114,"next_callsite":3139191,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139297,"position_among_mapped_render_targets":37,"mapped_render_target_count":70,"previous_callsite":3139249,"next_callsite":3139336,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/half_res.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: main_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 信息 ID: e9b1acc2d14389f8cc33575c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe6e1 -> 0x50cdb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 信息 ID: b4db2d1500bed5afd8986c65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe58e -> 0x50b210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 信息 ID: f47d0c2d2f1ea240c6ac3bf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe65a -> 0x50bfe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
// 信息 ID: 5e604eb30fa355416ce58034
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32672; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=34984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=38980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=41292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
