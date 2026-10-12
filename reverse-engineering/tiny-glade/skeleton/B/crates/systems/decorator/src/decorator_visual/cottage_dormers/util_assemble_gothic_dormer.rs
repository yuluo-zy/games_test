// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/util_assemble_gothic_dormer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d668d8b2b3ae644c31ba43fa1da8fb1c00bb8a1163668740603e6150b13ab253
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util_assemble_gothic_dormer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1543
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6>
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1543,"record_offset":264,"rva":19467056,"va":5388176176}
// 编译实例 RVAs: 0x1290b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13096408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8282a38ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388177907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6242019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e81dc1a0ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5388176574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269710
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-163596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8f480fdff","size":5,"soft_fail":false,"target_va":5388015376,"va":5388178967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128d0e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e818bfffff","size":5,"soft_fail":false,"target_va":5388161248,"va":5388177859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141940140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7008423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8a7f06a00","size":5,"soft_fail":false,"target_va":5395185984,"va":5388177556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420071b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14115154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e85261d700","size":5,"soft_fail":false,"target_va":5402292656,"va":5388177497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23669840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8502c6901","size":5,"soft_fail":false,"target_va":5411847776,"va":5388177931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23667868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e89c246901","size":5,"soft_fail":false,"target_va":5411847808,"va":5388179935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23671239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8c7316901","size":5,"soft_fail":false,"target_va":5411848096,"va":5388176852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_assemble_gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23668233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e809266901","size":5,"soft_fail":false,"target_va":5411848144,"va":5388179906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23360607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e85f746401","size":5,"soft_fail":false,"target_va":5411541776,"va":5388181164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23361122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e862766401","size":5,"soft_fail":false,"target_va":5411542064,"va":5388180937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23361195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8ab766401","size":5,"soft_fail":false,"target_va":5411542296,"va":5388181096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23360991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8df756401","size":5,"soft_fail":false,"target_va":5411541904,"va":5388180908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10146743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8492c65ff","size":5,"soft_fail":false,"target_va":5378029600,"va":5388176338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8872633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8479d78ff","size":5,"soft_fail":false,"target_va":5379303968,"va":5388176596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6170095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e811daa1ff","size":5,"soft_fail":false,"target_va":5382009728,"va":5388179818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::closest_point_on_perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6263132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8a46ea0ff","size":5,"soft_fail":false,"target_va":5381913408,"va":5388176535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6250078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e8a2a1a0ff","size":5,"soft_fail":false,"target_va":5381926432,"va":5388176505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6294261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388176176,"raw_hex":"e80bf59fff","size":5,"soft_fail":false,"target_va":5381882704,"va":5388176960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":2612,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1543,"record_offset":1332,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1543,"record_offset":6888,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":28525,"source_line":867,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1543,"record_offset":2640,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1543,"record_offset":708,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1543,"record_offset":2352,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1543,"record_offset":680,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1543,"record_offset":1864,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25309,"source_line":237,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":6916,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":5764,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":6128,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13688,"source_line":894,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5732,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":924,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":832,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5496,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":744,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":1216,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5404,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1543,"record_offset":1360,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1543,"record_offset":6748,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1543,"record_offset":2584,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1543,"record_offset":1304,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":1543,"record_offset":2324,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":71790,"source_line":330,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1543,"record_offset":4028,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1543,"record_offset":2748,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1543,"record_offset":3816,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1543,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1543,"record_offset":2512,"parent_record_offset":2424,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1543,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1543,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1543,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1543,"record_offset":4116,"parent_record_offset":4056,"ancestor_record_offset":264,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1543,"record_offset":1988,"parent_record_offset":1956,"ancestor_record_offset":264,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1543,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":264,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1543,"record_offset":2024,"parent_record_offset":1956,"ancestor_record_offset":264,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":264,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1543,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":264,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":6416,"parent_record_offset":6156,"ancestor_record_offset":264,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":6268,"parent_record_offset":6156,"ancestor_record_offset":264,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1543,"record_offset":6236,"parent_record_offset":6156,"ancestor_record_offset":264,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1543,"record_offset":3968,"parent_record_offset":3816,"ancestor_record_offset":264,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1543,"record_offset":3996,"parent_record_offset":3816,"ancestor_record_offset":264,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1543,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1543,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1543,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":264,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1543,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1543,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1543,"record_offset":2292,"parent_record_offset":1864,"ancestor_record_offset":264,"inlinee":21746,"source_line":1891,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1543,"record_offset":2144,"parent_record_offset":1888,"ancestor_record_offset":264,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1543,"record_offset":2224,"parent_record_offset":2196,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1543,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1543,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":264,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1543,"record_offset":992,"parent_record_offset":952,"ancestor_record_offset":264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1543,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":264,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1543,"record_offset":3856,"parent_record_offset":3816,"ancestor_record_offset":264,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1543,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1543,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1543,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1543,"record_offset":1448,"parent_record_offset":1400,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1543,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1543,"record_offset":4148,"parent_record_offset":4116,"ancestor_record_offset":264,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1543,"record_offset":4180,"parent_record_offset":4148,"ancestor_record_offset":264,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1543,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1543,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1543,"record_offset":1480,"parent_record_offset":1400,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1543,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":264,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1543,"record_offset":1932,"parent_record_offset":1888,"ancestor_record_offset":264,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1543,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":264,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1543,"record_offset":6948,"parent_record_offset":6916,"ancestor_record_offset":264,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1543,"record_offset":5888,"parent_record_offset":5788,"ancestor_record_offset":264,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":264,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":7104,"parent_record_offset":6916,"ancestor_record_offset":264,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1543,"record_offset":6156,"parent_record_offset":6128,"ancestor_record_offset":264,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":264,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5560,"parent_record_offset":5524,"ancestor_record_offset":264,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5524,"parent_record_offset":5496,"ancestor_record_offset":264,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":264,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":264,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":5296,"parent_record_offset":5100,"ancestor_record_offset":264,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1543,"record_offset":952,"parent_record_offset":924,"ancestor_record_offset":264,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1543,"record_offset":1568,"parent_record_offset":1360,"ancestor_record_offset":264,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1543,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":264,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1543,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":264,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1543,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":264,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1543,"record_offset":4056,"parent_record_offset":4028,"ancestor_record_offset":264,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1543,"record_offset":4088,"parent_record_offset":4056,"ancestor_record_offset":264,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1543,"record_offset":4224,"parent_record_offset":4056,"ancestor_record_offset":264,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01543.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1543,"record_offset":4360,"parent_record_offset":4056,"ancestor_record_offset":264,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/01543.jsonl.gz"}
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
// 完整结构及原始类型记录: util_assemble_gothic_dormer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util_assemble_gothic_dormer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util_assemble_gothic_dormer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6>
// 信息 ID: 617732d622dc59aab239888c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles -> ["system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x7cd01c -> 0x1290b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
