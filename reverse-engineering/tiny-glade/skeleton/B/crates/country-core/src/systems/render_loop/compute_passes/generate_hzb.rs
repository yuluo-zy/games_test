// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 674724f159e876af00a6bdc30d5d9622ddc589a3a6f9390cd4843740a15da546
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_hzb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 507
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":33612,"rva":9602800,"va":5378311920}
// 编译实例 RVAs: 0x9286f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3231799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8c9afceff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378313282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3230749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8e3b3ceff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378312248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-598066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8cedff6ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378314205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-241157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8fb51fcff","size":5,"soft_fail":false,"target_va":5378071808,"va":5378312960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-239571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e82d58fcff","size":5,"soft_fail":false,"target_va":5378073152,"va":5378312718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096c310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t277297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8313b0400","size":5,"soft_fail":false,"target_va":5378589456,"va":5378312154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t739019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8cb460b00","size":5,"soft_fail":false,"target_va":5379052752,"va":5378313728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2810789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8a5e32a00","size":5,"soft_fail":false,"target_va":5381123296,"va":5378312502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2856591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e88f962b00","size":5,"soft_fail":false,"target_va":5381170352,"va":5378313756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13478076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8bca8cd00","size":5,"soft_fail":false,"target_va":5391790752,"va":5378312671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33534423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8d7b1ff01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378312740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4528131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e803184500","size":5,"soft_fail":false,"target_va":5382840208,"va":5378312072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33227054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e82e01fb01","size":5,"soft_fail":false,"target_va":5411540963,"va":5378313904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4340987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8fb3c4200","size":5,"soft_fail":false,"target_va":5382655024,"va":5378314032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33228995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8c308fb01","size":5,"soft_fail":false,"target_va":5411542880,"va":5378313880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t282927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e82f510400","size":5,"soft_fail":false,"target_va":5378596192,"va":5378313260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2803291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e85bc62a00","size":5,"soft_fail":false,"target_va":5381115424,"va":5378312128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::all_mip_levels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2804687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8cfcb2a00","size":5,"soft_fail":false,"target_va":5381117088,"va":5378312396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e84acf2a00","size":5,"soft_fail":false,"target_va":5381117840,"va":5378312257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2804190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8dec92a00","size":5,"soft_fail":false,"target_va":5381116576,"va":5378312381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e86ace2a00","size":5,"soft_fail":false,"target_va":5381118176,"va":5378312817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e897cf2a00","size":5,"soft_fail":false,"target_va":5381118448,"va":5378312788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e889cd2a00","size":5,"soft_fail":false,"target_va":5381118240,"va":5378313106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e864cd2a00","size":5,"soft_fail":false,"target_va":5381117856,"va":5378312759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2805066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e84acd2a00","size":5,"soft_fail":false,"target_va":5381117920,"va":5378312849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2886509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e86d0b2c00","size":5,"soft_fail":false,"target_va":5381199056,"va":5378312542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3042377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378311920,"raw_hex":"e8496c2e00","size":5,"soft_fail":false,"target_va":5381354480,"va":5378312098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":507,"record_offset":33828,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":34432,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":34588,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":34320,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":28846,"source_line":3357,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":34064,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":507,"record_offset":33992,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":507,"record_offset":35488,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":28848,"source_line":464,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":507,"record_offset":34808,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":507,"record_offset":34700,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":507,"record_offset":34652,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":507,"record_offset":34460,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":507,"record_offset":34292,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":507,"record_offset":33936,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":507,"record_offset":33908,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8806,"source_line":469,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":35136,"parent_record_offset":35112,"ancestor_record_offset":33612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":35160,"parent_record_offset":35136,"ancestor_record_offset":33612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":34240,"parent_record_offset":34216,"ancestor_record_offset":33612,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":35112,"parent_record_offset":35084,"ancestor_record_offset":33612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":34216,"parent_record_offset":34160,"ancestor_record_offset":33612,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":507,"record_offset":33876,"parent_record_offset":33852,"ancestor_record_offset":33612,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":35024,"parent_record_offset":34996,"ancestor_record_offset":33612,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":34188,"parent_record_offset":34160,"ancestor_record_offset":33612,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":34160,"parent_record_offset":34136,"ancestor_record_offset":33612,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":35084,"parent_record_offset":35052,"ancestor_record_offset":33612,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":35052,"parent_record_offset":35024,"ancestor_record_offset":33612,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":34136,"parent_record_offset":34112,"ancestor_record_offset":33612,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":34960,"parent_record_offset":34928,"ancestor_record_offset":33612,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":34928,"parent_record_offset":34896,"ancestor_record_offset":33612,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":34864,"parent_record_offset":34808,"ancestor_record_offset":33612,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":34896,"parent_record_offset":34864,"ancestor_record_offset":33612,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":34996,"parent_record_offset":34960,"ancestor_record_offset":33612,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":507,"record_offset":34528,"parent_record_offset":34460,"ancestor_record_offset":33612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":35236,"parent_record_offset":35212,"ancestor_record_offset":33612,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":34344,"parent_record_offset":34320,"ancestor_record_offset":33612,"inlinee":28847,"source_line":3324,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":33852,"parent_record_offset":33828,"ancestor_record_offset":33612,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":507,"record_offset":35212,"parent_record_offset":34960,"ancestor_record_offset":33612,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":34032,"parent_record_offset":33992,"ancestor_record_offset":33612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":507,"record_offset":35520,"parent_record_offset":35488,"ancestor_record_offset":33612,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":507,"record_offset":34500,"parent_record_offset":34460,"ancestor_record_offset":33612,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":507,"record_offset":34556,"parent_record_offset":34460,"ancestor_record_offset":33612,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":507,"record_offset":33960,"parent_record_offset":33936,"ancestor_record_offset":33612,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00507.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_hzb.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_hzb.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: generate_hzb.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":507,"record_offset":34652,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00507.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":507,"record_offset":34700,"parent_record_offset":33612,"ancestor_record_offset":33612,"inlinee":8812,"source_line":161,"archive":"_pdb/B/modules/00507.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"2c4a2b2a575723da617cef89","relative_path":"_generate_hzb.cs.hlsl_cs_main_0.bin","sha256":"30811cb17132c7da1a9b9e7eabb10bb8f95b2fa597c103af76b8c1184c9ebaff","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"generate_hzb","prefix_candidates":["generate_hzb","hzb_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _generate_hzb.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"spdConstants","storage_class":"PushConstant","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.SpdConstants","record":{"opcode":"OpTypeStruct","operands":[28,28,35,35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"mips","type_id":28,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"numWorkGroups","type_id":28,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"workGroupOffset","type_id":35,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":3,"name":"input_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"imgSrc","storage_class":"UniformConstant","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"imgDst","storage_class":"UniformConstant","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeArray","operands":[14,60]},"decorations":[],"element":{"id":14,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[53,1,2,1,0,2,9]},"decorations":[]}}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"imgDst6","storage_class":"UniformConstant","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,14]},"decorations":[],"pointee":{"id":14,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[53,1,2,1,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"spdGlobalAtomic","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.RWStructuredBuffer.SpdGlobalAtomicBuffer","record":{"opcode":"OpTypeStruct","operands":[23]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":23,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","BufferBlock","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,69]},"decorations":[],"pointee":{"id":69,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,28]},"decorations":[],"pointee":{"id":28,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,28]},"decorations":[],"pointee":{"id":28,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162412,"position_among_mapped_render_targets":31,"mapped_render_target_count":59,"previous_callsite":5162373,"next_callsite":5162442,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_hzb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 信息 ID: 4df0fe09f0758a366580c053
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec5ac -> 0x9286f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
