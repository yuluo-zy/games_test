// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_sheep.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a10c2701c13d38e71916ba6d6598e09865531bb6cffc3612f2d30878b27dc7e7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_sheep.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 537, 1610
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_sheep::startup_sheep
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":95324,"rva":10523616,"va":5379232736}
// 编译实例 RVAs: 0xa093e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a680
//     owner=unresolved_namespace; count=1
//     target_aliases=24 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9177207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e889f773ff","size":5,"soft_fail":false,"target_va":5370062464,"va":5379239666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4555828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8cc7bbaff","size":5,"soft_fail":false,"target_va":5374679664,"va":5379235487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4152406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8aaa3c0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379233889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4156130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e81e95c0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379237629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e81be4e7ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379237648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088bf70
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1568287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8e111e8ff","size":5,"soft_fail":false,"target_va":5377671024,"va":5379239306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088fc00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1551069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e82355e8ff","size":5,"soft_fail":false,"target_va":5377686528,"va":5379237592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t318731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e80bdd0400","size":5,"soft_fail":false,"target_va":5379552880,"va":5379234144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0eeb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1071566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8ce591000","size":5,"soft_fail":false,"target_va":5380304560,"va":5379232989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1272190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e87e691300","size":5,"soft_fail":false,"target_va":5380505152,"va":5379232957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9d180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2701126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e846372900","size":5,"soft_fail":false,"target_va":5381935488,"va":5379234357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412bb7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9114062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8ce118b00","size":5,"soft_fail":false,"target_va":5388351472,"va":5379237405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130efe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9457681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e811509000","size":5,"soft_fail":false,"target_va":5388693472,"va":5379235786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32610025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8e996f101","size":5,"soft_fail":false,"target_va":5411847168,"va":5379237138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32611323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8fb9bf101","size":5,"soft_fail":false,"target_va":5411847200,"va":5379235872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32303158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e836e8ec01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379237829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1398280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e808561500","size":5,"soft_fail":false,"target_va":5380635504,"va":5379237219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32303040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8c0e7ec01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379237918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3416508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8bc213400","size":5,"soft_fail":false,"target_va":5382655024,"va":5379238511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9274614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8f6848d00","size":5,"soft_fail":false,"target_va":5388509584,"va":5379234965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32304099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8e3ebec01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379237672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t32303995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e87bebec01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379238296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32304984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e858efec01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379237891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-900855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e80941f2ff","size":5,"soft_fail":false,"target_va":5378332016,"va":5379232866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-859295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e861e3f2ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5379233594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t306185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e809ac0400","size":5,"soft_fail":false,"target_va":5379540032,"va":5379233842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::critters::BirdSpotManager::create_spots
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5753109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e815c95700","size":5,"soft_fail":false,"target_va":5384987456,"va":5379234342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1120018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e812171100","size":5,"soft_fail":false,"target_va":5380353184,"va":5379233161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_sheep::startup_sheep::panic_cold_explicit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t32103298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e882dbe901","size":5,"soft_fail":false,"target_va":5411341088,"va":5379237785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::debug::sheep_controls::SheepControl::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13529557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e8d571ce00","size":5,"soft_fail":false,"target_va":5392764576,"va":5379235014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2775564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379232736,"raw_hex":"e80c5a2a00","size":5,"soft_fail":false,"target_va":5382009728,"va":5379234159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":96708,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96860,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":98268,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":103648,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":98544,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":537,"record_offset":96620,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":95840,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":537,"record_offset":98296,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":95804,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":103900,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":537,"record_offset":95576,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":537,"record_offset":95520,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":537,"record_offset":95936,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":27239,"source_line":321,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::BirdSpotManager::gen_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":537,"record_offset":96676,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":30863,"source_line":316,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":537,"record_offset":97124,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":537,"record_offset":96944,"parent_record_offset":95324,"ancestor_record_offset":95324,"inlinee":28174,"source_line":66,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":96352,"parent_record_offset":96328,"ancestor_record_offset":95324,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":96376,"parent_record_offset":96352,"ancestor_record_offset":95324,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":104088,"parent_record_offset":104064,"ancestor_record_offset":95324,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":96328,"parent_record_offset":96300,"ancestor_record_offset":95324,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":104064,"parent_record_offset":103972,"ancestor_record_offset":95324,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":103724,"parent_record_offset":103692,"ancestor_record_offset":95324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":96240,"parent_record_offset":96212,"ancestor_record_offset":95324,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":104000,"parent_record_offset":103972,"ancestor_record_offset":95324,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":103972,"parent_record_offset":103948,"ancestor_record_offset":95324,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":103772,"parent_record_offset":103748,"ancestor_record_offset":95324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":103748,"parent_record_offset":103724,"ancestor_record_offset":95324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":96300,"parent_record_offset":96268,"ancestor_record_offset":95324,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":96268,"parent_record_offset":96240,"ancestor_record_offset":95324,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":103948,"parent_record_offset":103924,"ancestor_record_offset":95324,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96176,"parent_record_offset":96148,"ancestor_record_offset":95324,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96884,"parent_record_offset":96860,"ancestor_record_offset":95324,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96148,"parent_record_offset":96120,"ancestor_record_offset":95324,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96092,"parent_record_offset":96064,"ancestor_record_offset":95324,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96120,"parent_record_offset":96092,"ancestor_record_offset":95324,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":96064,"parent_record_offset":96036,"ancestor_record_offset":95324,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":537,"record_offset":96036,"parent_record_offset":95936,"ancestor_record_offset":95324,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":537,"record_offset":95544,"parent_record_offset":95520,"ancestor_record_offset":95324,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":103692,"parent_record_offset":103648,"ancestor_record_offset":95324,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":96908,"parent_record_offset":96884,"ancestor_record_offset":95324,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":96212,"parent_record_offset":96176,"ancestor_record_offset":95324,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":98320,"parent_record_offset":98296,"ancestor_record_offset":95324,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":537,"record_offset":96508,"parent_record_offset":95936,"ancestor_record_offset":95324,"inlinee":27241,"source_line":647,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":537,"record_offset":96644,"parent_record_offset":96620,"ancestor_record_offset":95324,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":95896,"parent_record_offset":95868,"ancestor_record_offset":95324,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":97916,"parent_record_offset":97888,"ancestor_record_offset":95324,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":537,"record_offset":97416,"parent_record_offset":97392,"ancestor_record_offset":95324,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":537,"record_offset":97500,"parent_record_offset":97476,"ancestor_record_offset":95324,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":537,"record_offset":96452,"parent_record_offset":96428,"ancestor_record_offset":95324,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":537,"record_offset":98092,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":98480,"parent_record_offset":98412,"ancestor_record_offset":95324,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":95868,"parent_record_offset":95840,"ancestor_record_offset":95324,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":537,"record_offset":98344,"parent_record_offset":98320,"ancestor_record_offset":95324,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":98504,"parent_record_offset":98480,"ancestor_record_offset":95324,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":104032,"parent_record_offset":104000,"ancestor_record_offset":95324,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":537,"record_offset":97648,"parent_record_offset":97620,"ancestor_record_offset":95324,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":537,"record_offset":97620,"parent_record_offset":97592,"ancestor_record_offset":95324,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":95980,"parent_record_offset":95936,"ancestor_record_offset":95324,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":537,"record_offset":96428,"parent_record_offset":96176,"ancestor_record_offset":95324,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":95620,"parent_record_offset":95576,"ancestor_record_offset":95324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":97768,"parent_record_offset":97744,"ancestor_record_offset":95324,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":96816,"parent_record_offset":96788,"ancestor_record_offset":95324,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":103812,"parent_record_offset":103648,"ancestor_record_offset":95324,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":537,"record_offset":98000,"parent_record_offset":97972,"ancestor_record_offset":95324,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":537,"record_offset":98028,"parent_record_offset":98000,"ancestor_record_offset":95324,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":537,"record_offset":98572,"parent_record_offset":98544,"ancestor_record_offset":95324,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":537,"record_offset":98600,"parent_record_offset":98572,"ancestor_record_offset":95324,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":537,"record_offset":97944,"parent_record_offset":97916,"ancestor_record_offset":95324,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":537,"record_offset":97560,"parent_record_offset":97532,"ancestor_record_offset":95324,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":537,"record_offset":97592,"parent_record_offset":97560,"ancestor_record_offset":95324,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":537,"record_offset":97532,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":537,"record_offset":97392,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":537,"record_offset":98120,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":537,"record_offset":97476,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":537,"record_offset":97364,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":537,"record_offset":97888,"parent_record_offset":97860,"ancestor_record_offset":95324,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":537,"record_offset":97148,"parent_record_offset":97124,"ancestor_record_offset":95324,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":537,"record_offset":97172,"parent_record_offset":97148,"ancestor_record_offset":95324,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":537,"record_offset":97860,"parent_record_offset":97692,"ancestor_record_offset":95324,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":537,"record_offset":97204,"parent_record_offset":97172,"ancestor_record_offset":95324,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97800,"parent_record_offset":97716,"ancestor_record_offset":95324,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97744,"parent_record_offset":97716,"ancestor_record_offset":95324,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":98212,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97716,"parent_record_offset":97692,"ancestor_record_offset":95324,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97256,"parent_record_offset":97232,"ancestor_record_offset":95324,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97232,"parent_record_offset":97172,"ancestor_record_offset":95324,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97280,"parent_record_offset":97256,"ancestor_record_offset":95324,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97336,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":537,"record_offset":97692,"parent_record_offset":97280,"ancestor_record_offset":95324,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_sheep::startup_sheep::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12078
// 首条位置: {"module":1610,"inlinee":73848}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1610,"record_offset":42452,"parent_record_offset":42416,"ancestor_record_offset":42416,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":42536,"parent_record_offset":42416,"ancestor_record_offset":42416,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1610,"record_offset":42500,"parent_record_offset":42476,"ancestor_record_offset":42416,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":42476,"parent_record_offset":42452,"ancestor_record_offset":42416,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":42576,"parent_record_offset":42536,"ancestor_record_offset":42416,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_sheep.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_sheep.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_sheep.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_sheep::startup_sheep
// 信息 ID: 7d0f677909ddf499513cad81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c043fcb3193ae162da223cca: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)> > > @ 0x14bc0e5 -> 0xa093e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)> > @ 0x17e1768 -> 0xa093e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_sheep::startup_sheep::closure$0
// 信息 ID: 4a5b693016c0700a81d9ce35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_sheep::startup_sheep", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::mesh::Mesh,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; module=1610, inline_record=42416; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; module=1610, inline_record=42808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
