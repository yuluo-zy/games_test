// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/utils/half_timber.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4e9800334531e3cfc7b9e077ff9717177fc2f71d1ad503ea63baa300ea9eaf0c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_timber.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1415, 1417, 1421, 1435, 1444, 3006, 3013, 3024, 4061, 4062, 4067, 4068
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
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
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
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
// 首条位置: {"module":3006,"record_offset":2580,"rva":33365040,"va":5402074160}
// 编译实例 RVAs: 0x1fd1c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33121226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e8369c06fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5402074949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32808173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e813630bfe","size":5,"soft_fail":false,"target_va":5369267664,"va":5402075832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24333653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e8abb28cfe","size":5,"soft_fail":false,"target_va":5377742176,"va":5402075824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20140758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e82aadccfe","size":5,"soft_fail":false,"target_va":5381934528,"va":5402075281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14442200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e828a123ff","size":5,"soft_fail":false,"target_va":5387633712,"va":5402075907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9773070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e80e209500","size":5,"soft_fail":false,"target_va":5411847952,"va":5402074877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9466104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e8f8709000","size":5,"soft_fail":false,"target_va":5411542064,"va":5402075955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9465852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e8fc6f9000","size":5,"soft_fail":false,"target_va":5411541904,"va":5402076047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14494175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e821d622ff","size":5,"soft_fail":false,"target_va":5387580928,"va":5402075098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20163201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e87f55ccfe","size":5,"soft_fail":false,"target_va":5381911968,"va":5402075164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20191010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e8dee8cbfe","size":5,"soft_fail":false,"target_va":5381884480,"va":5402075485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20184790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402074160,"raw_hex":"e82a01ccfe","size":5,"soft_fail":false,"target_va":5381890192,"va":5402074977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3006,"record_offset":3024,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3006,"record_offset":5696,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3006,"record_offset":2868,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3006,"record_offset":3952,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4900,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":3784,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4192,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4620,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4436,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4104,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4528,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3006,"record_offset":2896,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":3006,"record_offset":5724,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":70934,"source_line":218,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3006,"record_offset":5112,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3006,"record_offset":3572,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3006,"record_offset":3308,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3006,"record_offset":3172,"parent_record_offset":3052,"ancestor_record_offset":2580,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3006,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":2580,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3006,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2580,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3006,"record_offset":5604,"parent_record_offset":5140,"ancestor_record_offset":2580,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3006,"record_offset":5404,"parent_record_offset":5352,"ancestor_record_offset":2580,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3006,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":2580,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3006,"record_offset":3340,"parent_record_offset":3308,"ancestor_record_offset":2580,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3006,"record_offset":3752,"parent_record_offset":3572,"ancestor_record_offset":2580,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3006,"record_offset":3088,"parent_record_offset":3052,"ancestor_record_offset":2580,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3006,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":2580,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3006,"record_offset":2988,"parent_record_offset":2896,"ancestor_record_offset":2580,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3006,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":2580,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3006,"record_offset":4560,"parent_record_offset":4528,"ancestor_record_offset":2580,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3006,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":2580,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3006,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2580,"inlinee":11824,"source_line":46,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3006,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2580,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3006,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2580,"inlinee":13189,"source_line":163,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3006,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":2580,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3006,"record_offset":3612,"parent_record_offset":3572,"ancestor_record_offset":2580,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3006,"record_offset":3228,"parent_record_offset":3200,"ancestor_record_offset":2580,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":3256,"parent_record_offset":3228,"ancestor_record_offset":2580,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":3006,"record_offset":5140,"parent_record_offset":5112,"ancestor_record_offset":2580,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":3006,"record_offset":5184,"parent_record_offset":5140,"ancestor_record_offset":2580,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3006,"record_offset":5256,"parent_record_offset":5184,"ancestor_record_offset":2580,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3006,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":2580,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3006,"record_offset":5352,"parent_record_offset":5140,"ancestor_record_offset":2580,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3006,"record_offset":4032,"parent_record_offset":3976,"ancestor_record_offset":2580,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3006,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":2580,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4300,"parent_record_offset":4264,"ancestor_record_offset":2580,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4264,"parent_record_offset":4224,"ancestor_record_offset":2580,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4224,"parent_record_offset":4192,"ancestor_record_offset":2580,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":2580,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":3892,"parent_record_offset":3784,"ancestor_record_offset":2580,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3006,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":2580,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3006,"record_offset":4064,"parent_record_offset":4032,"ancestor_record_offset":2580,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3006,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2580,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3006,"record_offset":3368,"parent_record_offset":3308,"ancestor_record_offset":2580,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/03006.jsonl.gz"}
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
// 首条位置: {"module":1435,"record_offset":9516,"rva":18976304,"va":5387685424}
// 编译实例 RVAs: 0x1218e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=7
//     target_aliases=532 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18740584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8980ae2fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5387694307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=16
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18425811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e82dd8e6fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387693470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16116566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8aa140aff","size":5,"soft_fail":false,"target_va":5371570656,"va":5387687217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13021961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f74c39ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5387698292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12905709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e813133bff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387719128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12607006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8e2a13fff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387688489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=50
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12606871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e869a23fff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387688370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12605539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89da73fff","size":5,"soft_fail":false,"target_va":5375081536,"va":5387687070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10395997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8a35e61ff","size":5,"soft_fail":false,"target_va":5377307184,"va":5387703176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9953198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8522068ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387695369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4920
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9657802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e836a26cff","size":5,"soft_fail":false,"target_va":5378033952,"va":5387691749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c37760
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6182161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8efaaa1ff","size":5,"soft_fail":false,"target_va":5381519200,"va":5387701356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5808658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8ee5da7ff","size":5,"soft_fail":false,"target_va":5381883040,"va":5387691693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5760226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e81e1ba8ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387694749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5767491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8bdfea7ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387702046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f86cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2696087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e869dcd6ff","size":5,"soft_fail":false,"target_va":5384989904,"va":5387685986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-130092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8d403feff","size":5,"soft_fail":false,"target_va":5387557152,"va":5387687239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fc650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-127984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8100cfeff","size":5,"soft_fail":false,"target_va":5387568720,"va":5387696699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-140941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e873d9fdff","size":5,"soft_fail":false,"target_va":5387573328,"va":5387714264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141201180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e87a7cfeff","size":5,"soft_fail":false,"target_va":5387587968,"va":5387687169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412021c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8dc5efeff","size":5,"soft_fail":false,"target_va":5387592128,"va":5387698911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141203970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e842a5feff","size":5,"soft_fail":false,"target_va":5387598192,"va":5387686953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141204c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8418afeff","size":5,"soft_fail":false,"target_va":5387602960,"va":5387698634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141204ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e874bbfeff","size":5,"soft_fail":false,"target_va":5387603952,"va":5387687031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141206480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-76800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e800d4feff","size":5,"soft_fail":false,"target_va":5387609216,"va":5387686011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141207c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-93546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89692feff","size":5,"soft_fail":false,"target_va":5387615248,"va":5387708789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-70416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f0ecfeff","size":5,"soft_fail":false,"target_va":5387618304,"va":5387688715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209600
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e843a1feff","size":5,"soft_fail":false,"target_va":5387621888,"va":5387711672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-85662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e862b1feff","size":5,"soft_fail":false,"target_va":5387622160,"va":5387707817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e81b0dffff","size":5,"soft_fail":false,"target_va":5387623488,"va":5387685664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e88e0effff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387695517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-50777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8a739ffff","size":5,"soft_fail":false,"target_va":5387635152,"va":5387685924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t58445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e84de40000","size":5,"soft_fail":false,"target_va":5387766000,"va":5387707550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122cc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t59239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e867e70000","size":5,"soft_fail":false,"target_va":5387766848,"va":5387707604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122cd30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t54739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8d3d50000","size":5,"soft_fail":false,"target_va":5387767088,"va":5387712344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122ce20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t70278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e886120100","size":5,"soft_fail":false,"target_va":5387767328,"va":5387697045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t165719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e857870200","size":5,"soft_fail":false,"target_va":5387874224,"va":5387708500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141762b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5536833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8417c5400","size":5,"soft_fail":false,"target_va":5393230704,"va":5387693866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fd16d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14375352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8b859db00","size":5,"soft_fail":false,"target_va":5402072784,"va":5387697427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fd1c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14380788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f46edb00","size":5,"soft_fail":false,"target_va":5402074160,"va":5387693367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdc7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14408704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e800dcdb00","size":5,"soft_fail":false,"target_va":5402118064,"va":5387709355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24161419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e88bac7001","size":5,"soft_fail":false,"target_va":5411847152,"va":5387685728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24155807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89f967001","size":5,"soft_fail":false,"target_va":5411847168,"va":5387691356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24160377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e879a87001","size":5,"soft_fail":false,"target_va":5411847184,"va":5387686802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24160437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8b5a87001","size":5,"soft_fail":false,"target_va":5411847776,"va":5387687334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24157244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e83c9c7001","size":5,"soft_fail":false,"target_va":5411847952,"va":5387690703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23827896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8b8956b01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387713091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t-12357838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8326f43ff","size":5,"soft_fail":false,"target_va":5375328336,"va":5387686169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23827468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e80c946b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5387713490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t23853256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8c8f86b01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387688515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23770137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e819b46a01","size":5,"soft_fail":false,"target_va":5411482433,"va":5387712291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t23834189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e84dae6b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387707870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t23829488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f09b6b01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387712803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23833646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e82eac6b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387708253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23829441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8c19b6b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5387713434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t78576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f0320100","size":5,"soft_fail":false,"target_va":5387775696,"va":5387697115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8969324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8942377ff","size":5,"soft_fail":false,"target_va":5378733712,"va":5387703031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8968036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89c2877ff","size":5,"soft_fail":false,"target_va":5378734080,"va":5387702111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8878188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8948778ff","size":5,"soft_fail":false,"target_va":5378821744,"va":5387699927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8878610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8ee8578ff","size":5,"soft_fail":false,"target_va":5378821312,"va":5387699917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9657128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8d8a46cff","size":5,"soft_fail":false,"target_va":5378028512,"va":5387685635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9663524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8dc8b6cff","size":5,"soft_fail":false,"target_va":5378028128,"va":5387691647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9657232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e870a46cff","size":5,"soft_fail":false,"target_va":5378028416,"va":5387685643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9673042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8ae666cff","size":5,"soft_fail":false,"target_va":5378028432,"va":5387701469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10402148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89c4661ff","size":5,"soft_fail":false,"target_va":5377307232,"va":5387709375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8398861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f3d77fff","size":5,"soft_fail":false,"target_va":5379310272,"va":5387709128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7307789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f37d90ff","size":5,"soft_fail":false,"target_va":5380401520,"va":5387709304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7269037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8531591ff","size":5,"soft_fail":false,"target_va":5380417056,"va":5387686088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7268885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8eb1591ff","size":5,"soft_fail":false,"target_va":5380417152,"va":5387686032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7268561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e82f1791ff","size":5,"soft_fail":false,"target_va":5380417024,"va":5387685580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7284533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8cbd890ff","size":5,"soft_fail":false,"target_va":5380417376,"va":5387701904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7225698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e89ebe91ff","size":5,"soft_fail":false,"target_va":5380460416,"va":5387686109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-110532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e83c50feff","size":5,"soft_fail":false,"target_va":5387581248,"va":5387691775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-113788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e88443feff","size":5,"soft_fail":false,"target_va":5387580928,"va":5387694711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5683944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e81845a9ff","size":5,"soft_fail":false,"target_va":5382009728,"va":5387693667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23780058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8dada6a01","size":5,"soft_fail":false,"target_va":5411492368,"va":5387712305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23781435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e83be06a01","size":5,"soft_fail":false,"target_va":5411494800,"va":5387713360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5862217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8b78ca6ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387685812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5888015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f127a6ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387712026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5030904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8083cb3ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5387688755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4935005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8a3b2b4ff","size":5,"soft_fail":false,"target_va":5382753792,"va":5387688792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t31589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8657b0000","size":5,"soft_fail":false,"target_va":5387722608,"va":5387691014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::as_f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5071723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e86b634d00","size":5,"soft_fail":false,"target_va":5392773648,"va":5387701920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t156117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8d5610200","size":5,"soft_fail":false,"target_va":5387845824,"va":5387689702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t156224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e840620200","size":5,"soft_fail":false,"target_va":5387845952,"va":5387689723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t155629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8ed5f0200","size":5,"soft_fail":false,"target_va":5387845616,"va":5387689982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t157255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e847660200","size":5,"soft_fail":false,"target_va":5387846384,"va":5387689124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t151825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e811510200","size":5,"soft_fail":false,"target_va":5387846000,"va":5387694170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t156525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e86d630200","size":5,"soft_fail":false,"target_va":5387846368,"va":5387689838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5793529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e80799a7ff","size":5,"soft_fail":false,"target_va":5381912800,"va":5387706324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5782588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8c4c3a7ff","size":5,"soft_fail":false,"target_va":5381911968,"va":5387694551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5793317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8db99a7ff","size":5,"soft_fail":false,"target_va":5381912944,"va":5387706256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5785326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e812b9a7ff","size":5,"soft_fail":false,"target_va":5381923136,"va":5387708457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5786537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e857b4a7ff","size":5,"soft_fail":false,"target_va":5381923344,"va":5387709876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5771803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8e5eda7ff","size":5,"soft_fail":false,"target_va":5381926080,"va":5387697878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5788184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8e8ada7ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387708435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5777745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8afd6a7ff","size":5,"soft_fail":false,"target_va":5381920464,"va":5387698204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5784184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e888bda7ff","size":5,"soft_fail":false,"target_va":5381920048,"va":5387704227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5786638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f2b3a7ff","size":5,"soft_fail":false,"target_va":5381907008,"va":5387693641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5810431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e80157a7ff","size":5,"soft_fail":false,"target_va":5381884480,"va":5387694906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14417350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8c6fddb00","size":5,"soft_fail":false,"target_va":5402116816,"va":5387699461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14416117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e8f5f8db00","size":5,"soft_fail":false,"target_va":5402116944,"va":5387700822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5804165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e87b6fa7ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5387694352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5810149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387685424,"raw_hex":"e81b58a7ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387696048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387685424,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5387685748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":19524,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1435,"record_offset":11100,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":52640,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10753,"source_line":3172,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11992,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7068,"source_line":1977,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":10860,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":10920,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11288,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7686,"source_line":2096,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":12492,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":21560,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":15360,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":13392,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":19784,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":26960,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":37968,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1435,"record_offset":19056,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1435,"record_offset":57976,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4555,"source_line":56,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1435,"record_offset":57712,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":13144,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1435,"record_offset":30672,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":22548,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":38680,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":18968,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":18940,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1435,"record_offset":13172,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1435,"record_offset":33636,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1435,"record_offset":13500,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":10796,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$121::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1435,"record_offset":42816,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":14177,"source_line":57,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1435,"record_offset":57652,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":24913,"source_line":35,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":20696,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":26560,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":27372,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$62::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":42516,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":19712,"source_line":343,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":24916,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":10888,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":50676,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":10260,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":16048,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":39968,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":22484,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":12324,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":18796,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":35412,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":15576,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":39796,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":38024,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7630,"source_line":3024,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":18644,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":18880,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1435,"record_offset":10016,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":10296,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1435,"record_offset":10768,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":70929,"source_line":429,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1435,"record_offset":29176,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13688,"source_line":894,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":28384,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":46728,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":43180,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":43208,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":43300,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":29876,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28216,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28940,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28752,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":29784,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":46544,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28848,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":46636,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1435,"record_offset":38200,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1435,"record_offset":22580,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":35752,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":32512,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":53016,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":17280,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":7395,"source_line":249,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":56004,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":56404,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":53664,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1435,"record_offset":13116,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":30700,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":70934,"source_line":218,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":13584,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":70930,"source_line":344,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":20088,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":70933,"source_line":1872,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1435,"record_offset":30060,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":12836,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":54716,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":10740,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13280,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":28000,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":45824,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":70935,"source_line":65,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":27692,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":55052,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1435,"record_offset":57180,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1435,"record_offset":57148,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1435,"record_offset":26676,"parent_record_offset":9516,"ancestor_record_offset":9516,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":9516,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":17096,"parent_record_offset":17072,"ancestor_record_offset":9516,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":9516,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":16260,"parent_record_offset":16236,"ancestor_record_offset":9516,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":17048,"parent_record_offset":16872,"ancestor_record_offset":9516,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":12704,"parent_record_offset":12656,"ancestor_record_offset":9516,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":16236,"parent_record_offset":16144,"ancestor_record_offset":9516,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1435,"record_offset":64388,"parent_record_offset":64364,"ancestor_record_offset":9516,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":9516,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16808,"parent_record_offset":16772,"ancestor_record_offset":9516,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":12572,"parent_record_offset":12532,"ancestor_record_offset":9516,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16172,"parent_record_offset":16144,"ancestor_record_offset":9516,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16144,"parent_record_offset":16120,"ancestor_record_offset":9516,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":9516,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":9516,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":12656,"parent_record_offset":12612,"ancestor_record_offset":9516,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16840,"parent_record_offset":16808,"ancestor_record_offset":9516,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":12612,"parent_record_offset":12572,"ancestor_record_offset":9516,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":9516,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":16908,"parent_record_offset":16872,"ancestor_record_offset":9516,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1435,"record_offset":21684,"parent_record_offset":21648,"ancestor_record_offset":9516,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1435,"record_offset":21648,"parent_record_offset":21596,"ancestor_record_offset":9516,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1435,"record_offset":21596,"parent_record_offset":21560,"ancestor_record_offset":9516,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1435,"record_offset":11136,"parent_record_offset":11100,"ancestor_record_offset":9516,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":10956,"parent_record_offset":10920,"ancestor_record_offset":9516,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":22232,"parent_record_offset":22208,"ancestor_record_offset":9516,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":22208,"parent_record_offset":21596,"ancestor_record_offset":9516,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":52668,"parent_record_offset":52640,"ancestor_record_offset":9516,"inlinee":10754,"source_line":2264,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":9516,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11364,"parent_record_offset":11316,"ancestor_record_offset":9516,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11316,"parent_record_offset":11288,"ancestor_record_offset":9516,"inlinee":7687,"source_line":2122,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11756,"parent_record_offset":11732,"ancestor_record_offset":9516,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":16736,"parent_record_offset":16680,"ancestor_record_offset":9516,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":16772,"parent_record_offset":16736,"ancestor_record_offset":9516,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":9516,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":35252,"parent_record_offset":33676,"ancestor_record_offset":9516,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11428,"parent_record_offset":11392,"ancestor_record_offset":9516,"inlinee":7691,"source_line":2100,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":9516,"inlinee":7696,"source_line":2156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":11392,"parent_record_offset":11316,"ancestor_record_offset":9516,"inlinee":7689,"source_line":2176,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":61760,"parent_record_offset":61736,"ancestor_record_offset":9516,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":52760,"parent_record_offset":52668,"ancestor_record_offset":9516,"inlinee":10756,"source_line":3173,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":26984,"parent_record_offset":26960,"ancestor_record_offset":9516,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1435,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":9516,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1435,"record_offset":19284,"parent_record_offset":19260,"ancestor_record_offset":9516,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1435,"record_offset":33832,"parent_record_offset":33804,"ancestor_record_offset":9516,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1435,"record_offset":17152,"parent_record_offset":16680,"ancestor_record_offset":9516,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1435,"record_offset":12532,"parent_record_offset":12492,"ancestor_record_offset":9516,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1435,"record_offset":16636,"parent_record_offset":16592,"ancestor_record_offset":9516,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1435,"record_offset":16680,"parent_record_offset":16636,"ancestor_record_offset":9516,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1435,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":9516,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1435,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":9516,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1435,"record_offset":57840,"parent_record_offset":57740,"ancestor_record_offset":9516,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1435,"record_offset":17840,"parent_record_offset":17808,"ancestor_record_offset":9516,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1435,"record_offset":17900,"parent_record_offset":17868,"ancestor_record_offset":9516,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$18::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1435,"record_offset":42548,"parent_record_offset":42516,"ancestor_record_offset":9516,"inlinee":11764,"source_line":336,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":20744,"parent_record_offset":20720,"ancestor_record_offset":9516,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":31180,"parent_record_offset":31024,"ancestor_record_offset":9516,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":52784,"parent_record_offset":52760,"ancestor_record_offset":9516,"inlinee":10758,"source_line":1976,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":36524,"parent_record_offset":36500,"ancestor_record_offset":9516,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":20768,"parent_record_offset":20744,"ancestor_record_offset":9516,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":31204,"parent_record_offset":31180,"ancestor_record_offset":9516,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":9516,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":10088,"parent_record_offset":10048,"ancestor_record_offset":9516,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1435,"record_offset":30580,"parent_record_offset":30088,"ancestor_record_offset":9516,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":53224,"parent_record_offset":53192,"ancestor_record_offset":9516,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":9516,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":36152,"parent_record_offset":36128,"ancestor_record_offset":9516,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":29488,"parent_record_offset":29204,"ancestor_record_offset":9516,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":29404,"parent_record_offset":29204,"ancestor_record_offset":9516,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":30352,"parent_record_offset":30328,"ancestor_record_offset":9516,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":29268,"parent_record_offset":29204,"ancestor_record_offset":9516,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":14644,"parent_record_offset":14616,"ancestor_record_offset":9516,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":23224,"parent_record_offset":23132,"ancestor_record_offset":9516,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":20164,"parent_record_offset":20124,"ancestor_record_offset":9516,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":23256,"parent_record_offset":23132,"ancestor_record_offset":9516,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1435,"record_offset":28180,"parent_record_offset":28000,"ancestor_record_offset":9516,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1435,"record_offset":24252,"parent_record_offset":24144,"ancestor_record_offset":9516,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1435,"record_offset":11644,"parent_record_offset":11496,"ancestor_record_offset":9516,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1435,"record_offset":11216,"parent_record_offset":11172,"ancestor_record_offset":9516,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1435,"record_offset":13972,"parent_record_offset":13940,"ancestor_record_offset":9516,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1435,"record_offset":14076,"parent_record_offset":14048,"ancestor_record_offset":9516,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1435,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":9516,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1435,"record_offset":34456,"parent_record_offset":34432,"ancestor_record_offset":9516,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1435,"record_offset":33732,"parent_record_offset":33676,"ancestor_record_offset":9516,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1435,"record_offset":33676,"parent_record_offset":33636,"ancestor_record_offset":9516,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1435,"record_offset":34380,"parent_record_offset":34356,"ancestor_record_offset":9516,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1435,"record_offset":34356,"parent_record_offset":33676,"ancestor_record_offset":9516,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1435,"record_offset":13828,"parent_record_offset":13788,"ancestor_record_offset":9516,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1435,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":9516,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1435,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":9516,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1435,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":9516,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1435,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":9516,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1435,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":9516,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":12952,"parent_record_offset":12836,"ancestor_record_offset":9516,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":9516,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":9516,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":14160,"parent_record_offset":14132,"ancestor_record_offset":9516,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1435,"record_offset":11684,"parent_record_offset":11316,"ancestor_record_offset":9516,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1435,"record_offset":57868,"parent_record_offset":57840,"ancestor_record_offset":9516,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1435,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":9516,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1435,"record_offset":39868,"parent_record_offset":39828,"ancestor_record_offset":9516,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":9516,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":10664,"parent_record_offset":10636,"ancestor_record_offset":9516,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":9516,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":57804,"parent_record_offset":57776,"ancestor_record_offset":9516,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":16204,"parent_record_offset":16172,"ancestor_record_offset":9516,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":17868,"parent_record_offset":17808,"ancestor_record_offset":9516,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":36300,"parent_record_offset":36272,"ancestor_record_offset":9516,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":36272,"parent_record_offset":36244,"ancestor_record_offset":9516,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":15212,"parent_record_offset":15128,"ancestor_record_offset":9516,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":15184,"parent_record_offset":15128,"ancestor_record_offset":9516,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":46576,"parent_record_offset":46544,"ancestor_record_offset":9516,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":46668,"parent_record_offset":46636,"ancestor_record_offset":9516,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":14588,"parent_record_offset":13680,"ancestor_record_offset":9516,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1435,"record_offset":17444,"parent_record_offset":17408,"ancestor_record_offset":9516,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1435,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":9516,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1435,"record_offset":57776,"parent_record_offset":57740,"ancestor_record_offset":9516,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1435,"record_offset":57740,"parent_record_offset":57712,"ancestor_record_offset":9516,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":20720,"parent_record_offset":20696,"ancestor_record_offset":9516,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":23396,"parent_record_offset":23132,"ancestor_record_offset":9516,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":33772,"parent_record_offset":33732,"ancestor_record_offset":9516,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":10568,"parent_record_offset":10296,"ancestor_record_offset":9516,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":23192,"parent_record_offset":23132,"ancestor_record_offset":9516,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":14420,"parent_record_offset":14188,"ancestor_record_offset":9516,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":9516,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":10376,"parent_record_offset":10296,"ancestor_record_offset":9516,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":10604,"parent_record_offset":10296,"ancestor_record_offset":9516,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1435,"record_offset":24188,"parent_record_offset":24144,"ancestor_record_offset":9516,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1435,"record_offset":22144,"parent_record_offset":21684,"ancestor_record_offset":9516,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1435,"record_offset":57896,"parent_record_offset":57868,"ancestor_record_offset":9516,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=87
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":11708,"parent_record_offset":11684,"ancestor_record_offset":9516,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1435,"record_offset":12144,"parent_record_offset":11992,"ancestor_record_offset":9516,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1435,"record_offset":17648,"parent_record_offset":17532,"ancestor_record_offset":9516,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1435,"record_offset":36416,"parent_record_offset":36392,"ancestor_record_offset":9516,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1435,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":9516,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1435,"record_offset":19124,"parent_record_offset":19056,"ancestor_record_offset":9516,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1435,"record_offset":13240,"parent_record_offset":13200,"ancestor_record_offset":9516,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":17724,"parent_record_offset":17696,"ancestor_record_offset":9516,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":9516,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":17672,"parent_record_offset":17648,"ancestor_record_offset":9516,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":11068,"parent_record_offset":10920,"ancestor_record_offset":9516,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1435,"record_offset":13040,"parent_record_offset":13008,"ancestor_record_offset":9516,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1435,"record_offset":10172,"parent_record_offset":10144,"ancestor_record_offset":9516,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1435,"record_offset":10200,"parent_record_offset":10172,"ancestor_record_offset":9516,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::as_mut_ptr_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":39908,"parent_record_offset":39796,"ancestor_record_offset":9516,"inlinee":11315,"source_line":850,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":58164,"parent_record_offset":58136,"ancestor_record_offset":9516,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":12868,"parent_record_offset":12836,"ancestor_record_offset":9516,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":10540,"parent_record_offset":10296,"ancestor_record_offset":9516,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":39828,"parent_record_offset":39796,"ancestor_record_offset":9516,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":58224,"parent_record_offset":58196,"ancestor_record_offset":9516,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":15392,"parent_record_offset":15360,"ancestor_record_offset":9516,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":13424,"parent_record_offset":13392,"ancestor_record_offset":9516,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":22512,"parent_record_offset":22484,"ancestor_record_offset":9516,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":58196,"parent_record_offset":58164,"ancestor_record_offset":9516,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":15424,"parent_record_offset":15392,"ancestor_record_offset":9516,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":13456,"parent_record_offset":13424,"ancestor_record_offset":9516,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1435,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":9516,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":9516,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1435,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":9516,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":9516,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":14000,"parent_record_offset":13940,"ancestor_record_offset":9516,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":18708,"parent_record_offset":18644,"ancestor_record_offset":9516,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":22388,"parent_record_offset":22356,"ancestor_record_offset":9516,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1435,"record_offset":11172,"parent_record_offset":11136,"ancestor_record_offset":9516,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":1435,"record_offset":38060,"parent_record_offset":38024,"ancestor_record_offset":9516,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1435,"record_offset":10116,"parent_record_offset":10088,"ancestor_record_offset":9516,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":64032,"parent_record_offset":63964,"ancestor_record_offset":9516,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":10452,"parent_record_offset":10412,"ancestor_record_offset":9516,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":64060,"parent_record_offset":64032,"ancestor_record_offset":9516,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":9516,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1435,"record_offset":10048,"parent_record_offset":10016,"ancestor_record_offset":9516,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":10636,"parent_record_offset":10604,"ancestor_record_offset":9516,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":63932,"parent_record_offset":63904,"ancestor_record_offset":9516,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1435,"record_offset":30088,"parent_record_offset":30060,"ancestor_record_offset":9516,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1435,"record_offset":30144,"parent_record_offset":30088,"ancestor_record_offset":9516,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1435,"record_offset":29204,"parent_record_offset":29176,"ancestor_record_offset":9516,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":30232,"parent_record_offset":30144,"ancestor_record_offset":9516,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":9516,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":30328,"parent_record_offset":30088,"ancestor_record_offset":9516,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":28464,"parent_record_offset":28408,"ancestor_record_offset":9516,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":46756,"parent_record_offset":46728,"ancestor_record_offset":9516,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":46844,"parent_record_offset":46728,"ancestor_record_offset":9516,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":28408,"parent_record_offset":28384,"ancestor_record_offset":9516,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":50732,"parent_record_offset":50708,"ancestor_record_offset":9516,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":29040,"parent_record_offset":29004,"ancestor_record_offset":9516,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":29004,"parent_record_offset":28968,"ancestor_record_offset":9516,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":9516,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":9516,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":28324,"parent_record_offset":28216,"ancestor_record_offset":9516,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":9516,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":36212,"parent_record_offset":36184,"ancestor_record_offset":9516,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":36244,"parent_record_offset":36212,"ancestor_record_offset":9516,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":54004,"parent_record_offset":53804,"ancestor_record_offset":9516,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":36184,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":36044,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":53192,"parent_record_offset":53164,"ancestor_record_offset":9516,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":36596,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":53256,"parent_record_offset":53192,"ancestor_record_offset":9516,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":54108,"parent_record_offset":53752,"ancestor_record_offset":9516,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":36128,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1435,"record_offset":36016,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1435,"record_offset":36476,"parent_record_offset":36452,"ancestor_record_offset":9516,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":35780,"parent_record_offset":35752,"ancestor_record_offset":9516,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":9516,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":56060,"parent_record_offset":56032,"ancestor_record_offset":9516,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":53044,"parent_record_offset":53016,"ancestor_record_offset":9516,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":36452,"parent_record_offset":36344,"ancestor_record_offset":9516,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":56432,"parent_record_offset":56404,"ancestor_record_offset":9516,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":53692,"parent_record_offset":53664,"ancestor_record_offset":9516,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":35844,"parent_record_offset":35808,"ancestor_record_offset":9516,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":36392,"parent_record_offset":36368,"ancestor_record_offset":9516,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":36368,"parent_record_offset":36344,"ancestor_record_offset":9516,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":54216,"parent_record_offset":53752,"ancestor_record_offset":9516,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":36688,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53164,"parent_record_offset":53100,"ancestor_record_offset":9516,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53752,"parent_record_offset":53720,"ancestor_record_offset":9516,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":35900,"parent_record_offset":35872,"ancestor_record_offset":9516,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":35872,"parent_record_offset":35808,"ancestor_record_offset":9516,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53072,"parent_record_offset":53044,"ancestor_record_offset":9516,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53132,"parent_record_offset":53100,"ancestor_record_offset":9516,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":35932,"parent_record_offset":35900,"ancestor_record_offset":9516,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53100,"parent_record_offset":53072,"ancestor_record_offset":9516,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":35988,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":61692,"parent_record_offset":61668,"ancestor_record_offset":9516,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":53720,"parent_record_offset":53692,"ancestor_record_offset":9516,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":36344,"parent_record_offset":35932,"ancestor_record_offset":9516,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1435,"record_offset":14048,"parent_record_offset":13788,"ancestor_record_offset":9516,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1435,"record_offset":13788,"parent_record_offset":13680,"ancestor_record_offset":9516,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1435,"record_offset":15128,"parent_record_offset":13584,"ancestor_record_offset":9516,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1435,"record_offset":10412,"parent_record_offset":10296,"ancestor_record_offset":9516,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1435,"record_offset":63964,"parent_record_offset":63932,"ancestor_record_offset":9516,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1435,"record_offset":10508,"parent_record_offset":10412,"ancestor_record_offset":9516,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1435,"record_offset":64000,"parent_record_offset":63964,"ancestor_record_offset":9516,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":56032,"parent_record_offset":56004,"ancestor_record_offset":9516,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":9516,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1435,"record_offset":28496,"parent_record_offset":28464,"ancestor_record_offset":9516,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1435,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":9516,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1435,"record_offset":17608,"parent_record_offset":17584,"ancestor_record_offset":9516,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1435,"record_offset":17472,"parent_record_offset":17444,"ancestor_record_offset":9516,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1435,"record_offset":17808,"parent_record_offset":17408,"ancestor_record_offset":9516,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1435,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":9516,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1435,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":9516,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1435,"record_offset":17532,"parent_record_offset":17500,"ancestor_record_offset":9516,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1435,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":9516,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1435,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":9516,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$10
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":23132,"parent_record_offset":23104,"ancestor_record_offset":9516,"inlinee":70758,"source_line":570,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$16
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":50708,"parent_record_offset":50676,"ancestor_record_offset":9516,"inlinee":70936,"source_line":1447,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$3
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":11456,"parent_record_offset":11428,"ancestor_record_offset":9516,"inlinee":70751,"source_line":266,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":9516,"inlinee":70931,"source_line":340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1435,"record_offset":13856,"parent_record_offset":13828,"ancestor_record_offset":9516,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1435,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":9516,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":13008,"parent_record_offset":12836,"ancestor_record_offset":9516,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":13760,"parent_record_offset":13680,"ancestor_record_offset":9516,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":15100,"parent_record_offset":13584,"ancestor_record_offset":9516,"inlinee":24639,"source_line":1347,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":55160,"parent_record_offset":55052,"ancestor_record_offset":9516,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":55016,"parent_record_offset":54716,"ancestor_record_offset":9516,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13644,"parent_record_offset":13584,"ancestor_record_offset":9516,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13680,"parent_record_offset":13644,"ancestor_record_offset":9516,"inlinee":24631,"source_line":243,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":54764,"parent_record_offset":54716,"ancestor_record_offset":9516,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13316,"parent_record_offset":13280,"ancestor_record_offset":9516,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":54912,"parent_record_offset":54880,"ancestor_record_offset":9516,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":54880,"parent_record_offset":54716,"ancestor_record_offset":9516,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":9516,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":9516,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":52812,"parent_record_offset":52784,"ancestor_record_offset":9516,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":20124,"parent_record_offset":20088,"ancestor_record_offset":9516,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":20192,"parent_record_offset":20124,"ancestor_record_offset":9516,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1435,"record_offset":14616,"parent_record_offset":13680,"ancestor_record_offset":9516,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/01435.jsonl.gz"}
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
// 首条位置: {"module":1435,"record_offset":71216,"rva":19011744,"va":5387720864}
// 编译实例 RVAs: 0x12218a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12640556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e8d41e3fff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387722055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13362390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e82a1b34ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5387721425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8410732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e894a97fff","size":5,"soft_fail":false,"target_va":5379310272,"va":5387720999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7319753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e8374f90ff","size":5,"soft_fail":false,"target_va":5380401520,"va":5387721268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e849030000","size":5,"soft_fail":false,"target_va":5387722144,"va":5387721298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t124347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e8bbe50100","size":5,"soft_fail":false,"target_va":5387845824,"va":5387721472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t124032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e880e40100","size":5,"soft_fail":false,"target_va":5387845616,"va":5387721579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t124830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e89ee70100","size":5,"soft_fail":false,"target_va":5387846384,"va":5387721549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t124618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e8cae60100","size":5,"soft_fail":false,"target_va":5387846000,"va":5387721377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t124879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387720864,"raw_hex":"e8cfe70100","size":5,"soft_fail":false,"target_va":5387846368,"va":5387721484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":71380,"parent_record_offset":71216,"ancestor_record_offset":71216,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":71768,"parent_record_offset":71216,"ancestor_record_offset":71216,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":72880,"parent_record_offset":71216,"ancestor_record_offset":71216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":71408,"parent_record_offset":71216,"ancestor_record_offset":71216,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":71952,"parent_record_offset":71216,"ancestor_record_offset":71216,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":73068,"parent_record_offset":73044,"ancestor_record_offset":71216,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":73044,"parent_record_offset":72952,"ancestor_record_offset":71216,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":71832,"parent_record_offset":71800,"ancestor_record_offset":71216,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":72980,"parent_record_offset":72952,"ancestor_record_offset":71216,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":72952,"parent_record_offset":72928,"ancestor_record_offset":71216,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":71880,"parent_record_offset":71856,"ancestor_record_offset":71216,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":71856,"parent_record_offset":71832,"ancestor_record_offset":71216,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":72928,"parent_record_offset":72904,"ancestor_record_offset":71216,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":71800,"parent_record_offset":71768,"ancestor_record_offset":71216,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":71620,"parent_record_offset":71592,"ancestor_record_offset":71216,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":71680,"parent_record_offset":71648,"ancestor_record_offset":71216,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":73012,"parent_record_offset":72980,"ancestor_record_offset":71216,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":72232,"parent_record_offset":72200,"ancestor_record_offset":71216,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":72200,"parent_record_offset":72168,"ancestor_record_offset":71216,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1435,"record_offset":71716,"parent_record_offset":71564,"ancestor_record_offset":71216,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":72904,"parent_record_offset":72880,"ancestor_record_offset":71216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1435,"record_offset":72492,"parent_record_offset":72468,"ancestor_record_offset":71216,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":71920,"parent_record_offset":71768,"ancestor_record_offset":71216,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":72128,"parent_record_offset":72088,"ancestor_record_offset":71216,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":72168,"parent_record_offset":72128,"ancestor_record_offset":71216,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":72276,"parent_record_offset":72088,"ancestor_record_offset":71216,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":72088,"parent_record_offset":72036,"ancestor_record_offset":71216,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":71592,"parent_record_offset":71564,"ancestor_record_offset":71216,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":71648,"parent_record_offset":71592,"ancestor_record_offset":71216,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":72368,"parent_record_offset":72036,"ancestor_record_offset":71216,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":71456,"parent_record_offset":71432,"ancestor_record_offset":71216,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":71976,"parent_record_offset":71952,"ancestor_record_offset":71216,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":72468,"parent_record_offset":72036,"ancestor_record_offset":71216,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":71564,"parent_record_offset":71504,"ancestor_record_offset":71216,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":72036,"parent_record_offset":72000,"ancestor_record_offset":71216,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":71480,"parent_record_offset":71456,"ancestor_record_offset":71216,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":71536,"parent_record_offset":71504,"ancestor_record_offset":71216,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":71504,"parent_record_offset":71480,"ancestor_record_offset":71216,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":72000,"parent_record_offset":71976,"ancestor_record_offset":71216,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":71432,"parent_record_offset":71408,"ancestor_record_offset":71216,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01435.jsonl.gz"}
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
// 首条位置: {"module":1435,"record_offset":272,"rva":18971616,"va":5387680736}
// 编译实例 RVAs: 0x1217be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18415614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e80200e7fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387683273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12871892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e82c973bff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387685311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12820673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e83f5f3cff","size":5,"soft_fail":false,"target_va":5374864432,"va":5387685100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12600467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e86dbb3fff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387681966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9941094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e89a4f68ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387683265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5747175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8194ea8ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387681698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-128725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e82b09feff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387682528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141202890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e825a7feff","size":5,"soft_fail":false,"target_va":5387593872,"va":5387682150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-57418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8b61fffff","size":5,"soft_fail":false,"target_va":5387623488,"va":5387680901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-49664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8003effff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387683371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24166176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e820bf7001","size":5,"soft_fail":false,"target_va":5411847152,"va":5387680971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24166083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8c3be7001","size":5,"soft_fail":false,"target_va":5411848096,"va":5387682008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23857049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e899076c01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387684722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23797857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e861206b01","size":5,"soft_fail":false,"target_va":5411482433,"va":5387684571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23857673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8090a6c01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387684386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23857680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8100a6c01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387684611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23857542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e886096c01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387684357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9653593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8a7b26cff","size":5,"soft_fail":false,"target_va":5378027696,"va":5387681284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9653445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e83bb36cff","size":5,"soft_fail":false,"target_va":5378028128,"va":5387681568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9652668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e844b66cff","size":5,"soft_fail":false,"target_va":5378028576,"va":5387681239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9651017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8b7bc6cff","size":5,"soft_fail":false,"target_va":5378030912,"va":5387681924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7928679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8990487ff","size":5,"soft_fail":false,"target_va":5379752512,"va":5387681186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7929044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e82c0387ff","size":5,"soft_fail":false,"target_va":5379752128,"va":5387681167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7264390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e87a2791ff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387681361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7263846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e89a2991ff","size":5,"soft_fail":false,"target_va":5380417024,"va":5387680865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-100126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8e278feff","size":5,"soft_fail":false,"target_va":5387581248,"va":5387681369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23807778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e822476b01","size":5,"soft_fail":false,"target_va":5411492368,"va":5387684585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23810090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e82a506b01","size":5,"soft_fail":false,"target_va":5411494800,"va":5387684705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5857447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8599fa6ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387681042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5860132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8dc94a6ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387684143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5764228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e87c0ba8ff","size":5,"soft_fail":false,"target_va":5381917360,"va":5387681583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5764397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8d30aa8ff","size":5,"soft_fail":false,"target_va":5381916928,"va":5387681320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5764455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8990aa8ff","size":5,"soft_fail":false,"target_va":5381916816,"va":5387681266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5761322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8d616a8ff","size":5,"soft_fail":false,"target_va":5381920064,"va":5387681381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5798467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8bd85a7ff","size":5,"soft_fail":false,"target_va":5381884480,"va":5387682942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5797658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680736,"raw_hex":"e8e688a7ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387683557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387680736,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5387680991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1435,"record_offset":7112,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":2880,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1435,"record_offset":2992,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1435,"record_offset":1240,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":2908,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":772,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":2352,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1435,"record_offset":532,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1435,"record_offset":5060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":29550,"source_line":1151,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":2140,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":7252,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":7168,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":7440,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":1736,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":1792,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":5944,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":1824,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":2232,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":6104,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":7528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":5416,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":7196,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":6408,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1435,"record_offset":2852,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":7140,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70928,"source_line":70,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":1324,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70926,"source_line":94,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1435,"record_offset":6500,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":3884,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":2324,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":2540,"parent_record_offset":2448,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":3252,"parent_record_offset":3216,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":600,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1435,"record_offset":7020,"parent_record_offset":6528,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1440,"parent_record_offset":1352,"ancestor_record_offset":272,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":272,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":272,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1408,"parent_record_offset":1352,"ancestor_record_offset":272,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":3500,"parent_record_offset":3348,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1435,"record_offset":3528,"parent_record_offset":3348,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1435,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1435,"record_offset":3032,"parent_record_offset":2992,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1435,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":272,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":272,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":2508,"parent_record_offset":2476,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":4280,"parent_record_offset":4248,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":6440,"parent_record_offset":6408,"ancestor_record_offset":272,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1435,"record_offset":3176,"parent_record_offset":3136,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":3388,"parent_record_offset":3348,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":1072,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":3136,"parent_record_offset":2992,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":884,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":1108,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1435,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":272,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1435,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":272,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":1044,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":272,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1435,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":272,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":9212,"parent_record_offset":9148,"ancestor_record_offset":272,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":956,"parent_record_offset":920,"ancestor_record_offset":272,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":272,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1435,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1435,"record_offset":560,"parent_record_offset":532,"ancestor_record_offset":272,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":272,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1435,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":272,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1435,"record_offset":6528,"parent_record_offset":6500,"ancestor_record_offset":272,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1435,"record_offset":6192,"parent_record_offset":6160,"ancestor_record_offset":272,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1435,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":272,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1435,"record_offset":6584,"parent_record_offset":6528,"ancestor_record_offset":272,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":6672,"parent_record_offset":6584,"ancestor_record_offset":272,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":272,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1435,"record_offset":6768,"parent_record_offset":6528,"ancestor_record_offset":272,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1435,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":272,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1435,"record_offset":5096,"parent_record_offset":5060,"ancestor_record_offset":272,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1435,"record_offset":5328,"parent_record_offset":5060,"ancestor_record_offset":272,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":7304,"parent_record_offset":7276,"ancestor_record_offset":272,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":272,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":272,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":1472,"parent_record_offset":1324,"ancestor_record_offset":272,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":6160,"parent_record_offset":6128,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":272,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1435,"record_offset":5384,"parent_record_offset":5060,"ancestor_record_offset":272,"inlinee":30322,"source_line":199,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1435,"record_offset":920,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1435,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":272,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1435,"record_offset":1012,"parent_record_offset":920,"ancestor_record_offset":272,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1435,"record_offset":9184,"parent_record_offset":9148,"ancestor_record_offset":272,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1435,"record_offset":2196,"parent_record_offset":2164,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::roof_outline::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":3216,"parent_record_offset":3176,"ancestor_record_offset":272,"inlinee":70927,"source_line":122,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":3916,"parent_record_offset":3884,"ancestor_record_offset":272,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":3952,"parent_record_offset":3916,"ancestor_record_offset":272,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":4076,"parent_record_offset":3916,"ancestor_record_offset":272,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":3348,"parent_record_offset":3216,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1435,"record_offset":4200,"parent_record_offset":3916,"ancestor_record_offset":272,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11510
// 首条位置: {"module":1435,"inlinee":70928}
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
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1150E
// 首条位置: {"module":1435,"inlinee":70926}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1435,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1324,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":1472,"parent_record_offset":1324,"ancestor_record_offset":1324,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1440,"parent_record_offset":1352,"ancestor_record_offset":1324,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":1324,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1435,"record_offset":1408,"parent_record_offset":1352,"ancestor_record_offset":1324,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1324,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1147D
// 首条位置: {"module":1415,"inlinee":70781}
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
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1150F
// 首条位置: {"module":1435,"inlinee":70927}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1435,"record_offset":3252,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":3348,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":3216,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":3216,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":3500,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1435,"record_offset":3528,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":3216,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":3388,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":3216,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1435,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":3216,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11516
// 首条位置: {"module":1435,"inlinee":70934}
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
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11490
// 首条位置: {"module":1415,"inlinee":70800}
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
// 记录形式: inline_instance, inlinee_declaration; instances=45
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11543
// 首条位置: {"module":1444,"inlinee":70979}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1444,"record_offset":592,"parent_record_offset":572,"ancestor_record_offset":572,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1145F
// 首条位置: {"module":1415,"inlinee":70751}
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
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1148C
// 首条位置: {"module":1415,"inlinee":70796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E5C
// 首条位置: {"module":3006,"inlinee":106076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3006,"record_offset":1648,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3006,"record_offset":2088,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3006,"record_offset":2016,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":2336,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3006,"record_offset":1676,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":2500,"parent_record_offset":2408,"ancestor_record_offset":1540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2200,"parent_record_offset":2148,"ancestor_record_offset":1540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":1540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":1540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":1540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":1540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3006,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3006,"record_offset":2148,"parent_record_offset":2088,"ancestor_record_offset":1540,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3006,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1540,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3006,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1540,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3006,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1540,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3006,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":1540,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3006,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1540,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3006,"record_offset":1768,"parent_record_offset":1716,"ancestor_record_offset":1540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":2300,"parent_record_offset":2088,"ancestor_record_offset":1540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3006,"record_offset":1716,"parent_record_offset":1676,"ancestor_record_offset":1540,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3006,"record_offset":1812,"parent_record_offset":1676,"ancestor_record_offset":1540,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":3006,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":1540,"inlinee":106078,"source_line":294,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E5E
// 首条位置: {"module":3006,"inlinee":106078}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11513
// 首条位置: {"module":1435,"inlinee":70931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":14492,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 344
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11512
// 首条位置: {"module":1435,"inlinee":70930}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1435,"record_offset":15128,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":15100,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":24639,"source_line":1347,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13644,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":14644,"parent_record_offset":14616,"ancestor_record_offset":13584,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1435,"record_offset":13972,"parent_record_offset":13940,"ancestor_record_offset":13584,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1435,"record_offset":14076,"parent_record_offset":14048,"ancestor_record_offset":13584,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1435,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":13584,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1435,"record_offset":13828,"parent_record_offset":13788,"ancestor_record_offset":13584,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1435,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":13584,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1435,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":13584,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1435,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":13584,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1435,"record_offset":14160,"parent_record_offset":14132,"ancestor_record_offset":13584,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":15212,"parent_record_offset":15128,"ancestor_record_offset":13584,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":15184,"parent_record_offset":15128,"ancestor_record_offset":13584,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":14588,"parent_record_offset":13680,"ancestor_record_offset":13584,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1435,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":13584,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":14420,"parent_record_offset":14188,"ancestor_record_offset":13584,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":13584,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1435,"record_offset":14368,"parent_record_offset":14328,"ancestor_record_offset":13584,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":13584,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1435,"record_offset":14000,"parent_record_offset":13940,"ancestor_record_offset":13584,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1435,"record_offset":14048,"parent_record_offset":13788,"ancestor_record_offset":13584,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1435,"record_offset":13788,"parent_record_offset":13680,"ancestor_record_offset":13584,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1435,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":13584,"inlinee":70931,"source_line":340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"take-until","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1435,"record_offset":13856,"parent_record_offset":13828,"ancestor_record_offset":13584,"inlinee":24638,"source_line":89,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1435,"record_offset":13760,"parent_record_offset":13680,"ancestor_record_offset":13584,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":13680,"parent_record_offset":13644,"ancestor_record_offset":13584,"inlinee":24631,"source_line":243,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1435,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":13584,"inlinee":24633,"source_line":255,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1435,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":13584,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1435,"record_offset":14616,"parent_record_offset":13680,"ancestor_record_offset":13584,"inlinee":24636,"source_line":121,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11493
// 首条位置: {"module":1415,"inlinee":70803}
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
// 记录形式: inline_instance, inlinee_declaration; instances=45
// 源码行: 543
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11545
// 首条位置: {"module":1444,"inlinee":70981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1444,"record_offset":3200,"parent_record_offset":3180,"ancestor_record_offset":3180,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 570
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11466
// 首条位置: {"module":1415,"inlinee":70758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":23224,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":23256,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":23396,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":23192,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1435,"record_offset":23284,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":23444,"parent_record_offset":23420,"ancestor_record_offset":23132,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1435,"record_offset":23468,"parent_record_offset":23444,"ancestor_record_offset":23132,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":23364,"parent_record_offset":23284,"ancestor_record_offset":23132,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":23308,"parent_record_offset":23284,"ancestor_record_offset":23132,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1435,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":23132,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 881
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E5B
// 首条位置: {"module":3006,"inlinee":106075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":624,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3006,"record_offset":1004,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":1088,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":540,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":540,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":540,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3006,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3006,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":540,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3006,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":540,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3006,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":540,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3006,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":540,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03006.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 959
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11488
// 首条位置: {"module":1415,"inlinee":70792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1415,"record_offset":65828,"parent_record_offset":65800,"ancestor_record_offset":65800,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1415,"record_offset":65916,"parent_record_offset":65892,"ancestor_record_offset":65800,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1415,"record_offset":65940,"parent_record_offset":65916,"ancestor_record_offset":65800,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1415,"record_offset":65980,"parent_record_offset":65828,"ancestor_record_offset":65800,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1415,"record_offset":66008,"parent_record_offset":65828,"ancestor_record_offset":65800,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1415,"record_offset":65892,"parent_record_offset":65868,"ancestor_record_offset":65800,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1415,"record_offset":65868,"parent_record_offset":65828,"ancestor_record_offset":65800,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 965
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11471
// 首条位置: {"module":1415,"inlinee":70769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1415,"record_offset":26228,"parent_record_offset":26200,"ancestor_record_offset":26200,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1415,"record_offset":26316,"parent_record_offset":26292,"ancestor_record_offset":26200,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1415,"record_offset":26340,"parent_record_offset":26316,"ancestor_record_offset":26200,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1415,"record_offset":26380,"parent_record_offset":26228,"ancestor_record_offset":26200,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1415,"record_offset":26408,"parent_record_offset":26228,"ancestor_record_offset":26200,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1415,"record_offset":26292,"parent_record_offset":26268,"ancestor_record_offset":26200,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1415,"record_offset":26268,"parent_record_offset":26228,"ancestor_record_offset":26200,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=106
// 源码行: 968
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114C5
// 首条位置: {"module":1421,"inlinee":70853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":1668,"parent_record_offset":1648,"ancestor_record_offset":1648,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1444,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15532,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 1437
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11544
// 首条位置: {"module":1444,"inlinee":70980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1444,"record_offset":1904,"parent_record_offset":1884,"ancestor_record_offset":1884,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11518
// 首条位置: {"module":1435,"inlinee":70936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":50732,"parent_record_offset":50708,"ancestor_record_offset":50708,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":50800,"parent_record_offset":50760,"ancestor_record_offset":50708,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":50828,"parent_record_offset":50760,"ancestor_record_offset":50708,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1435,"record_offset":50760,"parent_record_offset":50732,"ancestor_record_offset":50708,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1435,"record_offset":50864,"parent_record_offset":50828,"ancestor_record_offset":50708,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::get_window_aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 1822
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1435,"record_offset":73120,"rva":19013024,"va":5387722144}
// 编译实例 RVAs: 0x1221da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12640955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e8451d3fff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387722454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdc7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14395809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e8a1a9db00","size":5,"soft_fail":false,"target_va":5402118064,"va":5387722250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23819251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e8f3736b01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387722520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8900490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e8763078ff","size":5,"soft_fail":false,"target_va":5378821744,"va":5387722229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8901046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e84a2e78ff","size":5,"soft_fail":false,"target_va":5378821312,"va":5387722353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10414980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e87c1461ff","size":5,"soft_fail":false,"target_va":5377307232,"va":5387722207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722144,"raw_hex":"e8fce00100","size":5,"soft_fail":false,"target_va":5387845616,"va":5387722479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":73376,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1435,"record_offset":73432,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":73528,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1435,"record_offset":73404,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1435,"record_offset":73472,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1435,"record_offset":73348,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1435,"record_offset":73320,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1435,"record_offset":73292,"parent_record_offset":73120,"ancestor_record_offset":73120,"inlinee":28102,"source_line":156,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":73712,"parent_record_offset":73688,"ancestor_record_offset":73120,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1435,"record_offset":73688,"parent_record_offset":73600,"ancestor_record_offset":73120,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":73628,"parent_record_offset":73600,"ancestor_record_offset":73120,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":73600,"parent_record_offset":73576,"ancestor_record_offset":73120,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1435,"record_offset":73576,"parent_record_offset":73552,"ancestor_record_offset":73120,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1435,"record_offset":73656,"parent_record_offset":73628,"ancestor_record_offset":73120,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1435,"record_offset":73496,"parent_record_offset":73472,"ancestor_record_offset":73120,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1435,"record_offset":73552,"parent_record_offset":73528,"ancestor_record_offset":73120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::wrap_around
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1872
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11515
// 首条位置: {"module":1435,"inlinee":70933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":20124,"parent_record_offset":20088,"ancestor_record_offset":20088,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1435,"record_offset":20164,"parent_record_offset":20124,"ancestor_record_offset":20088,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1435,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":20088,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1435,"record_offset":20192,"parent_record_offset":20124,"ancestor_record_offset":20088,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 1876
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1435,"record_offset":73996,"rva":19013488,"va":5387722608}
// 编译实例 RVAs: 0x1221f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8412434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8eea27fff","size":5,"soft_fail":false,"target_va":5379310272,"va":5387722701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7321498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8664890ff","size":5,"soft_fail":false,"target_va":5380401520,"va":5387723013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e885fcffff","size":5,"soft_fail":false,"target_va":5387722144,"va":5387723030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8a1e00100","size":5,"soft_fail":false,"target_va":5387845824,"va":5387722778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8cfe00100","size":5,"soft_fail":false,"target_va":5387845952,"va":5387722860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t122766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e88edf0100","size":5,"soft_fail":false,"target_va":5387845616,"va":5387722845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8a8e20100","size":5,"soft_fail":false,"target_va":5387846384,"va":5387722819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387722608,"raw_hex":"e8a4e20100","size":5,"soft_fail":false,"target_va":5387846368,"va":5387722807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":74164,"parent_record_offset":73996,"ancestor_record_offset":73996,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":74540,"parent_record_offset":73996,"ancestor_record_offset":73996,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":74420,"parent_record_offset":74392,"ancestor_record_offset":73996,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1435,"record_offset":74480,"parent_record_offset":74448,"ancestor_record_offset":73996,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":74820,"parent_record_offset":74788,"ancestor_record_offset":73996,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1435,"record_offset":74788,"parent_record_offset":74756,"ancestor_record_offset":73996,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1435,"record_offset":74364,"parent_record_offset":74336,"ancestor_record_offset":73996,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1435,"record_offset":75080,"parent_record_offset":75056,"ancestor_record_offset":73996,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":74716,"parent_record_offset":74676,"ancestor_record_offset":73996,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":74756,"parent_record_offset":74716,"ancestor_record_offset":73996,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":74864,"parent_record_offset":74676,"ancestor_record_offset":73996,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1435,"record_offset":74676,"parent_record_offset":74628,"ancestor_record_offset":73996,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":74392,"parent_record_offset":74336,"ancestor_record_offset":73996,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":74448,"parent_record_offset":74392,"ancestor_record_offset":73996,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1435,"record_offset":74956,"parent_record_offset":74628,"ancestor_record_offset":73996,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":74220,"parent_record_offset":74192,"ancestor_record_offset":73996,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1435,"record_offset":74568,"parent_record_offset":74540,"ancestor_record_offset":73996,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":75056,"parent_record_offset":74628,"ancestor_record_offset":73996,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74336,"parent_record_offset":74276,"ancestor_record_offset":73996,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74628,"parent_record_offset":74596,"ancestor_record_offset":73996,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74248,"parent_record_offset":74220,"ancestor_record_offset":73996,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74308,"parent_record_offset":74276,"ancestor_record_offset":73996,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74276,"parent_record_offset":74248,"ancestor_record_offset":73996,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1435,"record_offset":74596,"parent_record_offset":74568,"ancestor_record_offset":73996,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01435.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1435,"record_offset":74192,"parent_record_offset":74164,"ancestor_record_offset":73996,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01435.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$8", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::half_timber::half_timbering::InitialBeamProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::utils::half_timber::half_timbering::InitialBeamProposal,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::utils::half_timber::half_timbering::InitialBeamProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// 信息 ID: 5495f085bace6bb875625448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::utils::half_timber::add_underwindow_pattern -> ["system_wall_constructor::utils::half_timber::half_timbering::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ad37 -> 0x1fd1c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 信息 ID: a0a5bfa66403b625e76cda15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121aca0 -> 0x1221f70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a406 -> 0x1221f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::get_window_aabb
// 信息 ID: 6610e4ff460089803f9eabb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221a52 -> 0x1221da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x1222116 -> 0x1221da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering
// 信息 ID: 690694aec9004664bc6f48ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758e2f -> 0x1218e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 信息 ID: a09536eee3d0634a97694dc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$0 <- system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=30700; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=32480; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=44944; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=48084; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=52096; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=59800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=61048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; module=3006, inline_record=5724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$1
// 信息 ID: 637cb83c1434b7974e676e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1415, inline_record=85880; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1415, inline_record=86564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 信息 ID: 962d94bac288ad681746d777
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=23132; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=23620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 信息 ID: 4730930348eb9496a9332eac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>; module=3006, inline_record=540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 信息 ID: 6fc3d60c4783b56cac366ae3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$12 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> >; module=1415, inline_record=65800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 信息 ID: e3af75157bf46c65d87b5d33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$13 <- alloc::vec::spec_extend::impl$1::spec_extend<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$1::spec_extend<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>,alloc::alloc::Global>; module=1415, inline_record=26200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 信息 ID: 4dfb429d2160cf73edc9ec82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::unstable::quicksort::quicksort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::shared::pivot::median3_rec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::unstable::heapsort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 100 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1421, inline_record=1648; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1421, inline_record=1756; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1421, inline_record=1836; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1421, inline_record=1920; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1444, inline_record=14300; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1444, inline_record=14504; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1444, inline_record=14708; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1444, inline_record=14912; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 信息 ID: b3dd8269fdb33988058fa11c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 15, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::stable::driftsort_main<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>,alloc::vec::Vec<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::stable::drift::sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::stable::quicksort::quicksort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=1884; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=1984; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=2180; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=2296; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=2480; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=2816; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=41364; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1444, inline_record=41568; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 信息 ID: 6408e68b9fa9ddc4ac21bc84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 16, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=50708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$2
// 信息 ID: 6adab2816252e9c3abe693ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::shared::pivot::median3_rec<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=572; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=672; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=780; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=896; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=1076; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=1412; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=8884; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1444, inline_record=9264; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 信息 ID: 3783baf302a9af84a16523e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=11456; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=11604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$4
// 信息 ID: 7f20b8622aceee6e72309a3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >; module=1415, inline_record=70368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 信息 ID: ba4868ca6bd3134429fea60f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; module=3006, inline_record=1540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
// 信息 ID: 9a743b0f336cd2a907427893
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; module=3006, inline_record=2052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 信息 ID: 2f93b04fa5190037c4f2d47b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=14492; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=15016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 信息 ID: edd07f200f820f6a961de337
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=13584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$8
// 信息 ID: 62bd7d8c4e7d09b06413a96b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$8 <- core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$8 <- core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > >; module=1417, inline_record=9188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$9
// 信息 ID: 8e7390ff6ff6869cd81397f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::shared::pivot::median3_rec<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=3180; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=3280; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=3388; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=3504; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=3684; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=4020; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=10096; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1444, inline_record=10476; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::overlapping_windows
// 信息 ID: 8cdd20c3c27ec4c9f5dac2eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > > @ 0x12088b9 -> 0x12218a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline
// 信息 ID: e53e1656b9fd92c365f0bffa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758e6f -> 0x1217be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 信息 ID: 147d1c4d15cfdceb8e57c07e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=7140; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=8328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$1
// 信息 ID: 671e1a4b3b9141f1ba316c25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=1324; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=1532; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=1880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 信息 ID: 9b9db336159850da248c7546
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::roof_outline::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1415, inline_record=32196; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1415, inline_record=32472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$3
// 信息 ID: fa5fd2e2a4fc8705fdac8487
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1435, inline_record=3216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::wrap_around
// 信息 ID: 28af8d79799bf30a5422c4aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=20088; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1435, inline_record=20260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
