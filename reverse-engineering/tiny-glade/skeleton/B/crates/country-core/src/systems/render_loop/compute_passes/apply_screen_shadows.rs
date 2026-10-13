// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a56503f9f8b1c59d3eaf1088dbf937a4c57e538ab3cdfc158a56386b974b157c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_screen_shadows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 544
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":75200,"rva":10722928,"va":5379432048}
// 编译实例 RVAs: 0xa39e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1717398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e86acbe5ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379433537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1361154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8fe3aebff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379432957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1359372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8f441ebff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379432519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1265496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8a8b0ecff","size":5,"soft_fail":false,"target_va":5378167392,"va":5379432883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-846481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e86f15f3ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5379432444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-846116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8dc16f3ff","size":5,"soft_fail":false,"target_va":5378586704,"va":5379432815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-838259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e88d35f3ff","size":5,"soft_fail":false,"target_va":5378593936,"va":5379432190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-831574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8aa4ff3ff","size":5,"soft_fail":false,"target_va":5378601104,"va":5379432673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-380613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e83b31faff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379433360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1736959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8ff801a00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379433388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12358349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8cd92bc00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379432398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141606ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12372588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e86ccabc00","size":5,"soft_fail":false,"target_va":5391805600,"va":5379433007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32414688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8e09bee01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379432475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3221274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e81a273100","size":5,"soft_fail":false,"target_va":5382655024,"va":5379433745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32109379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e843f3e901","size":5,"soft_fail":false,"target_va":5411542880,"va":5379433496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1685735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8e7b81900","size":5,"soft_fail":false,"target_va":5381117840,"va":5379432100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1684447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8dfb31900","size":5,"soft_fail":false,"target_va":5381116576,"va":5379432124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1684637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e89db41900","size":5,"soft_fail":false,"target_va":5381116784,"va":5379432142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1684654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379432048,"raw_hex":"e8aeb41900","size":5,"soft_fail":false,"target_va":5381116816,"va":5379432157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":75432,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/bind_resources.rs
//     evidence={"module":544,"record_offset":75776,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":76012,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":75936,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":75584,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":75508,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":544,"record_offset":75404,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":75552,"parent_record_offset":75508,"ancestor_record_offset":75200,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":75476,"parent_record_offset":75432,"ancestor_record_offset":75200,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":76044,"parent_record_offset":76012,"ancestor_record_offset":75200,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":75884,"parent_record_offset":75776,"ancestor_record_offset":75200,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":75808,"parent_record_offset":75776,"ancestor_record_offset":75200,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":76348,"rva":10725104,"va":5379434224}
// 编译实例 RVAs: 0xa3a6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-381887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434224,"raw_hex":"e8412cfaff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379434634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1735692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434224,"raw_hex":"e80c7c1a00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379434655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12356445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434224,"raw_hex":"e85d8bbc00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379434302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141606390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12368923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434224,"raw_hex":"e81bbcbc00","size":5,"soft_fail":false,"target_va":5391803280,"va":5379434352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141606ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12371268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379434224,"raw_hex":"e844c5bc00","size":5,"soft_fail":false,"target_va":5391805600,"va":5379434327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":76532,"parent_record_offset":76348,"ancestor_record_offset":76348,"inlinee":26675,"source_line":448,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":76632,"parent_record_offset":76560,"ancestor_record_offset":76348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":76560,"parent_record_offset":76532,"ancestor_record_offset":76348,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00544.jsonl.gz"}
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
// 完整结构及原始类型记录: apply_screen_shadows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: apply_screen_shadows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: apply_screen_shadows.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":544,"record_offset":75508,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":544,"record_offset":75584,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_shadowmap_binding_api","symbol":"country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap","relationship":"pdb_inline_direct","evidence":{"module":544,"record_offset":75776,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":75808,"parent_record_offset":75776,"ancestor_record_offset":75200,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":75884,"parent_record_offset":75776,"ancestor_record_offset":75200,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":544,"record_offset":75936,"parent_record_offset":75200,"ancestor_record_offset":75200,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00544.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"712783d61808a0bac7ec0798","relative_path":"_apply_screen_shadows.cs.hlsl_cs_main_0.bin","sha256":"c413fe2f7ef4e2b9504c12272837c987de79b0b8c8c78c330b6dd99a5af04322","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"apply_screen_shadows","prefix_candidates":["apply_screen_shadows"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _apply_screen_shadows.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[16,16,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_shadow_nearest","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"sampler_shadow_linear","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"frame_constants","storage_class":"Uniform","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[82,82,82,82,82,82,82,82,61,59,59,61,112,61,59,69,59,69,61,69,61,59,61,61,61,82,53]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":82,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":82,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":82,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":82,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":82,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":82,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":82,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":82,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":59,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":59,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":112,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":59,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":69,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":59,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":69,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":69,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":82,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":53,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"$Globals","storage_class":"Uniform","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[112,112,112,112,61,112]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"gbuffer_tex_dims","type_id":112,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"depth_tex_dims","type_id":112,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"history_tex_dims","type_id":112,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"reprojection_tex_dims","type_id":112,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"history_valid","type_id":61,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"output_tex_dims","type_id":112,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"gbuffer_tex","storage_class":"UniformConstant","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,38]},"decorations":[],"pointee":{"id":38,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[61,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"history_tex","storage_class":"UniformConstant","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"shadow_map","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,43]},"decorations":[],"pointee":{"id":43,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[59,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"output_tex","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,45]},"decorations":[],"pointee":{"id":45,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"__alias_shadow_mask_histogram","storage_class":"Uniform","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[54]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,122]},"decorations":[],"pointee":{"id":122,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163523,"position_among_mapped_render_targets":47,"mapped_render_target_count":59,"previous_callsite":5163493,"next_callsite":5163623,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163992,"position_among_mapped_render_targets":51,"mapped_render_target_count":59,"previous_callsite":5163981,"next_callsite":5164046,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: apply_screen_shadows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// 信息 ID: d312107e5c31a040a8c7184b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4eca03 -> 0xa39e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
// 信息 ID: 1f95ef450716b31600959475
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecbd8 -> 0xa3a6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
