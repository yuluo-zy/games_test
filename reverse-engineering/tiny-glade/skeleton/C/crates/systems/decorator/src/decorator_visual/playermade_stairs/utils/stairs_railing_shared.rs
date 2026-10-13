// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_railing_shared.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: db30eacec293353ca05817e128a60925b6611363fe32333cc752f7f98d65a684
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_railing_shared.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 463, 465, 466, 3187
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.13.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::side_positions
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B61
// 首条位置: {"module":466,"inlinee":27489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::set_side_position
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B62
// 首条位置: {"module":466,"inlinee":27490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B64
// 首条位置: {"module":463,"record_offset":272,"rva":9083792,"va":5377792912}
// 编译实例 RVAs: 0x8a9b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408a9d20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377792912,"raw_hex":"e89e000000","size":5,"soft_fail":false,"target_va":5377793312,"va":5377793149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a6850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12569425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377792912,"raw_hex":"e851cbbf00","size":5,"soft_fail":false,"target_va":5390362704,"va":5377793274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":464,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":463,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":463,"record_offset":552,"rva":9084192,"va":5377793312}
// 编译实例 RVAs: 0x8a9d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4313910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8ca2cbeff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377793681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ccc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4313414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8ba2ebeff","size":5,"soft_fail":false,"target_va":5373480128,"va":5377793537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5531108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8e4655400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377793911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12657660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8fc23c100","size":5,"soft_fail":false,"target_va":5390451360,"va":5377793695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railing_shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12658012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e85c25c100","size":5,"soft_fail":false,"target_va":5390451568,"va":5377793551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38827232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8e0745002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377794525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1758651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8bbd51a00","size":5,"soft_fail":false,"target_va":5379552272,"va":5377793616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1704620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8ac021a00","size":5,"soft_fail":false,"target_va":5379498400,"va":5377793775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1372351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377793312,"raw_hex":"e8bff01400","size":5,"soft_fail":false,"target_va":5379166640,"va":5377794284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":463,"record_offset":2740,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":2264,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":16567,"source_line":225,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":463,"record_offset":3128,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":2768,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":2404,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":2584,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":2980,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":2676,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edge_weight
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":463,"record_offset":792,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9864,"source_line":525,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":463,"record_offset":1524,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":12046,"source_line":454,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":463,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":463,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":463,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":552,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":552,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":992,"parent_record_offset":964,"ancestor_record_offset":552,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":552,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":552,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":552,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":552,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":3012,"parent_record_offset":2980,"ancestor_record_offset":552,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":463,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":552,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":552,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":552,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":463,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":552,"inlinee":14084,"source_line":30,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":463,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":552,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":463,"record_offset":1176,"parent_record_offset":896,"ancestor_record_offset":552,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::inner::Core::as_entries
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/inner.rs
//     evidence={"module":463,"record_offset":964,"parent_record_offset":936,"ancestor_record_offset":552,"inlinee":9868,"source_line":125,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::as_entries
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":463,"record_offset":936,"parent_record_offset":896,"ancestor_record_offset":552,"inlinee":9867,"source_line":193,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":463,"record_offset":852,"parent_record_offset":792,"ancestor_record_offset":552,"inlinee":9866,"source_line":838,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get_index_of
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":463,"record_offset":896,"parent_record_offset":852,"ancestor_record_offset":552,"inlinee":14846,"source_line":885,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":463,"record_offset":1260,"parent_record_offset":896,"ancestor_record_offset":552,"inlinee":14822,"source_line":819,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::edge_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":463,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":552,"inlinee":9865,"source_line":234,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":463,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":552,"inlinee":11051,"source_line":719,"archive":"_pdb/C/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCC2
// 首条位置: {"module":3187,"inlinee":117954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3187,"record_offset":740,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3187,"record_offset":712,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3187,"record_offset":648,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3187,"record_offset":1036,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3187,"record_offset":1148,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3187,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":552,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3187,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":552,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3187,"record_offset":680,"parent_record_offset":648,"ancestor_record_offset":552,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3187,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":552,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3187,"record_offset":840,"parent_record_offset":804,"ancestor_record_offset":552,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3187,"record_offset":804,"parent_record_offset":764,"ancestor_record_offset":552,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3187,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":552,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/03187.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3187,"record_offset":876,"parent_record_offset":840,"ancestor_record_offset":552,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03187.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_railing_shared.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::array::iter::IntoIter<bool,2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::array::iter::IntoIter<bool,2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_railing_shared.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_railing_shared.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::set_side_position
// 信息 ID: 629c4dea4846a3f6e0bb7177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=2168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement::side_positions
// 信息 ID: 4edc6a77a7dcb866ffe5aeb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=3816; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=3940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements
// 信息 ID: 3b78191e25a5dacb3159d2e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x369cf6 -> 0x8a9b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37d041 -> 0x8a9b90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips; module=465, inline_record=15812; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure$0
// 信息 ID: e6dda733940b923509c2cdf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements @ 0x8a9c7d -> 0x8a9d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements @ 0x8a9c97 -> 0x8a9d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips @ 0x8b1dfc -> 0x8a9d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips @ 0x8b1e18 -> 0x8a9d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure$1
// 信息 ID: 11139c2978e103f903025cc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::RailingTipPlacement,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::array::iter::IntoIter<bool,2> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<usize,bool> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<usize,bool> >,system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements::closure_env$1>; module=3187, inline_record=552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
