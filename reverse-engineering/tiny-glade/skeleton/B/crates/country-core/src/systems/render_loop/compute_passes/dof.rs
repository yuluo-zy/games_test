// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/dof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e6b05752eab0c2c73b7a213921de90df6175981b074e1e034911a2791cc90707
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 549, 1585, 1702
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
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::dof_prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":2660,"rva":10854432,"va":5379563552}
// 编译实例 RVAs: 0xa5a020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1848614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8dacae3ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379564753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1492337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e88f3ae9ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379564140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1490893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e83340e9ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379564040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-511787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8d530f8ff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379564534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1559625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e849cc1700","size":5,"soft_fail":false,"target_va":5381123296,"va":5379563666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1605792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8a0801800","size":5,"soft_fail":false,"target_va":5381170352,"va":5379564555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12226754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8c290ba00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379563993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12270469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8853bbb00","size":5,"soft_fail":false,"target_va":5391834560,"va":5379564086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32283098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8da99ec01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379564065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_up_extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1553596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8bcb41700","size":5,"soft_fail":false,"target_va":5381117408,"va":5379563807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1554231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e837b71700","size":5,"soft_fail":false,"target_va":5381117840,"va":5379563604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1553143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e8f7b21700","size":5,"soft_fail":false,"target_va":5381116976,"va":5379563828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1552943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e82fb21700","size":5,"soft_fail":false,"target_va":5381116576,"va":5379563628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1635345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563552,"raw_hex":"e811f41800","size":5,"soft_fail":false,"target_va":5381199056,"va":5379563706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":3252,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":3156,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":3108,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":549,"record_offset":2856,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":549,"record_offset":2828,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":2920,"parent_record_offset":2856,"ancestor_record_offset":2660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":3352,"parent_record_offset":3280,"ancestor_record_offset":2660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":2660,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":549,"record_offset":2896,"parent_record_offset":2856,"ancestor_record_offset":2660,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":549,"record_offset":2948,"parent_record_offset":2856,"ancestor_record_offset":2660,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":3588,"rva":10855792,"va":5379564912}
// 编译实例 RVAs: 0xa5a570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9949397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e82b2f68ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379565504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9415469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8d35470ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5379573592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4485277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8638fbbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379566760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4486809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e86789bbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379568308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=36
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1857725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e843a7e3ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379573864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1494204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e84433e9ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379566007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=19
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1492550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8ba39e9ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379565697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409056d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1397402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e866adeaff","size":5,"soft_fail":false,"target_va":5378168528,"va":5379565925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-978899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e82d10f1ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5379566078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-513693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e86329f8ff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379566440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1558136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e878c61700","size":5,"soft_fail":false,"target_va":5381123296,"va":5379565155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6d00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1556249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e819bf1700","size":5,"soft_fail":false,"target_va":5381123328,"va":5379567074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1603886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e82e791800","size":5,"soft_fail":false,"target_va":5381170352,"va":5379566461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fdad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9055875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8832e8a00","size":5,"soft_fail":false,"target_va":5388622544,"va":5379566664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fd820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12196800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8c01bba00","size":5,"soft_fail":false,"target_va":5391767584,"va":5379570779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ffc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12206757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8a542ba00","size":5,"soft_fail":false,"target_va":5391776864,"va":5379570102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141600570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12207458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e86245ba00","size":5,"soft_fail":false,"target_va":5391779184,"va":5379571721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12225094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8468aba00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379565653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160d850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12260949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e85516bb00","size":5,"soft_fail":false,"target_va":5391833168,"va":5379572214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=39
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32281449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e86993ec01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379565714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12899181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e86dd3c400","size":5,"soft_fail":false,"target_va":5392466848,"va":5379567662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31967840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e860cae701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379573118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31975256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e858e7e701","size":5,"soft_fail":false,"target_va":5411541776,"va":5379566515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-979411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e82d0ef1ff","size":5,"soft_fail":false,"target_va":5378594784,"va":5379574190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31970211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8a3d3e701","size":5,"soft_fail":false,"target_va":5411543264,"va":5379573048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_raw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-970038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8ca32f1ff","size":5,"soft_fail":false,"target_va":5378598208,"va":5379568241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-970892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8742ff1ff","size":5,"soft_fail":false,"target_va":5378599040,"va":5379569927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-973489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e84f25f1ff","size":5,"soft_fail":false,"target_va":5378595520,"va":5379569004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-971939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e85d2bf1ff","size":5,"soft_fail":false,"target_va":5378596192,"va":5379568126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31919335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8e70ce701","size":5,"soft_fail":false,"target_va":5411492368,"va":5379573028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31921728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e84016e701","size":5,"soft_fail":false,"target_va":5411494800,"va":5379573067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2255898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e81a6c2200","size":5,"soft_fail":false,"target_va":5381823600,"va":5379567697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2256121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8f96c2200","size":5,"soft_fail":false,"target_va":5381824016,"va":5379567890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1548375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e857a01700","size":5,"soft_fail":false,"target_va":5381115424,"va":5379567044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1734311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8a7761a00","size":5,"soft_fail":false,"target_va":5381302080,"va":5379567764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1732896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e820711a00","size":5,"soft_fail":false,"target_va":5381300912,"va":5379568011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1552831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8bfb11700","size":5,"soft_fail":false,"target_va":5381117840,"va":5379565004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1551857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e8f1ad1700","size":5,"soft_fail":false,"target_va":5381116976,"va":5379565114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1552685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e82db11700","size":5,"soft_fail":false,"target_va":5381117744,"va":5379565054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1551208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e868ab1700","size":5,"soft_fail":false,"target_va":5381116576,"va":5379565363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1633849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379564912,"raw_hex":"e839ee1800","size":5,"soft_fail":false,"target_va":5381199056,"va":5379565202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":549,"record_offset":13624,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":31489,"source_line":79,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":549,"record_offset":3744,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":31488,"source_line":162,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":3588,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3588,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":3588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5136,"parent_record_offset":4952,"ancestor_record_offset":3588,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":8688,"parent_record_offset":8632,"ancestor_record_offset":3588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4880,"parent_record_offset":4844,"ancestor_record_offset":3588,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":3588,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":3588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":3588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":3588,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4916,"parent_record_offset":4880,"ancestor_record_offset":3588,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":3588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4996,"parent_record_offset":4952,"ancestor_record_offset":3588,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":4804,"parent_record_offset":4768,"ancestor_record_offset":3588,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":4768,"parent_record_offset":4696,"ancestor_record_offset":3588,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":8152,"parent_record_offset":8104,"ancestor_record_offset":3588,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":3588,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":7156,"parent_record_offset":7120,"ancestor_record_offset":3588,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":3588,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4740,"parent_record_offset":4696,"ancestor_record_offset":3588,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":7120,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4844,"parent_record_offset":4804,"ancestor_record_offset":3588,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4660,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":3588,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4696,"parent_record_offset":4660,"ancestor_record_offset":3588,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":3588,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3588,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6544,"parent_record_offset":6512,"ancestor_record_offset":3588,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6488,"parent_record_offset":5952,"ancestor_record_offset":3588,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":3588,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":3588,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6572,"parent_record_offset":6512,"ancestor_record_offset":3588,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":3588,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3588,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":3588,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5912,"parent_record_offset":5884,"ancestor_record_offset":3588,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5884,"parent_record_offset":5856,"ancestor_record_offset":3588,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5952,"parent_record_offset":5884,"ancestor_record_offset":3588,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":549,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":3588,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":549,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":3588,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":549,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":3588,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":4288,"parent_record_offset":4220,"ancestor_record_offset":3588,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":3588,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":549,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":3588,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4576,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4412,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":6954,"source_line":221,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":3588,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":549,"record_offset":7760,"parent_record_offset":7732,"ancestor_record_offset":3588,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":549,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":3588,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":3588,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3588,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":9228,"parent_record_offset":9196,"ancestor_record_offset":3588,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":4348,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":6120,"parent_record_offset":6056,"ancestor_record_offset":3588,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":11176,"parent_record_offset":11136,"ancestor_record_offset":3588,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":3588,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6184,"parent_record_offset":6004,"ancestor_record_offset":3588,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6672,"parent_record_offset":5856,"ancestor_record_offset":3588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":4504,"parent_record_offset":4468,"ancestor_record_offset":3588,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":5236,"parent_record_offset":4804,"ancestor_record_offset":3588,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7684,"parent_record_offset":7512,"ancestor_record_offset":3588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":3588,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":5356,"parent_record_offset":5308,"ancestor_record_offset":3588,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":3588,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7268,"parent_record_offset":7120,"ancestor_record_offset":3588,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":7616,"parent_record_offset":7512,"ancestor_record_offset":3588,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":3588,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":3588,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":3588,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6412,"parent_record_offset":6232,"ancestor_record_offset":3588,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3588,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":11588,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5308,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5384,"parent_record_offset":5308,"ancestor_record_offset":3588,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":8036,"parent_record_offset":7972,"ancestor_record_offset":3588,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":7364,"parent_record_offset":7328,"ancestor_record_offset":3588,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":3588,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":3588,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":15036,"parent_record_offset":13624,"ancestor_record_offset":3588,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8764,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":26675,"source_line":448,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9420,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8796,"parent_record_offset":8764,"ancestor_record_offset":3588,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8104,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9376,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9332,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":11136,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":549,"record_offset":10888,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":27146,"source_line":94,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::dof::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":549,"record_offset":4536,"parent_record_offset":4468,"ancestor_record_offset":3588,"inlinee":31491,"source_line":113,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":549,"record_offset":5856,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":549,"record_offset":4468,"parent_record_offset":4440,"ancestor_record_offset":3588,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7300,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7456,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7944,"parent_record_offset":7916,"ancestor_record_offset":3588,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":3588,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":3588,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":549,"record_offset":7420,"parent_record_offset":7328,"ancestor_record_offset":3588,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":549,"record_offset":8008,"parent_record_offset":7972,"ancestor_record_offset":3588,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":549,"record_offset":5512,"parent_record_offset":5472,"ancestor_record_offset":3588,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":549,"record_offset":7560,"parent_record_offset":7512,"ancestor_record_offset":3588,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":549,"record_offset":7512,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":26614,"source_line":150,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":549,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":3588,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":549,"record_offset":4260,"parent_record_offset":4220,"ancestor_record_offset":3588,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":549,"record_offset":4220,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":549,"record_offset":4164,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":549,"record_offset":4316,"parent_record_offset":4220,"ancestor_record_offset":3588,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B01
// 首条位置: {"module":549,"inlinee":31489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":15036,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":14912,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":14720,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":14960,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":549,"record_offset":14692,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":27146,"source_line":94,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":549,"record_offset":13844,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":549,"record_offset":13716,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":549,"record_offset":13688,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":13624,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":13624,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":13624,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13624,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":14260,"parent_record_offset":14220,"ancestor_record_offset":13624,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":14220,"parent_record_offset":14196,"ancestor_record_offset":13624,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":14608,"parent_record_offset":14584,"ancestor_record_offset":13624,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":13624,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":14532,"parent_record_offset":14500,"ancestor_record_offset":13624,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":14476,"parent_record_offset":13940,"ancestor_record_offset":13624,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":13624,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":13624,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":14560,"parent_record_offset":14500,"ancestor_record_offset":13624,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13624,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13624,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":13624,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":13900,"parent_record_offset":13872,"ancestor_record_offset":13624,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":13872,"parent_record_offset":13844,"ancestor_record_offset":13624,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":13940,"parent_record_offset":13872,"ancestor_record_offset":13624,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":13784,"parent_record_offset":13716,"ancestor_record_offset":13624,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":14108,"parent_record_offset":14044,"ancestor_record_offset":13624,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":15004,"parent_record_offset":14960,"ancestor_record_offset":13624,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":14132,"parent_record_offset":14108,"ancestor_record_offset":13624,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":14172,"parent_record_offset":13992,"ancestor_record_offset":13624,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":14660,"parent_record_offset":13844,"ancestor_record_offset":13624,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":15140,"parent_record_offset":15068,"ancestor_record_offset":13624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":13624,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":13624,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":14080,"parent_record_offset":14044,"ancestor_record_offset":13624,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":14400,"parent_record_offset":14220,"ancestor_record_offset":13624,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":15068,"parent_record_offset":15036,"ancestor_record_offset":13624,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":549,"record_offset":13756,"parent_record_offset":13716,"ancestor_record_offset":13624,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":549,"record_offset":13812,"parent_record_offset":13716,"ancestor_record_offset":13624,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::_::impl$0::next
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B03
// 首条位置: {"module":549,"inlinee":31491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B00
// 首条位置: {"module":549,"inlinee":31488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":7120,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4660,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4576,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4412,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":6954,"source_line":221,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":4348,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7916,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":11588,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5308,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8764,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":26675,"source_line":448,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9420,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8104,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9376,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=20
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":9332,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":11136,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":549,"record_offset":10888,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":27146,"source_line":94,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":549,"record_offset":5856,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7300,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7456,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":549,"record_offset":7512,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":26614,"source_line":150,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":549,"record_offset":4220,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":549,"record_offset":4164,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":3744,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3744,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":3744,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":5136,"parent_record_offset":4952,"ancestor_record_offset":3744,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":8688,"parent_record_offset":8632,"ancestor_record_offset":3744,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3744,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4880,"parent_record_offset":4844,"ancestor_record_offset":3744,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":3744,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":3744,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":3744,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3744,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":3744,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4916,"parent_record_offset":4880,"ancestor_record_offset":3744,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":3744,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":4996,"parent_record_offset":4952,"ancestor_record_offset":3744,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":4804,"parent_record_offset":4768,"ancestor_record_offset":3744,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":4768,"parent_record_offset":4696,"ancestor_record_offset":3744,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":8152,"parent_record_offset":8104,"ancestor_record_offset":3744,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":3744,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":7156,"parent_record_offset":7120,"ancestor_record_offset":3744,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":3744,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4740,"parent_record_offset":4696,"ancestor_record_offset":3744,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4844,"parent_record_offset":4804,"ancestor_record_offset":3744,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":3744,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":4696,"parent_record_offset":4660,"ancestor_record_offset":3744,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":3744,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3744,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":549,"record_offset":6544,"parent_record_offset":6512,"ancestor_record_offset":3744,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6488,"parent_record_offset":5952,"ancestor_record_offset":3744,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":3744,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":3744,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":549,"record_offset":6572,"parent_record_offset":6512,"ancestor_record_offset":3744,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":3744,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3744,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":549,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":3744,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5912,"parent_record_offset":5884,"ancestor_record_offset":3744,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5884,"parent_record_offset":5856,"ancestor_record_offset":3744,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":549,"record_offset":5952,"parent_record_offset":5884,"ancestor_record_offset":3744,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":549,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":3744,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":549,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":3744,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":549,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":3744,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":4288,"parent_record_offset":4220,"ancestor_record_offset":3744,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":3744,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":549,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":3744,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":549,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":3744,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":549,"record_offset":7760,"parent_record_offset":7732,"ancestor_record_offset":3744,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":549,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":3744,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":3744,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3744,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":9228,"parent_record_offset":9196,"ancestor_record_offset":3744,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":6120,"parent_record_offset":6056,"ancestor_record_offset":3744,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":11176,"parent_record_offset":11136,"ancestor_record_offset":3744,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":3744,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6184,"parent_record_offset":6004,"ancestor_record_offset":3744,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":6672,"parent_record_offset":5856,"ancestor_record_offset":3744,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":4504,"parent_record_offset":4468,"ancestor_record_offset":3744,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":5236,"parent_record_offset":4804,"ancestor_record_offset":3744,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7684,"parent_record_offset":7512,"ancestor_record_offset":3744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":3744,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":5356,"parent_record_offset":5308,"ancestor_record_offset":3744,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":3744,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":7268,"parent_record_offset":7120,"ancestor_record_offset":3744,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":7616,"parent_record_offset":7512,"ancestor_record_offset":3744,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":549,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":3744,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":3744,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":3744,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":549,"record_offset":6412,"parent_record_offset":6232,"ancestor_record_offset":3744,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3744,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":5384,"parent_record_offset":5308,"ancestor_record_offset":3744,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":8036,"parent_record_offset":7972,"ancestor_record_offset":3744,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":7364,"parent_record_offset":7328,"ancestor_record_offset":3744,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":3744,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":3744,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":549,"record_offset":8796,"parent_record_offset":8764,"ancestor_record_offset":3744,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::dof::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":549,"record_offset":4536,"parent_record_offset":4468,"ancestor_record_offset":3744,"inlinee":31491,"source_line":113,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":549,"record_offset":4468,"parent_record_offset":4440,"ancestor_record_offset":3744,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":7944,"parent_record_offset":7916,"ancestor_record_offset":3744,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":3744,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":3744,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":549,"record_offset":7420,"parent_record_offset":7328,"ancestor_record_offset":3744,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":549,"record_offset":8008,"parent_record_offset":7972,"ancestor_record_offset":3744,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":549,"record_offset":5512,"parent_record_offset":5472,"ancestor_record_offset":3744,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":549,"record_offset":7560,"parent_record_offset":7512,"ancestor_record_offset":3744,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":549,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":3744,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":549,"record_offset":4260,"parent_record_offset":4220,"ancestor_record_offset":3744,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":549,"record_offset":4316,"parent_record_offset":4220,"ancestor_record_offset":3744,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AA3
// 首条位置: {"module":1585,"inlinee":72355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: dof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::HalfResAllDof
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["country_core::systems::render_loop::compute_passes::dof::HalfResAllDof"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::HalfResDof
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["country_core::systems::render_loop::compute_passes::dof::HalfResDof"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::IndirectArgsType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>", "constructor": "enum_iterator::All", "arguments": ["country_core::systems::render_loop::compute_passes::dof::IndirectArgsType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>", "country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u32", "core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/dof.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: dof.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::dof_prepare
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":3108,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":3156,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":549,"record_offset":9332,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_transitive","evidence":{"module":549,"record_offset":9376,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":549,"record_offset":11136,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":14720,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":14912,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":14960,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":9332,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":9376,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":549,"record_offset":11136,"parent_record_offset":3744,"ancestor_record_offset":3744,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00549.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"bcd84b3d810dfcd6e8234ad8","relative_path":"_dof_analyze.cs.hlsl_cs_main_0.bin","sha256":"75510235b057be3be42c635028c53f0affc008dfe68c1482d69fc740e5495b6b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"8a1afe2a4d5c148d0b69c69e","relative_path":"_dof_autofocus.cs.hlsl_cs_main_0.bin","sha256":"dd61ca97962c9cb04e3347f7e354a9da08dbe381d34a1c478e572b752695772c","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"d281faf7f31839b71888edc1","relative_path":"_dof_clear_halfres.cs.hlsl_cs_main_0.bin","sha256":"d47553444054f67f1f9b35bede3cef987034840f82da28cabb81a95b1a87462c","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"10d90a52021e3936967e848e","relative_path":"_dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"98f11465f68f8d84de6919a5a6d494c6dea2e57fe7744f8cd91c77d5e519942a","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"3280d361c8a7a84cf92c1d0c","relative_path":"_dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"3fe811581363d806735e8857e02cd6297429f513df3550ddd8fcdcb82cc9700b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c4b62cd52862fc9544905a65","relative_path":"_dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"a4762a5df0b2244487b75aca3b7f7a97860f1335b09adfe25ac292599971e223","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"0997215182c620efbc8bfb0a","relative_path":"_dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"368a33de5f05c8f61d5d32dd4c18386af95c2aa47ee8f0b84d297a32764a961f","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"31cfc97df500fe29ca2ac7c9","relative_path":"_dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"438967455810fbe6b0f044a25130c9f70a5c7a3194c1a07ea84853c740c734cb","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"f4eb7a83b56a59d253577864","relative_path":"_dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"adeca4e066766fa46bdcf915738bbe5c445c39371c7373c111c32ec533ab7314","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"af1d9e763d952b94aa5c96a4","relative_path":"_dof_fullres_upsample.cs.hlsl_cs_main_0.bin","sha256":"b2deee990e6df11974d6a79993c02658fb209fdce2ce9f6ff1dfc3c4525cf734","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"65a1bf56321b1d9d6155d46c","relative_path":"_dof_fused.cs.hlsl_cs_main_0.bin","sha256":"43c0079571555f0a8a3615335b2ffb0667b23a54527c46e5fcb1583e49922ebd","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"df4e74a22dc74ab454da82e1","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin","sha256":"7211a95403eebaf67b7a3225df029a7afb62a6578ade7b06641d062dc2043e5b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c333923b002dca84f7e75de2","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin","sha256":"4bd9fd35c6253a088148950110db4ec97f8302f7ba0a5c19e7d66bc1c248ef45","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"b60358e7ab1083395f48712e","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin","sha256":"371136e9f1ef0fa0175071a51e323fb1b6c74b5d475808f2010e47a55dd1c3db","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"e61b82f740875882bc890204","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin","sha256":"3c0f85d081a8fa968e746f856b09b0b80f263c646f65e52db0e9504093b97767","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"b318b50906776544c7a448bb","relative_path":"_dof_init_buffers.cs.hlsl_cs_main_0.bin","sha256":"5ba58e0cac151637d80e30bbbc3e28ff0fb4b10d292463211847be199982907c","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"83efbc6ca3d2aefa2cfc8a46","relative_path":"_dof_prepare.cs.hlsl_cs_main_0.bin","sha256":"ca491a564b30bfe3c844c8ae34e183e545c781cd9238c5df980f2b1b5d5a95a9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,61,60,60,61,70,61,60,117,60,117,61,117,61,60,61,61,61,88,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":70,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":117,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":117,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":117,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,70,70,70,70,70,70]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":129,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":131,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":131,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"frame_constants","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[85,85,85,85,85,85,85,85,65,64,64,65,75,65,64,129,64,129,65,129,65,64,65,65,65,85,54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":85,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":85,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":85,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":65,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":64,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":64,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":65,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":75,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":65,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":64,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":129,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":64,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":129,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":65,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":129,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":65,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":64,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":65,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":65,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":65,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":85,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":54,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"$Globals","storage_class":"Uniform","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[64,75,75,75,75,75,75,75]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":64,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"halfres_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[56]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"color_tex","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"img_output","storage_class":"UniformConstant","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":47,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,46]},"decorations":[],"pointee":{"id":46,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":55,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":143,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[57]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":22,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[22]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":144,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,51]},"decorations":[],"pointee":{"id":51,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[58]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":58,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":23,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[23]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":146,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,145]},"decorations":[],"pointee":{"id":145,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[65,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":146,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,145]},"decorations":[],"pointee":{"id":145,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[65,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":147,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":147,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[87,87,87,87,87,87,87,87,60,59,59,60,69,60,59,121,59,121,60,121,60,59,60,60,60,87,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":87,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":87,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":87,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":59,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":59,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":69,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":60,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":59,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":121,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":59,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":121,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":60,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":121,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":60,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":60,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":60,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":87,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[59,69,69,69,69,69,69]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":59,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":130,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":131,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,132]},"decorations":[],"pointee":{"id":132,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[60,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,132]},"decorations":[],"pointee":{"id":132,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[60,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"frame_constants","storage_class":"Uniform","type":{"id":131,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[86,86,86,86,86,86,86,86,65,64,64,65,76,65,64,128,64,128,65,128,65,64,65,65,65,86,54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":86,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":86,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":86,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":65,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":64,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":64,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":65,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":76,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":65,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":64,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":128,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":64,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":128,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":65,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":128,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":65,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":64,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":65,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":65,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":65,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":86,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":54,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"$Globals","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[64,76,76,76,76,76,76,76]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":64,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"halfres_tex_dims","type_id":76,"decorations":[{"decoration":"Offset","operands":[56]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"color_tex","storage_class":"UniformConstant","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"img_output","storage_class":"UniformConstant","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[64,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":47,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":141,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,46]},"decorations":[],"pointee":{"id":46,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":55,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[57]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":22,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[22]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":143,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,51]},"decorations":[],"pointee":{"id":51,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[58]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":58,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":23,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[23]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,144]},"decorations":[],"pointee":{"id":144,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[65,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,144]},"decorations":[],"pointee":{"id":144,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[65,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":146,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":146,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fused.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"frame_constants","storage_class":"Uniform","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,27]},"decorations":[],"pointee":{"id":27,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[66,66,66,66,66,66,66,66,46,45,45,46,59,46,45,52,45,52,46,52,46,45,46,46,46,66,40]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":66,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":46,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":45,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":45,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":46,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":59,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":46,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":45,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":52,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":45,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":52,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":46,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":52,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":46,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":45,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":46,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":46,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":46,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":66,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":40,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"$Globals","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[45,59,59,59,59,59]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":45,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"depth_tex_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"coc_tex_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"img_output_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"color_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"img_output","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,36]},"decorations":[],"pointee":{"id":36,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,110]},"decorations":[],"pointee":{"id":110,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[46,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_prepare.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"frame_constants","storage_class":"Uniform","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[60,60,60,60,60,60,60,60,38,37,37,38,46,38,37,65,37,65,38,65,38,37,38,38,38,60,33]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":60,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":37,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":37,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":46,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":38,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":37,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":65,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":37,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":65,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":38,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":65,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":38,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":37,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":38,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":38,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":60,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":33,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"$Globals","storage_class":"Uniform","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[46,46,46]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":46,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"coc_out_tex_dims","type_id":46,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"tile_output_tex_dims","type_id":46,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"autofocus_buf","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"type.StructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"coc_out_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,27]},"decorations":[],"pointee":{"id":27,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"tile_output_tex","storage_class":"UniformConstant","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,75]},"decorations":[],"pointee":{"id":75,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[38,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_autofocus.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"frame_constants","storage_class":"Uniform","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,20]},"decorations":[],"pointee":{"id":20,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[68,68,68,68,68,68,68,68,46,45,45,46,75,46,45,51,45,51,46,51,46,45,46,46,46,68,35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":68,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":46,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":45,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":45,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":46,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":75,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":46,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":45,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":45,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":51,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":46,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":51,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":46,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":45,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":46,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":46,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":46,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":68,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":35,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"$Globals","storage_class":"Uniform","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[75,50,46,46,46,45,46,45]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"point_uv","type_id":50,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"use_dual_focus","type_id":46,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"focus_lock_request","type_id":46,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"focus_is_locked","type_id":46,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":5,"name":"focus_box_height","type_id":45,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":6,"name":"use_tilt_shift","type_id":46,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":7,"name":"aperture_size","type_id":45,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,23]},"decorations":[],"pointee":{"id":23,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"__alias_autofocus_buf","storage_class":"Uniform","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,27]},"decorations":[],"pointee":{"id":27,"name":"type.RWStructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[36]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"__alias_focus_readback_buf","storage_class":"Uniform","type":{"id":82,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,30]},"decorations":[],"pointee":{"id":30,"name":"type.RWStructuredBuffer.DofFocusReadback","record":{"opcode":"OpTypeStruct","operands":[37]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":37,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,86]},"decorations":[],"pointee":{"id":86,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[46,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_upsample.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,22]},"decorations":[],"pointee":{"id":22,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"img_output","storage_class":"UniformConstant","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[33]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":33,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,62]},"decorations":[],"pointee":{"id":62,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,62]},"decorations":[],"pointee":{"id":62,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_init_buffers.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"push_constants","storage_class":"PushConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.PushConstants","record":{"opcode":"OpTypeStruct","operands":[24]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"args_type_count","type_id":24,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"tile_offsets_initial_values","storage_class":"Uniform","type":{"id":31,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.ConstantBuffer.DofTileOffsetsPacked","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"offsets","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"__alias_dispatch_args_buffer","storage_class":"Uniform","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,16]},"decorations":[],"pointee":{"id":16,"name":"type.RWStructuredBuffer.DofTileOffsetsPacked","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,34]},"decorations":[],"pointee":{"id":34,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres_all","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[75,75,75,75,75,75,75,75,61,60,60,61,70,61,60,121,60,121,61,121,61,60,61,61,61,75,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":75,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":75,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":75,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":70,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":121,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":121,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":121,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":75,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,70,70,70,70,70,70]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_analyze.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"$Globals","storage_class":"Uniform","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[57,57]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"coc_tiles_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"coc_tiles_dilated_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"coc_tiles_tex","storage_class":"UniformConstant","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[45,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"__alias_dispatch_args_buffer","storage_class":"Uniform","type":{"id":82,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"__alias_tile_coords_buffer","storage_class":"Uniform","type":{"id":83,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.RWStructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[36]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,84]},"decorations":[],"pointee":{"id":84,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[46,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[77,77,77,77,77,77,77,77,61,60,60,61,72,61,60,120,60,120,61,120,61,60,61,61,61,77,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":77,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":77,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":77,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":77,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":77,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":77,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":77,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":77,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":72,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":120,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":120,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":120,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":77,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,72,72,72,72,72,72]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":72,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,61,60,60,61,70,61,60,122,60,122,61,122,61,60,61,61,61,88,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":70,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":122,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":122,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":122,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,70,70,70,70,70,70]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":70,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,134]},"decorations":[],"pointee":{"id":134,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_clear_halfres.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2,3]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"img_output","storage_class":"UniformConstant","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[24,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,16]},"decorations":[],"pointee":{"id":16,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[76,76,76,76,76,76,76,76,61,60,60,61,71,61,60,122,60,122,61,122,61,60,61,61,61,76,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":76,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":71,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":122,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":122,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":122,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":76,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,71,71,71,71,71,71]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":133,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,135]},"decorations":[],"pointee":{"id":135,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,135]},"decorations":[],"pointee":{"id":135,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":137,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":137,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres_all","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[76,76,76,76,76,76,76,76,61,60,60,61,71,61,60,119,60,119,61,119,61,60,61,61,61,76,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":76,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":61,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":60,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":71,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":61,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":60,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":119,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":119,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":61,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":119,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":61,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":61,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":61,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":61,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":76,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,71,71,71,71,71,71]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":71,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[60,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":131,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,133]},"decorations":[],"pointee":{"id":133,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":134,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,133]},"decorations":[],"pointee":{"id":133,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[61,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":135,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[87,87,87,87,87,87,87,87,60,59,59,60,69,60,59,116,59,116,60,116,60,59,60,60,60,87,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":87,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":87,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":87,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":87,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":59,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":59,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":69,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":60,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":59,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":116,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":59,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":116,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":60,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":116,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":60,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":60,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":60,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":87,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":50,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"$Globals","storage_class":"Uniform","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[59,69,69,69,69,69,69]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":59,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":69,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"color_tex","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"img_output","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[59,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":126,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[52]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":52,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[53]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":129,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,128]},"decorations":[],"pointee":{"id":128,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[60,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":129,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,128]},"decorations":[],"pointee":{"id":128,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[60,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":130,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":130,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5164070,"position_among_mapped_render_targets":53,"mapped_render_target_count":59,"previous_callsite":5164046,"next_callsite":5164258,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5164297,"position_among_mapped_render_targets":55,"mapped_render_target_count":59,"previous_callsite":5164258,"next_callsite":5164596,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/taa.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 16; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<country_core::systems::ui_move_freehand_walls::MoveWallCache>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<country_core::systems::ui_move_freehand_walls::MoveWallCache>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_freehand_walls::MoveWallCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesNextToWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::_::impl$0::next
// 信息 ID: bca4c3cf6ff35b6295f3b03a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=549, inline_record=4536; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1585, inline_record=213992; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1585, inline_record=214592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof
// 信息 ID: cd701da6acfb7ab61159d172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecd09 -> 0xa5a570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// 信息 ID: d169d41d3fc3e936ff533f13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=549, inline_record=13624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// 信息 ID: 9eb455db67c67cb0a37eade5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=549, inline_record=3744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure$0
// 信息 ID: 85d857049dab9ed2f768281f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::dof::calc_dof_multires", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1585, inline_record=214072; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1585, inline_record=214672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::dof_prepare
// 信息 ID: 460105994e74a9f9769d3479
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecc26 -> 0xa5a020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
