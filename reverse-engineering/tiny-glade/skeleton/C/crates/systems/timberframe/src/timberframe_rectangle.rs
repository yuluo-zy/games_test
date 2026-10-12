// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/timberframe/src/timberframe_rectangle.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 65163c9121f7d57d886d373b88ef1d60ac0ccf6d323d73d59b3949d5ef11774b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: timberframe_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2461
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
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
// use timberframe; // namespace candidate
//   package identities: [{"name":"timberframe","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_rectangle::construct_timberframe_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2461,"record_offset":268,"rva":32793472,"va":5401502592}
// 编译实例 RVAs: 0x1f46380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16900327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8191ffefe","size":5,"soft_fail":false,"target_va":5384609408,"va":5401509730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25824112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e890f475fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5401502795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=16
//     evidence={"assembly":"\tcallq\t-25825880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8a8ed75fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5401504579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32780593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8cfce0bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5401502780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32270961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e88f9513fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5401511484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32269681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e88f9a13fe","size":5,"soft_fail":false,"target_va":5369240608,"va":5401510284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31965855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8613d18fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5401507850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19777867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8b536d2fe","size":5,"soft_fail":false,"target_va":5381726576,"va":5401504438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18181452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8b492eafe","size":5,"soft_fail":false,"target_va":5383325056,"va":5401506503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f426e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e803c2ffff","size":5,"soft_fail":false,"target_va":5401487072,"va":5401502936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142832530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9355539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e813c18e00","size":5,"soft_fail":false,"target_va":5410858288,"va":5401502744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142832800
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9354482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8f2bc8e00","size":5,"soft_fail":false,"target_va":5410859008,"va":5401504521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142832f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9357639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e847c98e00","size":5,"soft_fail":false,"target_va":5410860944,"va":5401503300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15552869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e86551ed00","size":5,"soft_fail":false,"target_va":5417057792,"va":5401504918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15110869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8d592e600","size":5,"soft_fail":false,"target_va":5416620769,"va":5401509895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t15118429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e85db0e600","size":5,"soft_fail":false,"target_va":5416621600,"va":5401503166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15119081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8e9b2e600","size":5,"soft_fail":false,"target_va":5416622240,"va":5401503154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21531039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e86176b7fe","size":5,"soft_fail":false,"target_va":5379976800,"va":5401507834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20313098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8f60bcafe","size":5,"soft_fail":false,"target_va":5381194720,"va":5401507813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20313269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e84b0bcafe","size":5,"soft_fail":false,"target_va":5381194512,"va":5401507776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23279599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e811c89cfe","size":5,"soft_fail":false,"target_va":5378226528,"va":5401506122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15073635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e86301e600","size":5,"soft_fail":false,"target_va":5416576832,"va":5401503192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15066273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8a1e4e500","size":5,"soft_fail":false,"target_va":5416576144,"va":5401509866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17518216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e878b1f4fe","size":5,"soft_fail":false,"target_va":5383984848,"va":5401503059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17524392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e85899f4fe","size":5,"soft_fail":false,"target_va":5383985360,"va":5401509747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-18096178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8cedfebfe","size":5,"soft_fail":false,"target_va":5383408256,"va":5401504429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18186257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8ef7feafe","size":5,"soft_fail":false,"target_va":5383321632,"va":5401507884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18181304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e84893eafe","size":5,"soft_fail":false,"target_va":5383322368,"va":5401503667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18185658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e84682eafe","size":5,"soft_fail":false,"target_va":5383323216,"va":5401508869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18189240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e84874eafe","size":5,"soft_fail":false,"target_va":5383314544,"va":5401503779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18185400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e84883eafe","size":5,"soft_fail":false,"target_va":5383323424,"va":5401508819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18229048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401502592,"raw_hex":"e8c8d8e9fe","size":5,"soft_fail":false,"target_va":5383277472,"va":5401506515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5401502592,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5401502765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2461,"record_offset":1240,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2461,"record_offset":2712,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2461,"record_offset":1944,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2461,"record_offset":14808,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2461,"record_offset":4044,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2461,"record_offset":7176,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":2461,"record_offset":10612,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2461,"record_offset":5360,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2461,"record_offset":10472,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":1420,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":544,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":660,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2461,"record_offset":1296,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2461,"record_offset":7092,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2461,"record_offset":3004,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2461,"record_offset":600,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2461,"record_offset":696,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26135,"source_line":155,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2461,"record_offset":1484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2461,"record_offset":3068,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":16667,"source_line":277,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2461,"record_offset":8036,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2461,"record_offset":7932,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":5092,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":10796,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":7720,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":10676,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":7828,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2461,"record_offset":4740,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2461,"record_offset":14864,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2461,"record_offset":14724,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2461,"record_offset":5304,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"timberframe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_rectangle.rs
//     evidence={"module":2461,"record_offset":4072,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":105175,"source_line":40,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":2461,"record_offset":3560,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":804,"parent_record_offset":772,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2461,"record_offset":4316,"parent_record_offset":4212,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2461,"record_offset":772,"parent_record_offset":736,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2461,"record_offset":736,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2461,"record_offset":4248,"parent_record_offset":4212,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2461,"record_offset":4212,"parent_record_offset":4184,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2461,"record_offset":4184,"parent_record_offset":4156,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2461,"record_offset":928,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":12983,"source_line":1913,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2461,"record_offset":980,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2461,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2461,"record_offset":2260,"parent_record_offset":2032,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2461,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2461,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2461,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2461,"record_offset":2008,"parent_record_offset":1968,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":2461,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2461,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":268,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2461,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":268,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2461,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":268,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2461,"record_offset":3952,"parent_record_offset":3584,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":2461,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":268,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2461,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":268,"inlinee":6415,"source_line":22,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2461,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":268,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2461,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":268,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2461,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":268,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2461,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2461,"record_offset":1036,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":6414,"source_line":270,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2461,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2461,"record_offset":2772,"parent_record_offset":2740,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2461,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":2461,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":268,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":2461,"record_offset":10520,"parent_record_offset":10496,"ancestor_record_offset":268,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":2461,"record_offset":10640,"parent_record_offset":10612,"ancestor_record_offset":268,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2461,"record_offset":5388,"parent_record_offset":5360,"ancestor_record_offset":268,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2461,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":268,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2461,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2461,"record_offset":2076,"parent_record_offset":2032,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2461,"record_offset":4284,"parent_record_offset":4248,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2461,"record_offset":7760,"parent_record_offset":7720,"ancestor_record_offset":268,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2461,"record_offset":8316,"parent_record_offset":8064,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2461,"record_offset":7864,"parent_record_offset":7828,"ancestor_record_offset":268,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2461,"record_offset":5148,"parent_record_offset":5116,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":1764,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":268,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":14832,"parent_record_offset":14808,"ancestor_record_offset":268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":2852,"parent_record_offset":2772,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":1564,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2461,"record_offset":1804,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::panicking::panic_display
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panicking.rs
//     evidence={"module":2461,"record_offset":1204,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":4970,"source_line":258,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2461,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2461,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2461,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2461,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2461,"record_offset":3032,"parent_record_offset":3004,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2461,"record_offset":2132,"parent_record_offset":2032,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2461,"record_offset":2500,"parent_record_offset":2032,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2461,"record_offset":1732,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2461,"record_offset":2188,"parent_record_offset":2160,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2461,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2461,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2461,"record_offset":2928,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":15096,"parent_record_offset":15032,"ancestor_record_offset":268,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":1640,"parent_record_offset":1600,"ancestor_record_offset":268,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":1148,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":1092,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":268,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":268,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2461,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":268,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2461,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2461,"record_offset":1840,"parent_record_offset":1804,"ancestor_record_offset":268,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2461,"record_offset":15004,"parent_record_offset":14976,"ancestor_record_offset":268,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":2461,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":268,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2461,"record_offset":3216,"parent_record_offset":3092,"ancestor_record_offset":268,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":2461,"record_offset":3628,"parent_record_offset":3584,"ancestor_record_offset":268,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2461,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":268,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":2461,"record_offset":3692,"parent_record_offset":3628,"ancestor_record_offset":268,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":2461,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":268,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":2461,"record_offset":3784,"parent_record_offset":3584,"ancestor_record_offset":268,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2461,"record_offset":8188,"parent_record_offset":8128,"ancestor_record_offset":268,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2461,"record_offset":8128,"parent_record_offset":8064,"ancestor_record_offset":268,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2461,"record_offset":8064,"parent_record_offset":8036,"ancestor_record_offset":268,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":5204,"parent_record_offset":5116,"ancestor_record_offset":268,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":10872,"parent_record_offset":10824,"ancestor_record_offset":268,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":268,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2461,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":268,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2461,"record_offset":4860,"parent_record_offset":4740,"ancestor_record_offset":268,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2461,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":268,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2461,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":268,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2461,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":268,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":2461,"record_offset":1600,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":2461,"record_offset":15032,"parent_record_offset":15004,"ancestor_record_offset":268,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":2461,"record_offset":1696,"parent_record_offset":1600,"ancestor_record_offset":268,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":2461,"record_offset":15068,"parent_record_offset":15032,"ancestor_record_offset":268,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2461,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19AD7
// 首条位置: {"module":2461,"inlinee":105175}
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
// 完整结构及原始类型记录: timberframe_rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: timberframe_rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: timberframe_rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>", "bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle
// 信息 ID: 8ef02b276dd2cb91b71cba9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6a5106819f01ef413c69ef30: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> @ 0xfda543 -> 0x1f46380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> > @ 0x1f4c49a -> 0x1f46380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 信息 ID: c6cd965135e9f92cebcb81cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "timberframe::timberframe_rectangle::construct_timberframe_rectangle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2461, inline_record=4072; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2461, inline_record=6456; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2461, inline_record=9544; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2461, inline_record=12064; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2461, inline_record=14696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
