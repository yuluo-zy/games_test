// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d999f7305979f05a28feed7ee633ddbd118b419bbfc13c6e01fcdec8fb30a11c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: common_includes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 495, 497, 498, 502, 514, 521, 528, 529, 530, 531, 532, 536, 538, 540, 542, 545, 547, 550, 551, 552, 589, 1615, 1632, 1638
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
// 记录形式: inline_instance, inlinee_declaration; instances=80
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B53
// 首条位置: {"module":495,"inlinee":27475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=52
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":495,"record_offset":82824,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":26032,"parent_record_offset":26000,"ancestor_record_offset":26000,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x764F
// 首条位置: {"module":529,"inlinee":30287}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":30692,"parent_record_offset":30660,"ancestor_record_offset":30660,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":61936,"parent_record_offset":61904,"ancestor_record_offset":61904,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1632,"record_offset":182352,"rva":23126832,"va":5391835952}
// 编译实例 RVAs: 0x160e330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14120436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e80c8a28ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5391836575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13668652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e8d46e2fff","size":5,"soft_fail":false,"target_va":5378167392,"va":5391836039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140970320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13230250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e8561f36ff","size":5,"soft_fail":false,"target_va":5378605856,"va":5391836101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a994f
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19510231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e8d7b32901","size":5,"soft_fail":false,"target_va":5411346767,"va":5391836531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20011096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e858583101","size":5,"soft_fail":false,"target_va":5411847168,"va":5391836067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17479246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391835952,"raw_hex":"e8b249f5fe","size":5,"soft_fail":false,"target_va":5374357104,"va":5391836345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":182672,"parent_record_offset":182352,"ancestor_record_offset":182352,"inlinee":15628,"source_line":733,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":182624,"parent_record_offset":182352,"ancestor_record_offset":182352,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":183760,"parent_record_offset":183736,"ancestor_record_offset":182352,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":183808,"parent_record_offset":183784,"ancestor_record_offset":182352,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":183784,"parent_record_offset":183760,"ancestor_record_offset":182352,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":183736,"parent_record_offset":183696,"ancestor_record_offset":182352,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":183696,"parent_record_offset":182672,"ancestor_record_offset":182352,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1632,"record_offset":183396,"parent_record_offset":183372,"ancestor_record_offset":182352,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1632,"record_offset":182964,"parent_record_offset":182940,"ancestor_record_offset":182352,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1632,"record_offset":183048,"parent_record_offset":183024,"ancestor_record_offset":182352,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1632,"record_offset":182988,"parent_record_offset":182964,"ancestor_record_offset":182352,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1632,"record_offset":183464,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1632,"record_offset":183196,"parent_record_offset":183168,"ancestor_record_offset":182352,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1632,"record_offset":183168,"parent_record_offset":183140,"ancestor_record_offset":182352,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":183640,"parent_record_offset":182672,"ancestor_record_offset":182352,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1632,"record_offset":183312,"parent_record_offset":183288,"ancestor_record_offset":182352,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":183848,"parent_record_offset":183696,"ancestor_record_offset":182352,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":183108,"parent_record_offset":183080,"ancestor_record_offset":182352,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":183140,"parent_record_offset":183108,"ancestor_record_offset":182352,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":183080,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":182940,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":183492,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":183024,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1632,"record_offset":183372,"parent_record_offset":183348,"ancestor_record_offset":182352,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":182756,"parent_record_offset":182732,"ancestor_record_offset":182352,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":182780,"parent_record_offset":182756,"ancestor_record_offset":182352,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":183348,"parent_record_offset":183240,"ancestor_record_offset":182352,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":183288,"parent_record_offset":183264,"ancestor_record_offset":182352,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":183264,"parent_record_offset":183240,"ancestor_record_offset":182352,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":183584,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":182832,"parent_record_offset":182808,"ancestor_record_offset":182352,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":182808,"parent_record_offset":182780,"ancestor_record_offset":182352,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":182860,"parent_record_offset":182832,"ancestor_record_offset":182352,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":182912,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":183240,"parent_record_offset":182860,"ancestor_record_offset":182352,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1632,"record_offset":183420,"parent_record_offset":183396,"ancestor_record_offset":182352,"inlinee":9602,"source_line":15,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1632,"record_offset":183668,"parent_record_offset":182672,"ancestor_record_offset":182352,"inlinee":9610,"source_line":75,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1632,"record_offset":182732,"parent_record_offset":182672,"ancestor_record_offset":182352,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1638,"record_offset":139380,"rva":23273360,"va":5391982480}
// 编译实例 RVAs: 0x1631f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14281424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391982480,"raw_hex":"e8301526ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391982731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13800344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391982480,"raw_hex":"e8686c2dff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391982563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162a190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391982480,"raw_hex":"e87381ffff","size":5,"soft_fail":false,"target_va":5391950224,"va":5391982616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19864575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391982480,"raw_hex":"e8ff1b2f01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391982588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":139640,"parent_record_offset":139380,"ancestor_record_offset":139380,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1632,"record_offset":183936,"rva":23127600,"va":5391836720}
// 编译实例 RVAs: 0x160e630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14120960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391836720,"raw_hex":"e8008828ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5391837099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13669419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391836720,"raw_hex":"e8d56b2fff","size":5,"soft_fail":false,"target_va":5378167392,"va":5391836806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: common_includes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20010329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391836720,"raw_hex":"e859553101","size":5,"soft_fail":false,"target_va":5411847168,"va":5391836834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":184200,"parent_record_offset":183936,"ancestor_record_offset":183936,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/01632.jsonl.gz"}
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
// 完整结构及原始类型记录: common_includes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: common_includes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: common_includes.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":495,"record_offset":82824,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00495.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":26032,"parent_record_offset":26000,"ancestor_record_offset":26000,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":30692,"parent_record_offset":30660,"ancestor_record_offset":30660,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":61936,"parent_record_offset":61904,"ancestor_record_offset":61904,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":1632,"record_offset":182624,"parent_record_offset":182352,"ancestor_record_offset":182352,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/01632.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::draw_builder::DrawBuilder>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":1638,"record_offset":139640,"parent_record_offset":139380,"ancestor_record_offset":139380,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01638.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":1632,"record_offset":184200,"parent_record_offset":183936,"ancestor_record_offset":183936,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/01632.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":9325760,"caller_aliases":{"alias_group_rva":9325760,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9326266,"position_among_mapped_render_targets":1,"mapped_render_target_count":2,"previous_callsite":null,"next_callsite":9327212,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9325760,"caller_aliases":{"alias_group_rva":9325760,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9327212,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":9326266,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9391360,"caller_aliases":{"alias_group_rva":9391360,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9391648,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10331520,"caller_aliases":{"alias_group_rva":10331520,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10331736,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10332000,"caller_aliases":{"alias_group_rva":10332000,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10332216,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10332480,"caller_aliases":{"alias_group_rva":10332480,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10332696,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10627968,"caller_aliases":{"alias_group_rva":10627968,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10628356,"position_among_mapped_render_targets":1,"mapped_render_target_count":4,"previous_callsite":null,"next_callsite":10629257,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10627968,"caller_aliases":{"alias_group_rva":10627968,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10629257,"position_among_mapped_render_targets":2,"mapped_render_target_count":4,"previous_callsite":10628356,"next_callsite":10629900,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10627968,"caller_aliases":{"alias_group_rva":10627968,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10629900,"position_among_mapped_render_targets":3,"mapped_render_target_count":4,"previous_callsite":10629257,"next_callsite":10630942,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10627968,"caller_aliases":{"alias_group_rva":10627968,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10630942,"position_among_mapped_render_targets":4,"mapped_render_target_count":4,"previous_callsite":10629900,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: common_includes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 41; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
// 信息 ID: e3f43bfc3b2e9e6b89e8f51e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 58 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke; module=495, inline_record=82792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader; module=497, inline_record=26000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader; module=497, inline_record=27824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings; module=498, inline_record=41240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings; module=498, inline_record=42036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings; module=498, inline_record=42312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=77416; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=79440; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
// 信息 ID: 36c797fd5f72f8824da2359c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug; module=529, inline_record=30660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass; module=529, inline_record=37348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=529, inline_record=42232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=529, inline_record=42860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw; module=532, inline_record=61904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=107320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=109020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: 2fcd6739847b2de2022eaf0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute @ 0x8f4e20 -> 0x160e330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: 256d133ec78b2fec9edda752
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw @ 0x9da658 -> 0x1631f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw @ 0x9da838 -> 0x1631f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw @ 0x9daa18 -> 0x1631f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: 397cdaaeb7653d880d89ce1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x8e4eba -> 0x160e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x8e526c -> 0x160e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0xa22d04 -> 0x160e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0xa23089 -> 0x160e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0xa2330c -> 0x160e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0xa2371e -> 0x160e630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
