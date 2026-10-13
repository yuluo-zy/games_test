// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5433e422d8df2b4a84f4eda7376ac2568413f294a7950ef050ddb62b7a99a78f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: blur_pyramid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 535
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":535,"record_offset":33888,"rva":10410080,"va":5379119200}
// 编译实例 RVAs: 0x9ed860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1404946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8ee8feaff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379121085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1048534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e82a00f0ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379120337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1046738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e82e07f0ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379119885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-533948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e844daf7ff","size":5,"soft_fail":false,"target_va":5378586464,"va":5379120407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-533785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8e7daf7ff","size":5,"soft_fail":false,"target_va":5378586704,"va":5379120484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-68068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e81cf6feff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379120815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2003914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8ca931e00","size":5,"soft_fail":false,"target_va":5381123296,"va":5379119377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2049500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8dc451f00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379120847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12671055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e84f58c100","size":5,"soft_fail":false,"target_va":5391790752,"va":5379119692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32727258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8da60f301","size":5,"soft_fail":false,"target_va":5411847168,"va":5379119905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e83d090000","size":5,"soft_fail":false,"target_va":5379121632,"va":5379119262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1997983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e89f7c1e00","size":5,"soft_fail":false,"target_va":5381117600,"va":5379119612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1997467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e89b7a1e00","size":5,"soft_fail":false,"target_va":5381116784,"va":5379119312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1997484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8ac7a1e00","size":5,"soft_fail":false,"target_va":5381116816,"va":5379119327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1998419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8537e1e00","size":5,"soft_fail":false,"target_va":5381118176,"va":5379119752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1998453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8757e1e00","size":5,"soft_fail":false,"target_va":5381118240,"va":5379119782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1998144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8407d1e00","size":5,"soft_fail":false,"target_va":5381117856,"va":5379119707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::usage
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1998248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e8a87d1e00","size":5,"soft_fail":false,"target_va":5381117984,"va":5379119731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2079627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379119200,"raw_hex":"e88bbb1f00","size":5,"soft_fail":false,"target_va":5381199056,"va":5379119424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs
//     evidence={"module":535,"record_offset":34064,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":30682,"source_line":98,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":34540,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":34400,"parent_record_offset":34372,"ancestor_record_offset":33888,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":535,"record_offset":34264,"parent_record_offset":34196,"ancestor_record_offset":33888,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":535,"record_offset":34324,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34428,"parent_record_offset":34372,"ancestor_record_offset":33888,"inlinee":30683,"source_line":209,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34372,"parent_record_offset":34348,"ancestor_record_offset":33888,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34348,"parent_record_offset":34324,"ancestor_record_offset":33888,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":535,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":33888,"inlinee":30684,"source_line":750,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":535,"record_offset":34696,"parent_record_offset":34652,"ancestor_record_offset":33888,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":34908,"parent_record_offset":34836,"ancestor_record_offset":33888,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_3d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34808,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":29686,"source_line":456,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34836,"parent_record_offset":34808,"ancestor_record_offset":33888,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34612,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34496,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":28228,"source_line":718,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34652,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":535,"record_offset":34236,"parent_record_offset":34196,"ancestor_record_offset":33888,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":535,"record_offset":34196,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":535,"record_offset":34292,"parent_record_offset":34196,"ancestor_record_offset":33888,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":535,"record_offset":35148,"rva":10412512,"va":5379121632}
// 编译实例 RVAs: 0x9ee1e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1408063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8c183eaff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379124202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1050540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e854f8efff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379122343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1048981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e86bfeefff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379122128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-535981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e853d2f7ff","size":5,"soft_fail":false,"target_va":5378586464,"va":5379122440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-70095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e831eefeff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379122842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2001384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8e8891e00","size":5,"soft_fail":false,"target_va":5381123296,"va":5379121907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2047477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8f53d1f00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379122870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12668669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8fd4ec100","size":5,"soft_fail":false,"target_va":5391790752,"va":5379122078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32725010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e81258f301","size":5,"soft_fail":false,"target_va":5411847168,"va":5379122153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::all_mip_levels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e823721e00","size":5,"soft_fail":false,"target_va":5381117088,"va":5379121784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_extent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1994086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8666d1e00","size":5,"soft_fail":false,"target_va":5381117600,"va":5379123509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8b8721e00","size":5,"soft_fail":false,"target_va":5381117840,"va":5379122387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8d8711e00","size":5,"soft_fail":false,"target_va":5381116976,"va":5379121747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1996016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8f0741e00","size":5,"soft_fail":false,"target_va":5381117744,"va":5379121723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e802711e00","size":5,"soft_fail":false,"target_va":5381116784,"va":5379121769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e82d711e00","size":5,"soft_fail":false,"target_va":5381116816,"va":5379121758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8c2741e00","size":5,"soft_fail":false,"target_va":5381118176,"va":5379122201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8d9741e00","size":5,"soft_fail":false,"target_va":5381118240,"va":5379122242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1995679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e89f731e00","size":5,"soft_fail":false,"target_va":5381117856,"va":5379122172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::usage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1994918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8a6701e00","size":5,"soft_fail":false,"target_va":5381117984,"va":5379123061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2077104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379121632,"raw_hex":"e8b0b11f00","size":5,"soft_fail":false,"target_va":5381199056,"va":5379121947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":35372,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":36020,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$6::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":535,"record_offset":35344,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":30685,"source_line":2455,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":35688,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":35576,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":35528,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":36104,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":28228,"source_line":718,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":35616,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":535,"record_offset":35400,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35148,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":535,"record_offset":35468,"parent_record_offset":35400,"ancestor_record_offset":35148,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":36044,"parent_record_offset":36020,"ancestor_record_offset":35148,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":535,"record_offset":35656,"parent_record_offset":35616,"ancestor_record_offset":35148,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":35788,"parent_record_offset":35720,"ancestor_record_offset":35148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":35720,"parent_record_offset":35688,"ancestor_record_offset":35148,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":535,"record_offset":35440,"parent_record_offset":35400,"ancestor_record_offset":35148,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":535,"record_offset":35496,"parent_record_offset":35400,"ancestor_record_offset":35148,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77DA
// 首条位置: {"module":535,"inlinee":30682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":34540,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":535,"record_offset":34324,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_3d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34808,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":29686,"source_line":456,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34612,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34496,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":28228,"source_line":718,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34652,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":535,"record_offset":34196,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":34400,"parent_record_offset":34372,"ancestor_record_offset":34064,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":535,"record_offset":34264,"parent_record_offset":34196,"ancestor_record_offset":34064,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34428,"parent_record_offset":34372,"ancestor_record_offset":34064,"inlinee":30683,"source_line":209,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34372,"parent_record_offset":34348,"ancestor_record_offset":34064,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":535,"record_offset":34348,"parent_record_offset":34324,"ancestor_record_offset":34064,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":535,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":34064,"inlinee":30684,"source_line":750,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":535,"record_offset":34696,"parent_record_offset":34652,"ancestor_record_offset":34064,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":34908,"parent_record_offset":34836,"ancestor_record_offset":34064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":535,"record_offset":34836,"parent_record_offset":34808,"ancestor_record_offset":34064,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":535,"record_offset":34236,"parent_record_offset":34196,"ancestor_record_offset":34064,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":535,"record_offset":34292,"parent_record_offset":34196,"ancestor_record_offset":34064,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00535.jsonl.gz"}
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
// 完整结构及原始类型记录: blur_pyramid.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: blur_pyramid.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: blur_pyramid.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_transitive","evidence":{"module":535,"record_offset":34612,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":535,"record_offset":34652,"parent_record_offset":34064,"ancestor_record_offset":33888,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":535,"record_offset":35528,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":535,"record_offset":35576,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":535,"record_offset":35616,"parent_record_offset":35148,"ancestor_record_offset":35148,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":535,"record_offset":34612,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":535,"record_offset":34652,"parent_record_offset":34064,"ancestor_record_offset":34064,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00535.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"501b0a65f7be985a9f24d123","relative_path":"_blur_pyramid_blur.cs.hlsl_cs_main_0.bin","sha256":"e4cc119893363988eb1b606eabe81bdeff94aa24ae941a922e76d3e18d9a9ead","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"blur_pyramid","prefix_candidates":["blur_pyramid"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"ebc9ef50b6267b8ae02c6fb1","relative_path":"_blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin","sha256":"cf8410f4fb3691bfc701cd30a04078893552e7a31df325f7d1c99b88b552f3df","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"blur_pyramid","prefix_candidates":["blur_pyramid"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _blur_pyramid_blur.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"input_tex","storage_class":"UniformConstant","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"output_tex","storage_class":"UniformConstant","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,14]},"decorations":[],"pointee":{"id":14,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"$Globals","storage_class":"Uniform","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,16]},"decorations":[],"pointee":{"id":16,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_mip_dims","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[22,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[22,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[22,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,10]},"decorations":[],"pointee":{"id":10,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"input_tail_tex","storage_class":"UniformConstant","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[22,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"input_tex","storage_class":"UniformConstant","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[22,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"output_tex","storage_class":"UniformConstant","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[22,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"$Globals","storage_class":"Uniform","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,23]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"output_extent","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sample_self","type_id":23,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,41]},"decorations":[],"pointee":{"id":41,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[23,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1453902,"position_among_mapped_render_targets":5,"mapped_render_target_count":25,"previous_callsite":1452793,"next_callsite":1454123,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9080176,"caller_aliases":{"alias_group_rva":9080176,"count":1,"display_examples":["country_core::systems::render_loop::vermeer::vermeer_render_from_target"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9080478,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10410080,"caller_aliases":{"alias_group_rva":10410080,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10410142,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: blur_pyramid.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid
// 信息 ID: fa3e2a7adff999961a56a405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x162f4e -> 0x9ed860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// 信息 ID: a8665753a1b1f93b4d9aee26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a8e9e -> 0x9ee1e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid @ 0x9ed89e -> 0x9ee1e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
// 信息 ID: 0bb0bb6781c082751c1f06c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid; module=535, inline_record=34064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
