// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/utils/wall_corners.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2fa1830a8bf7204c16ff0ee1be40fec6f8d144c969eb2c95ef631004f8e5656b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_corners.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1489, 1504, 3152
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1489,"record_offset":280,"rva":21192736,"va":5389901856}
// 编译实例 RVAs: 0x1436020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14225599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e841ef26ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389904282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-14227383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e849e826ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389906082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21180492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8b4cfbcfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389902679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18022078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e84201edfe","size":5,"soft_fail":false,"target_va":5371879952,"va":5389902025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16326450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8cee006ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5389902077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14276811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8352726ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5389902870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11787689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e857224cff","size":5,"soft_fail":false,"target_va":5378120480,"va":5389908164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11781401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8e73a4cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389908340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6620993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8bff89aff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389904172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-197936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8d0fafcff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389902603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414789f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t265681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8d10d0400","size":5,"soft_fail":false,"target_va":5390174704,"va":5389909018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27154276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e864579e01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389903383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26711564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e80c969701","size":5,"soft_fail":false,"target_va":5416620769,"va":5389909200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26472982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e816f29301","size":5,"soft_fail":false,"target_va":5416381568,"va":5389908581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26711566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e80e969701","size":5,"soft_fail":false,"target_va":5416621600,"va":5389910029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t26712264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8c8989701","size":5,"soft_fail":false,"target_va":5416621762,"va":5389909493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26712544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8e0999701","size":5,"soft_fail":false,"target_va":5416622560,"va":5389910011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26712975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e88f9b9701","size":5,"soft_fail":false,"target_va":5416622240,"va":5389909260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26712156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e85c989701","size":5,"soft_fail":false,"target_va":5416621632,"va":5389909471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11426877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8c3a351ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5389908552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8669446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8fab67bff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389904049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8670443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e815b37bff","size":5,"soft_fail":false,"target_va":5381231744,"va":5389902182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6520731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e865809cff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389903878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t269527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8d71c0400","size":5,"soft_fail":false,"target_va":5390175104,"va":5389905572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::find_corners
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17482233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8f9c10a01","size":5,"soft_fail":false,"target_va":5407384480,"va":5389902242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6602424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e848419bff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389904659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6602362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e886419bff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389904261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6605535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e821359bff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389907818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6603806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8e23b9bff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389905545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6602705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e82f409bff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389904924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6629424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8d0d79aff","size":5,"soft_fail":false,"target_va":5383274080,"va":5389903499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6625707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e855e69aff","size":5,"soft_fail":false,"target_va":5383282224,"va":5389907926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6621079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e869f89aff","size":5,"soft_fail":false,"target_va":5383283152,"va":5389904226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6629395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389901856,"raw_hex":"e8edd79aff","size":5,"soft_fail":false,"target_va":5383278560,"va":5389907950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1489,"record_offset":3976,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$23::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":4160,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":12134,"source_line":3746,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1489,"record_offset":7316,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":15156,"source_line":47,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":916,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":7560,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":1092,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":944,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":1000,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":2620,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1489,"record_offset":8112,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":2528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":4560,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":4300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1489,"record_offset":2732,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1489,"record_offset":4420,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":8024,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":10464,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":5136,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":2648,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1489,"record_offset":2352,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1489,"record_offset":2416,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1489,"record_offset":888,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":3796,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":3920,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79708,"source_line":1359,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1489,"record_offset":7784,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":3628,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1489,"record_offset":2564,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":1489,"record_offset":8140,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79492,"source_line":158,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":5804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79709,"source_line":522,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":644,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":9160,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":14735,"source_line":687,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":4616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":1064,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":1284,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26651,"source_line":157,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":1404,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1489,"record_offset":4392,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":27051,"source_line":156,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1489,"record_offset":3888,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":27689,"source_line":257,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":4036,"parent_record_offset":4004,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":5300,"parent_record_offset":5208,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1489,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":7480,"parent_record_offset":7348,"ancestor_record_offset":280,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":7348,"parent_record_offset":7316,"ancestor_record_offset":280,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":7388,"parent_record_offset":7348,"ancestor_record_offset":280,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":280,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":5236,"parent_record_offset":5208,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":7628,"parent_record_offset":7588,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1489,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1489,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1489,"record_offset":2788,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":760,"parent_record_offset":644,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1489,"record_offset":5268,"parent_record_offset":5236,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1489,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":280,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":1692,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":1664,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":1636,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1489,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1489,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1489,"record_offset":5920,"parent_record_offset":5888,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":1364,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":2900,"parent_record_offset":2788,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1489,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1489,"record_offset":2868,"parent_record_offset":2812,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1489,"record_offset":1028,"parent_record_offset":1000,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":7748,"parent_record_offset":7588,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1489,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":280,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1489,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1489,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1489,"record_offset":3128,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1489,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":280,"inlinee":79706,"source_line":312,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":8052,"parent_record_offset":8024,"ancestor_record_offset":280,"inlinee":79711,"source_line":618,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":816,"parent_record_offset":644,"ancestor_record_offset":280,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":1452,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":6036,"parent_record_offset":5804,"ancestor_record_offset":280,"inlinee":79710,"source_line":1347,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":1544,"parent_record_offset":1404,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1489,"record_offset":5888,"parent_record_offset":5804,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1489,"record_offset":12668,"rva":21202800,"va":5389911920}
// 编译实例 RVAs: 0x1438770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14235207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8b9c926ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389913890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-14236773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e89bc326ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389915472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21190917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8fba6bcfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389913104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18032135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8f9d9ecfe","size":5,"soft_fail":false,"target_va":5371879952,"va":5389912082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16336498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e88eb906ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5389912125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11788234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e836204cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389915173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4e290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8301850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8e65281ff","size":5,"soft_fail":false,"target_va":5381612176,"va":5389914021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6630614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e82ad39aff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389913793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6590020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8bc719bff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389915039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-208364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e814d2fcff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389913031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27144398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8ce309e01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389913261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26704973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e84d7c9701","size":5,"soft_fail":false,"target_va":5416620769,"va":5389915791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26466164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e874d79301","size":5,"soft_fail":false,"target_va":5416381568,"va":5389915399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26705644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8ec7e9701","size":5,"soft_fail":false,"target_va":5416621600,"va":5389915951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t26705896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8e87f9701","size":5,"soft_fail":false,"target_va":5416621762,"va":5389915861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26706655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8df829701","size":5,"soft_fail":false,"target_va":5416622560,"va":5389915900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26705779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8737f9701","size":5,"soft_fail":false,"target_va":5416621632,"va":5389915848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11433691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8258951ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5389915366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8680466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8ee8b7bff","size":5,"soft_fail":false,"target_va":5381232208,"va":5389912669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8678576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e850937bff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389913179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8680463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8f18b7bff","size":5,"soft_fail":false,"target_va":5381231744,"va":5389912202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6530455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8695a9cff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389913602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t260998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e886fb0300","size":5,"soft_fail":false,"target_va":5390175104,"va":5389914101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::find_corners
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17472213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8d59a0a01","size":5,"soft_fail":false,"target_va":5407384480,"va":5389912262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6611896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8481c9bff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389914131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::dims
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6612050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8ae1b9bff","size":5,"soft_fail":false,"target_va":5383302192,"va":5389914237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6611970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8fe1b9bff","size":5,"soft_fail":false,"target_va":5383301904,"va":5389913869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6612647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e859199bff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389914930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6612335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8911a9bff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389914074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6611932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8241c9bff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389914151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6639321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e827b19aff","size":5,"soft_fail":false,"target_va":5383274080,"va":5389913396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6632800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8a0ca9aff","size":5,"soft_fail":false,"target_va":5383282224,"va":5389915019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6630696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8d8d29aff","size":5,"soft_fail":false,"target_va":5383283152,"va":5389913843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6636556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389911920,"raw_hex":"e8f4bb9aff","size":5,"soft_fail":false,"target_va":5383278560,"va":5389915111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1489,"record_offset":17604,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$23::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":17788,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":12134,"source_line":3746,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1489,"record_offset":18012,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":15156,"source_line":47,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":13516,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":13304,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":13420,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":16252,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1489,"record_offset":18664,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":16168,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1489,"record_offset":16364,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1489,"record_offset":17844,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14044,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":20456,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":18868,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":16280,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1489,"record_offset":13276,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1489,"record_offset":17816,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":79718,"source_line":429,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":13796,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":17424,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":17548,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":79708,"source_line":1359,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":13708,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1489,"record_offset":18516,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1489,"record_offset":18604,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":17284,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1489,"record_offset":16196,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":1489,"record_offset":18256,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":79492,"source_line":158,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":13032,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":17956,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":13488,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":15476,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":26651,"source_line":157,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":15592,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1489,"record_offset":17516,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":27689,"source_line":257,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":17688,"parent_record_offset":17664,"ancestor_record_offset":12668,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":17712,"parent_record_offset":17688,"ancestor_record_offset":12668,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":19076,"parent_record_offset":19052,"ancestor_record_offset":12668,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":12668,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":17736,"parent_record_offset":17712,"ancestor_record_offset":12668,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":12668,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":17664,"parent_record_offset":17632,"ancestor_record_offset":12668,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1489,"record_offset":19052,"parent_record_offset":18964,"ancestor_record_offset":12668,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1489,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":12668,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":18176,"parent_record_offset":18044,"ancestor_record_offset":12668,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":18044,"parent_record_offset":18012,"ancestor_record_offset":12668,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":18084,"parent_record_offset":18044,"ancestor_record_offset":12668,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1489,"record_offset":18108,"parent_record_offset":18084,"ancestor_record_offset":12668,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18388,"parent_record_offset":18356,"ancestor_record_offset":12668,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":12668,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18964,"parent_record_offset":18940,"ancestor_record_offset":12668,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18436,"parent_record_offset":18412,"ancestor_record_offset":12668,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18412,"parent_record_offset":18388,"ancestor_record_offset":12668,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1489,"record_offset":18940,"parent_record_offset":18916,"ancestor_record_offset":12668,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":18356,"parent_record_offset":18312,"ancestor_record_offset":12668,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":18284,"parent_record_offset":18256,"ancestor_record_offset":12668,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":18312,"parent_record_offset":18284,"ancestor_record_offset":12668,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":14416,"parent_record_offset":14384,"ancestor_record_offset":12668,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12668,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":19496,"parent_record_offset":19472,"ancestor_record_offset":12668,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1489,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":12668,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":15420,"parent_record_offset":15312,"ancestor_record_offset":12668,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":14696,"parent_record_offset":14672,"ancestor_record_offset":12668,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":15312,"parent_record_offset":15288,"ancestor_record_offset":12668,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":12668,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":12668,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":13940,"parent_record_offset":13916,"ancestor_record_offset":12668,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1489,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":12668,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1489,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":12668,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1489,"record_offset":14932,"parent_record_offset":14780,"ancestor_record_offset":12668,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1489,"record_offset":14744,"parent_record_offset":14080,"ancestor_record_offset":12668,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1489,"record_offset":14128,"parent_record_offset":14080,"ancestor_record_offset":12668,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1489,"record_offset":16436,"parent_record_offset":16400,"ancestor_record_offset":12668,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1489,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":12668,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1489,"record_offset":14648,"parent_record_offset":14192,"ancestor_record_offset":12668,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":13148,"parent_record_offset":13032,"ancestor_record_offset":12668,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14160,"parent_record_offset":14128,"ancestor_record_offset":12668,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14232,"parent_record_offset":14192,"ancestor_record_offset":12668,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14780,"parent_record_offset":14744,"ancestor_record_offset":12668,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":15288,"parent_record_offset":14932,"ancestor_record_offset":12668,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14080,"parent_record_offset":14044,"ancestor_record_offset":12668,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":14192,"parent_record_offset":14160,"ancestor_record_offset":12668,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1489,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":12668,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1489,"record_offset":13332,"parent_record_offset":13304,"ancestor_record_offset":12668,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1489,"record_offset":13356,"parent_record_offset":13332,"ancestor_record_offset":12668,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1489,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":12668,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1489,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":12668,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":15876,"parent_record_offset":15592,"ancestor_record_offset":12668,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":13980,"parent_record_offset":13820,"ancestor_record_offset":12668,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":12668,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1489,"record_offset":14268,"parent_record_offset":14232,"ancestor_record_offset":12668,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1489,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":12668,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":15556,"parent_record_offset":15476,"ancestor_record_offset":12668,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":14344,"parent_record_offset":14304,"ancestor_record_offset":12668,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":12668,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":12668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1489,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":12668,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1489,"record_offset":16524,"parent_record_offset":16464,"ancestor_record_offset":12668,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1489,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":12668,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1489,"record_offset":18476,"parent_record_offset":18312,"ancestor_record_offset":12668,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1489,"record_offset":13232,"parent_record_offset":13204,"ancestor_record_offset":12668,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":12668,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1489,"record_offset":13064,"parent_record_offset":13032,"ancestor_record_offset":12668,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":12668,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":12668,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":13572,"parent_record_offset":13544,"ancestor_record_offset":12668,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1489,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":12668,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1489,"record_offset":16464,"parent_record_offset":16436,"ancestor_record_offset":12668,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14512,"parent_record_offset":14384,"ancestor_record_offset":12668,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":13884,"parent_record_offset":13852,"ancestor_record_offset":12668,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":13852,"parent_record_offset":13820,"ancestor_record_offset":12668,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":12668,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":12668,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1489,"record_offset":16784,"parent_record_offset":16400,"ancestor_record_offset":12668,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1489,"record_offset":16400,"parent_record_offset":16364,"ancestor_record_offset":12668,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":16628,"parent_record_offset":16604,"ancestor_record_offset":12668,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1489,"record_offset":13916,"parent_record_offset":13884,"ancestor_record_offset":12668,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":14304,"parent_record_offset":14268,"ancestor_record_offset":12668,"inlinee":79713,"source_line":64,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":14384,"parent_record_offset":14344,"ancestor_record_offset":12668,"inlinee":79715,"source_line":66,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":12668,"inlinee":79716,"source_line":68,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$3
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":16604,"parent_record_offset":16580,"ancestor_record_offset":12668,"inlinee":79717,"source_line":101,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":13204,"parent_record_offset":13032,"ancestor_record_offset":12668,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":15636,"parent_record_offset":15592,"ancestor_record_offset":12668,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":15728,"parent_record_offset":15592,"ancestor_record_offset":12668,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13761
// 首条位置: {"module":1489,"inlinee":79713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1489,"record_offset":14344,"parent_record_offset":14304,"ancestor_record_offset":14304,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":14416,"parent_record_offset":14384,"ancestor_record_offset":14304,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":14564,"parent_record_offset":14536,"ancestor_record_offset":14304,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":14444,"parent_record_offset":14416,"ancestor_record_offset":14304,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":14472,"parent_record_offset":14444,"ancestor_record_offset":14304,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14512,"parent_record_offset":14384,"ancestor_record_offset":14304,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14592,"parent_record_offset":14536,"ancestor_record_offset":14304,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":14304,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1489,"record_offset":14384,"parent_record_offset":14344,"ancestor_record_offset":14304,"inlinee":79715,"source_line":66,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13763
// 首条位置: {"module":1489,"inlinee":79715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1489,"record_offset":14416,"parent_record_offset":14384,"ancestor_record_offset":14384,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14512,"parent_record_offset":14384,"ancestor_record_offset":14384,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":14564,"parent_record_offset":14536,"ancestor_record_offset":14384,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":14444,"parent_record_offset":14416,"ancestor_record_offset":14384,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1489,"record_offset":14472,"parent_record_offset":14444,"ancestor_record_offset":14384,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14592,"parent_record_offset":14536,"ancestor_record_offset":14384,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1489,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":14384,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13764
// 首条位置: {"module":1489,"inlinee":79716}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1489,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":15360,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13765
// 首条位置: {"module":1489,"inlinee":79717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":16628,"parent_record_offset":16604,"ancestor_record_offset":16604,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":16736,"parent_record_offset":16628,"ancestor_record_offset":16604,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":16680,"parent_record_offset":16628,"ancestor_record_offset":16604,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":16604,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1375A
// 首条位置: {"module":1489,"inlinee":79706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2948,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":3080,"parent_record_offset":2972,"ancestor_record_offset":2948,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":3024,"parent_record_offset":2972,"ancestor_record_offset":2948,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":2948,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 522
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1375D
// 首条位置: {"module":1489,"inlinee":79709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1489,"record_offset":6064,"parent_record_offset":5804,"ancestor_record_offset":5804,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1489,"record_offset":6036,"parent_record_offset":5804,"ancestor_record_offset":5804,"inlinee":79710,"source_line":1347,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1489,"record_offset":5888,"parent_record_offset":5804,"ancestor_record_offset":5804,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":5804,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1489,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":5804,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":6984,"parent_record_offset":6868,"ancestor_record_offset":6612,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5804,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1489,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":5804,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1489,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":5804,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1489,"record_offset":5920,"parent_record_offset":5888,"ancestor_record_offset":5804,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 618
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1375F
// 首条位置: {"module":1489,"inlinee":79711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::find_corners
// 记录形式: compiled_procedure; instances=1
// 源码行: 662
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3152,"record_offset":280,"rva":38675360,"va":5407384480}
// 编译实例 RVAs: 0x24e23a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31706575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407384480,"raw_hex":"e831321cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407385274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9673151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407384480,"raw_hex":"e8bf999300","size":5,"soft_fail":false,"target_va":5417057696,"va":5407384540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8981249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407384480,"raw_hex":"e8010b8900","size":5,"soft_fail":false,"target_va":5416365968,"va":5407384714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9236562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407384480,"raw_hex":"e852f08c00","size":5,"soft_fail":false,"target_va":5416621762,"va":5407385195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3152,"record_offset":504,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3152,"record_offset":988,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3152,"record_offset":1276,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3152,"record_offset":2700,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3152,"record_offset":1192,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3152,"record_offset":448,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::check_angle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":3152,"record_offset":532,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":117789,"source_line":688,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3152,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3152,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3152,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3152,"record_offset":2864,"parent_record_offset":2772,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":2496,"parent_record_offset":2460,"ancestor_record_offset":280,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":2800,"parent_record_offset":2772,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3152,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3152,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3152,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3152,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3152,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3152,"record_offset":844,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3152,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3152,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3152,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3152,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3152,"record_offset":2832,"parent_record_offset":2800,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3152,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3152,"record_offset":872,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3152,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3152,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3152,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3152,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3152,"record_offset":1428,"parent_record_offset":1348,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3152,"record_offset":1388,"parent_record_offset":1348,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3152,"record_offset":1156,"parent_record_offset":1012,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3152,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3152,"record_offset":1348,"parent_record_offset":1308,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":280,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$30::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3152,"record_offset":1460,"parent_record_offset":1308,"ancestor_record_offset":280,"inlinee":27233,"source_line":374,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3152,"record_offset":1308,"parent_record_offset":1276,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3152,"record_offset":784,"parent_record_offset":676,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3152,"record_offset":676,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":117790,"source_line":488,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::check_angle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 688
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC1D
// 首条位置: {"module":3152,"inlinee":117789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3152,"record_offset":844,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3152,"record_offset":872,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3152,"record_offset":676,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":117790,"source_line":488,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3152,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":532,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3152,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":532,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3152,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":532,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3152,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":532,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3152,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":532,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":532,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3152,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":532,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/03152.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3152,"record_offset":784,"parent_record_offset":676,"ancestor_record_offset":532,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03152.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_corners.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_corners.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_corners.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0
// 信息 ID: 7b1ac0016ebf4efec4d4b0b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=14304; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=14964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$0::closure$0
// 信息 ID: 0a6dcabb8949aa48eb24eef0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=14384; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=15032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$1
// 信息 ID: b826aa4bab90a0e1d0629533
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=15360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners::closure$3
// 信息 ID: 4f8c1f54fab0a9545e753a97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=16604; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=17076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: 4e24bddfa003407000ee212f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8053c2 -> 0x1438770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 信息 ID: fbcef74128381a427d8f84e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=2948; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=3420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 信息 ID: b12c6cd712f79339b501812d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=5804; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=6208; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=6612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 信息 ID: 238dacd1df3b65ab236fc05b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1489, inline_record=8052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: caee8931c333a26dc650ef56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::utils::wall_corners::add_wall_corners -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804d40 -> 0x1436020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::check_angle
// 信息 ID: efe8d321ad2998e2a9b7b7dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::find_corners; module=3152, inline_record=532; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::find_corners; module=3152, inline_record=1896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::find_corners
// 信息 ID: 935cb96c20639eb34419ad64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14361a2 -> 0x24e23a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14388c6 -> 0x24e23a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
