// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 29613eaed9fba8cbf95716e1ae06cc1dbb5d96a31700b82798327ca99925a344
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_wall_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 468, 1559, 1608
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":468,"record_offset":412,"rva":9163584,"va":5377872704}
// 编译实例 RVAs: 0x8bd340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5401073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8f1695200","size":5,"soft_fail":false,"target_va":5383273968,"va":5377872890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5410113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8418d5200","size":5,"soft_fail":false,"target_va":5383283184,"va":5377873066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5451404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e88c2e5300","size":5,"soft_fail":false,"target_va":5383325056,"va":5377873647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414991f0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12434549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e875bcbd00","size":5,"soft_fail":false,"target_va":5390307824,"va":5377873270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499930
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12436441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8d9c3bd00","size":5,"soft_fail":false,"target_va":5390309680,"va":5377873234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38748393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8e9404f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377873810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38747911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8073f4f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377873846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3358701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e8ed3f3300","size":5,"soft_fail":false,"target_va":5381231744,"va":5377873038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5409080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377872704,"raw_hex":"e838895200","size":5,"soft_fail":false,"target_va":5383282224,"va":5377873139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":2416,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":468,"record_offset":588,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":468,"record_offset":984,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":468,"record_offset":924,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":468,"record_offset":868,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":27587,"source_line":221,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":1080,"parent_record_offset":984,"ancestor_record_offset":412,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":412,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":412,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":696,"parent_record_offset":588,"ancestor_record_offset":412,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":640,"parent_record_offset":588,"ancestor_record_offset":412,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":412,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":468,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":412,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":468,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":412,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1332,"parent_record_offset":1300,"ancestor_record_offset":412,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":412,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":412,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":412,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":1164,"parent_record_offset":984,"ancestor_record_offset":412,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":468,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":412,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":468,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":412,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":468,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":412,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":468,"record_offset":1228,"parent_record_offset":984,"ancestor_record_offset":412,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::gothic_hat_visible_height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BC6
// 首条位置: {"module":468,"inlinee":27590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":10080,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9104,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9600,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":468,"record_offset":9168,"parent_record_offset":9136,"ancestor_record_offset":9056,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9200,"parent_record_offset":9168,"ancestor_record_offset":9056,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":9056,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9296,"parent_record_offset":9264,"ancestor_record_offset":9056,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9528,"parent_record_offset":9392,"ancestor_record_offset":9056,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":10008,"parent_record_offset":9888,"ancestor_record_offset":9056,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9392,"parent_record_offset":9360,"ancestor_record_offset":9056,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":9056,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":9488,"parent_record_offset":9460,"ancestor_record_offset":9056,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9360,"parent_record_offset":9328,"ancestor_record_offset":9056,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":9056,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":9056,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":468,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":9056,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":468,"record_offset":9432,"parent_record_offset":9392,"ancestor_record_offset":9056,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":468,"record_offset":9264,"parent_record_offset":9232,"ancestor_record_offset":9056,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":468,"record_offset":9328,"parent_record_offset":9296,"ancestor_record_offset":9056,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":468,"record_offset":2620,"rva":9164736,"va":5377873856}
// 编译实例 RVAs: 0x8bd7c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2201459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e88d68deff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377880142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2202014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e86266deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377880713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9157940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8cc4274ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377880127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4396136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e898ebbcff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377875171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4395118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e892efbcff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377874889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t244479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8ffba0300","size":5,"soft_fail":false,"target_va":5378120480,"va":5377875996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t244519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e827bb0300","size":5,"soft_fail":false,"target_va":5378120496,"va":5377875972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t251584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8c0d60300","size":5,"soft_fail":false,"target_va":5378126944,"va":5377875355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t249682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e852cf0300","size":5,"soft_fail":false,"target_va":5378127504,"va":5377877817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t587066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e83af50800","size":5,"soft_fail":false,"target_va":5378462960,"va":5377875889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1600770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8026d1800","size":5,"soft_fail":false,"target_va":5379475248,"va":5377874473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3786341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e865c63900","size":5,"soft_fail":false,"target_va":5381660768,"va":5377874422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4003488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8a0163d00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377883899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5449191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8e7255300","size":5,"soft_fail":false,"target_va":5383325024,"va":5377875828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5443641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e839105300","size":5,"soft_fail":false,"target_va":5383325056,"va":5377881410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11790245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8a5e7b300","size":5,"soft_fail":false,"target_va":5389669280,"va":5377879030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12465169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e81134be00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377875402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38738019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e863184f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377882745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t38505333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8758b4b02","size":5,"soft_fail":false,"target_va":5416382400,"va":5377877062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38738765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e84d1b4f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377882638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38739038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e85e1c4f02","size":5,"soft_fail":false,"target_va":5416621536,"va":5377882493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38739132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8bc1c4f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377882463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38739479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8171e4f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377882724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38739682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8e21e4f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377882553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8fa310000","size":5,"soft_fail":false,"target_va":5377886816,"va":5377874017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38738522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e85a1a4f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377882849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stone_foundation_frames::stone_foundation_replaces_gothic_half_dormer_hat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3354827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8cb303300","size":5,"soft_fail":false,"target_va":5381231440,"va":5377876608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t251028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e894d40300","size":5,"soft_fail":false,"target_va":5378126544,"va":5377875511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1600471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8d76b1800","size":5,"soft_fail":false,"target_va":5379475184,"va":5377874708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3775489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8019c3900","size":5,"soft_fail":false,"target_va":5381650864,"va":5377875370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3779236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8a4aa3900","size":5,"soft_fail":false,"target_va":5381654320,"va":5377875079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::gothic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2494790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e846112600","size":5,"soft_fail":false,"target_va":5380376512,"va":5377881717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3355399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e807333300","size":5,"soft_fail":false,"target_va":5381231968,"va":5377876564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutout::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t312235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8abc30400","size":5,"soft_fail":false,"target_va":5378193456,"va":5377881216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutKind::gothic_rank
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t312338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e812c40400","size":5,"soft_fail":false,"target_va":5378193520,"va":5377881177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6089403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8bbea5c00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377874384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6097910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8f60b5d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377874309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6082565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e805d05c00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377882342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6088824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e878e85c00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377873987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38835043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e863935002","size":5,"soft_fail":false,"target_va":5416717696,"va":5377882648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38736686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e82e134f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377882669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t190245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e825e70200","size":5,"soft_fail":false,"target_va":5378070256,"va":5377880006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t195902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e83efd0200","size":5,"soft_fail":false,"target_va":5378071840,"va":5377875933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8bcf4ffff","size":5,"soft_fail":false,"target_va":5377872704,"va":5377875583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5444300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8cc125300","size":5,"soft_fail":false,"target_va":5383324048,"va":5377879743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5446888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8e81c5300","size":5,"soft_fail":false,"target_va":5383324736,"va":5377877843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5404403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377873856,"raw_hex":"e8f3765200","size":5,"soft_fail":false,"target_va":5383280256,"va":5377875848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10196,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":468,"record_offset":15696,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":468,"record_offset":8856,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":468,"record_offset":8912,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":468,"record_offset":5224,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":468,"record_offset":5196,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":15792,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":8420,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":11056,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26677,"source_line":298,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":12324,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26686,"source_line":309,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":5140,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":468,"record_offset":8968,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":468,"record_offset":10424,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":468,"record_offset":13704,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":8644,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":8828,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":8560,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":13648,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":13592,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":14572,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":16924,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":468,"record_offset":8588,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":4332,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":2928,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":3320,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":468,"record_offset":7432,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":468,"record_offset":14660,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_wall_window::get_window_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs
//     evidence={"module":468,"record_offset":14756,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":27591,"source_line":565,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_wall_window::gothic_hat_visible_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs
//     evidence={"module":468,"record_offset":9056,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":27590,"source_line":58,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":468,"record_offset":10168,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":468,"record_offset":10508,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":27050,"source_line":181,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":468,"record_offset":10140,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":468,"record_offset":17012,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11331,"source_line":74,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":468,"record_offset":8940,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":468,"record_offset":16100,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":27051,"source_line":156,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::x
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":468,"record_offset":16676,"parent_record_offset":2620,"ancestor_record_offset":2620,"inlinee":27592,"source_line":151,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14896,"parent_record_offset":14872,"ancestor_record_offset":2620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14920,"parent_record_offset":14896,"ancestor_record_offset":2620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":2620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":2620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":2620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":16032,"parent_record_offset":16008,"ancestor_record_offset":2620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14872,"parent_record_offset":14840,"ancestor_record_offset":2620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":2620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":468,"record_offset":14840,"parent_record_offset":14812,"ancestor_record_offset":2620,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":468,"record_offset":14812,"parent_record_offset":14756,"ancestor_record_offset":2620,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":10300,"parent_record_offset":10268,"ancestor_record_offset":2620,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":2620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":10348,"parent_record_offset":10324,"ancestor_record_offset":2620,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":10324,"parent_record_offset":10300,"ancestor_record_offset":2620,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":2620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":468,"record_offset":20256,"parent_record_offset":20220,"ancestor_record_offset":2620,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":468,"record_offset":12172,"parent_record_offset":11092,"ancestor_record_offset":2620,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":468,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":2620,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10268,"parent_record_offset":10224,"ancestor_record_offset":2620,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10224,"parent_record_offset":10196,"ancestor_record_offset":2620,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":468,"record_offset":15816,"parent_record_offset":15792,"ancestor_record_offset":2620,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":468,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":2620,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":468,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":2620,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":468,"record_offset":9168,"parent_record_offset":9136,"ancestor_record_offset":2620,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9200,"parent_record_offset":9168,"ancestor_record_offset":2620,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":2620,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":468,"record_offset":9296,"parent_record_offset":9264,"ancestor_record_offset":2620,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":2620,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4688,"parent_record_offset":4660,"ancestor_record_offset":2620,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":2620,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":2620,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4748,"parent_record_offset":4632,"ancestor_record_offset":2620,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":468,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":2620,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9528,"parent_record_offset":9392,"ancestor_record_offset":2620,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":10008,"parent_record_offset":9888,"ancestor_record_offset":2620,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":2620,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9392,"parent_record_offset":9360,"ancestor_record_offset":2620,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":2620,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":468,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":2620,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":468,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":2620,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":468,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":2620,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":468,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":2620,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":468,"record_offset":14232,"parent_record_offset":13812,"ancestor_record_offset":2620,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":468,"record_offset":14140,"parent_record_offset":14084,"ancestor_record_offset":2620,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":468,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":2620,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":468,"record_offset":13940,"parent_record_offset":13856,"ancestor_record_offset":2620,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":468,"record_offset":14004,"parent_record_offset":13812,"ancestor_record_offset":2620,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":10080,"parent_record_offset":9056,"ancestor_record_offset":2620,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":9488,"parent_record_offset":9460,"ancestor_record_offset":2620,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":468,"record_offset":8780,"parent_record_offset":8756,"ancestor_record_offset":2620,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":468,"record_offset":7216,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9104,"parent_record_offset":9056,"ancestor_record_offset":2620,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9360,"parent_record_offset":9328,"ancestor_record_offset":2620,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9600,"parent_record_offset":9056,"ancestor_record_offset":2620,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":2620,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":2620,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":468,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":2620,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":468,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":2620,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":468,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":2620,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":14600,"parent_record_offset":14572,"ancestor_record_offset":2620,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":2620,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":16952,"parent_record_offset":16924,"ancestor_record_offset":2620,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":468,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":2620,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":468,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":2620,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":468,"record_offset":4480,"parent_record_offset":4440,"ancestor_record_offset":2620,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":468,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":2620,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":468,"record_offset":9264,"parent_record_offset":9232,"ancestor_record_offset":2620,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":468,"record_offset":9328,"parent_record_offset":9296,"ancestor_record_offset":2620,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":14692,"parent_record_offset":14660,"ancestor_record_offset":2620,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":2620,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":5728,"parent_record_offset":5224,"ancestor_record_offset":2620,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":7388,"parent_record_offset":6420,"ancestor_record_offset":2620,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":8488,"parent_record_offset":8452,"ancestor_record_offset":2620,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":468,"record_offset":3268,"parent_record_offset":3240,"ancestor_record_offset":2620,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":468,"record_offset":5096,"parent_record_offset":4440,"ancestor_record_offset":2620,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":2620,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":4812,"parent_record_offset":4580,"ancestor_record_offset":2620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":468,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2620,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":20288,"parent_record_offset":20256,"ancestor_record_offset":2620,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":2620,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":15724,"parent_record_offset":15696,"ancestor_record_offset":2620,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":2620,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":2620,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":15752,"parent_record_offset":15724,"ancestor_record_offset":2620,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":10388,"parent_record_offset":10224,"ancestor_record_offset":2620,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":468,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":2620,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":468,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":2620,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":468,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":2620,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":468,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":2620,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":468,"record_offset":4052,"parent_record_offset":4024,"ancestor_record_offset":2620,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":468,"record_offset":3644,"parent_record_offset":3528,"ancestor_record_offset":2620,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":468,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":2620,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":468,"record_offset":3356,"parent_record_offset":3320,"ancestor_record_offset":2620,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":468,"record_offset":3496,"parent_record_offset":3464,"ancestor_record_offset":2620,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":468,"record_offset":3464,"parent_record_offset":3428,"ancestor_record_offset":2620,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":468,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":2620,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":468,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":2620,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":468,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":2620,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":468,"record_offset":3968,"parent_record_offset":3428,"ancestor_record_offset":2620,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3032,"parent_record_offset":3000,"ancestor_record_offset":2620,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":2620,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":2620,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3216,"parent_record_offset":3160,"ancestor_record_offset":2620,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3128,"parent_record_offset":3096,"ancestor_record_offset":2620,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":2972,"parent_record_offset":2928,"ancestor_record_offset":2620,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":468,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":2620,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":468,"record_offset":3528,"parent_record_offset":3496,"ancestor_record_offset":2620,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":468,"record_offset":3812,"parent_record_offset":3528,"ancestor_record_offset":2620,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":468,"record_offset":3784,"parent_record_offset":3528,"ancestor_record_offset":2620,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":468,"record_offset":3588,"parent_record_offset":3528,"ancestor_record_offset":2620,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":12200,"parent_record_offset":12172,"ancestor_record_offset":2620,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":20104,"parent_record_offset":20068,"ancestor_record_offset":2620,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":2620,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":2620,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":20160,"parent_record_offset":20068,"ancestor_record_offset":2620,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":468,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":2620,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":468,"record_offset":6420,"parent_record_offset":6388,"ancestor_record_offset":2620,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":11092,"parent_record_offset":11056,"ancestor_record_offset":2620,"inlinee":26679,"source_line":249,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":8452,"parent_record_offset":8420,"ancestor_record_offset":2620,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":12360,"parent_record_offset":12324,"ancestor_record_offset":2620,"inlinee":26687,"source_line":258,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":11948,"parent_record_offset":11920,"ancestor_record_offset":2620,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":12284,"parent_record_offset":12260,"ancestor_record_offset":2620,"inlinee":26703,"source_line":252,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":8520,"parent_record_offset":8488,"ancestor_record_offset":2620,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":468,"record_offset":13552,"parent_record_offset":13528,"ancestor_record_offset":2620,"inlinee":26705,"source_line":261,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":468,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":2620,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":468,"record_offset":9020,"parent_record_offset":8996,"ancestor_record_offset":2620,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":468,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":2620,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":468,"record_offset":13812,"parent_record_offset":13704,"ancestor_record_offset":2620,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":14260,"parent_record_offset":13812,"ancestor_record_offset":2620,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":2620,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":2620,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":2620,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":2620,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":468,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":2620,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":468,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":2620,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":468,"record_offset":13856,"parent_record_offset":13812,"ancestor_record_offset":2620,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":468,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":2620,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":468,"record_offset":5452,"parent_record_offset":5420,"ancestor_record_offset":2620,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":468,"record_offset":5392,"parent_record_offset":5352,"ancestor_record_offset":2620,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":468,"record_offset":6692,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":468,"record_offset":5516,"parent_record_offset":5352,"ancestor_record_offset":2620,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":468,"record_offset":7244,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":468,"record_offset":6776,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":468,"record_offset":6636,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":468,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":2620,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":468,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":2620,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":468,"record_offset":7100,"parent_record_offset":6992,"ancestor_record_offset":2620,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":468,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":2620,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":468,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":2620,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":468,"record_offset":6504,"parent_record_offset":6472,"ancestor_record_offset":2620,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":2620,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":5572,"parent_record_offset":5352,"ancestor_record_offset":2620,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":7336,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":5352,"parent_record_offset":5316,"ancestor_record_offset":2620,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":2620,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":2620,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":6532,"parent_record_offset":6472,"ancestor_record_offset":2620,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":2620,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":6664,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":5316,"parent_record_offset":5280,"ancestor_record_offset":2620,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":468,"record_offset":6992,"parent_record_offset":6580,"ancestor_record_offset":2620,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":468,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":2620,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":4368,"parent_record_offset":4332,"ancestor_record_offset":2620,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":18504,"parent_record_offset":18472,"ancestor_record_offset":2620,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":18956,"parent_record_offset":18752,"ancestor_record_offset":2620,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":468,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":2620,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":4580,"parent_record_offset":4440,"ancestor_record_offset":2620,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":468,"record_offset":4784,"parent_record_offset":4580,"ancestor_record_offset":2620,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":468,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":2620,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":468,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":2620,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":468,"record_offset":5848,"parent_record_offset":5816,"ancestor_record_offset":2620,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":468,"record_offset":8756,"parent_record_offset":8724,"ancestor_record_offset":2620,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":468,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":2620,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":468,"record_offset":4440,"parent_record_offset":4404,"ancestor_record_offset":2620,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":468,"record_offset":4404,"parent_record_offset":4368,"ancestor_record_offset":2620,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws
// 记录形式: compiled_procedure; instances=1
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":468,"record_offset":20696,"rva":9175600,"va":5377884720}
// 编译实例 RVAs: 0x8c0230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2206172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e82456deff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377884855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2206585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e88754deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377885284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9162653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e8633074ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377884840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8645007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e871167cff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377885610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a8db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12486705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e83188be00","size":5,"soft_fail":false,"target_va":5390372272,"va":5377885562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38735386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e81a0e4f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377885403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e879020000","size":5,"soft_fail":false,"target_va":5377885632,"va":5377884994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38736335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377884720,"raw_hex":"e8cf114f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377885422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":20888,"parent_record_offset":20696,"ancestor_record_offset":20696,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":21280,"parent_record_offset":20696,"ancestor_record_offset":20696,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":21164,"parent_record_offset":20696,"ancestor_record_offset":20696,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":468,"record_offset":21376,"parent_record_offset":20696,"ancestor_record_offset":20696,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":21592,"parent_record_offset":20696,"ancestor_record_offset":20696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21056,"parent_record_offset":21032,"ancestor_record_offset":20696,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21080,"parent_record_offset":21056,"ancestor_record_offset":20696,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":20696,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":20696,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":20696,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21924,"parent_record_offset":21900,"ancestor_record_offset":20696,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21032,"parent_record_offset":21004,"ancestor_record_offset":20696,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":21852,"parent_record_offset":21760,"ancestor_record_offset":20696,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":20944,"parent_record_offset":20916,"ancestor_record_offset":20696,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":21788,"parent_record_offset":21760,"ancestor_record_offset":20696,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":20696,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":21004,"parent_record_offset":20972,"ancestor_record_offset":20696,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":20972,"parent_record_offset":20944,"ancestor_record_offset":20696,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":20696,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":20916,"parent_record_offset":20888,"ancestor_record_offset":20696,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":20696,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":468,"record_offset":21236,"parent_record_offset":21212,"ancestor_record_offset":20696,"inlinee":17057,"source_line":1180,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\mod.rs
//     evidence={"module":468,"record_offset":21188,"parent_record_offset":21164,"ancestor_record_offset":20696,"inlinee":17055,"source_line":374,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":468,"record_offset":21212,"parent_record_offset":21188,"ancestor_record_offset":20696,"inlinee":17056,"source_line":1314,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":468,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":20696,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":468,"record_offset":21820,"parent_record_offset":21788,"ancestor_record_offset":20696,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":20696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":468,"record_offset":21308,"parent_record_offset":21280,"ancestor_record_offset":20696,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":468,"record_offset":21336,"parent_record_offset":21308,"ancestor_record_offset":20696,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BCA
// 首条位置: {"module":468,"inlinee":27594}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":23616,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":23728,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":468,"record_offset":23952,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":24760,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":468,"record_offset":23644,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":468,"record_offset":24456,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":24948,"parent_record_offset":24924,"ancestor_record_offset":23504,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":24972,"parent_record_offset":24948,"ancestor_record_offset":23504,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":24996,"parent_record_offset":24972,"ancestor_record_offset":23504,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":24924,"parent_record_offset":24832,"ancestor_record_offset":23504,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":23828,"parent_record_offset":23796,"ancestor_record_offset":23504,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":24860,"parent_record_offset":24832,"ancestor_record_offset":23504,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":24832,"parent_record_offset":24808,"ancestor_record_offset":23504,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":23504,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":23852,"parent_record_offset":23828,"ancestor_record_offset":23504,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":468,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":23504,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":23796,"parent_record_offset":23756,"ancestor_record_offset":23504,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":23756,"parent_record_offset":23728,"ancestor_record_offset":23504,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":468,"record_offset":24892,"parent_record_offset":24860,"ancestor_record_offset":23504,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":468,"record_offset":23984,"parent_record_offset":23952,"ancestor_record_offset":23504,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":24784,"parent_record_offset":24760,"ancestor_record_offset":23504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":468,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":23504,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":23916,"parent_record_offset":23756,"ancestor_record_offset":23504,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":468,"record_offset":24488,"parent_record_offset":24456,"ancestor_record_offset":23504,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":468,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":23504,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A1F
// 首条位置: {"module":1559,"inlinee":80415}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1559,"record_offset":11412,"parent_record_offset":11300,"ancestor_record_offset":11300,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1559,"record_offset":11480,"parent_record_offset":11300,"ancestor_record_offset":11300,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1559,"record_offset":11444,"parent_record_offset":11300,"ancestor_record_offset":11300,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":11884,"parent_record_offset":11300,"ancestor_record_offset":11300,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1559,"record_offset":11516,"parent_record_offset":11300,"ancestor_record_offset":11300,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1559,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":11300,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1559,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":11300,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1559,"record_offset":11592,"parent_record_offset":11552,"ancestor_record_offset":11300,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1559,"record_offset":12996,"parent_record_offset":12528,"ancestor_record_offset":11300,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":12784,"parent_record_offset":12752,"ancestor_record_offset":11300,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":12752,"parent_record_offset":12724,"ancestor_record_offset":11300,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":12724,"parent_record_offset":12528,"ancestor_record_offset":11300,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":11912,"parent_record_offset":11884,"ancestor_record_offset":11300,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1559,"record_offset":12668,"parent_record_offset":12528,"ancestor_record_offset":11300,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1559,"record_offset":11552,"parent_record_offset":11516,"ancestor_record_offset":11300,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1559,"record_offset":11764,"parent_record_offset":11516,"ancestor_record_offset":11300,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1559,"record_offset":12816,"parent_record_offset":12784,"ancestor_record_offset":11300,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::get_window_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 565
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BC7
// 首条位置: {"module":468,"inlinee":27591}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":468,"record_offset":14812,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14896,"parent_record_offset":14872,"ancestor_record_offset":14756,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14920,"parent_record_offset":14896,"ancestor_record_offset":14756,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":14756,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":14872,"parent_record_offset":14840,"ancestor_record_offset":14756,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":468,"record_offset":14840,"parent_record_offset":14812,"ancestor_record_offset":14756,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00468.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_wall_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_wall_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_wall_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws
// 信息 ID: 436f7d6211072effbf93a4b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x8870a1 -> 0x8c0230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0
// 信息 ID: 075a873fd9bd7ae2f2f430b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0 <- core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >; module=468, inline_record=23504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure$0
// 信息 ID: 24ce0c34df069292759effb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0 <- core::iter::traits::iterator::impl$2::next<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::traits::iterator::impl$2::next<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0> >; module=1559, inline_record=11300; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1608, inline_record=17200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows
// 信息 ID: 4db89deeec1536edc4e110bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cece2b7d32cab5f8619d1680: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x2b774a -> 0x8bd7c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x3ae000 -> 0x8bd7c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
// 信息 ID: 18e4d30cb120dba1e9b2fb91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x877e01 -> 0x8bd340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x886f8d -> 0x8bd340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8bde7f -> 0x8bd340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c6343 -> 0x8bd340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::get_window_holes
// 信息 ID: 91890accd3cbc979cd22c55c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=468, inline_record=14756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::gothic_hat_visible_height
// 信息 ID: f64b6b315501b193c66c4760
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=468, inline_record=9056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
