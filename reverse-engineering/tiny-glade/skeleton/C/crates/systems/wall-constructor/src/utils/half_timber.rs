// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/utils/half_timber.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6c0841b3b4edf8c79c451f15bbaf6294f1066c8775d73d776393ab47512af572
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_timber.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1455, 1482, 1490, 1500, 1524, 3153, 3159, 3174, 4370, 4371, 4375, 4376
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3153,"record_offset":280,"rva":38676192,"va":5407385312}
// 编译实例 RVAs: 0x24e26e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38663873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e83f09b2fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407386060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37844901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e85b88befd","size":5,"soft_fail":false,"target_va":5369542000,"va":5407386896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29259981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8338741fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5407386920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25660359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8397478fe","size":5,"soft_fail":false,"target_va":5381726576,"va":5407386930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24061381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e83bda90fe","size":5,"soft_fail":false,"target_va":5383325024,"va":5407386400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17693014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8aa06f2fe","size":5,"soft_fail":false,"target_va":5389694032,"va":5407387041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9671846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8a6949300","size":5,"soft_fail":false,"target_va":5417057840,"va":5407385989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9235117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8adea8c00","size":5,"soft_fail":false,"target_va":5416622208,"va":5407387086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9235108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8a4ea8c00","size":5,"soft_fail":false,"target_va":5416622240,"va":5407387127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27789003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e835f957fe","size":5,"soft_fail":false,"target_va":5379597200,"va":5407386198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-23978632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8781d92fe","size":5,"soft_fail":false,"target_va":5383408256,"va":5407386883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24082334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8628890fe","size":5,"soft_fail":false,"target_va":5383303936,"va":5407386265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24109156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e89c1f90fe","size":5,"soft_fail":false,"target_va":5383277472,"va":5407386623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24103869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407385312,"raw_hex":"e8433490fe","size":5,"soft_fail":false,"target_va":5383282224,"va":5407386088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":3153,"record_offset":716,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3153,"record_offset":3464,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3153,"record_offset":568,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3153,"record_offset":1528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2536,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1360,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2264,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1900,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1708,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2176,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1808,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3153,"record_offset":596,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":3153,"record_offset":3520,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79730,"source_line":220,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3153,"record_offset":2784,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3153,"record_offset":1212,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3153,"record_offset":948,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3153,"record_offset":744,"parent_record_offset":716,"ancestor_record_offset":280,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3153,"record_offset":808,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3153,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3153,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3153,"record_offset":2864,"parent_record_offset":2816,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":3153,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":280,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3153,"record_offset":3200,"parent_record_offset":3120,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3153,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":280,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3153,"record_offset":980,"parent_record_offset":948,"ancestor_record_offset":280,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3153,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3153,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":280,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3153,"record_offset":652,"parent_record_offset":596,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3153,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3153,"record_offset":1840,"parent_record_offset":1808,"ancestor_record_offset":280,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3153,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3153,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":280,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3153,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3153,"record_offset":1244,"parent_record_offset":1212,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3153,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":280,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3153,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":3153,"record_offset":2816,"parent_record_offset":2784,"ancestor_record_offset":280,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":3153,"record_offset":2948,"parent_record_offset":2816,"ancestor_record_offset":280,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3153,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":280,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3153,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":280,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":3153,"record_offset":3120,"parent_record_offset":2816,"ancestor_record_offset":280,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3153,"record_offset":1608,"parent_record_offset":1552,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3153,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2372,"parent_record_offset":2336,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2336,"parent_record_offset":2296,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2296,"parent_record_offset":2264,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":1468,"parent_record_offset":1360,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3153,"record_offset":2564,"parent_record_offset":2536,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3153,"record_offset":1640,"parent_record_offset":1608,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3153,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3153,"record_offset":1008,"parent_record_offset":948,"ancestor_record_offset":280,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/03153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":9516,"rva":21212704,"va":5389921824}
// 编译实例 RVAs: 0x143ae20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5280144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8706eafff","size":5,"soft_fail":false,"target_va":5384644912,"va":5389925051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-14248122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8469726ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389926805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14244706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e89ea426ff","size":5,"soft_fail":false,"target_va":5375678736,"va":5389923437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=49
//     evidence={"assembly":"\tcallq\t-14246174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8e29e26ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389924873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=11
//     target_aliases=1699 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21201237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8ab7ebcfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389923424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20711581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e863f7c3fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5389952104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=9
//     target_aliases=17 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20387548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e824e9c8fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389929543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18043624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e818adecfe","size":5,"soft_fail":false,"target_va":5371879952,"va":5389923571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16455782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e89ae704ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5389934817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16347998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8a28c06ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5389923625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15846040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e868350eff","size":5,"soft_fail":false,"target_va":5374098704,"va":5389944739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14311700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8ec9e25ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5389937759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11802224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e890e94bff","size":5,"soft_fail":false,"target_va":5378120480,"va":5389922699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=5
//     target_aliases=20 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11802633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8f7e74bff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389929572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11813103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e811bf4bff","size":5,"soft_fail":false,"target_va":5378127136,"va":5389940234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be8f00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8730386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8eec87aff","size":5,"soft_fail":false,"target_va":5381197568,"va":5389927949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4e290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8310192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8503281ff","size":5,"soft_fail":false,"target_va":5381612176,"va":5389922363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8203019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8f5d482ff","size":5,"soft_fail":false,"target_va":5381726576,"va":5389929590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6653950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e802789aff","size":5,"soft_fail":false,"target_va":5383273968,"va":5389927913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6604050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8ee3a9bff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389929069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6613202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e82e179bff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389938253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-302437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e89b62fbff","size":5,"soft_fail":false,"target_va":5389619632,"va":5389922064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f34e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-294894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e81280fbff","size":5,"soft_fail":false,"target_va":5389628640,"va":5389923529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f5c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-286378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e856a1fbff","size":5,"soft_fail":false,"target_va":5389638672,"va":5389925045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f8020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-275421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e823ccfbff","size":5,"soft_fail":false,"target_va":5389647904,"va":5389923320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f86d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-284274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88ea9fbff","size":5,"soft_fail":false,"target_va":5389649616,"va":5389933885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f96d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-268675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e87de6fbff","size":5,"soft_fail":false,"target_va":5389653712,"va":5389922382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fa940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-276596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88cc7fbff","size":5,"soft_fail":false,"target_va":5389658432,"va":5389935023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413faaa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-264620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e854f6fbff","size":5,"soft_fail":false,"target_va":5389658784,"va":5389923399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e85767fcff","size":5,"soft_fail":false,"target_va":5389694032,"va":5389929716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141414dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-166722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8be74fdff","size":5,"soft_fail":false,"target_va":5389766096,"va":5389932813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8afc10200","size":5,"soft_fail":false,"target_va":5390124368,"va":5389943708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t181173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8b5c30200","size":5,"soft_fail":false,"target_va":5390124832,"va":5389943654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t191844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e864ed0200","size":5,"soft_fail":false,"target_va":5390125360,"va":5389933511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e862bf0200","size":5,"soft_fail":false,"target_va":5390125920,"va":5389945849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8a0c10200","size":5,"soft_fail":false,"target_va":5390126352,"va":5389945707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141472fe0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t227341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e80d780300","size":5,"soft_fail":false,"target_va":5390151648,"va":5389924302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141480ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e874070400","size":5,"soft_fail":false,"target_va":5390208160,"va":5389944103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14199edb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5643244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8ec1b5600","size":5,"soft_fail":false,"target_va":5395574192,"va":5389930943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e26e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17454853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e805570a01","size":5,"soft_fail":false,"target_va":5407385312,"va":5389930454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26414657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8410e9301","size":5,"soft_fail":false,"target_va":5416350832,"va":5389936170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26427312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8b03f9301","size":5,"soft_fail":false,"target_va":5416352064,"va":5389924747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26409038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e84ef89201","size":5,"soft_fail":false,"target_va":5416353040,"va":5389943997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27129484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88cf69d01","size":5,"soft_fail":false,"target_va":5417057056,"va":5389927567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27133992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e828089e01","size":5,"soft_fail":false,"target_va":5417057072,"va":5389923075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27133941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8f5079e01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389923718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27130893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e80dfc9d01","size":5,"soft_fail":false,"target_va":5417057840,"va":5389926942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26675853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88d0a9701","size":5,"soft_fail":false,"target_va":5416620769,"va":5389944911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t26443537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8117f9301","size":5,"soft_fail":false,"target_va":5416366080,"va":5389922538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26687155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8b3369701","size":5,"soft_fail":false,"target_va":5416620794,"va":5389933634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t26699407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88f669701","size":5,"soft_fail":false,"target_va":5416621600,"va":5389922188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t26688539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e81b3c9701","size":5,"soft_fail":false,"target_va":5416622208,"va":5389933664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t26675437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8ed089701","size":5,"soft_fail":false,"target_va":5416621762,"va":5389946320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26677220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8e40f9701","size":5,"soft_fail":false,"target_va":5416622240,"va":5389945015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26676549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8450d9701","size":5,"soft_fail":false,"target_va":5416621632,"va":5389945078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t214234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8da440300","size":5,"soft_fail":false,"target_va":5390147824,"va":5389933585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10032039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e859ec66ff","size":5,"soft_fail":false,"target_va":5379908016,"va":5389940050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10031475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88dee66ff","size":5,"soft_fail":false,"target_va":5379908400,"va":5389939870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10031101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e803f066ff","size":5,"soft_fail":false,"target_va":5379908992,"va":5389940088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11072883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88d0a57ff","size":5,"soft_fail":false,"target_va":5378863520,"va":5389936398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11072564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8cc0b57ff","size":5,"soft_fail":false,"target_va":5378863824,"va":5389936383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8727080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8d8d57aff","size":5,"soft_fail":false,"target_va":5381194960,"va":5389922035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8730767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e871c77aff","size":5,"soft_fail":false,"target_va":5381197120,"va":5389927882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8724592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e890df7aff","size":5,"soft_fail":false,"target_va":5381197456,"va":5389922043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8742833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e84f987aff","size":5,"soft_fail":false,"target_va":5381195024,"va":5389937852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8687858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e80e6f7bff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389922461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8690585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e867647bff","size":5,"soft_fail":false,"target_va":5381231824,"va":5389922404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_transition_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8689291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e875697bff","size":5,"soft_fail":false,"target_va":5381233488,"va":5389922774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8689809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e86f677bff","size":5,"soft_fail":false,"target_va":5381232176,"va":5389921980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8707719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e879217bff","size":5,"soft_fail":false,"target_va":5381231952,"va":5389939666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11695960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8a8884dff","size":5,"soft_fail":false,"target_va":5378226528,"va":5389922483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10331602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e82e5a62ff","size":5,"soft_fail":false,"target_va":5379596368,"va":5389927965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10331790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8725962ff","size":5,"soft_fail":false,"target_va":5379597200,"va":5389928985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6547600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e870179cff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389930747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26654625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8a1b79601","size":5,"soft_fail":false,"target_va":5416576832,"va":5389922202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26629214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e85e549601","size":5,"soft_fail":false,"target_va":5416576144,"va":5389946925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5937299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e86d67a5ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5389922142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5960127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8410ea5ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5389945482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6521272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8487e9cff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389929523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5219296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8205cb0ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5389945979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t33700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8a4830000","size":5,"soft_fail":false,"target_va":5389960944,"va":5389927239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::as_f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8133336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8d81a7c00","size":5,"soft_fail":false,"target_va":5398073024,"va":5389939683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6623707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e825ee9aff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389925942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6623665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e84fee9aff","size":5,"soft_fail":false,"target_va":5383302304,"va":5389925964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6624325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8bbeb9aff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389926224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6623157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e84bf09aff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389925440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6629512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e878d79aff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389931251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6623859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e88ded9aff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389926078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6637674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e896b79aff","size":5,"soft_fail":false,"target_va":5383304736,"va":5389942405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6627718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e87ade9aff","size":5,"soft_fail":false,"target_va":5383303936,"va":5389931649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6635225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e827c19aff","size":5,"soft_fail":false,"target_va":5383307120,"va":5389942340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6620343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e849fb9aff","size":5,"soft_fail":false,"target_va":5383314112,"va":5389934450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6625617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8afe69aff","size":5,"soft_fail":false,"target_va":5383309120,"va":5389934732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6616975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e871089bff","size":5,"soft_fail":false,"target_va":5383323424,"va":5389940394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6632741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8dbca9aff","size":5,"soft_fail":false,"target_va":5383297984,"va":5389930720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6654541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8b3759aff","size":5,"soft_fail":false,"target_va":5383277472,"va":5389932008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17452086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8364c0a01","size":5,"soft_fail":false,"target_va":5407388048,"va":5389935957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17449908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8b4430a01","size":5,"soft_fail":false,"target_va":5407387136,"va":5389937223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6646677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e86b949aff","size":5,"soft_fail":false,"target_va":5383282224,"va":5389928896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6653461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389921824,"raw_hex":"e8eb799aff","size":5,"soft_fail":false,"target_va":5383275712,"va":5389929168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5389921824,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5389922085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1490,"record_offset":34060,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1490,"record_offset":19064,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1490,"record_offset":10940,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":51740,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":12540,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11069,"source_line":2246,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":10672,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":10732,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":11128,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":13344,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":21048,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":15268,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":14316,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":19248,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":28296,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":36136,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1490,"record_offset":18600,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":14052,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":53876,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":26868,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":17760,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":37980,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":18512,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":18484,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1490,"record_offset":14080,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":14424,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1490,"record_offset":52444,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":10616,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":44808,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$121::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1490,"record_offset":41736,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9820,"source_line":49,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":20188,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":25132,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":28500,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$64::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":41500,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":12115,"source_line":354,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":24400,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":10700,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":49756,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":10064,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":15984,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":39128,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":21936,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":13176,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":18344,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":33692,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":15484,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":38956,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":37456,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":17906,"source_line":3188,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":18252,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":18428,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":10004,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":10100,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":11540,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1490,"record_offset":10588,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":79718,"source_line":429,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1490,"record_offset":30348,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":29384,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":46452,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":42064,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":42092,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":42180,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":30844,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29216,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29900,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":25760,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":25860,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":45548,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":26144,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":45640,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1490,"record_offset":37628,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1490,"record_offset":22032,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1490,"record_offset":53780,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":34088,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":32272,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":52152,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":16764,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":52792,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":13996,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":26924,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":79730,"source_line":220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":14508,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":79727,"source_line":356,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":19496,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":79729,"source_line":2028,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1490,"record_offset":26236,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":13716,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":10560,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14176,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14288,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":25612,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":44552,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":79567,"source_line":65,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":25484,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":28024,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":9516,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":13604,"parent_record_offset":13580,"ancestor_record_offset":9516,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":9516,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":9516,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":9516,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":13628,"parent_record_offset":13604,"ancestor_record_offset":9516,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":9516,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":19128,"parent_record_offset":19096,"ancestor_record_offset":9516,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":13556,"parent_record_offset":13508,"ancestor_record_offset":9516,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":16172,"parent_record_offset":16080,"ancestor_record_offset":9516,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1490,"record_offset":19096,"parent_record_offset":19064,"ancestor_record_offset":9516,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1490,"record_offset":59104,"parent_record_offset":59080,"ancestor_record_offset":9516,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":9516,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":9516,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":21188,"parent_record_offset":21160,"ancestor_record_offset":9516,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":13424,"parent_record_offset":13384,"ancestor_record_offset":9516,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":36400,"parent_record_offset":36372,"ancestor_record_offset":9516,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":16108,"parent_record_offset":16080,"ancestor_record_offset":9516,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=45
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":9516,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":36372,"parent_record_offset":36336,"ancestor_record_offset":9516,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":9516,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":9516,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":36336,"parent_record_offset":36304,"ancestor_record_offset":9516,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":13508,"parent_record_offset":13464,"ancestor_record_offset":9516,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":21216,"parent_record_offset":21188,"ancestor_record_offset":9516,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":13464,"parent_record_offset":13424,"ancestor_record_offset":9516,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=45
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":9516,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":21284,"parent_record_offset":21248,"ancestor_record_offset":9516,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1490,"record_offset":21080,"parent_record_offset":21048,"ancestor_record_offset":9516,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1490,"record_offset":21124,"parent_record_offset":21080,"ancestor_record_offset":9516,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1490,"record_offset":10976,"parent_record_offset":10940,"ancestor_record_offset":9516,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":10792,"parent_record_offset":10756,"ancestor_record_offset":9516,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":9516,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":9516,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":21616,"parent_record_offset":21080,"ancestor_record_offset":9516,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":51768,"parent_record_offset":51740,"ancestor_record_offset":9516,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":36216,"parent_record_offset":36176,"ancestor_record_offset":9516,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::insert_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":9516,"inlinee":10769,"source_line":2277,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":11248,"parent_record_offset":11164,"ancestor_record_offset":9516,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":10756,"parent_record_offset":10732,"ancestor_record_offset":9516,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":36276,"parent_record_offset":36216,"ancestor_record_offset":9516,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":11164,"parent_record_offset":11128,"ancestor_record_offset":9516,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":11504,"parent_record_offset":11164,"ancestor_record_offset":9516,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":21160,"parent_record_offset":21124,"ancestor_record_offset":9516,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":22664,"parent_record_offset":22552,"ancestor_record_offset":9516,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":10640,"parent_record_offset":10616,"ancestor_record_offset":9516,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":11276,"parent_record_offset":11164,"ancestor_record_offset":9516,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":23684,"parent_record_offset":23660,"ancestor_record_offset":9516,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":36992,"parent_record_offset":36968,"ancestor_record_offset":9516,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":55624,"parent_record_offset":55600,"ancestor_record_offset":9516,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":51864,"parent_record_offset":51768,"ancestor_record_offset":9516,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1490,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":9516,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1490,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":9516,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1490,"record_offset":37048,"parent_record_offset":36992,"ancestor_record_offset":9516,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1490,"record_offset":37412,"parent_record_offset":36216,"ancestor_record_offset":9516,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1490,"record_offset":37212,"parent_record_offset":37188,"ancestor_record_offset":9516,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1490,"record_offset":36176,"parent_record_offset":36136,"ancestor_record_offset":9516,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1490,"record_offset":13384,"parent_record_offset":13344,"ancestor_record_offset":9516,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1490,"record_offset":21308,"parent_record_offset":21284,"ancestor_record_offset":9516,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1490,"record_offset":17392,"parent_record_offset":17364,"ancestor_record_offset":9516,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1490,"record_offset":17284,"parent_record_offset":17064,"ancestor_record_offset":9516,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":1490,"record_offset":41532,"parent_record_offset":41500,"ancestor_record_offset":9516,"inlinee":8861,"source_line":614,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":9516,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":25312,"parent_record_offset":25160,"ancestor_record_offset":9516,"inlinee":6830,"source_line":2141,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":51888,"parent_record_offset":51864,"ancestor_record_offset":9516,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":34812,"parent_record_offset":34788,"ancestor_record_offset":9516,"inlinee":17499,"source_line":1878,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":58840,"parent_record_offset":58812,"ancestor_record_offset":9516,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":9516,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":25336,"parent_record_offset":25312,"ancestor_record_offset":9516,"inlinee":8911,"source_line":1916,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":9516,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":34788,"parent_record_offset":34764,"ancestor_record_offset":9516,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":26496,"parent_record_offset":26268,"ancestor_record_offset":9516,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1490,"record_offset":26628,"parent_record_offset":26604,"ancestor_record_offset":9516,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":52356,"parent_record_offset":52328,"ancestor_record_offset":9516,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":34356,"parent_record_offset":34332,"ancestor_record_offset":9516,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":34440,"parent_record_offset":34416,"ancestor_record_offset":9516,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":30640,"parent_record_offset":30372,"ancestor_record_offset":9516,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":26660,"parent_record_offset":26580,"ancestor_record_offset":9516,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":26604,"parent_record_offset":26580,"ancestor_record_offset":9516,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":30428,"parent_record_offset":30372,"ancestor_record_offset":9516,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":9516,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":23012,"parent_record_offset":22928,"ancestor_record_offset":9516,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":19572,"parent_record_offset":19532,"ancestor_record_offset":9516,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":23040,"parent_record_offset":22928,"ancestor_record_offset":9516,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":9516,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":29520,"parent_record_offset":29496,"ancestor_record_offset":9516,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1490,"record_offset":11332,"parent_record_offset":11164,"ancestor_record_offset":9516,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1490,"record_offset":11056,"parent_record_offset":11012,"ancestor_record_offset":9516,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1490,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":9516,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1490,"record_offset":27136,"parent_record_offset":27112,"ancestor_record_offset":9516,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1490,"record_offset":27184,"parent_record_offset":27160,"ancestor_record_offset":9516,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1490,"record_offset":36624,"parent_record_offset":36584,"ancestor_record_offset":9516,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1490,"record_offset":36760,"parent_record_offset":36664,"ancestor_record_offset":9516,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":9516,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":9516,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":9516,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1490,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":9516,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":13832,"parent_record_offset":13716,"ancestor_record_offset":9516,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":36584,"parent_record_offset":36216,"ancestor_record_offset":9516,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":36968,"parent_record_offset":36760,"ancestor_record_offset":9516,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":14820,"parent_record_offset":14592,"ancestor_record_offset":9516,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":27208,"parent_record_offset":27184,"ancestor_record_offset":9516,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1490,"record_offset":33380,"parent_record_offset":33352,"ancestor_record_offset":9516,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1490,"record_offset":33404,"parent_record_offset":33380,"ancestor_record_offset":9516,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1490,"record_offset":17244,"parent_record_offset":17216,"ancestor_record_offset":9516,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1490,"record_offset":39028,"parent_record_offset":38992,"ancestor_record_offset":9516,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":21332,"parent_record_offset":21308,"ancestor_record_offset":9516,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":10484,"parent_record_offset":10456,"ancestor_record_offset":9516,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":21360,"parent_record_offset":21332,"ancestor_record_offset":9516,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":37088,"parent_record_offset":36664,"ancestor_record_offset":9516,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":16140,"parent_record_offset":16108,"ancestor_record_offset":9516,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":17420,"parent_record_offset":17364,"ancestor_record_offset":9516,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":34588,"parent_record_offset":34560,"ancestor_record_offset":9516,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":34560,"parent_record_offset":34532,"ancestor_record_offset":9516,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":15120,"parent_record_offset":15036,"ancestor_record_offset":9516,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":15092,"parent_record_offset":15036,"ancestor_record_offset":9516,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":45580,"parent_record_offset":45548,"ancestor_record_offset":9516,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":45668,"parent_record_offset":45640,"ancestor_record_offset":9516,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":14904,"parent_record_offset":14592,"ancestor_record_offset":9516,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1490,"record_offset":16972,"parent_record_offset":16936,"ancestor_record_offset":9516,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":9516,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":23068,"parent_record_offset":22928,"ancestor_record_offset":9516,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":10380,"parent_record_offset":10100,"ancestor_record_offset":9516,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":22980,"parent_record_offset":22928,"ancestor_record_offset":9516,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":53904,"parent_record_offset":53876,"ancestor_record_offset":9516,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":27160,"parent_record_offset":27136,"ancestor_record_offset":9516,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":10180,"parent_record_offset":10100,"ancestor_record_offset":9516,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":10420,"parent_record_offset":10100,"ancestor_record_offset":9516,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":52392,"parent_record_offset":52300,"ancestor_record_offset":9516,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":21556,"parent_record_offset":21124,"ancestor_record_offset":9516,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":23748,"parent_record_offset":23684,"ancestor_record_offset":9516,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":11468,"parent_record_offset":11164,"ancestor_record_offset":9516,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=81
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":9516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":12716,"parent_record_offset":12564,"ancestor_record_offset":9516,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":54844,"parent_record_offset":54816,"ancestor_record_offset":9516,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":17188,"parent_record_offset":17064,"ancestor_record_offset":9516,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":34704,"parent_record_offset":34680,"ancestor_record_offset":9516,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1490,"record_offset":18312,"parent_record_offset":18252,"ancestor_record_offset":9516,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1490,"record_offset":18668,"parent_record_offset":18600,"ancestor_record_offset":9516,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1490,"record_offset":14136,"parent_record_offset":14108,"ancestor_record_offset":9516,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":9516,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":17216,"parent_record_offset":17188,"ancestor_record_offset":9516,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":10904,"parent_record_offset":10756,"ancestor_record_offset":9516,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1490,"record_offset":13920,"parent_record_offset":13888,"ancestor_record_offset":9516,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::as_mut_ptr_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":39068,"parent_record_offset":38956,"ancestor_record_offset":9516,"inlinee":11484,"source_line":836,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":13748,"parent_record_offset":13716,"ancestor_record_offset":9516,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":10348,"parent_record_offset":10100,"ancestor_record_offset":9516,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":38992,"parent_record_offset":38956,"ancestor_record_offset":9516,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":15300,"parent_record_offset":15268,"ancestor_record_offset":9516,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":14348,"parent_record_offset":14316,"ancestor_record_offset":9516,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":21964,"parent_record_offset":21936,"ancestor_record_offset":9516,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":15332,"parent_record_offset":15300,"ancestor_record_offset":9516,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":14380,"parent_record_offset":14348,"ancestor_record_offset":9516,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":9516,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":9516,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":36664,"parent_record_offset":36624,"ancestor_record_offset":9516,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":14108,"parent_record_offset":14080,"ancestor_record_offset":9516,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":21804,"parent_record_offset":21764,"ancestor_record_offset":9516,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":9516,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1490,"record_offset":11012,"parent_record_offset":10976,"ancestor_record_offset":9516,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":1490,"record_offset":37492,"parent_record_offset":37456,"ancestor_record_offset":9516,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":58408,"parent_record_offset":58340,"ancestor_record_offset":9516,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":10256,"parent_record_offset":10216,"ancestor_record_offset":9516,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":53808,"parent_record_offset":53780,"ancestor_record_offset":9516,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":58436,"parent_record_offset":58408,"ancestor_record_offset":9516,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":9516,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":53836,"parent_record_offset":53808,"ancestor_record_offset":9516,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":9516,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":10456,"parent_record_offset":10420,"ancestor_record_offset":9516,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":58308,"parent_record_offset":58280,"ancestor_record_offset":9516,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":9516,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":9516,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1490,"record_offset":26268,"parent_record_offset":26236,"ancestor_record_offset":9516,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1490,"record_offset":26316,"parent_record_offset":26268,"ancestor_record_offset":9516,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1490,"record_offset":30372,"parent_record_offset":30348,"ancestor_record_offset":9516,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":26400,"parent_record_offset":26316,"ancestor_record_offset":9516,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":9516,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":26580,"parent_record_offset":26268,"ancestor_record_offset":9516,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":29464,"parent_record_offset":29408,"ancestor_record_offset":9516,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":46480,"parent_record_offset":46452,"ancestor_record_offset":9516,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":46568,"parent_record_offset":46452,"ancestor_record_offset":9516,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":29408,"parent_record_offset":29384,"ancestor_record_offset":9516,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":49812,"parent_record_offset":49788,"ancestor_record_offset":9516,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":30000,"parent_record_offset":29964,"ancestor_record_offset":9516,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29964,"parent_record_offset":29928,"ancestor_record_offset":9516,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29928,"parent_record_offset":29900,"ancestor_record_offset":9516,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":9516,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":29324,"parent_record_offset":29216,"ancestor_record_offset":9516,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":30872,"parent_record_offset":30844,"ancestor_record_offset":9516,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":34500,"parent_record_offset":34472,"ancestor_record_offset":9516,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":34532,"parent_record_offset":34500,"ancestor_record_offset":9516,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":34472,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":34332,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":52328,"parent_record_offset":52300,"ancestor_record_offset":9516,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":34884,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":34416,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1490,"record_offset":34276,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1490,"record_offset":34764,"parent_record_offset":34740,"ancestor_record_offset":9516,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":34112,"parent_record_offset":34088,"ancestor_record_offset":9516,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":52180,"parent_record_offset":52152,"ancestor_record_offset":9516,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":34740,"parent_record_offset":34632,"ancestor_record_offset":9516,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":52820,"parent_record_offset":52792,"ancestor_record_offset":9516,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":34144,"parent_record_offset":34112,"ancestor_record_offset":9516,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":54816,"parent_record_offset":54140,"ancestor_record_offset":9516,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34680,"parent_record_offset":34656,"ancestor_record_offset":9516,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":53124,"parent_record_offset":52880,"ancestor_record_offset":9516,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34976,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52300,"parent_record_offset":52236,"ancestor_record_offset":9516,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52880,"parent_record_offset":52848,"ancestor_record_offset":9516,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34656,"parent_record_offset":34632,"ancestor_record_offset":9516,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":9516,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34172,"parent_record_offset":34112,"ancestor_record_offset":9516,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52208,"parent_record_offset":52180,"ancestor_record_offset":9516,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":55268,"parent_record_offset":55232,"ancestor_record_offset":9516,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52268,"parent_record_offset":52236,"ancestor_record_offset":9516,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":54140,"parent_record_offset":54068,"ancestor_record_offset":9516,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":54068,"parent_record_offset":54036,"ancestor_record_offset":9516,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34220,"parent_record_offset":34196,"ancestor_record_offset":9516,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":55232,"parent_record_offset":54068,"ancestor_record_offset":9516,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":54108,"parent_record_offset":54068,"ancestor_record_offset":9516,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52236,"parent_record_offset":52208,"ancestor_record_offset":9516,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34304,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":55300,"parent_record_offset":55268,"ancestor_record_offset":9516,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":54036,"parent_record_offset":54004,"ancestor_record_offset":9516,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":52848,"parent_record_offset":52820,"ancestor_record_offset":9516,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":34632,"parent_record_offset":34220,"ancestor_record_offset":9516,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1490,"record_offset":27112,"parent_record_offset":26984,"ancestor_record_offset":9516,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1490,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":9516,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1490,"record_offset":15036,"parent_record_offset":14508,"ancestor_record_offset":9516,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":10216,"parent_record_offset":10100,"ancestor_record_offset":9516,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":58340,"parent_record_offset":58308,"ancestor_record_offset":9516,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":10312,"parent_record_offset":10216,"ancestor_record_offset":9516,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":58376,"parent_record_offset":58340,"ancestor_record_offset":9516,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":16796,"parent_record_offset":16764,"ancestor_record_offset":9516,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":29496,"parent_record_offset":29464,"ancestor_record_offset":9516,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1490,"record_offset":16872,"parent_record_offset":16828,"ancestor_record_offset":9516,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1490,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":9516,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1490,"record_offset":17000,"parent_record_offset":16972,"ancestor_record_offset":9516,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1490,"record_offset":17364,"parent_record_offset":16936,"ancestor_record_offset":9516,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1490,"record_offset":16828,"parent_record_offset":16796,"ancestor_record_offset":9516,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1490,"record_offset":17032,"parent_record_offset":17000,"ancestor_record_offset":9516,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1490,"record_offset":17064,"parent_record_offset":17032,"ancestor_record_offset":9516,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1490,"record_offset":16936,"parent_record_offset":16904,"ancestor_record_offset":9516,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1490,"record_offset":16904,"parent_record_offset":16872,"ancestor_record_offset":9516,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$12
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":22928,"parent_record_offset":22904,"ancestor_record_offset":9516,"inlinee":79301,"source_line":582,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$15
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":9516,"inlinee":79322,"source_line":989,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$18
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":49788,"parent_record_offset":49756,"ancestor_record_offset":9516,"inlinee":79731,"source_line":1497,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$3
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":9516,"inlinee":79298,"source_line":268,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":9516,"inlinee":79295,"source_line":275,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1490,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":9516,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":13888,"parent_record_offset":13716,"ancestor_record_offset":9516,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":15008,"parent_record_offset":14508,"ancestor_record_offset":9516,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14560,"parent_record_offset":14508,"ancestor_record_offset":9516,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14592,"parent_record_offset":14560,"ancestor_record_offset":9516,"inlinee":15959,"source_line":243,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14212,"parent_record_offset":14176,"ancestor_record_offset":9516,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14680,"parent_record_offset":14592,"ancestor_record_offset":9516,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":36816,"parent_record_offset":36788,"ancestor_record_offset":9516,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":51916,"parent_record_offset":51888,"ancestor_record_offset":9516,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":19532,"parent_record_offset":19496,"ancestor_record_offset":9516,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":19600,"parent_record_offset":19532,"ancestor_record_offset":9516,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":14936,"parent_record_offset":14592,"ancestor_record_offset":9516,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::overlapping_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":69044,"rva":21245168,"va":5389954288}
// 编译实例 RVAs: 0x1442cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14276748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8742726ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389955447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26426528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8a03c9301","size":5,"soft_fail":false,"target_va":5416381344,"va":5389954811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10447305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8379660ff","size":5,"soft_fail":false,"target_va":5379507120,"va":5389954420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8300360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8b85881ff","size":5,"soft_fail":false,"target_va":5381654320,"va":5389954675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e89dfcffff","size":5,"soft_fail":false,"target_va":5389953840,"va":5389954702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6652634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8267d9aff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389954869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6653083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8657b9aff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389954982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6652668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8047d9aff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389954951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6653043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e88d7b9aff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389954782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6652663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389954288,"raw_hex":"e8097d9aff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389954882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":69208,"parent_record_offset":69044,"ancestor_record_offset":69044,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":69528,"parent_record_offset":69044,"ancestor_record_offset":69044,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":70596,"parent_record_offset":69044,"ancestor_record_offset":69044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":69236,"parent_record_offset":69044,"ancestor_record_offset":69044,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":69740,"parent_record_offset":69044,"ancestor_record_offset":69044,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":70784,"parent_record_offset":70760,"ancestor_record_offset":69044,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":70808,"parent_record_offset":70784,"ancestor_record_offset":69044,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":70832,"parent_record_offset":70808,"ancestor_record_offset":69044,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":70760,"parent_record_offset":70668,"ancestor_record_offset":69044,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":69616,"parent_record_offset":69584,"ancestor_record_offset":69044,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":70696,"parent_record_offset":70668,"ancestor_record_offset":69044,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":70668,"parent_record_offset":70644,"ancestor_record_offset":69044,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":69664,"parent_record_offset":69640,"ancestor_record_offset":69044,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":69640,"parent_record_offset":69616,"ancestor_record_offset":69044,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":70644,"parent_record_offset":70620,"ancestor_record_offset":69044,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":69584,"parent_record_offset":69552,"ancestor_record_offset":69044,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":69552,"parent_record_offset":69528,"ancestor_record_offset":69044,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":69444,"parent_record_offset":69420,"ancestor_record_offset":69044,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":70728,"parent_record_offset":70696,"ancestor_record_offset":69044,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":69972,"parent_record_offset":69948,"ancestor_record_offset":69044,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":69948,"parent_record_offset":69924,"ancestor_record_offset":69044,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":69476,"parent_record_offset":69392,"ancestor_record_offset":69044,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":70620,"parent_record_offset":70596,"ancestor_record_offset":69044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":70124,"parent_record_offset":70100,"ancestor_record_offset":69044,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":69704,"parent_record_offset":69552,"ancestor_record_offset":69044,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":69892,"parent_record_offset":69860,"ancestor_record_offset":69044,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":69924,"parent_record_offset":69892,"ancestor_record_offset":69044,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":70012,"parent_record_offset":69860,"ancestor_record_offset":69044,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":69860,"parent_record_offset":69820,"ancestor_record_offset":69044,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":69420,"parent_record_offset":69392,"ancestor_record_offset":69044,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":69284,"parent_record_offset":69260,"ancestor_record_offset":69044,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":69764,"parent_record_offset":69740,"ancestor_record_offset":69044,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":70100,"parent_record_offset":69820,"ancestor_record_offset":69044,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69392,"parent_record_offset":69332,"ancestor_record_offset":69044,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69820,"parent_record_offset":69788,"ancestor_record_offset":69044,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69308,"parent_record_offset":69284,"ancestor_record_offset":69044,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69364,"parent_record_offset":69332,"ancestor_record_offset":69044,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69332,"parent_record_offset":69308,"ancestor_record_offset":69044,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":69788,"parent_record_offset":69764,"ancestor_record_offset":69044,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":69260,"parent_record_offset":69236,"ancestor_record_offset":69044,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":280,"rva":21208256,"va":5389917376}
// 编译实例 RVAs: 0x1439cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14239914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e856b726ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389918613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20681189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e81b6ec4fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5389921712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20624818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e84e4ac5fe","size":5,"soft_fail":false,"target_va":5369296688,"va":5389921501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20377920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8c00ec9fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389919915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11792991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8a10d4cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389919930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8193370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8a6fa82ff","size":5,"soft_fail":false,"target_va":5381726576,"va":5389919941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6593329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8cf649bff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389918348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-297915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e84574fbff","size":5,"soft_fail":false,"target_va":5389619632,"va":5389917542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fc9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-252075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e85527fcff","size":5,"soft_fail":false,"target_va":5389666720,"va":5389918790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-226016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8208dfcff","size":5,"soft_fail":false,"target_va":5389694032,"va":5389920043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-214501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e81bbafcff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389919168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27139324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8fc1c9e01","size":5,"soft_fail":false,"target_va":5417057984,"va":5389918655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26703943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e847789701","size":5,"soft_fail":false,"target_va":5416621600,"va":5389917652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26700741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8c56b9701","size":5,"soft_fail":false,"target_va":5416621762,"va":5389921016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26701293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8ed6d9701","size":5,"soft_fail":false,"target_va":5416622240,"va":5389920942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9940789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8cb5068ff","size":5,"soft_fail":false,"target_va":5379977120,"va":5389917904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8721090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e83eed7aff","size":5,"soft_fail":false,"target_va":5381197120,"va":5389918205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8722582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e86ae77aff","size":5,"soft_fail":false,"target_va":5381195280,"va":5389917857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8725887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e881da7aff","size":5,"soft_fail":false,"target_va":5381192688,"va":5389918570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10847952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e830795aff","size":5,"soft_fail":false,"target_va":5379069856,"va":5389917803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10847804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8c4795aff","size":5,"soft_fail":false,"target_va":5379069984,"va":5389917783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8686084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8fc757bff","size":5,"soft_fail":false,"target_va":5381231904,"va":5389917983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8685335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8e9787bff","size":5,"soft_fail":false,"target_va":5381232176,"va":5389917506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10321629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8238162ff","size":5,"soft_fail":false,"target_va":5379596368,"va":5389917992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26659161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e859c99601","size":5,"soft_fail":false,"target_va":5416576832,"va":5389917666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26655023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e82fb99601","size":5,"soft_fail":false,"target_va":5416576144,"va":5389921116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5932763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e82579a5ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5389917606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5935427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8bd6ea5ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5389920782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6511651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8dda39cff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389919902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6610895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e831209bff","size":5,"soft_fail":false,"target_va":5383307344,"va":5389918234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6603562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8d63c9bff","size":5,"soft_fail":false,"target_va":5383314384,"va":5389917941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6599954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8ee4a9bff","size":5,"soft_fail":false,"target_va":5383317936,"va":5389917885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6603498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8163d9bff","size":5,"soft_fail":false,"target_va":5383314512,"va":5389918005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6642131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e82da69aff","size":5,"soft_fail":false,"target_va":5383277472,"va":5389919598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6644525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389917376,"raw_hex":"e8d39c9aff","size":5,"soft_fail":false,"target_va":5383275712,"va":5389920232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5389917376,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5389917563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":6900,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":2804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1490,"record_offset":2916,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":1096,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":2832,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":600,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":2136,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":540,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":636,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":1490,"record_offset":4832,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":1868,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":7068,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":6984,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":7284,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":1528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":1584,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":5764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":1616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":27398,"source_line":181,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":1988,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":5988,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":1180,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":5212,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":7012,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":6844,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":2748,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":6956,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79726,"source_line":72,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":1236,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79724,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1490,"record_offset":6136,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":3656,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":2108,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":2324,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":3152,"parent_record_offset":3120,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":6696,"parent_record_offset":6172,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1490,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":280,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":4976,"parent_record_offset":4864,"ancestor_record_offset":280,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":280,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":6044,"parent_record_offset":6012,"ancestor_record_offset":280,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":280,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":2952,"parent_record_offset":2916,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1490,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":2292,"parent_record_offset":2260,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":280,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1490,"record_offset":3084,"parent_record_offset":3048,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":3280,"parent_record_offset":3248,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":916,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":3048,"parent_record_offset":2916,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":716,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":956,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":884,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":9156,"parent_record_offset":9092,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":792,"parent_record_offset":752,"ancestor_record_offset":280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":280,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":280,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":9064,"parent_record_offset":9036,"ancestor_record_offset":280,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1490,"record_offset":6172,"parent_record_offset":6136,"ancestor_record_offset":280,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1490,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":280,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1490,"record_offset":6228,"parent_record_offset":6172,"ancestor_record_offset":280,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":6316,"parent_record_offset":6228,"ancestor_record_offset":280,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":280,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":6416,"parent_record_offset":6172,"ancestor_record_offset":280,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1490,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":280,"inlinee":11831,"source_line":110,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1490,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":280,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1490,"record_offset":5152,"parent_record_offset":4832,"ancestor_record_offset":280,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":7120,"parent_record_offset":7092,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":1324,"parent_record_offset":1236,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1490,"record_offset":5644,"parent_record_offset":5268,"ancestor_record_offset":280,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":752,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":9092,"parent_record_offset":9064,"ancestor_record_offset":280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":848,"parent_record_offset":752,"ancestor_record_offset":280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":9128,"parent_record_offset":9092,"ancestor_record_offset":280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::roof_outline::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":3120,"parent_record_offset":3084,"ancestor_record_offset":280,"inlinee":79725,"source_line":124,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":3688,"parent_record_offset":3656,"ancestor_record_offset":280,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":3724,"parent_record_offset":3688,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":3848,"parent_record_offset":3688,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":3248,"parent_record_offset":3120,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":3972,"parent_record_offset":3688,"ancestor_record_offset":280,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1376E
// 首条位置: {"module":1490,"inlinee":79726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1376C
// 首条位置: {"module":1490,"inlinee":79724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1490,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":1324,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":1236,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":1236,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13613
// 首条位置: {"module":1455,"inlinee":79379}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1376D
// 首条位置: {"module":1490,"inlinee":79725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":3152,"parent_record_offset":3120,"ancestor_record_offset":3120,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":3248,"parent_record_offset":3120,"ancestor_record_offset":3120,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":3120,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":3120,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":3120,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":3280,"parent_record_offset":3248,"ancestor_record_offset":3120,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":3120,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":3120,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13772
// 首条位置: {"module":1490,"inlinee":79730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135F9
// 首条位置: {"module":1455,"inlinee":79353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=43
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13809
// 首条位置: {"module":1500,"inlinee":79881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":11248,"parent_record_offset":11228,"ancestor_record_offset":11228,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135C2
// 首条位置: {"module":1455,"inlinee":79298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135BF
// 首条位置: {"module":1455,"inlinee":79295}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11800,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=43
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13808
// 首条位置: {"module":1500,"inlinee":79880}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":9996,"parent_record_offset":9976,"ancestor_record_offset":9976,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13605
// 首条位置: {"module":1455,"inlinee":79365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135B5
// 首条位置: {"module":1455,"inlinee":79285}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":1780,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":2264,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":2200,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1455,"record_offset":2128,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":2548,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1455,"record_offset":1808,"parent_record_offset":1676,"ancestor_record_offset":1676,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1676,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":1676,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":1676,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":2712,"parent_record_offset":2620,"ancestor_record_offset":1676,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2408,"parent_record_offset":2356,"ancestor_record_offset":1676,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2648,"parent_record_offset":2620,"ancestor_record_offset":1676,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1676,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2464,"parent_record_offset":2436,"ancestor_record_offset":1676,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":1676,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1676,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":2356,"parent_record_offset":2296,"ancestor_record_offset":1676,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":2296,"parent_record_offset":2264,"ancestor_record_offset":1676,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1455,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1676,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1455,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1676,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1455,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1676,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1455,"record_offset":2680,"parent_record_offset":2648,"ancestor_record_offset":1676,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1455,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1676,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1455,"record_offset":1884,"parent_record_offset":1848,"ancestor_record_offset":1676,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":2508,"parent_record_offset":2296,"ancestor_record_offset":1676,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1455,"record_offset":1848,"parent_record_offset":1808,"ancestor_record_offset":1676,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1455,"record_offset":1928,"parent_record_offset":1808,"ancestor_record_offset":1676,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$7::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1455,"record_offset":2164,"parent_record_offset":2128,"ancestor_record_offset":1676,"inlinee":79287,"source_line":306,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$7::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 306
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135B7
// 首条位置: {"module":1455,"inlinee":79287}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1376F
// 首条位置: {"module":1490,"inlinee":79727}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1490,"record_offset":15036,"parent_record_offset":14508,"ancestor_record_offset":14508,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":15008,"parent_record_offset":14508,"ancestor_record_offset":14508,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14560,"parent_record_offset":14508,"ancestor_record_offset":14508,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":14964,"parent_record_offset":14936,"ancestor_record_offset":14508,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":14508,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1490,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14508,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":14820,"parent_record_offset":14592,"ancestor_record_offset":14508,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":15120,"parent_record_offset":15036,"ancestor_record_offset":14508,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":15092,"parent_record_offset":15036,"ancestor_record_offset":14508,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":14904,"parent_record_offset":14592,"ancestor_record_offset":14508,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14508,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14508,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14508,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14508,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14592,"parent_record_offset":14560,"ancestor_record_offset":14508,"inlinee":15959,"source_line":243,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$0::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":14680,"parent_record_offset":14592,"ancestor_record_offset":14508,"inlinee":8729,"source_line":36,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1490,"record_offset":14936,"parent_record_offset":14592,"ancestor_record_offset":14508,"inlinee":15946,"source_line":121,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135EE
// 首条位置: {"module":1455,"inlinee":79342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=43
// 源码行: 555
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13803
// 首条位置: {"module":1500,"inlinee":79875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":4220,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 582
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135C5
// 首条位置: {"module":1455,"inlinee":79301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":23012,"parent_record_offset":22928,"ancestor_record_offset":22928,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":23040,"parent_record_offset":22928,"ancestor_record_offset":22928,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":23068,"parent_record_offset":22928,"ancestor_record_offset":22928,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":22980,"parent_record_offset":22928,"ancestor_record_offset":22928,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1490,"record_offset":23428,"parent_record_offset":23276,"ancestor_record_offset":23276,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":23116,"parent_record_offset":23092,"ancestor_record_offset":22928,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":23140,"parent_record_offset":23116,"ancestor_record_offset":22928,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":23508,"parent_record_offset":23428,"ancestor_record_offset":23276,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":23452,"parent_record_offset":23428,"ancestor_record_offset":23276,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":23092,"parent_record_offset":23068,"ancestor_record_offset":22928,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 897
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135B8
// 首条位置: {"module":1455,"inlinee":79288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1455,"record_offset":5900,"parent_record_offset":5780,"ancestor_record_offset":5780,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":6316,"parent_record_offset":5780,"ancestor_record_offset":5780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":5780,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":5780,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":5780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":5780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":5780,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":5780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":5972,"parent_record_offset":5936,"ancestor_record_offset":5780,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":5780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1455,"record_offset":5936,"parent_record_offset":5900,"ancestor_record_offset":5780,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1455,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":5780,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 983
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13604
// 首条位置: {"module":1455,"inlinee":79364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1455,"record_offset":141564,"parent_record_offset":141536,"ancestor_record_offset":141536,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1455,"record_offset":141644,"parent_record_offset":141620,"ancestor_record_offset":141536,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1455,"record_offset":141668,"parent_record_offset":141644,"ancestor_record_offset":141536,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1455,"record_offset":141620,"parent_record_offset":141596,"ancestor_record_offset":141536,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1455,"record_offset":141596,"parent_record_offset":141564,"ancestor_record_offset":141536,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 989
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135DA
// 首条位置: {"module":1455,"inlinee":79322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1490,"record_offset":36816,"parent_record_offset":36788,"ancestor_record_offset":36788,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":36896,"parent_record_offset":36872,"ancestor_record_offset":36788,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":36920,"parent_record_offset":36896,"ancestor_record_offset":36788,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":36872,"parent_record_offset":36848,"ancestor_record_offset":36788,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":36848,"parent_record_offset":36816,"ancestor_record_offset":36788,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=102
// 源码行: 992
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1380D
// 首条位置: {"module":1500,"inlinee":79885}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=97
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":32344,"parent_record_offset":32320,"ancestor_record_offset":32320,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1500,"record_offset":33860,"parent_record_offset":33836,"ancestor_record_offset":33836,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$17
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 1487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13802
// 首条位置: {"module":1500,"inlinee":79874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":2876,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$18
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13773
// 首条位置: {"module":1490,"inlinee":79731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":49812,"parent_record_offset":49788,"ancestor_record_offset":49788,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":49972,"parent_record_offset":49944,"ancestor_record_offset":49788,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":49880,"parent_record_offset":49840,"ancestor_record_offset":49788,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":49908,"parent_record_offset":49840,"ancestor_record_offset":49788,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":49840,"parent_record_offset":49812,"ancestor_record_offset":49788,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":49944,"parent_record_offset":49908,"ancestor_record_offset":49788,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams
// 记录形式: compiled_procedure; instances=1
// 源码行: 1536
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":70892,"rva":21246416,"va":5389955536}
// 编译实例 RVAs: 0x14431d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14280948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e80c1726ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389959647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21234461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8e3fcbbfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389956648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20416687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e85177c8fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389958682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15861114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e886fa0dff","size":5,"soft_fail":false,"target_va":5374098704,"va":5389959813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14332796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8844c25ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5389958855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11835616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e820674bff","size":5,"soft_fail":false,"target_va":5378120480,"va":5389956091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11835567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e851674bff","size":5,"soft_fail":false,"target_va":5378120496,"va":5389956058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11831765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e82b764bff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389958704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4e290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8343577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8e7af80ff","size":5,"soft_fail":false,"target_va":5381612176,"va":5389955748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8232148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e82c6382ff","size":5,"soft_fail":false,"target_va":5381726576,"va":5389958719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6633247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8e1c89aff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389958266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6631352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e848d09aff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389956403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-336189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8c3defaff","size":5,"soft_fail":false,"target_va":5389619632,"va":5389955816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f96d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-302059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e81564fbff","size":5,"soft_fail":false,"target_va":5389653712,"va":5389955766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-264794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8a6f5fbff","size":5,"soft_fail":false,"target_va":5389694032,"va":5389958821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-170305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8bf66fdff","size":5,"soft_fail":false,"target_va":5389786032,"va":5389956332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8317cfeff","size":5,"soft_fail":false,"target_va":5389857472,"va":5389956746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e32b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17430913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e881f90901","size":5,"soft_fail":false,"target_va":5407388336,"va":5389957418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26394457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e859bf9201","size":5,"soft_fail":false,"target_va":5416352064,"va":5389957602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26661606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8e6d29601","size":5,"soft_fail":false,"target_va":5416621600,"va":5389959989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26662144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e800d59601","size":5,"soft_fail":false,"target_va":5416622208,"va":5389960059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26661637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e805d39601","size":5,"soft_fail":false,"target_va":5416621762,"va":5389960120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8758275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8fd5b7aff","size":5,"soft_fail":false,"target_va":5381197456,"va":5389955726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8761513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8574f7aff","size":5,"soft_fail":false,"target_va":5381195024,"va":5389956532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11830900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e88c794bff","size":5,"soft_fail":false,"target_va":5378126544,"va":5389957439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10450113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e83f8b60ff","size":5,"soft_fail":false,"target_va":5379507120,"va":5389957228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8303064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8284e81ff","size":5,"soft_fail":false,"target_va":5381654320,"va":5389957379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8721835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e855ea7aff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389956438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_transition_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8722652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e824e77aff","size":5,"soft_fail":false,"target_va":5381233488,"va":5389956135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8723520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8c0e37aff","size":5,"soft_fail":false,"target_va":5381232176,"va":5389955691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26616800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8e0239601","size":5,"soft_fail":false,"target_va":5416576832,"va":5389960027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26616067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e803219601","size":5,"soft_fail":false,"target_va":5416576144,"va":5389960072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5971047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e899e3a4ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5389955890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5974475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e835d6a4ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5389959830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6550418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e86e0c9cff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389958669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6655269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8db729aff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389957504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6655224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e808739aff","size":5,"soft_fail":false,"target_va":5383302304,"va":5389957523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6655592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e898719aff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389957491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6655350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e88a729aff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389957569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6641352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e838a99aff","size":5,"soft_fail":false,"target_va":5383314944,"va":5389956291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6638274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e83eb59aff","size":5,"soft_fail":false,"target_va":5383319712,"va":5389957981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6633057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e89fc99aff","size":5,"soft_fail":false,"target_va":5383323216,"va":5389956268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6647501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e833919aff","size":5,"soft_fail":false,"target_va":5383309120,"va":5389956616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6633213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e803c99aff","size":5,"soft_fail":false,"target_va":5383323424,"va":5389956632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17430884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e864f90901","size":5,"soft_fail":false,"target_va":5407388048,"va":5389957159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17429455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e8cff30901","size":5,"soft_fail":false,"target_va":5407387136,"va":5389957676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6682597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389955536,"raw_hex":"e81b089aff","size":5,"soft_fail":false,"target_va":5383275712,"va":5389958304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5389955536,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5389955838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":71280,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":74212,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1490,"record_offset":74940,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1490,"record_offset":74680,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":78204,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":76340,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":73060,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":73032,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1490,"record_offset":74540,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":71956,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1490,"record_offset":71224,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1490,"record_offset":74624,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":73208,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":74428,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8913,"source_line":390,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":71368,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":77508,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":74456,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":73116,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":71308,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":71404,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1490,"record_offset":71196,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":79718,"source_line":429,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1490,"record_offset":75220,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":72052,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":72108,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":77184,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":75156,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":75716,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1490,"record_offset":78120,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":73348,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":73640,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::wooden_wall_stone_foundation_transition_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1490,"record_offset":71836,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":26243,"source_line":539,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1490,"record_offset":76396,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":79732,"source_line":1578,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1490,"record_offset":75772,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":72164,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":72468,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1490,"record_offset":74184,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1490,"record_offset":74156,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":77696,"parent_record_offset":77672,"ancestor_record_offset":70892,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":77720,"parent_record_offset":77696,"ancestor_record_offset":70892,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":77744,"parent_record_offset":77720,"ancestor_record_offset":70892,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":77672,"parent_record_offset":77580,"ancestor_record_offset":70892,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":74304,"parent_record_offset":74272,"ancestor_record_offset":70892,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":77608,"parent_record_offset":77580,"ancestor_record_offset":70892,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":77580,"parent_record_offset":77556,"ancestor_record_offset":70892,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":74352,"parent_record_offset":74328,"ancestor_record_offset":70892,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":74328,"parent_record_offset":74304,"ancestor_record_offset":70892,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":77556,"parent_record_offset":77532,"ancestor_record_offset":70892,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":74272,"parent_record_offset":74236,"ancestor_record_offset":70892,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":74236,"parent_record_offset":74212,"ancestor_record_offset":70892,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1490,"record_offset":71248,"parent_record_offset":71224,"ancestor_record_offset":70892,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1490,"record_offset":74964,"parent_record_offset":74940,"ancestor_record_offset":70892,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1490,"record_offset":74704,"parent_record_offset":74680,"ancestor_record_offset":70892,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1490,"record_offset":74816,"parent_record_offset":74728,"ancestor_record_offset":70892,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":73256,"parent_record_offset":73232,"ancestor_record_offset":70892,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":73280,"parent_record_offset":73256,"ancestor_record_offset":70892,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1490,"record_offset":72012,"parent_record_offset":71984,"ancestor_record_offset":70892,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1490,"record_offset":76248,"parent_record_offset":75796,"ancestor_record_offset":70892,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1490,"record_offset":76072,"parent_record_offset":76048,"ancestor_record_offset":70892,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":73556,"parent_record_offset":73532,"ancestor_record_offset":70892,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":75512,"parent_record_offset":75244,"ancestor_record_offset":70892,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":75428,"parent_record_offset":75244,"ancestor_record_offset":70892,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":76048,"parent_record_offset":76024,"ancestor_record_offset":70892,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1490,"record_offset":75300,"parent_record_offset":75244,"ancestor_record_offset":70892,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":72380,"parent_record_offset":72164,"ancestor_record_offset":70892,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":77380,"parent_record_offset":77356,"ancestor_record_offset":70892,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1490,"record_offset":71984,"parent_record_offset":71956,"ancestor_record_offset":70892,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1490,"record_offset":74840,"parent_record_offset":74816,"ancestor_record_offset":70892,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":71760,"parent_record_offset":71732,"ancestor_record_offset":70892,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":74784,"parent_record_offset":74760,"ancestor_record_offset":70892,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":77640,"parent_record_offset":77608,"ancestor_record_offset":70892,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":73872,"parent_record_offset":73848,"ancestor_record_offset":70892,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":73848,"parent_record_offset":73824,"ancestor_record_offset":70892,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":72132,"parent_record_offset":72108,"ancestor_record_offset":70892,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":72076,"parent_record_offset":72052,"ancestor_record_offset":70892,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":72884,"parent_record_offset":72860,"ancestor_record_offset":70892,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1490,"record_offset":74760,"parent_record_offset":74728,"ancestor_record_offset":70892,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1490,"record_offset":74728,"parent_record_offset":74704,"ancestor_record_offset":70892,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1490,"record_offset":73232,"parent_record_offset":73208,"ancestor_record_offset":70892,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":71668,"parent_record_offset":71404,"ancestor_record_offset":70892,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":78228,"parent_record_offset":78204,"ancestor_record_offset":70892,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":71480,"parent_record_offset":71404,"ancestor_record_offset":70892,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":71700,"parent_record_offset":71404,"ancestor_record_offset":70892,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":73588,"parent_record_offset":73504,"ancestor_record_offset":70892,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":74864,"parent_record_offset":74840,"ancestor_record_offset":70892,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":77532,"parent_record_offset":77508,"ancestor_record_offset":70892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":74504,"parent_record_offset":74480,"ancestor_record_offset":70892,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":74024,"parent_record_offset":74000,"ancestor_record_offset":70892,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1490,"record_offset":73176,"parent_record_offset":73116,"ancestor_record_offset":70892,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1490,"record_offset":73148,"parent_record_offset":73116,"ancestor_record_offset":70892,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":74888,"parent_record_offset":74864,"ancestor_record_offset":70892,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":74392,"parent_record_offset":74236,"ancestor_record_offset":70892,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":71640,"parent_record_offset":71404,"ancestor_record_offset":70892,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":72504,"parent_record_offset":72468,"ancestor_record_offset":70892,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1490,"record_offset":72532,"parent_record_offset":72504,"ancestor_record_offset":70892,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1490,"record_offset":74480,"parent_record_offset":74456,"ancestor_record_offset":70892,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1490,"record_offset":74564,"parent_record_offset":74540,"ancestor_record_offset":70892,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":78380,"parent_record_offset":78316,"ancestor_record_offset":70892,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":71552,"parent_record_offset":71516,"ancestor_record_offset":70892,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":78144,"parent_record_offset":78120,"ancestor_record_offset":70892,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":78404,"parent_record_offset":78380,"ancestor_record_offset":70892,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":71576,"parent_record_offset":71552,"ancestor_record_offset":70892,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1490,"record_offset":78168,"parent_record_offset":78144,"ancestor_record_offset":70892,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1490,"record_offset":71332,"parent_record_offset":71308,"ancestor_record_offset":70892,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":71732,"parent_record_offset":71700,"ancestor_record_offset":70892,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1490,"record_offset":78288,"parent_record_offset":78260,"ancestor_record_offset":70892,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":71864,"parent_record_offset":71836,"ancestor_record_offset":70892,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":71916,"parent_record_offset":71892,"ancestor_record_offset":70892,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1490,"record_offset":71892,"parent_record_offset":71864,"ancestor_record_offset":70892,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1490,"record_offset":75796,"parent_record_offset":75772,"ancestor_record_offset":70892,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1490,"record_offset":75848,"parent_record_offset":75796,"ancestor_record_offset":70892,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1490,"record_offset":75244,"parent_record_offset":75220,"ancestor_record_offset":70892,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":75928,"parent_record_offset":75848,"ancestor_record_offset":70892,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":76272,"parent_record_offset":76248,"ancestor_record_offset":70892,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1490,"record_offset":76024,"parent_record_offset":75796,"ancestor_record_offset":70892,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":77324,"parent_record_offset":77208,"ancestor_record_offset":70892,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":72916,"parent_record_offset":72664,"ancestor_record_offset":70892,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":72972,"parent_record_offset":72664,"ancestor_record_offset":70892,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1490,"record_offset":72860,"parent_record_offset":72664,"ancestor_record_offset":70892,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":73792,"parent_record_offset":73760,"ancestor_record_offset":70892,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":73824,"parent_record_offset":73792,"ancestor_record_offset":70892,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":73912,"parent_record_offset":73760,"ancestor_record_offset":70892,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":73760,"parent_record_offset":73720,"ancestor_record_offset":70892,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":73532,"parent_record_offset":73504,"ancestor_record_offset":70892,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":73396,"parent_record_offset":73372,"ancestor_record_offset":70892,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":73664,"parent_record_offset":73640,"ancestor_record_offset":70892,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":74000,"parent_record_offset":73720,"ancestor_record_offset":70892,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73504,"parent_record_offset":73444,"ancestor_record_offset":70892,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73720,"parent_record_offset":73688,"ancestor_record_offset":70892,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73420,"parent_record_offset":73396,"ancestor_record_offset":70892,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73476,"parent_record_offset":73444,"ancestor_record_offset":70892,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73444,"parent_record_offset":73420,"ancestor_record_offset":70892,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":73688,"parent_record_offset":73664,"ancestor_record_offset":70892,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":71516,"parent_record_offset":71404,"ancestor_record_offset":70892,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1490,"record_offset":78316,"parent_record_offset":78288,"ancestor_record_offset":70892,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":71608,"parent_record_offset":71516,"ancestor_record_offset":70892,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1490,"record_offset":78352,"parent_record_offset":78316,"ancestor_record_offset":70892,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":73372,"parent_record_offset":73348,"ancestor_record_offset":70892,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1490,"record_offset":77356,"parent_record_offset":77324,"ancestor_record_offset":70892,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1490,"record_offset":72568,"parent_record_offset":72468,"ancestor_record_offset":70892,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72436,"parent_record_offset":72164,"ancestor_record_offset":70892,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72408,"parent_record_offset":72164,"ancestor_record_offset":70892,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72212,"parent_record_offset":72164,"ancestor_record_offset":70892,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72244,"parent_record_offset":72212,"ancestor_record_offset":70892,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72344,"parent_record_offset":72316,"ancestor_record_offset":70892,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1490,"record_offset":72316,"parent_record_offset":72164,"ancestor_record_offset":70892,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 1559
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13723
// 首条位置: {"module":1482,"inlinee":79651}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13774
// 首条位置: {"module":1490,"inlinee":79732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::get_window_aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 1978
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":68056,"rva":21244720,"va":5389953840}
// 编译实例 RVAs: 0x1442b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14275431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e8992c26ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389954130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e32b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17434393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e819070a01","size":5,"soft_fail":false,"target_va":5407388336,"va":5389953938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26667410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e892e99601","size":5,"soft_fail":false,"target_va":5416621600,"va":5389954185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11090402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e81ec656ff","size":5,"soft_fail":false,"target_va":5378863520,"va":5389953917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11090221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e8d3c656ff","size":5,"soft_fail":false,"target_va":5378863824,"va":5389954040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11827356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e864874bff","size":5,"soft_fail":false,"target_va":5378126544,"va":5389953895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6652255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389953840,"raw_hex":"e8a17e9aff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389954154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":68312,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1490,"record_offset":68368,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":68464,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1490,"record_offset":68340,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1490,"record_offset":68408,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1490,"record_offset":68284,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1490,"record_offset":68256,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1490,"record_offset":68228,"parent_record_offset":68056,"ancestor_record_offset":68056,"inlinee":27051,"source_line":156,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":68648,"parent_record_offset":68624,"ancestor_record_offset":68056,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":68672,"parent_record_offset":68648,"ancestor_record_offset":68056,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":68696,"parent_record_offset":68672,"ancestor_record_offset":68056,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1490,"record_offset":68624,"parent_record_offset":68536,"ancestor_record_offset":68056,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":68564,"parent_record_offset":68536,"ancestor_record_offset":68056,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":68536,"parent_record_offset":68512,"ancestor_record_offset":68056,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1490,"record_offset":68512,"parent_record_offset":68488,"ancestor_record_offset":68056,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1490,"record_offset":68592,"parent_record_offset":68564,"ancestor_record_offset":68056,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1490,"record_offset":68432,"parent_record_offset":68408,"ancestor_record_offset":68056,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1490,"record_offset":68488,"parent_record_offset":68464,"ancestor_record_offset":68056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::wrap_around
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 2028
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13771
// 首条位置: {"module":1490,"inlinee":79729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":19532,"parent_record_offset":19496,"ancestor_record_offset":19496,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1490,"record_offset":19572,"parent_record_offset":19532,"ancestor_record_offset":19496,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1490,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":19496,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1490,"record_offset":19600,"parent_record_offset":19532,"ancestor_record_offset":19496,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 2032
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1490,"record_offset":79836,"rva":21251824,"va":5389960944}
// 编译实例 RVAs: 0x14446f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10453922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e85e7c60ff","size":5,"soft_fail":false,"target_va":5379507120,"va":5389961037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8307004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e8c43e81ff","size":5,"soft_fail":false,"target_va":5381654320,"va":5389961319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e8b3e2ffff","size":5,"soft_fail":false,"target_va":5389953840,"va":5389961336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6658879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e8c1649aff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389961114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6658897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e8af649aff","size":5,"soft_fail":false,"target_va":5383302304,"va":5389961196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6659282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e82e639aff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389961181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6658872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e8c8649aff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389961155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6658924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389960944,"raw_hex":"e894649aff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389961143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":80004,"parent_record_offset":79836,"ancestor_record_offset":79836,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":80332,"parent_record_offset":79836,"ancestor_record_offset":79836,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1490,"record_offset":80248,"parent_record_offset":80224,"ancestor_record_offset":79836,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":80568,"parent_record_offset":80544,"ancestor_record_offset":79836,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1490,"record_offset":80544,"parent_record_offset":80520,"ancestor_record_offset":79836,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1490,"record_offset":80280,"parent_record_offset":80196,"ancestor_record_offset":79836,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1490,"record_offset":80692,"parent_record_offset":80668,"ancestor_record_offset":79836,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":80488,"parent_record_offset":80460,"ancestor_record_offset":79836,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":80520,"parent_record_offset":80488,"ancestor_record_offset":79836,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1490,"record_offset":80460,"parent_record_offset":80420,"ancestor_record_offset":79836,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1490,"record_offset":80224,"parent_record_offset":80196,"ancestor_record_offset":79836,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":80068,"parent_record_offset":80036,"ancestor_record_offset":79836,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1490,"record_offset":80360,"parent_record_offset":80332,"ancestor_record_offset":79836,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80668,"parent_record_offset":80420,"ancestor_record_offset":79836,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80196,"parent_record_offset":80132,"ancestor_record_offset":79836,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80420,"parent_record_offset":80388,"ancestor_record_offset":79836,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80100,"parent_record_offset":80068,"ancestor_record_offset":79836,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80168,"parent_record_offset":80132,"ancestor_record_offset":79836,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80132,"parent_record_offset":80100,"ancestor_record_offset":79836,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1490,"record_offset":80388,"parent_record_offset":80360,"ancestor_record_offset":79836,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01490.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1490,"record_offset":80036,"parent_record_offset":80004,"ancestor_record_offset":79836,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01490.jsonl.gz"}
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
// 完整结构及原始类型记录: half_timber.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: half_timber.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: half_timber.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// 信息 ID: d529c2c530dffb072d56ff46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::utils::half_timber::add_underwindow_pattern -> ["system_wall_constructor::utils::half_timber::half_timbering::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143cfd6 -> 0x24e26e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 信息 ID: 29810593737162c05b08a331
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143cf34 -> 0x14446f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143c347 -> 0x14446f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::get_window_aabb
// 信息 ID: 4cdc25d8f8193d1bde6a1cdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1442e8e -> 0x1442b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x1444878 -> 0x1442b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering
// 信息 ID: e28f6063e1a545f33b564ba8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8050e9 -> 0x143ae20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 信息 ID: 92248628c7825072bcf7356c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$0 <- system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=26924; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=31692; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=43896; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=47772; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=51064; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; module=3153, inline_record=3520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$1
// 信息 ID: 6bc4310c6b368ec3aca3a2d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1455, inline_record=124664; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1455, inline_record=125360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 信息 ID: b597f21ae817462225d86b2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$10 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10> >; module=1455, inline_record=66656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 信息 ID: 5acf193099d741ab0d325045
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::slice::sort::stable::merge::merge<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 38 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=4200; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=4300; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=4408; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=4524; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=4752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=5088; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=24608; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; module=1500, inline_record=24992; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 信息 ID: 30b398c12b4f9ec7b2ebec72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=22928; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=23276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 信息 ID: 8e702cb607505bb1d7dd4386
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$13 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$13 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<usize,f32,f32>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$13 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>; module=1455, inline_record=5780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 信息 ID: fdc78753cfb04ace9fd58835
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1455, inline_record=141536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 信息 ID: b7e325d6e1fe4a71c8dc918f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 15, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=36788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 信息 ID: 47977ffb08bfbd54d9305e1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 16, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::unstable::quicksort::quicksort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::shared::pivot::median3_rec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::unstable::heapsort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 97 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=32320; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=32584; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=32824; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=33064; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=33328; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=33568; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=33836; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1500, inline_record=34124; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$17
// 信息 ID: 558063b73aaf25904461f1d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 17, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::stable::driftsort_main<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17>,alloc::vec::Vec<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::stable::drift::sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::stable::quicksort::quicksort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$17 <- core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=2856; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=2956; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=3152; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=3268; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=3500; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=3836; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=81288; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$17> >; module=1500, inline_record=81492; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$18
// 信息 ID: a31e3eb0df9dc9b662343204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 18, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=49788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$2
// 信息 ID: 444a56393ee09c3205ff766f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::merge::merge<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 38 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=11228; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=11328; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=11436; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=11552; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=11780; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=12116; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=23268; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1500, inline_record=23652; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 信息 ID: 12021d42f9ea6aee4c6102ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=11300; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=11424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$4
// 信息 ID: 337efbc51921dc159ac3387e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=11800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=11948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 信息 ID: 281f49f1d145253e980c2e74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::slice::sort::stable::merge::merge<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 38 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=9976; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=10076; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=10184; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=10300; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=10528; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=10864; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=30768; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; module=1500, inline_record=31152; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 信息 ID: 911c3183a0697e1819c7ee8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$6> >; module=1455, inline_record=144032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 信息 ID: e41204682ac3c1739b9fdddf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$7 <- core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$7 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>; module=1455, inline_record=1676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$7::closure$0
// 信息 ID: 3ee4173e013a17c120f371c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>; module=1455, inline_record=2164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$9
// 信息 ID: 8030d8253e5a962d90e0c419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=14508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::overlapping_windows
// 信息 ID: 637aea959eeb51d7fec7d8a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10> > @ 0x13f5d18 -> 0x1442cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams
// 信息 ID: 45f3acac822d35cad2118d24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x805167 -> 0x14431d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams::closure$0
// 信息 ID: ccd3b3eb051a5d22653572a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::place_rectangle_corner_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::place_rectangle_corner_beams::closure$1
// 信息 ID: adc9fbed643fdbb2ba28de29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::place_rectangle_corner_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams; module=1490, inline_record=76396; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams; module=1490, inline_record=77480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline
// 信息 ID: 2ef8c748b6ad171c7b2b7b49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8051e0 -> 0x1439cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 信息 ID: ebe82f0c6f6c0d6258b3c959
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=6956; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=8108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$1
// 信息 ID: b1c5d914615e02cd0f17c4d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=1236; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=1384; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=1672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 信息 ID: 9b50e44e3d6ff7da47118690
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::roof_outline::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1455, inline_record=172604; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1455, inline_record=172880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$3
// 信息 ID: f8f66bf249d92e0fff447bf4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1490, inline_record=3120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::wrap_around
// 信息 ID: 4d34333e1978f1c95c819462
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=19496; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1490, inline_record=19668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
