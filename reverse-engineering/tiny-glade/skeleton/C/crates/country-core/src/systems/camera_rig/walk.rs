// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/camera_rig/walk.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cfd237b871270c3b24c68ef837ecbe254ac0241e05b71147b72294163a67613a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: walk.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 549
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use dolly; // namespace candidate
//   package identities: [{"name":"dolly","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":78272,"rva":11268064,"va":5379977184}
// 编译实例 RVAs: 0xabefe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3346385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379977184,"raw_hex":"e8d10f3300","size":5,"soft_fail":false,"target_va":5383325056,"va":5379978666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37079286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379977184,"raw_hex":"e8f6c83502","size":5,"soft_fail":false,"target_va":5417057760,"va":5379978469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36643138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379977184,"raw_hex":"e842212f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379979097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-822144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379977184,"raw_hex":"e88074f3ff","size":5,"soft_fail":false,"target_va":5379155904,"va":5379978043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*56(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379977184,"raw_hex":"ff5038","size":3,"soft_fail":false,"va":5379977812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":80328,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":549,"record_offset":81464,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::clamp_to_rounded_glade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":81752,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":32245,"source_line":149,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::translate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":549,"record_offset":83412,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":33036,"source_line":34,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":79980,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":32976,"source_line":48,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83060,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":78564,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":80296,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79692,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":78508,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":80716,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79468,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::project_onto_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":81004,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":34155,"source_line":743,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79804,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":80624,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":81636,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":80536,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":80240,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":81132,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79916,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":81724,"parent_record_offset":78272,"ancestor_record_offset":78272,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":83252,"parent_record_offset":83228,"ancestor_record_offset":78272,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":80168,"parent_record_offset":80144,"ancestor_record_offset":78272,"inlinee":10140,"source_line":228,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":80192,"parent_record_offset":80168,"ancestor_record_offset":78272,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":83372,"parent_record_offset":83344,"ancestor_record_offset":78272,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":549,"record_offset":81512,"parent_record_offset":81488,"ancestor_record_offset":78272,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":78612,"parent_record_offset":78588,"ancestor_record_offset":78272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":79344,"parent_record_offset":78696,"ancestor_record_offset":78272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":79252,"parent_record_offset":79196,"ancestor_record_offset":78272,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":78792,"parent_record_offset":78768,"ancestor_record_offset":78272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":78824,"parent_record_offset":78740,"ancestor_record_offset":78272,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":78888,"parent_record_offset":78696,"ancestor_record_offset":78272,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":79640,"parent_record_offset":79616,"ancestor_record_offset":78272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":549,"record_offset":81488,"parent_record_offset":81464,"ancestor_record_offset":78272,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":81664,"parent_record_offset":81636,"ancestor_record_offset":78272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":79828,"parent_record_offset":79804,"ancestor_record_offset":78272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":79948,"parent_record_offset":79916,"ancestor_record_offset":78272,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":80564,"parent_record_offset":80536,"ancestor_record_offset":78272,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":81160,"parent_record_offset":81132,"ancestor_record_offset":78272,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":80656,"parent_record_offset":80624,"ancestor_record_offset":78272,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":80264,"parent_record_offset":80240,"ancestor_record_offset":78272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":83320,"parent_record_offset":83060,"ancestor_record_offset":78272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":80112,"parent_record_offset":80056,"ancestor_record_offset":78272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":80084,"parent_record_offset":80056,"ancestor_record_offset":78272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":80028,"parent_record_offset":80004,"ancestor_record_offset":78272,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":80056,"parent_record_offset":80028,"ancestor_record_offset":78272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":83112,"parent_record_offset":83088,"ancestor_record_offset":78272,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":83140,"parent_record_offset":83112,"ancestor_record_offset":78272,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":81792,"parent_record_offset":81752,"ancestor_record_offset":78272,"inlinee":32130,"source_line":170,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":549,"record_offset":82064,"parent_record_offset":81752,"ancestor_record_offset":78272,"inlinee":32133,"source_line":175,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":80004,"parent_record_offset":79980,"ancestor_record_offset":78272,"inlinee":32977,"source_line":58,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83088,"parent_record_offset":83060,"ancestor_record_offset":78272,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83580,"parent_record_offset":83556,"ancestor_record_offset":78272,"inlinee":32983,"source_line":49,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83344,"parent_record_offset":83320,"ancestor_record_offset":78272,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":80144,"parent_record_offset":80028,"ancestor_record_offset":78272,"inlinee":32979,"source_line":61,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83228,"parent_record_offset":83112,"ancestor_record_offset":78272,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":79716,"parent_record_offset":79692,"ancestor_record_offset":78272,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":78588,"parent_record_offset":78564,"ancestor_record_offset":78272,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":78696,"parent_record_offset":78588,"ancestor_record_offset":78272,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":78944,"parent_record_offset":78696,"ancestor_record_offset":78272,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":78660,"parent_record_offset":78636,"ancestor_record_offset":78272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":549,"record_offset":78636,"parent_record_offset":78612,"ancestor_record_offset":78272,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":82100,"parent_record_offset":82064,"ancestor_record_offset":78272,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":81816,"parent_record_offset":81792,"ancestor_record_offset":78272,"inlinee":32131,"source_line":867,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":81872,"parent_record_offset":81848,"ancestor_record_offset":78272,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":81848,"parent_record_offset":81816,"ancestor_record_offset":78272,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":82124,"parent_record_offset":82100,"ancestor_record_offset":78272,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":82328,"parent_record_offset":82064,"ancestor_record_offset":78272,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":82384,"parent_record_offset":82064,"ancestor_record_offset":78272,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":82160,"parent_record_offset":82064,"ancestor_record_offset":78272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":78532,"parent_record_offset":78508,"ancestor_record_offset":78272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79552,"parent_record_offset":79528,"ancestor_record_offset":78272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79528,"parent_record_offset":79496,"ancestor_record_offset":78272,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":79496,"parent_record_offset":79468,"ancestor_record_offset":78272,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":83440,"parent_record_offset":83412,"ancestor_record_offset":78272,"inlinee":33037,"source_line":1081,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":78768,"parent_record_offset":78740,"ancestor_record_offset":78272,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":549,"record_offset":78740,"parent_record_offset":78696,"ancestor_record_offset":78272,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":79616,"parent_record_offset":79584,"ancestor_record_offset":78272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":268,"rva":11215040,"va":5379924160}
// 编译实例 RVAs: 0xab20c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10698401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379924160,"raw_hex":"e85fc15cff","size":5,"soft_fail":false,"target_va":5369226112,"va":5379924508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] dolly::rig::CameraRigBuilder$LT$H$GT$::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15942207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379924160,"raw_hex":"e83f42f300","size":5,"soft_fail":false,"target_va":5395866464,"va":5379924252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379924160,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5379924352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::drivers::position::Position::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":549,"record_offset":444,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":32150,"source_line":24,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::builder
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":416,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":32148,"source_line":86,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":472,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":32154,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":560,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":720,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":684,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":652,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":616,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":32155,"source_line":67,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":980,"rva":11215424,"va":5379924544}
// 编译实例 RVAs: 0xab2240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..euler..EulerRot$u20$as$u20$glam..euler..EulerFromQuaternion$LT$glam..f32..sse2..quat..Quat$GT$$GT$::convert_quat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17532699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379924544,"raw_hex":"e81b870b01","size":5,"soft_fail":false,"target_va":5397458224,"va":5379925520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36696555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379924544,"raw_hex":"e8ebf12f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379925680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379924544,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5379924676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":1140,"parent_record_offset":980,"ancestor_record_offset":980,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::yaw_pitch::YawPitch::set_rotation_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/drivers/yaw_pitch.rs
//     evidence={"module":549,"record_offset":3284,"parent_record_offset":980,"ancestor_record_offset":980,"inlinee":32333,"source_line":75,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":1196,"parent_record_offset":980,"ancestor_record_offset":980,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::util::look_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/util.rs
//     evidence={"module":549,"record_offset":1892,"parent_record_offset":980,"ancestor_record_offset":980,"inlinee":16681,"source_line":57,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":980,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":980,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":980,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":980,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_degrees
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":3340,"parent_record_offset":3284,"ancestor_record_offset":980,"inlinee":32218,"source_line":932,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":980,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":980,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":2264,"parent_record_offset":1892,"ancestor_record_offset":980,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":1456,"parent_record_offset":1196,"ancestor_record_offset":980,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":1332,"parent_record_offset":1276,"ancestor_record_offset":980,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":980,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":980,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":980,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::handedness::impl$1::right_from_up_and_forward
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/handedness.rs
//     evidence={"module":549,"record_offset":2324,"parent_record_offset":2288,"ancestor_record_offset":980,"inlinee":9845,"source_line":56,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::handedness::impl$1::up_from_right_and_forward
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/handedness.rs
//     evidence={"module":549,"record_offset":2664,"parent_record_offset":2288,"ancestor_record_offset":980,"inlinee":9846,"source_line":68,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":1224,"parent_record_offset":1196,"ancestor_record_offset":980,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":980,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":1364,"parent_record_offset":1248,"ancestor_record_offset":980,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::util::look_at::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/util.rs
//     evidence={"module":549,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":980,"inlinee":16666,"source_line":66,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":549,"record_offset":2928,"parent_record_offset":2808,"ancestor_record_offset":980,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":2784,"parent_record_offset":2288,"ancestor_record_offset":980,"inlinee":16667,"source_line":253,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":980,"inlinee":16668,"source_line":192,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":980,"inlinee":32217,"source_line":397,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":980,"inlinee":9830,"source_line":209,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":980,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":980,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":2016,"parent_record_offset":1984,"ancestor_record_offset":980,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":1984,"parent_record_offset":1892,"ancestor_record_offset":980,"inlinee":16662,"source_line":523,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":2172,"parent_record_offset":1984,"ancestor_record_offset":980,"inlinee":16664,"source_line":1048,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$34::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_mint.rs
//     evidence={"module":549,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":980,"inlinee":32985,"source_line":47,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$36::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/features/impl_mint.rs
//     evidence={"module":549,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":980,"inlinee":9834,"source_line":62,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":549,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":980,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::rotation
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":756,"rva":11215408,"va":5379924528}
// 编译实例 RVAs: 0xab2230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":904,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":549,"record_offset":944,"parent_record_offset":924,"ancestor_record_offset":756,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_mint.rs
//     evidence={"module":549,"record_offset":924,"parent_record_offset":904,"ancestor_record_offset":756,"inlinee":16670,"source_line":104,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":77768,"rva":10891008,"va":5379600128}
// 编译实例 RVAs: 0xa62f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37021842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379600128,"raw_hex":"e892e83402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379600393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] dolly::drivers::yaw_pitch::YawPitch::rotate_yaw_pitch
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t17714533","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379600128,"raw_hex":"e9654d0e01","size":5,"soft_fail":false,"target_va":5397314864,"va":5379600326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379600128,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5379600224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":77920,"parent_record_offset":77768,"ancestor_record_offset":77768,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":78108,"parent_record_offset":78088,"ancestor_record_offset":77768,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":78132,"parent_record_offset":78108,"ancestor_record_offset":77768,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":78228,"parent_record_offset":78200,"ancestor_record_offset":77768,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":78176,"parent_record_offset":77920,"ancestor_record_offset":77768,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":78056,"parent_record_offset":78000,"ancestor_record_offset":77768,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":78028,"parent_record_offset":78000,"ancestor_record_offset":77768,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":77972,"parent_record_offset":77948,"ancestor_record_offset":77768,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":78000,"parent_record_offset":77972,"ancestor_record_offset":77768,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":77948,"parent_record_offset":77920,"ancestor_record_offset":77768,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":78200,"parent_record_offset":78176,"ancestor_record_offset":77768,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":78088,"parent_record_offset":77972,"ancestor_record_offset":77768,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":83652,"rva":11270064,"va":5379979184}
// 编译实例 RVAs: 0xabf7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1887344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979184,"raw_hex":"e870cc1c00","size":5,"soft_fail":false,"target_va":5381867296,"va":5379979947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36642030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979184,"raw_hex":"e8ee1c2f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379980205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-823756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979184,"raw_hex":"e8346ef3ff","size":5,"soft_fail":false,"target_va":5379155904,"va":5379979655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3298389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979184,"raw_hex":"e855543200","size":5,"soft_fail":false,"target_va":5383278464,"va":5379980070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*64(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379979184,"raw_hex":"ff5040","size":3,"soft_fail":false,"va":5379979332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":84184,"parent_record_offset":83652,"ancestor_record_offset":83652,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":84248,"parent_record_offset":83652,"ancestor_record_offset":83652,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":549,"record_offset":84276,"parent_record_offset":83652,"ancestor_record_offset":83652,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83832,"parent_record_offset":83652,"ancestor_record_offset":83652,"inlinee":32135,"source_line":31,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":549,"record_offset":84304,"parent_record_offset":83652,"ancestor_record_offset":83652,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":84024,"parent_record_offset":84000,"ancestor_record_offset":83652,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":84048,"parent_record_offset":84024,"ancestor_record_offset":83652,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":549,"record_offset":84144,"parent_record_offset":84116,"ancestor_record_offset":83652,"inlinee":9375,"source_line":930,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":84412,"parent_record_offset":84304,"ancestor_record_offset":83652,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":84356,"parent_record_offset":84304,"ancestor_record_offset":83652,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":84328,"parent_record_offset":84304,"ancestor_record_offset":83652,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":84092,"parent_record_offset":83832,"ancestor_record_offset":83652,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":83968,"parent_record_offset":83912,"ancestor_record_offset":83652,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":83940,"parent_record_offset":83912,"ancestor_record_offset":83652,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":83884,"parent_record_offset":83860,"ancestor_record_offset":83652,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":83912,"parent_record_offset":83884,"ancestor_record_offset":83652,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":83860,"parent_record_offset":83832,"ancestor_record_offset":83652,"inlinee":32136,"source_line":41,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":84116,"parent_record_offset":84092,"ancestor_record_offset":83652,"inlinee":32141,"source_line":32,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":84000,"parent_record_offset":83884,"ancestor_record_offset":83652,"inlinee":32139,"source_line":44,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":84504,"rva":10892288,"va":5379601408}
// 编译实例 RVAs: 0xa63400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379601408,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5379601522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/rig.rs
//     evidence={"module":549,"record_offset":84648,"parent_record_offset":84504,"ancestor_record_offset":84504,"inlinee":32155,"source_line":67,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] dolly::transform::Transform::into_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dolly-0.6.0/src/transform.rs
//     evidence={"module":549,"record_offset":84820,"parent_record_offset":84504,"ancestor_record_offset":84504,"inlinee":33039,"source_line":31,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":84720,"parent_record_offset":84696,"ancestor_record_offset":84504,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":84696,"parent_record_offset":84648,"ancestor_record_offset":84504,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":84780,"parent_record_offset":84752,"ancestor_record_offset":84504,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":549,"record_offset":84752,"parent_record_offset":84648,"ancestor_record_offset":84504,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00549.jsonl.gz"}
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
// 完整结构及原始类型记录: walk.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::walk::WalkCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::walk::WalkCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::walk::WalkCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: walk.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: walk.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::new
// 信息 ID: 087e0fbb2fd2d99b8834f856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::new @ 0xa1f31d -> 0xab20c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::rotation
// 信息 ID: 8a31fc68253f4a71420f4a21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x353b33 -> 0xab2230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 信息 ID: d62b335588925a71590036d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x354cf4 -> 0xab2240 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x35657f -> 0xab2240 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::init_walk_camera @ 0xa1e300 -> 0xab2240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain
// 信息 ID: 938af43388212526fff952ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::rotate_yaw_pitch
// 信息 ID: 5788c48c33d06f8af9089647
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::translate_screen_space
// 信息 ID: a3a59ffb0e18a1cb7b0635ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$2::update
// 信息 ID: a137627cc878b503a760d726
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
