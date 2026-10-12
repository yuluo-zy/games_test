// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d20ca7fc935e025c282e48f30fe64055ebaed2d574296f3d689ca6da450c3d15
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 459, 460, 464, 465, 466, 1557
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.13.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":460,"record_offset":272,"rva":9052784,"va":5377761904}
// 编译实例 RVAs: 0x8a2270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5562810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377761904,"raw_hex":"e8bae15400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377762209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3947927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377761904,"raw_hex":"e8973d3c00","size":5,"soft_fail":false,"target_va":5381709936,"va":5377762004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377761904,"raw_hex":"e809010000","size":5,"soft_fail":false,"target_va":5377762512,"va":5377762242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":460,"record_offset":532,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":460,"record_offset":476,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":460,"record_offset":644,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":460,"record_offset":588,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":460,"record_offset":860,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":460,"record_offset":916,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":460,"record_offset":1260,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":460,"record_offset":972,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":460,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":460,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":460,"record_offset":824,"parent_record_offset":668,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":460,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":460,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":460,"record_offset":728,"parent_record_offset":696,"ancestor_record_offset":272,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":460,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":272,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":460,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":272,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":460,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":460,"record_offset":1440,"rva":9053392,"va":5377762512}
// 编译实例 RVAs: 0x8a24d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$petgraph..graphmap..Edges$LT$N$C$E$C$Ty$C$S$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12686085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e80593c100","size":5,"soft_fail":false,"target_va":5390448800,"va":5377762710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4283914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e8f6a1beff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377762949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4282835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e82da6beff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377762606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a577d0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1790349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e88d511b00","size":5,"soft_fail":false,"target_va":5379553232,"va":5377762878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c663b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3947843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e8433d3c00","size":5,"soft_fail":false,"target_va":5381710768,"va":5377762920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12688735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e85f9dc100","size":5,"soft_fail":false,"target_va":5390451360,"va":5377762620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38858596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e864ef5002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377763161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1789724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377762512,"raw_hex":"e81c4f1b00","size":5,"soft_fail":false,"target_va":5379552272,"va":5377762543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":460,"record_offset":3568,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":460,"record_offset":3652,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":460,"record_offset":3884,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":460,"record_offset":3976,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":460,"record_offset":3856,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edges
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":460,"record_offset":1656,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":12045,"source_line":494,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":460,"record_offset":3680,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":2596,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":460,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1440,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":460,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1440,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":460,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1440,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":460,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":1440,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":460,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1440,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":460,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1440,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":460,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":1440,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1440,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1440,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":460,"record_offset":3312,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1440,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":3016,"parent_record_offset":2988,"ancestor_record_offset":1440,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":460,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1440,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":460,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":1440,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":460,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1440,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":460,"record_offset":2528,"parent_record_offset":2356,"ancestor_record_offset":1440,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":460,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":1440,"inlinee":8003,"source_line":124,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":460,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":1440,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1440,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":460,"record_offset":2040,"parent_record_offset":1764,"ancestor_record_offset":1440,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":1440,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2988,"parent_record_offset":2956,"ancestor_record_offset":1440,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2928,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2788,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":3340,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2872,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":460,"record_offset":3464,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":460,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":1440,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":460,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":1440,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":460,"record_offset":3196,"parent_record_offset":3088,"ancestor_record_offset":1440,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":460,"record_offset":3532,"parent_record_offset":2632,"ancestor_record_offset":1440,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":1440,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":3432,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":1440,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":2704,"parent_record_offset":2672,"ancestor_record_offset":1440,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":2672,"parent_record_offset":2632,"ancestor_record_offset":1440,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":1440,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":3492,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":460,"record_offset":3088,"parent_record_offset":2732,"ancestor_record_offset":1440,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::inner::Core::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/inner.rs
//     evidence={"module":460,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":1440,"inlinee":9868,"source_line":125,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":460,"record_offset":1800,"parent_record_offset":1764,"ancestor_record_offset":1440,"inlinee":9867,"source_line":193,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":460,"record_offset":1728,"parent_record_offset":1692,"ancestor_record_offset":1440,"inlinee":9866,"source_line":838,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get_index_of
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":460,"record_offset":1764,"parent_record_offset":1728,"ancestor_record_offset":1440,"inlinee":14846,"source_line":885,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":460,"record_offset":2096,"parent_record_offset":1764,"ancestor_record_offset":1440,"inlinee":14822,"source_line":819,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":460,"record_offset":1692,"parent_record_offset":1656,"ancestor_record_offset":1440,"inlinee":12046,"source_line":454,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":460,"record_offset":3752,"parent_record_offset":3716,"ancestor_record_offset":1440,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":460,"record_offset":3716,"parent_record_offset":3680,"ancestor_record_offset":1440,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00460.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::floor_offsets
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AFA
// 首条位置: {"module":459,"inlinee":27386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":459,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":459,"record_offset":7248,"parent_record_offset":7220,"ancestor_record_offset":7076,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":459,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":7076,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":459,"record_offset":7180,"parent_record_offset":7124,"ancestor_record_offset":7076,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":459,"record_offset":7220,"parent_record_offset":7180,"ancestor_record_offset":7076,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":459,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":7076,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":459,"record_offset":7152,"parent_record_offset":7124,"ancestor_record_offset":7076,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":459,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":7076,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":459,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":7076,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":459,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":7076,"inlinee":27388,"source_line":21,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":459,"record_offset":7348,"parent_record_offset":7320,"ancestor_record_offset":7076,"inlinee":27390,"source_line":23,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AFC
// 首条位置: {"module":459,"inlinee":27388}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":459,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":7296,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":459,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":7296,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":459,"record_offset":7348,"parent_record_offset":7320,"ancestor_record_offset":7296,"inlinee":27390,"source_line":23,"archive":"_pdb/C/modules/00459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AFE
// 首条位置: {"module":459,"inlinee":27390}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":459,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00459.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions
// 信息 ID: f02b261c8ef360d485bdf3b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x368164 -> 0x8a2270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37b7a8 -> 0x8a2270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph @ 0x8a7ee0 -> 0x8a2270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions::closure$0
// 信息 ID: eb415d1fb400576a8fb775d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions @ 0x8a23c2 -> 0x8a24d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions @ 0x8a23ea -> 0x8a24d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::floor_offsets
// 信息 ID: b51ccaff8c67c70481ff523d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::gen_stone_stair_floors; module=459, inline_record=7076; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=480; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=636; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
// 信息 ID: 239a44dace03e8960de75b01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::floor_offsets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::gen_stone_stair_floors; module=459, inline_record=7296; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::gen_stone_stair_floors; module=459, inline_record=7468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=700; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=872; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=856; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=1028; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=776; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
// 信息 ID: 68d65aa16c0f809dd2f2f8ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::floor_offsets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::gen_stone_stair_floors; module=459, inline_record=7348; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::gen_stone_stair_floors; module=459, inline_record=7520; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=752; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::assemble_stringers; module=464, inline_record=924; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=908; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=1080; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=828; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings; module=466, inline_record=1000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
