// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/pass_builder/bind_resources.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: dede89f34489e5ece296e54c8f2a69bc659dc6a44037af7b434d4b1b06e3c3c5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bind_resources.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 238, 487, 503, 529, 531, 536, 544, 546, 547, 1632, 1638
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69AA
// 首条位置: {"module":487,"inlinee":27050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36632,"parent_record_offset":36524,"ancestor_record_offset":36524,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36556,"parent_record_offset":36524,"ancestor_record_offset":36524,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":74428,"parent_record_offset":74316,"ancestor_record_offset":74316,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":74352,"parent_record_offset":74316,"ancestor_record_offset":74316,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":36600,"parent_record_offset":36556,"ancestor_record_offset":36524,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":238,"record_offset":22020,"rva":5805792,"va":5374514912}
// 编译实例 RVAs: 0x5896e0, 0x15fc070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b00e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2987585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374514912,"raw_hex":"e8bf69d2ff","size":5,"soft_fail":false,"target_va":5371527392,"va":5374514972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t523408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374514912,"raw_hex":"e890fc0700","size":5,"soft_fail":false,"target_va":5375038752,"va":5374515339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14045808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761520,"raw_hex":"e890ad29ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5391761947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13146257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761520,"raw_hex":"e86f6737ff","size":5,"soft_fail":false,"target_va":5378615328,"va":5391761580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a994f
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36831471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374514912,"raw_hex":"e8ef003202","size":5,"soft_fail":false,"target_va":5411346767,"va":5374515291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-158098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374514912,"raw_hex":"e86e96fdff","size":5,"soft_fail":false,"target_va":5374357104,"va":5374515197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t37331914","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374514912,"raw_hex":"e9caa33902","size":5,"soft_fail":false,"target_va":5411847168,"va":5374515249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":238,"record_offset":22256,"parent_record_offset":22020,"ancestor_record_offset":22020,"inlinee":15628,"source_line":733,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":23340,"parent_record_offset":23316,"ancestor_record_offset":22020,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":22020,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22020,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":23316,"parent_record_offset":23276,"ancestor_record_offset":22020,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":23276,"parent_record_offset":22256,"ancestor_record_offset":22020,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":238,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":22020,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":238,"record_offset":22544,"parent_record_offset":22520,"ancestor_record_offset":22020,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":238,"record_offset":22628,"parent_record_offset":22604,"ancestor_record_offset":22020,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":238,"record_offset":22568,"parent_record_offset":22544,"ancestor_record_offset":22020,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":238,"record_offset":23044,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":238,"record_offset":22776,"parent_record_offset":22748,"ancestor_record_offset":22020,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":238,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":22020,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":23220,"parent_record_offset":22256,"ancestor_record_offset":22020,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":238,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":22020,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":23428,"parent_record_offset":23276,"ancestor_record_offset":22020,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":238,"record_offset":22688,"parent_record_offset":22660,"ancestor_record_offset":22020,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":26636,"parent_record_offset":26608,"ancestor_record_offset":25968,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":238,"record_offset":22720,"parent_record_offset":22688,"ancestor_record_offset":22020,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":26668,"parent_record_offset":26636,"ancestor_record_offset":25968,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":238,"record_offset":22660,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":26608,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":238,"record_offset":22520,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":26468,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":238,"record_offset":23072,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":27020,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":238,"record_offset":22604,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":26552,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":238,"record_offset":22952,"parent_record_offset":22928,"ancestor_record_offset":22020,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":238,"record_offset":22336,"parent_record_offset":22312,"ancestor_record_offset":22020,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":25968,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":238,"record_offset":22360,"parent_record_offset":22336,"ancestor_record_offset":22020,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":26308,"parent_record_offset":26284,"ancestor_record_offset":25968,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":238,"record_offset":22928,"parent_record_offset":22820,"ancestor_record_offset":22020,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":26876,"parent_record_offset":26768,"ancestor_record_offset":25968,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":238,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":22020,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":238,"record_offset":22844,"parent_record_offset":22820,"ancestor_record_offset":22020,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":23164,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":27112,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":22020,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":25968,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":22388,"parent_record_offset":22360,"ancestor_record_offset":22020,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":26336,"parent_record_offset":26308,"ancestor_record_offset":25968,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":22440,"parent_record_offset":22412,"ancestor_record_offset":22020,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":26388,"parent_record_offset":26360,"ancestor_record_offset":25968,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":22492,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":26440,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":238,"record_offset":22820,"parent_record_offset":22440,"ancestor_record_offset":22020,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":26768,"parent_record_offset":26388,"ancestor_record_offset":25968,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":238,"record_offset":23000,"parent_record_offset":22976,"ancestor_record_offset":22020,"inlinee":9602,"source_line":15,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":238,"record_offset":23248,"parent_record_offset":22256,"ancestor_record_offset":22020,"inlinee":9610,"source_line":75,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":238,"record_offset":22312,"parent_record_offset":22256,"ancestor_record_offset":22020,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1638,"record_offset":260,"rva":23234288,"va":5391943408}
// 编译实例 RVAs: 0x16286f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14242816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e800ac26ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391944123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13328210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e8aea034ff","size":5,"soft_fail":false,"target_va":5378615328,"va":5391943533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141524340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1065971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e80dbcefff","size":5,"soft_fail":false,"target_va":5390877504,"va":5391943470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a994f
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19402677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e8b50f2801","size":5,"soft_fail":false,"target_va":5411346767,"va":5391944085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19903204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e8e4b22f01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391943959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17586822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391943408,"raw_hex":"e87aa5f3fe","size":5,"soft_fail":false,"target_va":5374357104,"va":5391943921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":488,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":74749,"source_line":553,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":2852,"parent_record_offset":2812,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":2812,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1638,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":260,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":1108,"parent_record_offset":1052,"ancestor_record_offset":260,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1638,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":260,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1638,"record_offset":1144,"parent_record_offset":1012,"ancestor_record_offset":260,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1638,"record_offset":1572,"parent_record_offset":1536,"ancestor_record_offset":260,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1638,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":260,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1638,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":2756,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1638,"record_offset":1668,"parent_record_offset":1640,"ancestor_record_offset":260,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1638,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":260,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1638,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":74751,"source_line":562,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":1392,"parent_record_offset":1208,"ancestor_record_offset":260,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":260,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":2196,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":916,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1638,"record_offset":1280,"parent_record_offset":1208,"ancestor_record_offset":260,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":2608,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1638,"record_offset":1052,"parent_record_offset":1012,"ancestor_record_offset":260,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":2140,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1638,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":260,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1638,"record_offset":1708,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1638,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":260,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":260,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":260,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":2464,"parent_record_offset":2356,"ancestor_record_offset":260,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":260,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":260,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1736,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":1480,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":260,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":260,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1612,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":260,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":828,"parent_record_offset":788,"ancestor_record_offset":260,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1012,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1208,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1244,"parent_record_offset":1208,"ancestor_record_offset":260,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":788,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":888,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":1512,"parent_record_offset":788,"ancestor_record_offset":260,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":2356,"parent_record_offset":1976,"ancestor_record_offset":260,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1638,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1638,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":260,"inlinee":9602,"source_line":15,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1638,"record_offset":2784,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":9610,"source_line":75,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":628,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":560,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":1836,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":684,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":260,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1632,"record_offset":27416,"rva":23052848,"va":5391761968}
// 编译实例 RVAs: 0x15fc230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22145943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e86914aefe","size":5,"soft_fail":false,"target_va":5369616112,"va":5391762050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14048604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8a4a229ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5391764743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13183948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e834d436ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5391762151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140970120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13157762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e87e3a37ff","size":5,"soft_fail":false,"target_va":5378605344,"va":5391763101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13150262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8ca5737ff","size":5,"soft_fail":false,"target_va":5378612512,"va":5391762769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13147318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e84a6337ff","size":5,"soft_fail":false,"target_va":5378616096,"va":5391763409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13146114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8fe6737ff","size":5,"soft_fail":false,"target_va":5378616352,"va":5391762461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a994f
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19582318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e86ecd2a01","size":5,"soft_fail":false,"target_va":5411346767,"va":5391764444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20084920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8b8783201","size":5,"soft_fail":false,"target_va":5411847152,"va":5391762227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20084733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8fd773201","size":5,"soft_fail":false,"target_va":5411847168,"va":5391762430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-17405591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e86969f6fe","size":5,"soft_fail":false,"target_va":5374357104,"va":5391762690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19777122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e862c62d01","size":5,"soft_fail":false,"target_va":5411541776,"va":5391764649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19718093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8cddf2c01","size":5,"soft_fail":false,"target_va":5411482433,"va":5391764335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19778469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391761968,"raw_hex":"e8a5cb2d01","size":5,"soft_fail":false,"target_va":5411543088,"va":5391764614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5391761968,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5391762258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1632,"record_offset":27668,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":28684,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1632,"record_offset":28444,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":28996,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":15628,"source_line":733,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":33928,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1632,"record_offset":33844,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":28092,"parent_record_offset":28068,"ancestor_record_offset":27416,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":28140,"parent_record_offset":28116,"ancestor_record_offset":27416,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":28116,"parent_record_offset":28092,"ancestor_record_offset":27416,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":30060,"parent_record_offset":30020,"ancestor_record_offset":27416,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":28068,"parent_record_offset":28044,"ancestor_record_offset":27416,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":28044,"parent_record_offset":28004,"ancestor_record_offset":27416,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":30020,"parent_record_offset":28996,"ancestor_record_offset":27416,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":28004,"parent_record_offset":27980,"ancestor_record_offset":27416,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1632,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":27416,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1632,"record_offset":28368,"parent_record_offset":28344,"ancestor_record_offset":27416,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1632,"record_offset":28316,"parent_record_offset":28284,"ancestor_record_offset":27416,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1632,"record_offset":28260,"parent_record_offset":27732,"ancestor_record_offset":27416,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1632,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":27416,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1632,"record_offset":27760,"parent_record_offset":27732,"ancestor_record_offset":27416,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1632,"record_offset":28344,"parent_record_offset":28284,"ancestor_record_offset":27416,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1632,"record_offset":27808,"parent_record_offset":27784,"ancestor_record_offset":27416,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1632,"record_offset":27784,"parent_record_offset":27760,"ancestor_record_offset":27416,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1632,"record_offset":27980,"parent_record_offset":27956,"ancestor_record_offset":27416,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1632,"record_offset":27696,"parent_record_offset":27668,"ancestor_record_offset":27416,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1632,"record_offset":27732,"parent_record_offset":27668,"ancestor_record_offset":27416,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1632,"record_offset":28512,"parent_record_offset":28472,"ancestor_record_offset":27416,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1632,"record_offset":29288,"parent_record_offset":29264,"ancestor_record_offset":27416,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1632,"record_offset":29372,"parent_record_offset":29348,"ancestor_record_offset":27416,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1632,"record_offset":29312,"parent_record_offset":29288,"ancestor_record_offset":27416,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1632,"record_offset":29788,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1632,"record_offset":29520,"parent_record_offset":29492,"ancestor_record_offset":27416,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1632,"record_offset":29492,"parent_record_offset":29464,"ancestor_record_offset":27416,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1632,"record_offset":27896,"parent_record_offset":27836,"ancestor_record_offset":27416,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":34100,"parent_record_offset":34036,"ancestor_record_offset":27416,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":33976,"parent_record_offset":33928,"ancestor_record_offset":27416,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":27916,"parent_record_offset":27896,"ancestor_record_offset":27416,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":29964,"parent_record_offset":28996,"ancestor_record_offset":27416,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":27956,"parent_record_offset":27784,"ancestor_record_offset":27416,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1632,"record_offset":35156,"parent_record_offset":35132,"ancestor_record_offset":27416,"inlinee":9631,"source_line":1475,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1632,"record_offset":29636,"parent_record_offset":29612,"ancestor_record_offset":27416,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":30172,"parent_record_offset":30020,"ancestor_record_offset":27416,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1632,"record_offset":28596,"parent_record_offset":28568,"ancestor_record_offset":27416,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1632,"record_offset":28624,"parent_record_offset":28596,"ancestor_record_offset":27416,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1632,"record_offset":27836,"parent_record_offset":27808,"ancestor_record_offset":27416,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":27416,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":35064,"parent_record_offset":34036,"ancestor_record_offset":27416,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":27868,"parent_record_offset":27836,"ancestor_record_offset":27416,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":28184,"parent_record_offset":28004,"ancestor_record_offset":27416,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":35092,"parent_record_offset":35064,"ancestor_record_offset":27416,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1632,"record_offset":28540,"parent_record_offset":28512,"ancestor_record_offset":27416,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":33868,"parent_record_offset":33844,"ancestor_record_offset":27416,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":33892,"parent_record_offset":33868,"ancestor_record_offset":27416,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1632,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":27416,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1632,"record_offset":34036,"parent_record_offset":34004,"ancestor_record_offset":27416,"inlinee":9606,"source_line":53,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1632,"record_offset":34004,"parent_record_offset":33928,"ancestor_record_offset":27416,"inlinee":14246,"source_line":37,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1632,"record_offset":34128,"parent_record_offset":34100,"ancestor_record_offset":27416,"inlinee":9608,"source_line":65,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1632,"record_offset":35132,"parent_record_offset":34036,"ancestor_record_offset":27416,"inlinee":9630,"source_line":43,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":27416,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":29464,"parent_record_offset":29432,"ancestor_record_offset":27416,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1632,"record_offset":29404,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":29264,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":29816,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1632,"record_offset":29348,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1632,"record_offset":29696,"parent_record_offset":29672,"ancestor_record_offset":27416,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":27416,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":29104,"parent_record_offset":29080,"ancestor_record_offset":27416,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1632,"record_offset":29672,"parent_record_offset":29564,"ancestor_record_offset":27416,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29612,"parent_record_offset":29588,"ancestor_record_offset":27416,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29588,"parent_record_offset":29564,"ancestor_record_offset":27416,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29908,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":27416,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29132,"parent_record_offset":29104,"ancestor_record_offset":27416,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29184,"parent_record_offset":29156,"ancestor_record_offset":27416,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29236,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1632,"record_offset":29564,"parent_record_offset":29184,"ancestor_record_offset":27416,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1632,"record_offset":29744,"parent_record_offset":29720,"ancestor_record_offset":27416,"inlinee":9602,"source_line":15,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1632,"record_offset":29992,"parent_record_offset":28996,"ancestor_record_offset":27416,"inlinee":9610,"source_line":75,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1632,"record_offset":29056,"parent_record_offset":28996,"ancestor_record_offset":27416,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1638,"record_offset":3004,"rva":23235024,"va":5391944144}
// 编译实例 RVAs: 0x16289d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14246594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e83e9d26ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391947901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13366519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8090b34ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5391944722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140970120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13340861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8436f34ff","size":5,"soft_fail":false,"target_va":5378605344,"va":5391946200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13333135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8718d34ff","size":5,"soft_fail":false,"target_va":5378612512,"va":5391945642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13328762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8869e34ff","size":5,"soft_fail":false,"target_va":5378616352,"va":5391945109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141524340
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1067535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8f1b5efff","size":5,"soft_fail":false,"target_va":5390877504,"va":5391945034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162c290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8e12e0000","size":5,"soft_fail":false,"target_va":5391958672,"va":5391946666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a994f
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19399194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e81a022801","size":5,"soft_fail":false,"target_va":5411346767,"va":5391947568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19902349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e88daf2f01","size":5,"soft_fail":false,"target_va":5411847152,"va":5391944798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19902165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e8d5ae2f01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391944998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17588405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e84b9ff3fe","size":5,"soft_fail":false,"target_va":5374357104,"va":5391945504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19594019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e823fb2a01","size":5,"soft_fail":false,"target_va":5411541776,"va":5391947752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19535155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e833152a01","size":5,"soft_fail":false,"target_va":5411482433,"va":5391947273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19595366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e866002b01","size":5,"soft_fail":false,"target_va":5411543088,"va":5391947717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19592282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e85af42a01","size":5,"soft_fail":false,"target_va":5411539504,"va":5391947217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19592586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391944144,"raw_hex":"e88af52a01","size":5,"soft_fail":false,"target_va":5411540288,"va":5391947697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5391944144,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5391944829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1638,"record_offset":3248,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":6984,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1638,"record_offset":6744,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":7296,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":74749,"source_line":553,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":15020,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1638,"record_offset":14936,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":3004,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":3004,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1638,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":3004,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":9696,"parent_record_offset":9656,"ancestor_record_offset":3004,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":3004,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":3004,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":9656,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1638,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":3004,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":1638,"record_offset":3296,"parent_record_offset":3248,"ancestor_record_offset":3004,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":1638,"record_offset":4296,"parent_record_offset":4252,"ancestor_record_offset":3004,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":1638,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":3004,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1638,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":3004,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1638,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":3004,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1638,"record_offset":6616,"parent_record_offset":6584,"ancestor_record_offset":3004,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1638,"record_offset":6560,"parent_record_offset":6028,"ancestor_record_offset":3004,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1638,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":3004,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1638,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":3004,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1638,"record_offset":6644,"parent_record_offset":6584,"ancestor_record_offset":3004,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1638,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3004,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1638,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3004,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1638,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":3004,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1638,"record_offset":6028,"parent_record_offset":3248,"ancestor_record_offset":3004,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":1638,"record_offset":3344,"parent_record_offset":3296,"ancestor_record_offset":3004,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":1638,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":3004,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1638,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3004,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1638,"record_offset":6812,"parent_record_offset":6772,"ancestor_record_offset":3004,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":3004,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":3004,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1638,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":3004,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1638,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":3004,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1638,"record_offset":7988,"parent_record_offset":7856,"ancestor_record_offset":3004,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1638,"record_offset":8416,"parent_record_offset":8380,"ancestor_record_offset":3004,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1638,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":3004,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1638,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3004,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1638,"record_offset":6192,"parent_record_offset":6132,"ancestor_record_offset":3004,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":15192,"parent_record_offset":15128,"ancestor_record_offset":3004,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":15068,"parent_record_offset":15020,"ancestor_record_offset":3004,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":3004,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":9600,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":5612,"parent_record_offset":4296,"ancestor_record_offset":3004,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":4252,"parent_record_offset":3296,"ancestor_record_offset":3004,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1638,"record_offset":6000,"parent_record_offset":3248,"ancestor_record_offset":3004,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1638,"record_offset":5668,"parent_record_offset":4296,"ancestor_record_offset":3004,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1638,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":3004,"inlinee":9631,"source_line":1475,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1638,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":3004,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1638,"record_offset":8604,"parent_record_offset":8580,"ancestor_record_offset":3004,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1638,"record_offset":8628,"parent_record_offset":8604,"ancestor_record_offset":3004,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1638,"record_offset":6896,"parent_record_offset":6868,"ancestor_record_offset":3004,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1638,"record_offset":6924,"parent_record_offset":6896,"ancestor_record_offset":3004,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1638,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":3004,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1638,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":3004,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1638,"record_offset":16156,"parent_record_offset":15128,"ancestor_record_offset":3004,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1638,"record_offset":6164,"parent_record_offset":6132,"ancestor_record_offset":3004,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1638,"record_offset":6484,"parent_record_offset":6304,"ancestor_record_offset":3004,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1638,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":3004,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1638,"record_offset":6840,"parent_record_offset":6812,"ancestor_record_offset":3004,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":3004,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":4508,"parent_record_offset":4388,"ancestor_record_offset":3004,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":5760,"parent_record_offset":5724,"ancestor_record_offset":3004,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":4424,"parent_record_offset":4388,"ancestor_record_offset":3004,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":3004,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3004,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3004,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1638,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":3004,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1638,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":3004,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1638,"record_offset":6772,"parent_record_offset":6744,"ancestor_record_offset":3004,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1638,"record_offset":7484,"parent_record_offset":7448,"ancestor_record_offset":3004,"inlinee":74751,"source_line":562,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1638,"record_offset":15128,"parent_record_offset":15096,"ancestor_record_offset":3004,"inlinee":9606,"source_line":53,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1638,"record_offset":15096,"parent_record_offset":15020,"ancestor_record_offset":3004,"inlinee":14246,"source_line":37,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1638,"record_offset":15220,"parent_record_offset":15192,"ancestor_record_offset":3004,"inlinee":9608,"source_line":65,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1638,"record_offset":16224,"parent_record_offset":15128,"ancestor_record_offset":3004,"inlinee":9630,"source_line":43,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":1638,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":3004,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":3724,"parent_record_offset":3696,"ancestor_record_offset":3004,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":3760,"parent_record_offset":3724,"ancestor_record_offset":3004,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1638,"record_offset":3696,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":3556,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1638,"record_offset":8124,"parent_record_offset":8052,"ancestor_record_offset":3004,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":3860,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1638,"record_offset":7896,"parent_record_offset":7856,"ancestor_record_offset":3004,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1638,"record_offset":3640,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1638,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":3004,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1638,"record_offset":8552,"parent_record_offset":7576,"ancestor_record_offset":3004,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1638,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":3004,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":3004,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":3004,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1638,"record_offset":4084,"parent_record_offset":3976,"ancestor_record_offset":3004,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":3004,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":9224,"parent_record_offset":9200,"ancestor_record_offset":3004,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":3004,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":8580,"parent_record_offset":7576,"ancestor_record_offset":3004,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3944,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3004,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":3004,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":3004,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":7576,"parent_record_offset":7544,"ancestor_record_offset":3004,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":8456,"parent_record_offset":7576,"ancestor_record_offset":3004,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":3004,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":7672,"parent_record_offset":7628,"ancestor_record_offset":3004,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":7856,"parent_record_offset":7672,"ancestor_record_offset":3004,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":8052,"parent_record_offset":7672,"ancestor_record_offset":3004,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":8088,"parent_record_offset":8052,"ancestor_record_offset":3004,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":7628,"parent_record_offset":7576,"ancestor_record_offset":3004,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3528,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1638,"record_offset":8356,"parent_record_offset":7628,"ancestor_record_offset":3004,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1638,"record_offset":3976,"parent_record_offset":3476,"ancestor_record_offset":3004,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1638,"record_offset":7544,"parent_record_offset":7512,"ancestor_record_offset":3004,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1638,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":3004,"inlinee":9602,"source_line":15,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1638,"record_offset":9628,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":9610,"source_line":75,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":7448,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":7376,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":8680,"parent_record_offset":7296,"ancestor_record_offset":3004,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":7512,"parent_record_offset":7448,"ancestor_record_offset":3004,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1638,"record_offset":7404,"parent_record_offset":7376,"ancestor_record_offset":3004,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1638,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3004,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1638,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":3004,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":1638,"record_offset":4352,"parent_record_offset":4296,"ancestor_record_offset":3004,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":1638,"record_offset":4568,"parent_record_offset":4352,"ancestor_record_offset":3004,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":1638,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":3004,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":1638,"record_offset":4388,"parent_record_offset":4352,"ancestor_record_offset":3004,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":1638,"record_offset":5724,"parent_record_offset":5696,"ancestor_record_offset":3004,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/01638.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":1638,"record_offset":4480,"parent_record_offset":4388,"ancestor_record_offset":3004,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/01638.jsonl.gz"}
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
// 完整结构及原始类型记录: bind_resources.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bind_resources.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bind_resources.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: 25d6be7849476987dbbc8615
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_color_library -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String> @ 0x587baf -> 0x5896e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x5883a0 -> 0x5896e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fd565 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fde75 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights> @ 0x15fe785 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0x15ff095 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketMain> @ 0x15ff9a5 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0x16002b5 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0x1600bb4 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::taa::TgsrMain> @ 0x16014c1 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketBush> @ 0x1601dc5 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0x16026d5 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> @ 0x1602fe7 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x16038f8 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0x1604205 -> 0x15fc070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: d1a607d751359336146c460e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_color_library -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common @ 0x90b7f4 -> 0x16286f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x9d2ce5 -> 0x16286f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: 715df2314db5c0debb16d7fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug @ 0x9bef27 -> 0x15fc230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x9c037b -> 0x15fc230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c19af -> 0x15fc230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c1e2d -> 0x15fc230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0xa0232d -> 0x15fc230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0xa02dd9 -> 0x15fc230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: 26926a7c0012b9349d62983a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x9d2cfe -> 0x16289d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw @ 0x9da0d9 -> 0x16289d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
// 信息 ID: a9fdf56dc2e2450f8afe8e5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics; module=487, inline_record=36524; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common; module=503, inline_record=74316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug; module=529, inline_record=30792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=529, inline_record=40996; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=77908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=106664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=108168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows; module=544, inline_record=75776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
