// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/timberframe/src/timberframe_rectangle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 029ca60e314d159e23075fe2f8f32068763e2a525f7b9ac6a69f879508d79ad0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: timberframe_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2290
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
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
// 首条位置: {"module":2290,"record_offset":260,"rva":27532192,"va":5396241312}
// 编译实例 RVAs: 0x1a41ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13603898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8c66b30ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5396248421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26978941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8835564fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5396246600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26181649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8ef7f70fe","size":5,"soft_fail":false,"target_va":5370067488,"va":5396249132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21436865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e83fe6b8fe","size":5,"soft_fail":false,"target_va":5374813424,"va":5396250284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21160097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e85f1fbdfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5396241580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21161804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8b418bdfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5396243303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20732224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8c0a6c3fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5396248251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18501011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e86db2e5fe","size":5,"soft_fail":false,"target_va":5377742176,"va":5396243182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14310636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e814a325ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5396245191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a3ce70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e833b1ffff","size":5,"soft_fail":false,"target_va":5396221552,"va":5396241720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b5d00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7815363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8c3407700","size":5,"soft_fail":false,"target_va":5404056832,"va":5396241464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b6000
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7814345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8c93c7700","size":5,"soft_fail":false,"target_va":5404057600,"va":5396243250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b62e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7816313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e879447700","size":5,"soft_fail":false,"target_va":5404058336,"va":5396242018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15098370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e80262e600","size":5,"soft_fail":false,"target_va":5411346944,"va":5396248569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15605619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8731fee00","size":5,"soft_fail":false,"target_va":5411847152,"va":5396241528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15604283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e83b1aee00","size":5,"soft_fail":false,"target_va":5411847904,"va":5396243616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15292210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e83257e900","size":5,"soft_fail":false,"target_va":5411540992,"va":5396248777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t15293057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8815ae900","size":5,"soft_fail":false,"target_va":5411541776,"va":5396248714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15233811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e81373e800","size":5,"soft_fail":false,"target_va":5411482433,"va":5396248617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18221374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8c2f6e9fe","size":5,"soft_fail":false,"target_va":5378025216,"va":5396246585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18216130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e83e0beafe","size":5,"soft_fail":false,"target_va":5378030432,"va":5396246557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18215832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8680ceafe","size":5,"soft_fail":false,"target_va":5378030704,"va":5396246531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15784428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e814260fff","size":5,"soft_fail":false,"target_va":5380460416,"va":5396244839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15243732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8d499e800","size":5,"soft_fail":false,"target_va":5411492368,"va":5396248631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15246146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e842a3e800","size":5,"soft_fail":false,"target_va":5411494800,"va":5396248649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14418314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e876fe23ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5396241909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14424426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e896e623ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5396248437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14328705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e87f5c25ff","size":5,"soft_fail":false,"target_va":5381917936,"va":5396246636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14323172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e81c7225ff","size":5,"soft_fail":false,"target_va":5381919216,"va":5396242383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14327314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8ee6125ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5396247565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14322357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e84b7525ff","size":5,"soft_fail":false,"target_va":5381920112,"va":5396242464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14327472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e8506125ff","size":5,"soft_fail":false,"target_va":5381920048,"va":5396247515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14360731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396241312,"raw_hex":"e865df24ff","size":5,"soft_fail":false,"target_va":5381884480,"va":5396245206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396241312,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396241548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2290,"record_offset":1380,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2290,"record_offset":3020,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2290,"record_offset":2232,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2290,"record_offset":14520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2290,"record_offset":4432,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2290,"record_offset":7556,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":2290,"record_offset":10752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2290,"record_offset":5720,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2290,"record_offset":10612,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":1740,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":540,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":836,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2290,"record_offset":1436,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2290,"record_offset":7472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2290,"record_offset":3368,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2290,"record_offset":596,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2290,"record_offset":872,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24520,"source_line":117,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2290,"record_offset":1804,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2290,"record_offset":3444,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2290,"record_offset":8476,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2290,"record_offset":8372,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":5496,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":5812,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":10904,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":8160,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":10816,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":8268,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2290,"record_offset":5016,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2290,"record_offset":14552,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2290,"record_offset":14436,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2290,"record_offset":5692,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"timberframe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_rectangle.rs
//     evidence={"module":2290,"record_offset":2932,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":93326,"source_line":40,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":2290,"record_offset":3852,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":972,"parent_record_offset":940,"ancestor_record_offset":260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2290,"record_offset":4704,"parent_record_offset":4600,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2290,"record_offset":912,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2290,"record_offset":4636,"parent_record_offset":4600,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2290,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2290,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2290,"record_offset":1096,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":12282,"source_line":1709,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2290,"record_offset":1148,"parent_record_offset":1096,"ancestor_record_offset":260,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2290,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2290,"record_offset":2552,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2290,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2290,"record_offset":2296,"parent_record_offset":2256,"ancestor_record_offset":260,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2290,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":260,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2290,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":260,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":2290,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":260,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2290,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":260,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2290,"record_offset":5776,"parent_record_offset":5748,"ancestor_record_offset":260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2290,"record_offset":664,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2290,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":260,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2290,"record_offset":4340,"parent_record_offset":3876,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2290,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":260,"inlinee":11098,"source_line":22,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2290,"record_offset":4140,"parent_record_offset":4088,"ancestor_record_offset":260,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2290,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":260,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":2290,"record_offset":1204,"parent_record_offset":1096,"ancestor_record_offset":260,"inlinee":11097,"source_line":268,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2290,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":260,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2290,"record_offset":3080,"parent_record_offset":3048,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2290,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":260,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":2290,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":260,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":2290,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":260,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":2290,"record_offset":10780,"parent_record_offset":10752,"ancestor_record_offset":260,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2290,"record_offset":5748,"parent_record_offset":5720,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2290,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":260,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2290,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2290,"record_offset":2368,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2290,"record_offset":4672,"parent_record_offset":4636,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2290,"record_offset":8200,"parent_record_offset":8160,"ancestor_record_offset":260,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2290,"record_offset":5868,"parent_record_offset":5840,"ancestor_record_offset":260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2290,"record_offset":8304,"parent_record_offset":8268,"ancestor_record_offset":260,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2290,"record_offset":5560,"parent_record_offset":5520,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":260,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":2064,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":260,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":3160,"parent_record_offset":3080,"ancestor_record_offset":260,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":1876,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2290,"record_offset":2100,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":2290,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":260,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":2290,"record_offset":14636,"parent_record_offset":14576,"ancestor_record_offset":260,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2290,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2290,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2290,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2290,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":260,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2290,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2290,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":260,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2290,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2290,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":260,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2290,"record_offset":2424,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2290,"record_offset":2776,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2290,"record_offset":2036,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2290,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2290,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":260,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2290,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2290,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2290,"record_offset":3236,"parent_record_offset":3184,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2290,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2290,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":260,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":14848,"parent_record_offset":14784,"ancestor_record_offset":260,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":1948,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":1316,"parent_record_offset":1096,"ancestor_record_offset":260,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":1260,"parent_record_offset":1096,"ancestor_record_offset":260,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":260,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":260,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2290,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":260,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2290,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":260,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2290,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":260,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":2290,"record_offset":14756,"parent_record_offset":14728,"ancestor_record_offset":260,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":2290,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":260,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":2290,"record_offset":3556,"parent_record_offset":3468,"ancestor_record_offset":260,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":2290,"record_offset":3916,"parent_record_offset":3876,"ancestor_record_offset":260,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2290,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":260,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2290,"record_offset":3988,"parent_record_offset":3916,"ancestor_record_offset":260,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2290,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":260,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2290,"record_offset":4088,"parent_record_offset":3876,"ancestor_record_offset":260,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2290,"record_offset":8604,"parent_record_offset":8552,"ancestor_record_offset":260,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2290,"record_offset":8552,"parent_record_offset":8504,"ancestor_record_offset":260,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2290,"record_offset":8504,"parent_record_offset":8476,"ancestor_record_offset":260,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":5588,"parent_record_offset":5520,"ancestor_record_offset":260,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":10968,"parent_record_offset":10932,"ancestor_record_offset":260,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":10932,"parent_record_offset":10904,"ancestor_record_offset":260,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":260,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2290,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":260,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2290,"record_offset":5200,"parent_record_offset":5016,"ancestor_record_offset":260,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2290,"record_offset":14664,"parent_record_offset":14576,"ancestor_record_offset":260,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2290,"record_offset":14576,"parent_record_offset":14552,"ancestor_record_offset":260,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2290,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":260,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2290,"record_offset":1912,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2290,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":260,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2290,"record_offset":2004,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2290,"record_offset":14820,"parent_record_offset":14784,"ancestor_record_offset":260,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2290,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":260,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02290.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16C8E
// 首条位置: {"module":2290,"inlinee":93326}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>", "bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle
// 信息 ID: bc814cee665da8f799865d50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3ff715fea51d70dcafa6eb5f: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> > @ 0xde7aab -> 0x1a41ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>)> > > @ 0x1a4652d -> 0x1a41ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_rectangle::construct_timberframe_rectangle::closure$0
// 信息 ID: 32b1167815378ea4f8f42b6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "timberframe::timberframe_rectangle::construct_timberframe_rectangle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2290, inline_record=2932; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2290, inline_record=4460; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2290, inline_record=6948; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2290, inline_record=9852; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_rectangle::construct_timberframe_rectangle; module=2290, inline_record=12116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
