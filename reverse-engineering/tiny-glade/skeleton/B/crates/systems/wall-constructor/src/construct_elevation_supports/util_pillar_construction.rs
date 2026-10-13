// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f8346e226995024f45f626252d050908968b4c6e527987eefa1d4158d563e9f2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util_pillar_construction.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3041, 4071
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3041,"record_offset":272,"rva":33431888,"va":5402141008}
// 编译实例 RVAs: 0x1fe2150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32874515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8ed5f0afe","size":5,"soft_fail":false,"target_va":5369267664,"va":5402142174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27280167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8d9bc5ffe","size":5,"soft_fail":false,"target_va":5374864432,"va":5402144594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27062134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e88a1063fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402143633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24399988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e88caf8bfe","size":5,"soft_fail":false,"target_va":5377742176,"va":5402142159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20259456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e880ddcafe","size":5,"soft_fail":false,"target_va":5381883040,"va":5402142491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20207062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e82aaacbfe","size":5,"soft_fail":false,"target_va":5381934560,"va":5402141617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14517660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8647a22ff","size":5,"soft_fail":false,"target_va":5387623488,"va":5402141143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14508582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8da9d22ff","size":5,"soft_fail":false,"target_va":5387633712,"va":5402142289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142418f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4416120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e878624300","size":5,"soft_fail":false,"target_va":5406560080,"va":5402143955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9705940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8d4199400","size":5,"soft_fail":false,"target_va":5411847152,"va":5402141207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26628218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e886af69fe","size":5,"soft_fail":false,"target_va":5375513472,"va":5402141685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9397394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e892648f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5402144377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9338210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8627d8e00","size":5,"soft_fail":false,"target_va":5411482433,"va":5402144218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9348131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e823a48e00","size":5,"soft_fail":false,"target_va":5411492368,"va":5402144232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9350545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e891ad8e00","size":5,"soft_fail":false,"target_va":5411494800,"va":5402144250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20317682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e80efac9fe","size":5,"soft_fail":false,"target_va":5381823600,"va":5402141277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20320016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8f0f0c9fe","size":5,"soft_fail":false,"target_va":5381824016,"va":5402144027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20258332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e8e4e1cafe","size":5,"soft_fail":false,"target_va":5381884480,"va":5402142807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20255895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402141008,"raw_hex":"e869ebcafe","size":5,"soft_fail":false,"target_va":5381885904,"va":5402141794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5402141008,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5402141227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3041,"record_offset":4324,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3041,"record_offset":1284,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3041,"record_offset":1484,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3041,"record_offset":5312,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3041,"record_offset":2372,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3041,"record_offset":2316,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3041,"record_offset":2684,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3041,"record_offset":2828,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3041,"record_offset":2288,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3041,"record_offset":1396,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":816,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":5072,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3041,"record_offset":1312,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3041,"record_offset":576,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":852,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3041,"record_offset":2884,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":30971,"source_line":1248,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3041,"record_offset":2940,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":4084,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3084,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3320,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":2996,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3408,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3041,"record_offset":2716,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3041,"record_offset":5460,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs
//     evidence={"module":3041,"record_offset":2344,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":106176,"source_line":42,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3041,"record_offset":1676,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3041,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3041,"record_offset":5236,"parent_record_offset":5144,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":1556,"parent_record_offset":1524,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3041,"record_offset":1524,"parent_record_offset":1484,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3041,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3041,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3041,"record_offset":2500,"parent_record_offset":2400,"ancestor_record_offset":272,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3041,"record_offset":644,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3041,"record_offset":2196,"parent_record_offset":1704,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3041,"record_offset":1996,"parent_record_offset":1944,"ancestor_record_offset":272,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3041,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":272,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3041,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3041,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3041,"record_offset":5204,"parent_record_offset":5172,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3041,"record_offset":2796,"parent_record_offset":2716,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3041,"record_offset":2768,"parent_record_offset":2716,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3041,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":272,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3041,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3041,"record_offset":2444,"parent_record_offset":2400,"ancestor_record_offset":272,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3041,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":272,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":1116,"parent_record_offset":852,"ancestor_record_offset":272,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":928,"parent_record_offset":852,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":1152,"parent_record_offset":852,"ancestor_record_offset":272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3041,"record_offset":2612,"parent_record_offset":2524,"ancestor_record_offset":272,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3041,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3041,"record_offset":5372,"parent_record_offset":5312,"ancestor_record_offset":272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3041,"record_offset":5400,"parent_record_offset":5312,"ancestor_record_offset":272,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3041,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":1644,"parent_record_offset":1484,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3041,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3041,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":272,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3041,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":272,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3041,"record_offset":1088,"parent_record_offset":852,"ancestor_record_offset":272,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3041,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":272,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3041,"record_offset":2556,"parent_record_offset":2524,"ancestor_record_offset":272,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3041,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3041,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3041,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":272,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":5952,"parent_record_offset":5888,"ancestor_record_offset":272,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":272,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":272,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3041,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":272,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":272,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":272,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":3041,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":272,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":3041,"record_offset":1760,"parent_record_offset":1704,"ancestor_record_offset":272,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3041,"record_offset":1848,"parent_record_offset":1760,"ancestor_record_offset":272,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3041,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":272,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3041,"record_offset":1944,"parent_record_offset":1704,"ancestor_record_offset":272,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3184,"parent_record_offset":3148,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3148,"parent_record_offset":3112,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":3112,"parent_record_offset":3084,"ancestor_record_offset":272,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3041,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3041,"record_offset":964,"parent_record_offset":852,"ancestor_record_offset":272,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3041,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":272,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3041,"record_offset":1056,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3041,"record_offset":5924,"parent_record_offset":5888,"ancestor_record_offset":272,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3041,"record_offset":3220,"parent_record_offset":3184,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EC0
// 首条位置: {"module":3041,"inlinee":106176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E80F
// 首条位置: {"module":4071,"inlinee":124943}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
// 记录形式: compiled_procedure; instances=1
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3041,"record_offset":6764,"rva":33436224,"va":5402145344}
// 编译实例 RVAs: 0x1fe3240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27281738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8b6b65ffe","size":5,"soft_fail":false,"target_va":5374864432,"va":5402146165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27064376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8c80763fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402145875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14521918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8c26922ff","size":5,"soft_fail":false,"target_va":5387623488,"va":5402145401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142418f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4414275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8435b4300","size":5,"soft_fail":false,"target_va":5406560080,"va":5402145800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_pillar_construction.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9701676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e82c099400","size":5,"soft_fail":false,"target_va":5411847152,"va":5402145471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9395786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e84a5e8f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5402145985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9336491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8ab768e00","size":5,"soft_fail":false,"target_va":5411482433,"va":5402145937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9346410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e86a9d8e00","size":5,"soft_fail":false,"target_va":5411492368,"va":5402145953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9348824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8d8a68e00","size":5,"soft_fail":false,"target_va":5411494800,"va":5402145971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20321950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e862e9c9fe","size":5,"soft_fail":false,"target_va":5381823600,"va":5402145545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20321824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402145344,"raw_hex":"e8e0e9c9fe","size":5,"soft_fail":false,"target_va":5381824016,"va":5402145835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5402145344,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5402145491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":7204,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":7836,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3041,"record_offset":7668,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3041,"record_offset":6968,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":7240,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3041,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":6764,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3041,"record_offset":8184,"parent_record_offset":8096,"ancestor_record_offset":6764,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":8124,"parent_record_offset":8096,"ancestor_record_offset":6764,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":6764,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3041,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":6764,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3041,"record_offset":7036,"parent_record_offset":6996,"ancestor_record_offset":6764,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3041,"record_offset":7592,"parent_record_offset":7564,"ancestor_record_offset":6764,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3041,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":6764,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":7500,"parent_record_offset":7240,"ancestor_record_offset":6764,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":7312,"parent_record_offset":7240,"ancestor_record_offset":6764,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3041,"record_offset":7536,"parent_record_offset":7240,"ancestor_record_offset":6764,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3041,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":6764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3041,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":6764,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3041,"record_offset":7716,"parent_record_offset":7668,"ancestor_record_offset":6764,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3041,"record_offset":7120,"parent_record_offset":7092,"ancestor_record_offset":6764,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3041,"record_offset":7148,"parent_record_offset":7120,"ancestor_record_offset":6764,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3041,"record_offset":7472,"parent_record_offset":7240,"ancestor_record_offset":6764,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3041,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":6764,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3041,"record_offset":7748,"parent_record_offset":7668,"ancestor_record_offset":6764,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3041,"record_offset":7064,"parent_record_offset":7036,"ancestor_record_offset":6764,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":7956,"parent_record_offset":7892,"ancestor_record_offset":6764,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":7384,"parent_record_offset":7348,"ancestor_record_offset":6764,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":6764,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3041,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":6764,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3041,"record_offset":6996,"parent_record_offset":6968,"ancestor_record_offset":6764,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":7564,"parent_record_offset":7536,"ancestor_record_offset":6764,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3041,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":6764,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3041,"record_offset":7348,"parent_record_offset":7240,"ancestor_record_offset":6764,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3041,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":6764,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3041,"record_offset":7440,"parent_record_offset":7348,"ancestor_record_offset":6764,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/03041.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3041,"record_offset":7928,"parent_record_offset":7892,"ancestor_record_offset":6764,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/03041.jsonl.gz"}
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
// 完整结构及原始类型记录: util_pillar_construction.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_elevation_supports::util_pillar_construction::SupportPillar
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::util_pillar_construction::SupportPillar,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_elevation_supports::util_pillar_construction::SupportPillar", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util_pillar_construction.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util_pillar_construction.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
// 信息 ID: 3dfd61b3d0c16094ef10e6d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124c47f -> 0x1fe3240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x125058c -> 0x1fe3240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
// 信息 ID: 6c14c8b40dcf0014162e3b7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124c408 -> 0x1fe2150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x1250515 -> 0x1fe2150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$0
// 信息 ID: 7261eab58154c10ec1481280
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base; module=3041, inline_record=2344; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base; module=3041, inline_record=4296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$1
// 信息 ID: 722b5bfab21bd0c2618e4f9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1 <- system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> >; module=4071, inline_record=4404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
