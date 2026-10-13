// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/utils/wall_corners.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 67b1da86b2b518ef98a1d7fc5f7b6e49244c10825a3cebc242074dcf9e4bb1e2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_corners.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1434, 1452
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
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1434,"record_offset":272,"rva":18961728,"va":5387670848}
// 编译实例 RVAs: 0x1215540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18718529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8bf60e2fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5387672252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16100376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8e8530aff","size":5,"soft_fail":false,"target_va":5371570656,"va":5387671027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12592079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e831dc3fff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387673562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12591502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e872de3fff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387673001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10216879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8511a64ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387676986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5680
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9770491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e805ea6aff","size":5,"soft_fail":false,"target_va":5377906304,"va":5387676790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c37760
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6153234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8ee1ba2ff","size":5,"soft_fail":false,"target_va":5381519200,"va":5387672429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5783750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e83abfa7ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5387673457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-118365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8a331feff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387672168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e81443feff","size":5,"soft_fail":false,"target_va":5387557152,"va":5387671047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123fe30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t167450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e81a8e0200","size":5,"soft_fail":false,"target_va":5387845168,"va":5387677713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24175122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e812e27001","size":5,"soft_fail":false,"target_va":5411847776,"va":5387672649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23862991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8cf1e6c01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387677996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6896371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e80dc596ff","size":5,"soft_fail":false,"target_va":5380775408,"va":5387671774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23863731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8b3216c01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387678040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t23864234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8aa236c01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387678057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23865438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e85e286c01","size":5,"soft_fail":false,"target_va":5411543584,"va":5387678141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23864527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8cf246c01","size":5,"soft_fail":false,"target_va":5411542880,"va":5387678348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9934665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8b76868ff","size":5,"soft_fail":false,"target_va":5377742032,"va":5387676692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7256285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8234791ff","size":5,"soft_fail":false,"target_va":5380417056,"va":5387673336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7253813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8cb5091ff","size":5,"soft_fail":false,"target_va":5380417344,"va":5387671152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5663474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e80e95a9ff","size":5,"soft_fail":false,"target_va":5382009728,"va":5387673197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t175330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8e2ac0200","size":5,"soft_fail":false,"target_va":5387850176,"va":5387674841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8e3110000","size":5,"soft_fail":false,"target_va":5387680288,"va":5387675704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t171881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8699f0200","size":5,"soft_fail":false,"target_va":5387845824,"va":5387673938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t172071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e827a00200","size":5,"soft_fail":false,"target_va":5387845616,"va":5387673540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t169934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8ce970200","size":5,"soft_fail":false,"target_va":5387846384,"va":5387676445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t171180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e8ac9c0200","size":5,"soft_fail":false,"target_va":5387846000,"va":5387674815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t172160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e880a00200","size":5,"soft_fail":false,"target_va":5387846368,"va":5387674203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5790066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e88ea6a7ff","size":5,"soft_fail":false,"target_va":5381882704,"va":5387672765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5786346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e816b5a7ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5387676533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5781955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e83dc6a7ff","size":5,"soft_fail":false,"target_va":5381891552,"va":5387673502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5790914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387670848,"raw_hex":"e83ea3a7ff","size":5,"soft_fail":false,"target_va":5381885648,"va":5387676557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":6796,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1434,"record_offset":6948,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26150,"source_line":3051,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1434,"record_offset":8692,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14547,"source_line":47,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":3464,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":8936,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":3640,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":3492,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1434,"record_offset":3548,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":5032,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1434,"record_offset":9416,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":4976,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":7352,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":7088,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1434,"record_offset":5428,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":7212,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":9328,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":11264,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":5144,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1434,"record_offset":5060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":4900,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1434,"record_offset":6976,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1434,"record_offset":808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1434,"record_offset":9240,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":6616,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":6740,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":30971,"source_line":1248,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1434,"record_offset":9128,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1434,"record_offset":6448,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1434,"record_offset":5004,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::find_corners
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1434,"record_offset":836,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70916,"source_line":492,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":564,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":7408,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":3612,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":3832,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":31919,"source_line":157,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":3952,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1434,"record_offset":7184,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":28102,"source_line":156,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1434,"record_offset":6708,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":25624,"source_line":257,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::x
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1434,"record_offset":9444,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70799,"source_line":151,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":6828,"parent_record_offset":6796,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":3388,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1434,"record_offset":8856,"parent_record_offset":8724,"ancestor_record_offset":272,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1434,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":272,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1434,"record_offset":8764,"parent_record_offset":8724,"ancestor_record_offset":272,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1434,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":272,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2848,"parent_record_offset":2808,"ancestor_record_offset":272,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":272,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":2904,"parent_record_offset":2808,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":936,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":2808,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1434,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1434,"record_offset":10520,"parent_record_offset":10496,"ancestor_record_offset":272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":1252,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1434,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":272,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1434,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":272,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1434,"record_offset":1480,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1434,"record_offset":5484,"parent_record_offset":5452,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1434,"record_offset":680,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1434,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":272,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1434,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1434,"record_offset":8812,"parent_record_offset":8788,"ancestor_record_offset":272,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":4240,"parent_record_offset":3952,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":4212,"parent_record_offset":3952,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1434,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1280,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":3912,"parent_record_offset":3832,"ancestor_record_offset":272,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":5660,"parent_record_offset":5484,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":272,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":3224,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1434,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1434,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1434,"record_offset":1612,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":272,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":3016,"parent_record_offset":2808,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1434,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1434,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":272,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1434,"record_offset":596,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1434,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1434,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":272,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1434,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":272,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1434,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1552,"parent_record_offset":1512,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1652,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1164,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$30::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1434,"record_offset":1740,"parent_record_offset":1512,"ancestor_record_offset":272,"inlinee":25314,"source_line":374,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1434,"record_offset":5888,"parent_record_offset":5452,"ancestor_record_offset":272,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1434,"record_offset":1512,"parent_record_offset":1480,"ancestor_record_offset":272,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1434,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1434,"record_offset":1220,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1434,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":272,"inlinee":70920,"source_line":142,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1434,"record_offset":9356,"parent_record_offset":9328,"ancestor_record_offset":272,"inlinee":70921,"source_line":448,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::check_angle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1434,"record_offset":964,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":70917,"source_line":518,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1434,"record_offset":1112,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":70918,"source_line":488,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":736,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":4000,"parent_record_offset":3952,"ancestor_record_offset":272,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":4092,"parent_record_offset":3952,"ancestor_record_offset":272,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11508
// 首条位置: {"module":1434,"inlinee":70920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1434,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":5708,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":5840,"parent_record_offset":5732,"ancestor_record_offset":5708,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":5784,"parent_record_offset":5732,"ancestor_record_offset":5708,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5708,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1434,"record_offset":13072,"rva":18971168,"va":5387680288}
// 编译实例 RVAs: 0x1217a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-126555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680288,"raw_hex":"e8a511feff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387680358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23861414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680288,"raw_hex":"e8a6186c01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387680645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23861634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680288,"raw_hex":"e882196c01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387680657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23861278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387680288,"raw_hex":"e81e186c01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387680621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1434,"record_offset":13476,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1434,"record_offset":13448,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":24639,"source_line":1347,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1434,"record_offset":13240,"parent_record_offset":13072,"ancestor_record_offset":13072,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":13072,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":13072,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":13388,"parent_record_offset":13240,"ancestor_record_offset":13072,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1434,"record_offset":13416,"parent_record_offset":13240,"ancestor_record_offset":13072,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":13612,"parent_record_offset":13476,"ancestor_record_offset":13072,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":13512,"parent_record_offset":13476,"ancestor_record_offset":13072,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":13584,"parent_record_offset":13476,"ancestor_record_offset":13072,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13072,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":13280,"parent_record_offset":13240,"ancestor_record_offset":13072,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11509
// 首条位置: {"module":1434,"inlinee":70921}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 492
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11504
// 首条位置: {"module":1434,"inlinee":70916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":936,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":2808,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1434,"record_offset":1480,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":3224,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1434,"record_offset":1396,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::check_angle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1434,"record_offset":964,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":70917,"source_line":518,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1434,"record_offset":3388,"parent_record_offset":3296,"ancestor_record_offset":836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2848,"parent_record_offset":2808,"ancestor_record_offset":836,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":836,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":836,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":836,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":836,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":836,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":836,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1434,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":836,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1434,"record_offset":2904,"parent_record_offset":2808,"ancestor_record_offset":836,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":836,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":836,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":1252,"parent_record_offset":964,"ancestor_record_offset":836,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1434,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":836,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1434,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":836,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1434,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":836,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1434,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":836,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":836,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1280,"parent_record_offset":964,"ancestor_record_offset":836,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":836,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1434,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":836,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1434,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":836,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1434,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":836,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1434,"record_offset":1612,"parent_record_offset":1552,"ancestor_record_offset":836,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1434,"record_offset":3016,"parent_record_offset":2808,"ancestor_record_offset":836,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1434,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":836,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":836,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1552,"parent_record_offset":1512,"ancestor_record_offset":836,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1434,"record_offset":1652,"parent_record_offset":1552,"ancestor_record_offset":836,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":836,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1164,"parent_record_offset":1112,"ancestor_record_offset":836,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":836,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$30::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1434,"record_offset":1740,"parent_record_offset":1512,"ancestor_record_offset":836,"inlinee":25314,"source_line":374,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1434,"record_offset":1512,"parent_record_offset":1480,"ancestor_record_offset":836,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1434,"record_offset":1220,"parent_record_offset":1112,"ancestor_record_offset":836,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1434,"record_offset":1112,"parent_record_offset":964,"ancestor_record_offset":836,"inlinee":70918,"source_line":488,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::check_angle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11505
// 首条位置: {"module":1434,"inlinee":70917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1434,"record_offset":1252,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1280,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1434,"record_offset":1112,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":70918,"source_line":488,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":964,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1434,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":964,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1434,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":964,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1434,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":964,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":964,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1434,"record_offset":1164,"parent_record_offset":1112,"ancestor_record_offset":964,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01434.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1434,"record_offset":1220,"parent_record_offset":1112,"ancestor_record_offset":964,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01434.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners
// 信息 ID: 0eb8f340b07d92e64c6dc930
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758cf3 -> 0x1215540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 信息 ID: e31ba929a60fb85c465e08ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=5708; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=6240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 信息 ID: 1682dc28aa4eb4f9e7479991
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216838 -> 0x1217a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121686e -> 0x1217a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121689f -> 0x1217a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 信息 ID: 06bf173590bc66f6139c50c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=9356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::check_angle
// 信息 ID: f5f5a75610139df5b2c460b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=964; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=2304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::find_corners
// 信息 ID: a80fc6c93eecb92a158698fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1434, inline_record=836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
