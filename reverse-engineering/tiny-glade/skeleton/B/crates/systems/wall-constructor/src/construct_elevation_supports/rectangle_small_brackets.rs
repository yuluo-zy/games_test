// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c5444198602b52da9ff3327bf9825cf95e9e5eba253125fc8b210dbc294b0fb2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle_small_brackets.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1483
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1483,"record_offset":1660,"rva":19205536,"va":5387914656}
// 编译实例 RVAs: 0x1250da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=6
//     target_aliases=12 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18648078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8f273e3fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387915737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18314593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e89f8ae8fe","size":5,"soft_fail":false,"target_va":5369600800,"va":5387915388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16281132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8d49107ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5387915543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13111590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8daee37ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387925009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13059369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8d7ba38ff","size":5,"soft_fail":false,"target_va":5374864432,"va":5387923796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12834927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e891273cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387916410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=21
//     target_aliases=1 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12835397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8bb253cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387916896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10461182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8026060ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387921289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10461149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8236060ff","size":5,"soft_fail":false,"target_va":5377460160,"va":5387921304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10178444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e874b064ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387920615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10012412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8043967ff","size":5,"soft_fail":false,"target_va":5377906288,"va":5387918695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7860666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8460e88ff","size":5,"soft_fail":false,"target_va":5380057424,"va":5387918085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5983645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e863b2a4ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387918168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5983594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e896b2a4ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387918149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-365253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e83b6dfaff","size":5,"soft_fail":false,"target_va":5387550576,"va":5387915824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f97b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-360450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8fe7ffaff","size":5,"soft_fail":false,"target_va":5387556784,"va":5387917229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fa6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-356243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e86d90faff","size":5,"soft_fail":false,"target_va":5387560624,"va":5387916862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fcf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-345347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8fdbafaff","size":5,"soft_fail":false,"target_va":5387570944,"va":5387916286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-296060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8847bfbff","size":5,"soft_fail":false,"target_va":5387623488,"va":5387919543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-288572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8c498fbff","size":5,"soft_fail":false,"target_va":5387632928,"va":5387921495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-287009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8df9efbff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387920716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23927528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8e81a6d01","size":5,"soft_fail":false,"target_va":5411847152,"va":5387919619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23930555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8bb266d01","size":5,"soft_fail":false,"target_va":5411847168,"va":5387916608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle_small_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23930156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e82c256d01","size":5,"soft_fail":false,"target_va":5411847952,"va":5387917791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13557217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e81f2231ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5387916252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23618691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e883646801","size":5,"soft_fail":false,"target_va":5411540963,"va":5387922267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23619561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8e9676801","size":5,"soft_fail":false,"target_va":5411541584,"va":5387922018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t23619770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8ba686801","size":5,"soft_fail":false,"target_va":5411541776,"va":5387922001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23560356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8a4806701","size":5,"soft_fail":false,"target_va":5411482433,"va":5387922072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23621914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e81a716801","size":5,"soft_fail":false,"target_va":5411542064,"va":5387920145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23621783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e897706801","size":5,"soft_fail":false,"target_va":5411541904,"va":5387920116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e823fbffff","size":5,"soft_fail":false,"target_va":5387913584,"va":5387914824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23628454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8a68a6801","size":5,"soft_fail":false,"target_va":5411543360,"va":5387914901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7498744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e808948dff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387915715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8123065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8470d84ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5387915684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9981839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e871b067ff","size":5,"soft_fail":false,"target_va":5377936416,"va":5387918250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-336955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8c5dbfaff","size":5,"soft_fail":false,"target_va":5387580928,"va":5387917878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5908882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e86ed6a5ff","size":5,"soft_fail":false,"target_va":5382009728,"va":5387918605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23570206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e81ea76701","size":5,"soft_fail":false,"target_va":5411492368,"va":5387922157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23573975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8d7b56701","size":5,"soft_fail":false,"target_va":5411494800,"va":5387920820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6096118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e80afba2ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387919713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6096778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e876f8a2ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387920789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6529787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8055d9cff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387915318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6524598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e84a719cff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387915233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6535279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e891479cff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387921754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6530099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8cd5b9cff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387914798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23718831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8afeb6901","size":5,"soft_fail":false,"target_va":5411640768,"va":5387921932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23615110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e886566801","size":5,"soft_fail":false,"target_va":5411537104,"va":5387921989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-72913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e82fe3feff","size":5,"soft_fail":false,"target_va":5387845616,"va":5387918524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-72468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8ece4feff","size":5,"soft_fail":false,"target_va":5387846080,"va":5387918543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-70112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e820eefeff","size":5,"soft_fail":false,"target_va":5387848464,"va":5387918571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5991756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8b492a4ff","size":5,"soft_fail":false,"target_va":5381924400,"va":5387916151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5993069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8938da4ff","size":5,"soft_fail":false,"target_va":5381923136,"va":5387916200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5995862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8aa82a4ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387916113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5995718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e83a83a4ff","size":5,"soft_fail":false,"target_va":5381920464,"va":5387916177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5998257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e84f79a4ff","size":5,"soft_fail":false,"target_va":5381920064,"va":5387918316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6032392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387914656,"raw_hex":"e8f8f3a3ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387918291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387914656,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387919650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":4408,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":6796,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1483,"record_offset":18304,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":14376,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1483,"record_offset":19028,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1483,"record_offset":15136,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1483,"record_offset":11840,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1483,"record_offset":6992,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1483,"record_offset":11552,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":14496,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":5792,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1483,"record_offset":10196,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":5524,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1483,"record_offset":5412,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1483,"record_offset":16864,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1483,"record_offset":17144,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1483,"record_offset":14580,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1483,"record_offset":14804,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1483,"record_offset":14524,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1483,"record_offset":15304,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1483,"record_offset":15392,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1483,"record_offset":15884,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1483,"record_offset":15164,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":3424,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":2000,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":2448,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":5856,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":5676,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":13280,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":4436,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":13668,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs
//     evidence={"module":1483,"record_offset":19056,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":71291,"source_line":279,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs
//     evidence={"module":1483,"record_offset":20356,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":71292,"source_line":303,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1483,"record_offset":18444,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1483,"record_offset":11384,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4633,"source_line":132,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1483,"record_offset":7132,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1483,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":1660,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1483,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":1660,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1483,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":1660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1483,"record_offset":7652,"parent_record_offset":7476,"ancestor_record_offset":1660,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1483,"record_offset":8588,"parent_record_offset":8500,"ancestor_record_offset":1660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":6872,"parent_record_offset":6840,"ancestor_record_offset":1660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":7412,"parent_record_offset":7380,"ancestor_record_offset":1660,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":8528,"parent_record_offset":8500,"ancestor_record_offset":1660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":1660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":1660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":1660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":7476,"parent_record_offset":7444,"ancestor_record_offset":1660,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":7444,"parent_record_offset":7412,"ancestor_record_offset":1660,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":1660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1483,"record_offset":7512,"parent_record_offset":7476,"ancestor_record_offset":1660,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1483,"record_offset":7340,"parent_record_offset":7304,"ancestor_record_offset":1660,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1483,"record_offset":7304,"parent_record_offset":7232,"ancestor_record_offset":1660,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":6840,"parent_record_offset":6796,"ancestor_record_offset":1660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":7276,"parent_record_offset":7232,"ancestor_record_offset":1660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":7380,"parent_record_offset":7340,"ancestor_record_offset":1660,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":7196,"parent_record_offset":7132,"ancestor_record_offset":1660,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":1660,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1483,"record_offset":7232,"parent_record_offset":7196,"ancestor_record_offset":1660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1483,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":1660,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1483,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":1660,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1483,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":1660,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1483,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":1660,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1483,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":1660,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":1660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":1660,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":1660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":1660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3708,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":1660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":3904,"parent_record_offset":3732,"ancestor_record_offset":1660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1483,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":1660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":11504,"parent_record_offset":11480,"ancestor_record_offset":1660,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":1660,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":12912,"parent_record_offset":12888,"ancestor_record_offset":1660,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":1660,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":1660,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":1660,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1483,"record_offset":12760,"parent_record_offset":12144,"ancestor_record_offset":1660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1483,"record_offset":13516,"parent_record_offset":13488,"ancestor_record_offset":1660,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1483,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":1660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1483,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":1660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1483,"record_offset":14744,"parent_record_offset":14580,"ancestor_record_offset":1660,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1483,"record_offset":14684,"parent_record_offset":14580,"ancestor_record_offset":1660,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1483,"record_offset":18708,"parent_record_offset":18684,"ancestor_record_offset":1660,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1483,"record_offset":14620,"parent_record_offset":14580,"ancestor_record_offset":1660,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1483,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":1660,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1483,"record_offset":13004,"parent_record_offset":12824,"ancestor_record_offset":1660,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1483,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":1660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1483,"record_offset":6616,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1483,"record_offset":11920,"parent_record_offset":11884,"ancestor_record_offset":1660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1483,"record_offset":11884,"parent_record_offset":11840,"ancestor_record_offset":1660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1483,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":1660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1483,"record_offset":12024,"parent_record_offset":11988,"ancestor_record_offset":1660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1483,"record_offset":11432,"parent_record_offset":11408,"ancestor_record_offset":1660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1483,"record_offset":11456,"parent_record_offset":11432,"ancestor_record_offset":1660,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1483,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":1660,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1483,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":1660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1483,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":1660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1483,"record_offset":21156,"parent_record_offset":21128,"ancestor_record_offset":1660,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1483,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":1660,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1483,"record_offset":17504,"parent_record_offset":17476,"ancestor_record_offset":1660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1483,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":1660,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1483,"record_offset":8556,"parent_record_offset":8528,"ancestor_record_offset":1660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1483,"record_offset":2148,"parent_record_offset":2116,"ancestor_record_offset":1660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1483,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":1660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1483,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":1660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12348,"parent_record_offset":12240,"ancestor_record_offset":1660,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12292,"parent_record_offset":12240,"ancestor_record_offset":1660,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":1660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1483,"record_offset":3576,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1483,"record_offset":12104,"parent_record_offset":12064,"ancestor_record_offset":1660,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1483,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":1660,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1483,"record_offset":12864,"parent_record_offset":12824,"ancestor_record_offset":1660,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":11960,"parent_record_offset":11920,"ancestor_record_offset":1660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":17412,"parent_record_offset":17144,"ancestor_record_offset":1660,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":12064,"parent_record_offset":12024,"ancestor_record_offset":1660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":13068,"parent_record_offset":11884,"ancestor_record_offset":1660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":17224,"parent_record_offset":17144,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1483,"record_offset":17448,"parent_record_offset":17144,"ancestor_record_offset":1660,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1483,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":1660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1483,"record_offset":4364,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1483,"record_offset":7752,"parent_record_offset":7340,"ancestor_record_offset":1660,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1483,"record_offset":3968,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1483,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":1660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1483,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":1660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1483,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":1660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1483,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":1660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1483,"record_offset":13176,"parent_record_offset":13116,"ancestor_record_offset":1660,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1483,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1483,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1483,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":1660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1483,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1483,"record_offset":17020,"parent_record_offset":16992,"ancestor_record_offset":1660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1483,"record_offset":17048,"parent_record_offset":17020,"ancestor_record_offset":1660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1483,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1483,"record_offset":17384,"parent_record_offset":17144,"ancestor_record_offset":1660,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1483,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":1660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":1660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":1660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":2764,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1483,"record_offset":5588,"parent_record_offset":5524,"ancestor_record_offset":1660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1483,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1483,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":1660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1483,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1483,"record_offset":2584,"parent_record_offset":2548,"ancestor_record_offset":1660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1483,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1483,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1483,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1483,"record_offset":3088,"parent_record_offset":2548,"ancestor_record_offset":1660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2248,"parent_record_offset":2216,"ancestor_record_offset":1660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2344,"parent_record_offset":2248,"ancestor_record_offset":1660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":1660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2032,"parent_record_offset":2000,"ancestor_record_offset":1660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1483,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":1660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1483,"record_offset":2648,"parent_record_offset":2616,"ancestor_record_offset":1660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1483,"record_offset":2932,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1483,"record_offset":2904,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1483,"record_offset":2708,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1483,"record_offset":19508,"parent_record_offset":19448,"ancestor_record_offset":1660,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1483,"record_offset":17296,"parent_record_offset":17260,"ancestor_record_offset":1660,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1483,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":1660,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1483,"record_offset":17320,"parent_record_offset":17296,"ancestor_record_offset":1660,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1483,"record_offset":16896,"parent_record_offset":16864,"ancestor_record_offset":1660,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1483,"record_offset":17476,"parent_record_offset":17448,"ancestor_record_offset":1660,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1483,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":1660,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1483,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":1660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1483,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":1660,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1483,"record_offset":18516,"parent_record_offset":18472,"ancestor_record_offset":1660,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1483,"record_offset":18588,"parent_record_offset":18516,"ancestor_record_offset":1660,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1483,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":1660,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1483,"record_offset":18684,"parent_record_offset":18472,"ancestor_record_offset":1660,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1483,"record_offset":6316,"parent_record_offset":6288,"ancestor_record_offset":1660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1483,"record_offset":6348,"parent_record_offset":6316,"ancestor_record_offset":1660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1483,"record_offset":14024,"parent_record_offset":13852,"ancestor_record_offset":1660,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1483,"record_offset":6288,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":6148,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":13488,"parent_record_offset":13460,"ancestor_record_offset":1660,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":6644,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":13544,"parent_record_offset":13488,"ancestor_record_offset":1660,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":14112,"parent_record_offset":13800,"ancestor_record_offset":1660,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1483,"record_offset":6232,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1483,"record_offset":6120,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1483,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":1660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":1660,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":1660,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":1660,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":1660,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":4880,"parent_record_offset":4520,"ancestor_record_offset":1660,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":1660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":1660,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":1660,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":1660,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1483,"record_offset":5960,"parent_record_offset":5928,"ancestor_record_offset":1660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":14212,"parent_record_offset":13800,"ancestor_record_offset":1660,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":6736,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13460,"parent_record_offset":13400,"ancestor_record_offset":1660,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13800,"parent_record_offset":13764,"ancestor_record_offset":1660,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":1660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":5988,"parent_record_offset":5928,"ancestor_record_offset":1660,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":1660,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":21096,"parent_record_offset":21064,"ancestor_record_offset":1660,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13432,"parent_record_offset":13400,"ancestor_record_offset":1660,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":21008,"parent_record_offset":20984,"ancestor_record_offset":1660,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":1660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":21064,"parent_record_offset":21008,"ancestor_record_offset":1660,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":21036,"parent_record_offset":21008,"ancestor_record_offset":1660,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":1660,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":6092,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":21128,"parent_record_offset":21096,"ancestor_record_offset":1660,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":20984,"parent_record_offset":20960,"ancestor_record_offset":1660,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":1660,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1483,"record_offset":6448,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1483,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":1660,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1483,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":1660,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1483,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":1660,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1483,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":1660,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1483,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":1660,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1483,"record_offset":12240,"parent_record_offset":12188,"ancestor_record_offset":1660,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1483,"record_offset":17260,"parent_record_offset":17144,"ancestor_record_offset":1660,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1483,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":1660,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1483,"record_offset":17352,"parent_record_offset":17260,"ancestor_record_offset":1660,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1483,"record_offset":19480,"parent_record_offset":19448,"ancestor_record_offset":1660,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":1660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":21792,"parent_record_offset":21760,"ancestor_record_offset":1660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":22308,"parent_record_offset":22048,"ancestor_record_offset":1660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1483,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":1660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":3680,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1483,"record_offset":3940,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1483,"record_offset":4464,"parent_record_offset":4436,"ancestor_record_offset":1660,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1483,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":1660,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1483,"record_offset":4692,"parent_record_offset":4636,"ancestor_record_offset":1660,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1483,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":1660,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1483,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1483,"record_offset":3532,"parent_record_offset":3496,"ancestor_record_offset":1660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1483,"record_offset":3496,"parent_record_offset":3460,"ancestor_record_offset":1660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs
//     evidence={"module":1483,"record_offset":12144,"parent_record_offset":12104,"ancestor_record_offset":1660,"inlinee":71290,"source_line":125,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1483,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":1660,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1483,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":1660,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1483,"record_offset":12732,"parent_record_offset":12144,"ancestor_record_offset":1660,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12380,"parent_record_offset":12188,"ancestor_record_offset":1660,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":1660,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12188,"parent_record_offset":12144,"ancestor_record_offset":1660,"inlinee":24689,"source_line":55,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12792,"parent_record_offset":12760,"ancestor_record_offset":1660,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1167A
// 首条位置: {"module":1483,"inlinee":71290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1483,"record_offset":12760,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1483,"record_offset":12732,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12188,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":24689,"source_line":55,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":12912,"parent_record_offset":12888,"ancestor_record_offset":12144,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1483,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":12144,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1483,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":12144,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1483,"record_offset":13004,"parent_record_offset":12824,"ancestor_record_offset":12144,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12348,"parent_record_offset":12240,"ancestor_record_offset":12144,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12292,"parent_record_offset":12240,"ancestor_record_offset":12144,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1483,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":12144,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1483,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":12144,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1483,"record_offset":12864,"parent_record_offset":12824,"ancestor_record_offset":12144,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1483,"record_offset":12240,"parent_record_offset":12188,"ancestor_record_offset":12144,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1483,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":12144,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1483,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":12144,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12380,"parent_record_offset":12188,"ancestor_record_offset":12144,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":12144,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1483,"record_offset":12792,"parent_record_offset":12760,"ancestor_record_offset":12144,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01483.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1167B
// 首条位置: {"module":1483,"inlinee":71291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 303
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1167C
// 首条位置: {"module":1483,"inlinee":71292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: rectangle_small_brackets.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle_small_brackets.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle_small_brackets.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets
// 信息 ID: 6faecd748a0f182f94abc1e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets @ 0x789c77 -> 0x1250da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$0
// 信息 ID: 5b7d0f1c2ff99e353733856a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1483, inline_record=12144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$1
// 信息 ID: e2d4f36f2029bd19c6b5d39f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1483, inline_record=19056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets::closure$2
// 信息 ID: 4f2aee0ac9e44a0557effd65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets; module=1483, inline_record=20356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
