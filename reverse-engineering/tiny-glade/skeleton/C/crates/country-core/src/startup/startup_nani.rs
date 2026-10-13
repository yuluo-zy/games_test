// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/startup_nani.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d2bff805886d203652386993e4440f07bed32290f1edbe6c11423b48249ff21d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_nani.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 526
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::update_world_subset_material_args
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":38060,"rva":10424096,"va":5379133216}
// 编译实例 RVAs: 0x9f0f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3458544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8103acbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379137243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f50
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9364440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8281c71ff","size":5,"soft_fail":false,"target_va":5369773904,"va":5379138339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098ee20
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-405653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86bcff9ff","size":5,"soft_fail":false,"target_va":5378731552,"va":5379137200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2442219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8eb432500","size":5,"soft_fail":false,"target_va":5381577744,"va":5379135520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0470
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4189764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e844ee3f00","size":5,"soft_fail":false,"target_va":5383324784,"va":5379135015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141558130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11953251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86364b600","size":5,"soft_fail":false,"target_va":5391089968,"va":5379136712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141560a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11990122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86af4b600","size":5,"soft_fail":false,"target_va":5391125008,"va":5379134881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141565170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12009275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e83b3fb700","size":5,"soft_fail":false,"target_va":5391143280,"va":5379134000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415683c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12021962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8ca70b700","size":5,"soft_fail":false,"target_va":5391156160,"va":5379134193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141569d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12024795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8db7bb700","size":5,"soft_fail":false,"target_va":5391162624,"va":5379137824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14156dc70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12044356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e844c8b700","size":5,"soft_fail":false,"target_va":5391178864,"va":5379134503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14156f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12049081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8b9dab700","size":5,"soft_fail":false,"target_va":5391185184,"va":5379136098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141570e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12057879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e817fdb700","size":5,"soft_fail":false,"target_va":5391191616,"va":5379133732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141572720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12061804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86c0cb800","size":5,"soft_fail":false,"target_va":5391197984,"va":5379136175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141573fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12067109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e82521b800","size":5,"soft_fail":false,"target_va":5391204304,"va":5379137190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157e480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12111732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e874cfb800","size":5,"soft_fail":false,"target_va":5391246464,"va":5379134727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415803f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12120081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e811f0b800","size":5,"soft_fail":false,"target_va":5391254512,"va":5379134426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141581d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12124073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8a9ffb800","size":5,"soft_fail":false,"target_va":5391260944,"va":5379136866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415835c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12132455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86720b900","size":5,"soft_fail":false,"target_va":5391267264,"va":5379134804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12151358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e83e6ab900","size":5,"soft_fail":false,"target_va":5391287808,"va":5379136445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141590850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12186893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e80df5b900","size":5,"soft_fail":false,"target_va":5391321168,"va":5379134270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141592100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12192289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8210aba00","size":5,"soft_fail":false,"target_va":5391327488,"va":5379135194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415939e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12197062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8c61cba00","size":5,"soft_fail":false,"target_va":5391333856,"va":5379136789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415983e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12215291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8fb63ba00","size":5,"soft_fail":false,"target_va":5391352800,"va":5379137504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ae4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12307349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e895cbbb00","size":5,"soft_fail":false,"target_va":5391443184,"va":5379135830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b3610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12328005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8451cbc00","size":5,"soft_fail":false,"target_va":5391463952,"va":5379135942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b4ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12336427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e82b3dbc00","size":5,"soft_fail":false,"target_va":5391470320,"va":5379133888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b6f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12343040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e80057bc00","size":5,"soft_fail":false,"target_va":5391478544,"va":5379135499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b87f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12349281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8616fbc00","size":5,"soft_fail":false,"target_va":5391484912,"va":5379135626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415bf960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12380327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8a7e8bc00","size":5,"soft_fail":false,"target_va":5391513952,"va":5379133620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415c3350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12392411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8db17bd00","size":5,"soft_fail":false,"target_va":5391528784,"va":5379136368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415c6520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12406144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8804dbd00","size":5,"soft_fail":false,"target_va":5391541536,"va":5379135387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415cad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12425348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e88498bd00","size":5,"soft_fail":false,"target_va":5391559968,"va":5379134615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415cfdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12444948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e814e5bd00","size":5,"soft_fail":false,"target_va":5391580656,"va":5379135703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e4140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12526426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e85a23bf00","size":5,"soft_fail":false,"target_va":5391663424,"va":5379136993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37483704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8b8f43b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379137891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37483627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e86bf43b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5379138000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=34
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t145498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e85a380200","size":5,"soft_fail":false,"target_va":5379278928,"va":5379133425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t143173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e8452f0200","size":5,"soft_fail":false,"target_va":5379280704,"va":5379137526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass2::load_grass2_lod0_materials
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t647952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e810e30900","size":5,"soft_fail":false,"target_va":5379784848,"va":5379136891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4350743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e817634200","size":5,"soft_fail":false,"target_va":5383487696,"va":5379136948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=34
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4191318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379133216,"raw_hex":"e856f43f00","size":5,"soft_fail":false,"target_va":5383324736,"va":5379133413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*8(%r8)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379133216,"raw_hex":"41ff5008","size":4,"soft_fail":false,"va":5379133320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":44372,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_bindless_handle_by_name_hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":526,"record_offset":39492,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":30266,"source_line":429,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=36
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":526,"record_offset":38560,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":41216,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":31059,"source_line":301,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::uber_materials
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":40680,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":31058,"source_line":187,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::update_grass2_subset_material_args
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":41684,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":31061,"source_line":557,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=28
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":526,"record_offset":38784,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$39::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":526,"record_offset":38244,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":29808,"source_line":78,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":38060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":38060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42320,"parent_record_offset":42296,"ancestor_record_offset":38060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42248,"parent_record_offset":42160,"ancestor_record_offset":38060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42188,"parent_record_offset":42160,"ancestor_record_offset":38060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42160,"parent_record_offset":42136,"ancestor_record_offset":38060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42136,"parent_record_offset":42112,"ancestor_record_offset":38060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":40220,"parent_record_offset":40196,"ancestor_record_offset":38060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":39728,"parent_record_offset":39704,"ancestor_record_offset":38060,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":39812,"parent_record_offset":39788,"ancestor_record_offset":38060,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":40260,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":526,"record_offset":41792,"parent_record_offset":41768,"ancestor_record_offset":38060,"inlinee":9288,"source_line":372,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":42216,"parent_record_offset":42188,"ancestor_record_offset":38060,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":526,"record_offset":38396,"parent_record_offset":38368,"ancestor_record_offset":38060,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":39960,"parent_record_offset":39932,"ancestor_record_offset":38060,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":39932,"parent_record_offset":39904,"ancestor_record_offset":38060,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":40768,"parent_record_offset":40680,"ancestor_record_offset":38060,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":39752,"parent_record_offset":39728,"ancestor_record_offset":38060,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":42088,"parent_record_offset":41684,"ancestor_record_offset":38060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":38060,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":38368,"parent_record_offset":38344,"ancestor_record_offset":38060,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":38424,"parent_record_offset":38368,"ancestor_record_offset":38060,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":38448,"parent_record_offset":38424,"ancestor_record_offset":38060,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":526,"record_offset":38320,"parent_record_offset":38296,"ancestor_record_offset":38060,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":526,"record_offset":38344,"parent_record_offset":38320,"ancestor_record_offset":38060,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":41864,"parent_record_offset":41824,"ancestor_record_offset":38060,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":38496,"parent_record_offset":38268,"ancestor_record_offset":38060,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":38520,"parent_record_offset":38496,"ancestor_record_offset":38060,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":40708,"parent_record_offset":40680,"ancestor_record_offset":38060,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":40732,"parent_record_offset":40708,"ancestor_record_offset":38060,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":41244,"parent_record_offset":41216,"ancestor_record_offset":38060,"inlinee":31060,"source_line":109,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_nani::update_grass2_subset_material_args::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":41824,"parent_record_offset":41684,"ancestor_record_offset":38060,"inlinee":31063,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":39872,"parent_record_offset":39844,"ancestor_record_offset":38060,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":39904,"parent_record_offset":39872,"ancestor_record_offset":38060,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":39844,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":39704,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":40288,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":39788,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":40196,"parent_record_offset":40172,"ancestor_record_offset":38060,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":39548,"parent_record_offset":39524,"ancestor_record_offset":38060,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":40172,"parent_record_offset":40004,"ancestor_record_offset":38060,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":40112,"parent_record_offset":40028,"ancestor_record_offset":38060,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":40056,"parent_record_offset":40028,"ancestor_record_offset":38060,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":40380,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":40028,"parent_record_offset":40004,"ancestor_record_offset":38060,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":39600,"parent_record_offset":39576,"ancestor_record_offset":38060,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":39576,"parent_record_offset":39548,"ancestor_record_offset":38060,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":39624,"parent_record_offset":39600,"ancestor_record_offset":38060,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":39676,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":40004,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":526,"record_offset":42056,"parent_record_offset":41684,"ancestor_record_offset":38060,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/bytes.rs
//     evidence={"module":526,"record_offset":41768,"parent_record_offset":41684,"ancestor_record_offset":38060,"inlinee":9287,"source_line":8,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":526,"record_offset":38296,"parent_record_offset":38268,"ancestor_record_offset":38060,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":526,"record_offset":38268,"parent_record_offset":38244,"ancestor_record_offset":38060,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":39524,"parent_record_offset":39492,"ancestor_record_offset":38060,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::startup_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":18560,"rva":10416976,"va":5379126096}
// 编译实例 RVAs: 0x9ef350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-329245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379126096,"raw_hex":"e8e3f9faff","size":5,"soft_fail":false,"target_va":5378797920,"va":5379127160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14432012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379126096,"raw_hex":"e80c37dc00","size":5,"soft_fail":false,"target_va":5393558928,"va":5379126911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37930189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379126096,"raw_hex":"e8cdc44202","size":5,"soft_fail":false,"target_va":5417057056,"va":5379126862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37495027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379126096,"raw_hex":"e8f3203c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379127208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%rax,%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379126096,"raw_hex":"42ff1410","size":4,"soft_fail":false,"va":5379127109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":20672,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":526,"record_offset":19320,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::assign_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":19376,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":30892,"source_line":741,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":20616,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":19588,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::runtime_check_limits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":18720,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":31052,"source_line":72,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":526,"record_offset":19532,"parent_record_offset":18560,"ancestor_record_offset":18560,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":526,"record_offset":19344,"parent_record_offset":19320,"ancestor_record_offset":18560,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":18560,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":526,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":18560,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":20316,"parent_record_offset":20292,"ancestor_record_offset":18560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":19884,"parent_record_offset":19860,"ancestor_record_offset":18560,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":19968,"parent_record_offset":19944,"ancestor_record_offset":18560,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":20384,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::is_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":19172,"parent_record_offset":19028,"ancestor_record_offset":18560,"inlinee":9486,"source_line":3742,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":20116,"parent_record_offset":20088,"ancestor_record_offset":18560,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":18560,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":20556,"parent_record_offset":19588,"ancestor_record_offset":18560,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":19556,"parent_record_offset":19532,"ancestor_record_offset":18560,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":18560,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":19472,"parent_record_offset":19376,"ancestor_record_offset":18560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":20232,"parent_record_offset":20208,"ancestor_record_offset":18560,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":18884,"parent_record_offset":18748,"ancestor_record_offset":18560,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::assert_valid_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":19028,"parent_record_offset":18720,"ancestor_record_offset":18560,"inlinee":29629,"source_line":77,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":18748,"parent_record_offset":18720,"ancestor_record_offset":18560,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":18560,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":18560,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":18560,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":20060,"parent_record_offset":20028,"ancestor_record_offset":18560,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":20000,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":19860,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":20412,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":19944,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":19804,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":18560,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":19640,"parent_record_offset":19616,"ancestor_record_offset":18560,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":20268,"parent_record_offset":20160,"ancestor_record_offset":18560,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":19672,"parent_record_offset":19640,"ancestor_record_offset":18560,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":18560,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":20504,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":18560,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":18560,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":19700,"parent_record_offset":19640,"ancestor_record_offset":18560,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":18560,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":19832,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":20160,"parent_record_offset":19748,"ancestor_record_offset":18560,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":19416,"parent_record_offset":19376,"ancestor_record_offset":18560,"inlinee":28135,"source_line":520,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":18560,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::runtime_check_limits
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794C
// 首条位置: {"module":526,"inlinee":31052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::nani::capacity::assert_valid_capacity
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":19028,"parent_record_offset":18720,"ancestor_record_offset":18720,"inlinee":29629,"source_line":77,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":18748,"parent_record_offset":18720,"ancestor_record_offset":18720,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::is_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":19172,"parent_record_offset":19028,"ancestor_record_offset":18720,"inlinee":9486,"source_line":3742,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":18884,"parent_record_offset":18748,"ancestor_record_offset":18720,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::nani_on_scene_load
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":20732,"rva":10418272,"va":5379127392}
// 编译实例 RVAs: 0x9ef860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=2
//     target_aliases=49 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6664394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8364f9aff","size":5,"soft_fail":false,"target_va":5372465520,"va":5379129909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-529628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e824ebf7ff","size":5,"soft_fail":false,"target_va":5378599808,"va":5379129431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-382396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8442afaff","size":5,"soft_fail":false,"target_va":5378747616,"va":5379130007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12853026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8221fc400","size":5,"soft_fail":false,"target_va":5391982352,"va":5379129321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14430376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8a830dc00","size":5,"soft_fail":false,"target_va":5393558928,"va":5379128547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37223383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8d7fb3702","size":5,"soft_fail":false,"target_va":5416353040,"va":5379129652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37491778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e842143c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379129817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37492532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e834173c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379129703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t392986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e81aff0500","size":5,"soft_fail":false,"target_va":5379521760,"va":5379128769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-61815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8890effff","size":5,"soft_fail":false,"target_va":5379067728,"va":5379129538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_nani::update_world_subset_material_args
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e80d100000","size":5,"soft_fail":false,"target_va":5379133216,"va":5379129102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37443224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e898563b02","size":5,"soft_fail":false,"target_va":5416572816,"va":5379129587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37445329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8d15e3b02","size":5,"soft_fail":false,"target_va":5416574944,"va":5379129610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4856017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e8d1184a00","size":5,"soft_fail":false,"target_va":5383984848,"va":5379128826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4855901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e85d184a00","size":5,"soft_fail":false,"target_va":5383985360,"va":5379129454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4573988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379127392,"raw_hex":"e824cb4500","size":5,"soft_fail":false,"target_va":5383702992,"va":5379128999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":23836,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":23588,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":526,"record_offset":21504,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::get_subset_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":22676,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":30894,"source_line":618,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":22648,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":30836,"source_line":233,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":21616,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::create_world_subset_drawable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":23072,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":31053,"source_line":118,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::runtime_check_limits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":20904,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":31052,"source_line":72,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":526,"record_offset":21560,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":20732,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23516,"parent_record_offset":23492,"ancestor_record_offset":20732,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23492,"parent_record_offset":23460,"ancestor_record_offset":20732,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":20732,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":526,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":20732,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":23404,"parent_record_offset":23368,"ancestor_record_offset":20732,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":23860,"parent_record_offset":23836,"ancestor_record_offset":20732,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23336,"parent_record_offset":23308,"ancestor_record_offset":20732,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23368,"parent_record_offset":23336,"ancestor_record_offset":20732,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23280,"parent_record_offset":23072,"ancestor_record_offset":20732,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23308,"parent_record_offset":23280,"ancestor_record_offset":20732,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":22348,"parent_record_offset":22324,"ancestor_record_offset":20732,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":20732,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":22000,"parent_record_offset":21976,"ancestor_record_offset":20732,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":22416,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::is_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":21356,"parent_record_offset":21212,"ancestor_record_offset":20732,"inlinee":9486,"source_line":3742,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":22148,"parent_record_offset":22120,"ancestor_record_offset":20732,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":20732,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":22996,"parent_record_offset":22968,"ancestor_record_offset":20732,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":23156,"parent_record_offset":23072,"ancestor_record_offset":20732,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":23024,"parent_record_offset":22968,"ancestor_record_offset":20732,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":22588,"parent_record_offset":21616,"ancestor_record_offset":20732,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":20732,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":20732,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":20732,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":23768,"parent_record_offset":23644,"ancestor_record_offset":20732,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":22780,"parent_record_offset":22744,"ancestor_record_offset":20732,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":23792,"parent_record_offset":23768,"ancestor_record_offset":20732,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":22804,"parent_record_offset":22780,"ancestor_record_offset":20732,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":21068,"parent_record_offset":20932,"ancestor_record_offset":20732,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::assert_valid_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":21212,"parent_record_offset":20904,"ancestor_record_offset":20732,"inlinee":29629,"source_line":77,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":20932,"parent_record_offset":20904,"ancestor_record_offset":20732,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":23220,"parent_record_offset":23072,"ancestor_record_offset":20732,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$10::get_subset_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":22968,"parent_record_offset":22932,"ancestor_record_offset":20732,"inlinee":30841,"source_line":622,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":22372,"parent_record_offset":22348,"ancestor_record_offset":20732,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":20732,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":22060,"parent_record_offset":22032,"ancestor_record_offset":20732,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":22092,"parent_record_offset":22060,"ancestor_record_offset":20732,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":22032,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":21892,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":22444,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":21976,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":21836,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":20732,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":21672,"parent_record_offset":21648,"ancestor_record_offset":20732,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":22300,"parent_record_offset":22192,"ancestor_record_offset":20732,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":20732,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":22240,"parent_record_offset":22216,"ancestor_record_offset":20732,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":22536,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":22216,"parent_record_offset":22192,"ancestor_record_offset":20732,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":21756,"parent_record_offset":21732,"ancestor_record_offset":20732,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":21732,"parent_record_offset":21672,"ancestor_record_offset":20732,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":21780,"parent_record_offset":21756,"ancestor_record_offset":20732,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":21864,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":22192,"parent_record_offset":21780,"ancestor_record_offset":20732,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":22716,"parent_record_offset":22676,"ancestor_record_offset":20732,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLockWriteGuard::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":22932,"parent_record_offset":22676,"ancestor_record_offset":20732,"inlinee":14425,"source_line":1619,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":23616,"parent_record_offset":23588,"ancestor_record_offset":20732,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":22836,"parent_record_offset":22744,"ancestor_record_offset":20732,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":23680,"parent_record_offset":23644,"ancestor_record_offset":20732,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":22744,"parent_record_offset":22716,"ancestor_record_offset":20732,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":23644,"parent_record_offset":23616,"ancestor_record_offset":20732,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":22864,"parent_record_offset":22836,"ancestor_record_offset":20732,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":20732,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":21648,"parent_record_offset":21616,"ancestor_record_offset":20732,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::create_world_subset_drawable
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794D
// 首条位置: {"module":526,"inlinee":31053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23280,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":23156,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":23184,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":23220,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":23252,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":23072,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23516,"parent_record_offset":23492,"ancestor_record_offset":23072,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23492,"parent_record_offset":23460,"ancestor_record_offset":23072,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":23072,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":23404,"parent_record_offset":23368,"ancestor_record_offset":23072,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23336,"parent_record_offset":23308,"ancestor_record_offset":23072,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23368,"parent_record_offset":23336,"ancestor_record_offset":23072,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":526,"record_offset":23308,"parent_record_offset":23280,"ancestor_record_offset":23072,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::uber_materials
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7952
// 首条位置: {"module":526,"inlinee":31058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":40768,"parent_record_offset":40680,"ancestor_record_offset":40680,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":40708,"parent_record_offset":40680,"ancestor_record_offset":40680,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":40732,"parent_record_offset":40708,"ancestor_record_offset":40680,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::update_grass2_subset_material_args
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 557
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7955
// 首条位置: {"module":526,"inlinee":31061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":42088,"parent_record_offset":41684,"ancestor_record_offset":41684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::update_grass2_subset_material_args::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":41824,"parent_record_offset":41684,"ancestor_record_offset":41684,"inlinee":31063,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":526,"record_offset":42056,"parent_record_offset":41684,"ancestor_record_offset":41684,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/bytes.rs
//     evidence={"module":526,"record_offset":41768,"parent_record_offset":41684,"ancestor_record_offset":41684,"inlinee":9287,"source_line":8,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":41684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":41684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42320,"parent_record_offset":42296,"ancestor_record_offset":41684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":42248,"parent_record_offset":42160,"ancestor_record_offset":41684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42188,"parent_record_offset":42160,"ancestor_record_offset":41684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42160,"parent_record_offset":42136,"ancestor_record_offset":41684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":42136,"parent_record_offset":42112,"ancestor_record_offset":41684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":526,"record_offset":41792,"parent_record_offset":41768,"ancestor_record_offset":41684,"inlinee":9288,"source_line":372,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":42216,"parent_record_offset":42188,"ancestor_record_offset":41684,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":42112,"parent_record_offset":42088,"ancestor_record_offset":41684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":41864,"parent_record_offset":41824,"ancestor_record_offset":41684,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::update_grass2_subset_material_args::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 572
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7957
// 首条位置: {"module":526,"inlinee":31063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":41864,"parent_record_offset":41824,"ancestor_record_offset":41824,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::fine_sorted_draw_fn
// 记录形式: compiled_procedure; instances=1
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":23896,"rva":10421264,"va":5379130384}
// 编译实例 RVAs: 0x9f0410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4332827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e81b1d4200","size":5,"soft_fail":false,"target_va":5383464240,"va":5379131408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5652197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e81bc1a9ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5379131232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141535fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11819147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e88b58b400","size":5,"soft_fail":false,"target_va":5390950304,"va":5379131152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141539720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11833339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8fb8fb400","size":5,"soft_fail":false,"target_va":5390964512,"va":5379131168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11889793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8816cb500","size":5,"soft_fail":false,"target_va":5391021408,"va":5379131610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11892156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8bc75b500","size":5,"soft_fail":false,"target_va":5391023760,"va":5379131599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37490592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8a00f3c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379131003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37490505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8490f3c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5379131698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37490044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e87c0d3c02","size":5,"soft_fail":false,"target_va":5416621762,"va":5379131713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37489953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8210d3c02","size":5,"soft_fail":false,"target_va":5416621632,"va":5379131674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4388823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8d7f74200","size":5,"soft_fail":false,"target_va":5383520416,"va":5379131588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4390126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8eefc4200","size":5,"soft_fail":false,"target_va":5383521616,"va":5379131485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t16438762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379130384,"raw_hex":"e8ead5fa00","size":5,"soft_fail":false,"target_va":5395570160,"va":5379131393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":24060,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":28524,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":28580,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":28828,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::list_index_to_cmd_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":28700,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":30905,"source_line":822,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::draw_list_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":28616,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":9325,"source_line":399,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$23::try_from_primitive
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":28552,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":30868,"source_line":48,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":25372,"parent_record_offset":25348,"ancestor_record_offset":23896,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":25420,"parent_record_offset":25396,"ancestor_record_offset":23896,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":25396,"parent_record_offset":25372,"ancestor_record_offset":23896,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":25348,"parent_record_offset":25324,"ancestor_record_offset":23896,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":25324,"parent_record_offset":25284,"ancestor_record_offset":23896,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":28728,"parent_record_offset":28700,"ancestor_record_offset":23896,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":25284,"parent_record_offset":25260,"ancestor_record_offset":23896,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":526,"record_offset":24088,"parent_record_offset":24060,"ancestor_record_offset":23896,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":23896,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":25620,"parent_record_offset":25596,"ancestor_record_offset":23896,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":25568,"parent_record_offset":25536,"ancestor_record_offset":23896,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":25512,"parent_record_offset":25004,"ancestor_record_offset":23896,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":23896,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":25032,"parent_record_offset":25004,"ancestor_record_offset":23896,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":25596,"parent_record_offset":25536,"ancestor_record_offset":23896,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":25080,"parent_record_offset":25056,"ancestor_record_offset":23896,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":25056,"parent_record_offset":25032,"ancestor_record_offset":23896,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":25260,"parent_record_offset":25236,"ancestor_record_offset":23896,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":25004,"parent_record_offset":24060,"ancestor_record_offset":23896,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":526,"record_offset":24112,"parent_record_offset":24088,"ancestor_record_offset":23896,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":526,"record_offset":24752,"parent_record_offset":24728,"ancestor_record_offset":23896,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":24780,"parent_record_offset":24752,"ancestor_record_offset":23896,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":23896,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":526,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":23896,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":24328,"parent_record_offset":24304,"ancestor_record_offset":23896,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":24412,"parent_record_offset":24388,"ancestor_record_offset":23896,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":29856,"parent_record_offset":29192,"ancestor_record_offset":23896,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":30536,"parent_record_offset":30476,"ancestor_record_offset":23896,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":24560,"parent_record_offset":24532,"ancestor_record_offset":23896,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":24532,"parent_record_offset":24504,"ancestor_record_offset":23896,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":526,"record_offset":25172,"parent_record_offset":25108,"ancestor_record_offset":23896,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":25196,"parent_record_offset":25172,"ancestor_record_offset":23896,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":25236,"parent_record_offset":25056,"ancestor_record_offset":23896,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":30392,"parent_record_offset":28964,"ancestor_record_offset":23896,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":24352,"parent_record_offset":24328,"ancestor_record_offset":23896,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":24672,"parent_record_offset":24648,"ancestor_record_offset":23896,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":25108,"parent_record_offset":25080,"ancestor_record_offset":23896,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":28756,"parent_record_offset":28728,"ancestor_record_offset":23896,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":25144,"parent_record_offset":25108,"ancestor_record_offset":23896,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":25464,"parent_record_offset":25284,"ancestor_record_offset":23896,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":28784,"parent_record_offset":28756,"ancestor_record_offset":23896,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":29808,"parent_record_offset":29784,"ancestor_record_offset":23896,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":23896,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":28904,"parent_record_offset":28880,"ancestor_record_offset":23896,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":23896,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":28880,"parent_record_offset":28828,"ancestor_record_offset":23896,"inlinee":30229,"source_line":593,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":30572,"parent_record_offset":28828,"ancestor_record_offset":23896,"inlinee":30234,"source_line":596,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":526,"record_offset":29024,"parent_record_offset":28964,"ancestor_record_offset":23896,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":30444,"parent_record_offset":30420,"ancestor_record_offset":23896,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30420,"parent_record_offset":28964,"ancestor_record_offset":23896,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_draws_per_list
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30476,"parent_record_offset":28964,"ancestor_record_offset":23896,"inlinee":29625,"source_line":383,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30504,"parent_record_offset":30476,"ancestor_record_offset":23896,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":23896,"inlinee":29773,"source_line":142,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":526,"record_offset":30032,"parent_record_offset":28964,"ancestor_record_offset":23896,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_nani::fine_sorted_draw_fn::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":28964,"parent_record_offset":28828,"ancestor_record_offset":23896,"inlinee":31055,"source_line":621,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":526,"record_offset":24728,"parent_record_offset":24708,"ancestor_record_offset":23896,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":24472,"parent_record_offset":24444,"ancestor_record_offset":23896,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":24504,"parent_record_offset":24472,"ancestor_record_offset":23896,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":24444,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":24304,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":24856,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":24388,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":29248,"parent_record_offset":29192,"ancestor_record_offset":23896,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":23896,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":24148,"parent_record_offset":24112,"ancestor_record_offset":23896,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":24708,"parent_record_offset":24604,"ancestor_record_offset":23896,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":29112,"parent_record_offset":29080,"ancestor_record_offset":23896,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24648,"parent_record_offset":24624,"ancestor_record_offset":23896,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24948,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24624,"parent_record_offset":24604,"ancestor_record_offset":23896,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":23896,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":23896,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":23896,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24276,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":24604,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":526,"record_offset":24828,"parent_record_offset":24224,"ancestor_record_offset":23896,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":526,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":23896,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30232,"parent_record_offset":30032,"ancestor_record_offset":23896,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30072,"parent_record_offset":30032,"ancestor_record_offset":23896,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":30156,"parent_record_offset":30128,"ancestor_record_offset":23896,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30260,"parent_record_offset":30232,"ancestor_record_offset":23896,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30100,"parent_record_offset":30072,"ancestor_record_offset":23896,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":30184,"parent_record_offset":30156,"ancestor_record_offset":23896,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":526,"record_offset":30128,"parent_record_offset":30100,"ancestor_record_offset":23896,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":29052,"parent_record_offset":29024,"ancestor_record_offset":23896,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::fine_sorted_draw_fn::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794F
// 首条位置: {"module":526,"inlinee":31055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":30392,"parent_record_offset":28964,"ancestor_record_offset":28964,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":526,"record_offset":29024,"parent_record_offset":28964,"ancestor_record_offset":28964,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::subset_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30420,"parent_record_offset":28964,"ancestor_record_offset":28964,"inlinee":30896,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_draws_per_list
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30476,"parent_record_offset":28964,"ancestor_record_offset":28964,"inlinee":29625,"source_line":383,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":526,"record_offset":30032,"parent_record_offset":28964,"ancestor_record_offset":28964,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":28964,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":29328,"parent_record_offset":29304,"ancestor_record_offset":28964,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":29412,"parent_record_offset":29388,"ancestor_record_offset":28964,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":29856,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":30536,"parent_record_offset":30476,"ancestor_record_offset":28964,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":29560,"parent_record_offset":29532,"ancestor_record_offset":28964,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":29532,"parent_record_offset":29504,"ancestor_record_offset":28964,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":29352,"parent_record_offset":29328,"ancestor_record_offset":28964,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":28964,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":29808,"parent_record_offset":29784,"ancestor_record_offset":28964,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":28964,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":526,"record_offset":30444,"parent_record_offset":30420,"ancestor_record_offset":28964,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":30504,"parent_record_offset":30476,"ancestor_record_offset":28964,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":29472,"parent_record_offset":29444,"ancestor_record_offset":28964,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":29504,"parent_record_offset":29472,"ancestor_record_offset":28964,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":29444,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":29304,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":29884,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":29388,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":29248,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":28964,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":29080,"parent_record_offset":29052,"ancestor_record_offset":28964,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":29712,"parent_record_offset":29604,"ancestor_record_offset":28964,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":29112,"parent_record_offset":29080,"ancestor_record_offset":28964,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":28964,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29976,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":28964,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29168,"parent_record_offset":29140,"ancestor_record_offset":28964,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29140,"parent_record_offset":29080,"ancestor_record_offset":28964,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29192,"parent_record_offset":29168,"ancestor_record_offset":28964,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29276,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":29604,"parent_record_offset":29192,"ancestor_record_offset":28964,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30232,"parent_record_offset":30032,"ancestor_record_offset":28964,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30072,"parent_record_offset":30032,"ancestor_record_offset":28964,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":30156,"parent_record_offset":30128,"ancestor_record_offset":28964,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30260,"parent_record_offset":30232,"ancestor_record_offset":28964,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":30100,"parent_record_offset":30072,"ancestor_record_offset":28964,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":30184,"parent_record_offset":30156,"ancestor_record_offset":28964,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":526,"record_offset":30128,"parent_record_offset":30100,"ancestor_record_offset":28964,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":29052,"parent_record_offset":29024,"ancestor_record_offset":28964,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::coarse_sorted_draw_fn
// 记录形式: compiled_procedure; instances=1
// 源码行: 639
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":30720,"rva":10422608,"va":5379131728}
// 编译实例 RVAs: 0x9f0950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..node_builder..draw_stream..IntoIndexBufferRef$GT$::into_index_buffer_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4331329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e841174200","size":5,"soft_fail":false,"target_va":5383464240,"va":5379132906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395120
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6667746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e81e429aff","size":5,"soft_fail":false,"target_va":5372465440,"va":5379133181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5653706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e836bba9ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5379132741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141539720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11831809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8018ab400","size":5,"soft_fail":false,"target_va":5390964512,"va":5379132698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11888397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e80d67b500","size":5,"soft_fail":false,"target_va":5391021408,"va":5379133006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t37489241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8590a3c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379132354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37489078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8b6093c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5379133125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37488617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8e9073c02","size":5,"soft_fail":false,"target_va":5416621762,"va":5379133140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37488559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8af073c02","size":5,"soft_fail":false,"target_va":5416621632,"va":5379133068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37437261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e84d3f3b02","size":5,"soft_fail":false,"target_va":5416569744,"va":5379132478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37439043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e843463b02","size":5,"soft_fail":false,"target_va":5416572144,"va":5379133096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4852320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8600a4a00","size":5,"soft_fail":false,"target_va":5383984848,"va":5379132523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4852760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e8180c4a00","size":5,"soft_fail":false,"target_va":5383985360,"va":5379132595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4386664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e868ef4200","size":5,"soft_fail":false,"target_va":5383519664,"va":5379132995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4388640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e820f74200","size":5,"soft_fail":false,"target_va":5383521616,"va":5379132971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t16437264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379131728,"raw_hex":"e810d0fa00","size":5,"soft_fail":false,"target_va":5395570160,"va":5379132891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":30900,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":35368,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":36068,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":35424,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":36528,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::get_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":35460,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":30890,"source_line":609,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::list_index_to_cmd_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":36400,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":30905,"source_line":822,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubset::atlas_mesh_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":36040,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":30848,"source_line":863,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::draw_list_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":36316,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":9325,"source_line":399,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$23::try_from_primitive
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":35396,"parent_record_offset":30720,"ancestor_record_offset":30720,"inlinee":30868,"source_line":48,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":32216,"parent_record_offset":32192,"ancestor_record_offset":30720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":32264,"parent_record_offset":32240,"ancestor_record_offset":30720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":32240,"parent_record_offset":32216,"ancestor_record_offset":30720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":32192,"parent_record_offset":32168,"ancestor_record_offset":30720,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":32168,"parent_record_offset":32128,"ancestor_record_offset":30720,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":36428,"parent_record_offset":36400,"ancestor_record_offset":30720,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":32128,"parent_record_offset":32104,"ancestor_record_offset":30720,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":526,"record_offset":30928,"parent_record_offset":30900,"ancestor_record_offset":30720,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":32488,"parent_record_offset":32464,"ancestor_record_offset":30720,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":30720,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":32412,"parent_record_offset":32380,"ancestor_record_offset":30720,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":32356,"parent_record_offset":31848,"ancestor_record_offset":30720,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":32380,"parent_record_offset":32356,"ancestor_record_offset":30720,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":31876,"parent_record_offset":31848,"ancestor_record_offset":30720,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":32440,"parent_record_offset":32380,"ancestor_record_offset":30720,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":31924,"parent_record_offset":31900,"ancestor_record_offset":30720,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":31900,"parent_record_offset":31876,"ancestor_record_offset":30720,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":32104,"parent_record_offset":32080,"ancestor_record_offset":30720,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":31848,"parent_record_offset":30900,"ancestor_record_offset":30720,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":526,"record_offset":30952,"parent_record_offset":30928,"ancestor_record_offset":30720,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":526,"record_offset":31596,"parent_record_offset":31572,"ancestor_record_offset":30720,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":31624,"parent_record_offset":31596,"ancestor_record_offset":30720,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":37364,"parent_record_offset":37340,"ancestor_record_offset":30720,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":526,"record_offset":36340,"parent_record_offset":36316,"ancestor_record_offset":30720,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":30720,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":31252,"parent_record_offset":31228,"ancestor_record_offset":30720,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":37460,"parent_record_offset":36796,"ancestor_record_offset":30720,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":35656,"parent_record_offset":35564,"ancestor_record_offset":30720,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":31400,"parent_record_offset":31372,"ancestor_record_offset":30720,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":31372,"parent_record_offset":31344,"ancestor_record_offset":30720,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":526,"record_offset":32016,"parent_record_offset":31952,"ancestor_record_offset":30720,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":32040,"parent_record_offset":32016,"ancestor_record_offset":30720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":32080,"parent_record_offset":31900,"ancestor_record_offset":30720,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":35936,"parent_record_offset":35880,"ancestor_record_offset":30720,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":31192,"parent_record_offset":31168,"ancestor_record_offset":30720,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":30720,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":31952,"parent_record_offset":31924,"ancestor_record_offset":30720,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":36456,"parent_record_offset":36428,"ancestor_record_offset":30720,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":31988,"parent_record_offset":31952,"ancestor_record_offset":30720,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":32308,"parent_record_offset":32128,"ancestor_record_offset":30720,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":36484,"parent_record_offset":36456,"ancestor_record_offset":30720,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":35684,"parent_record_offset":35564,"ancestor_record_offset":30720,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":36248,"parent_record_offset":36124,"ancestor_record_offset":30720,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":35600,"parent_record_offset":35564,"ancestor_record_offset":30720,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":35708,"parent_record_offset":35684,"ancestor_record_offset":30720,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":35624,"parent_record_offset":35600,"ancestor_record_offset":30720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":30720,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":30720,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":37388,"parent_record_offset":37364,"ancestor_record_offset":30720,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":36568,"parent_record_offset":36528,"ancestor_record_offset":30720,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":38024,"parent_record_offset":36528,"ancestor_record_offset":30720,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":526,"record_offset":36644,"parent_record_offset":36596,"ancestor_record_offset":30720,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$10::get_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":35880,"parent_record_offset":35840,"ancestor_record_offset":30720,"inlinee":30839,"source_line":613,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":36364,"parent_record_offset":36340,"ancestor_record_offset":30720,"inlinee":29773,"source_line":142,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":526,"record_offset":37636,"parent_record_offset":36596,"ancestor_record_offset":30720,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_nani::coarse_sorted_draw_fn::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":526,"record_offset":36596,"parent_record_offset":36528,"ancestor_record_offset":30720,"inlinee":31057,"source_line":659,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":526,"record_offset":31572,"parent_record_offset":31548,"ancestor_record_offset":30720,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":31312,"parent_record_offset":31284,"ancestor_record_offset":30720,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":31344,"parent_record_offset":31312,"ancestor_record_offset":30720,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":31284,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":31144,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":31700,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":31228,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":36852,"parent_record_offset":36796,"ancestor_record_offset":30720,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":37340,"parent_record_offset":37316,"ancestor_record_offset":30720,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":30988,"parent_record_offset":30952,"ancestor_record_offset":30720,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":31548,"parent_record_offset":31444,"ancestor_record_offset":30720,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":36720,"parent_record_offset":36692,"ancestor_record_offset":30720,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":30720,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31792,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31464,"parent_record_offset":31444,"ancestor_record_offset":30720,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":30720,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31016,"parent_record_offset":30988,"ancestor_record_offset":30720,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31064,"parent_record_offset":31040,"ancestor_record_offset":30720,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31116,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":31444,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":526,"record_offset":31672,"parent_record_offset":31064,"ancestor_record_offset":30720,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":35496,"parent_record_offset":35460,"ancestor_record_offset":30720,"inlinee":12306,"source_line":457,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":35840,"parent_record_offset":35460,"ancestor_record_offset":30720,"inlinee":14418,"source_line":1275,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":36096,"parent_record_offset":36068,"ancestor_record_offset":30720,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":526,"record_offset":33728,"parent_record_offset":33704,"ancestor_record_offset":30720,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":35744,"parent_record_offset":35524,"ancestor_record_offset":30720,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":36160,"parent_record_offset":36124,"ancestor_record_offset":30720,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":35564,"parent_record_offset":35524,"ancestor_record_offset":30720,"inlinee":12308,"source_line":508,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":35524,"parent_record_offset":35496,"ancestor_record_offset":30720,"inlinee":12307,"source_line":107,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":36124,"parent_record_offset":36096,"ancestor_record_offset":30720,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":35772,"parent_record_offset":35744,"ancestor_record_offset":30720,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":36188,"parent_record_offset":36160,"ancestor_record_offset":30720,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37832,"parent_record_offset":37636,"ancestor_record_offset":30720,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37672,"parent_record_offset":37636,"ancestor_record_offset":30720,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":37756,"parent_record_offset":37728,"ancestor_record_offset":30720,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37860,"parent_record_offset":37832,"ancestor_record_offset":30720,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37700,"parent_record_offset":37672,"ancestor_record_offset":30720,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":37784,"parent_record_offset":37756,"ancestor_record_offset":30720,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":526,"record_offset":37728,"parent_record_offset":37700,"ancestor_record_offset":30720,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":36668,"parent_record_offset":36644,"ancestor_record_offset":30720,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::coarse_sorted_draw_fn::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 659
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7951
// 首条位置: {"module":526,"inlinee":31057}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":37992,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":526,"record_offset":36644,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":30443,"source_line":38,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::vao::impl$1::bind_vao
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/vao.rs
//     evidence={"module":526,"record_offset":37636,"parent_record_offset":36596,"ancestor_record_offset":36596,"inlinee":29125,"source_line":32,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":37364,"parent_record_offset":37340,"ancestor_record_offset":36596,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":36932,"parent_record_offset":36908,"ancestor_record_offset":36596,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":36596,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":37460,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":37164,"parent_record_offset":37136,"ancestor_record_offset":36596,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":37136,"parent_record_offset":37108,"ancestor_record_offset":36596,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":36956,"parent_record_offset":36932,"ancestor_record_offset":36596,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":37280,"parent_record_offset":37256,"ancestor_record_offset":36596,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":36596,"inlinee":30447,"source_line":46,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":526,"record_offset":37388,"parent_record_offset":37364,"ancestor_record_offset":36596,"inlinee":30445,"source_line":111,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":37076,"parent_record_offset":37048,"ancestor_record_offset":36596,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":37108,"parent_record_offset":37076,"ancestor_record_offset":36596,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":37048,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":36908,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":37488,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":36992,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":36852,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":37340,"parent_record_offset":37316,"ancestor_record_offset":36596,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":36692,"parent_record_offset":36668,"ancestor_record_offset":36596,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":37316,"parent_record_offset":37208,"ancestor_record_offset":36596,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":36720,"parent_record_offset":36692,"ancestor_record_offset":36596,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":37256,"parent_record_offset":37232,"ancestor_record_offset":36596,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":37580,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":37232,"parent_record_offset":37208,"ancestor_record_offset":36596,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":36772,"parent_record_offset":36748,"ancestor_record_offset":36596,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":36748,"parent_record_offset":36692,"ancestor_record_offset":36596,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":36796,"parent_record_offset":36772,"ancestor_record_offset":36596,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":36880,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":37208,"parent_record_offset":36796,"ancestor_record_offset":36596,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::cull_mode
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37832,"parent_record_offset":37636,"ancestor_record_offset":36596,"inlinee":29084,"source_line":2108,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37672,"parent_record_offset":37636,"ancestor_record_offset":36596,"inlinee":10393,"source_line":1486,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawAtomBit::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":37756,"parent_record_offset":37728,"ancestor_record_offset":36596,"inlinee":10372,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::raster_opts_mut
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37860,"parent_record_offset":37832,"ancestor_record_offset":36596,"inlinee":29085,"source_line":562,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::set_index_buffer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
//     evidence={"module":526,"record_offset":37700,"parent_record_offset":37672,"ancestor_record_offset":36596,"inlinee":10391,"source_line":604,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::InternalBitFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":526,"record_offset":37784,"parent_record_offset":37756,"ancestor_record_offset":36596,"inlinee":10374,"source_line":701,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::draw_stream::packet::_::impl$43::bitor_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/public.rs
//     evidence={"module":526,"record_offset":37728,"parent_record_offset":37700,"ancestor_record_offset":36596,"inlinee":10370,"source_line":393,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":36668,"parent_record_offset":36644,"ancestor_record_offset":36596,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::reload_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 675
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":12824,"rva":10415744,"va":5379124864}
// 编译实例 RVAs: 0x9eee80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5519989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e8753a5400","size":5,"soft_fail":false,"target_va":5384644912,"va":5379124918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3447150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e89266cbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379125849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409774a0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-491175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e85981f8ff","size":5,"soft_fail":false,"target_va":5378634912,"va":5379126082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9f020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2818029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e8edff2a00","size":5,"soft_fail":false,"target_va":5381943328,"va":5379125294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca41e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2839053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e80d522b00","size":5,"soft_fail":false,"target_va":5381964256,"va":5379125198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14433202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e8b23bdc00","size":5,"soft_fail":false,"target_va":5393558928,"va":5379125721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d81710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37299893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e8b5263902","size":5,"soft_fail":false,"target_va":5416425232,"va":5379125334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37496418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e862263c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379125817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-57916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e8c41dffff","size":5,"soft_fail":false,"target_va":5379067728,"va":5379125639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5600802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379124864,"raw_hex":"e822765500","size":5,"soft_fail":false,"target_va":5384726688,"va":5379125881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":17388,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":526,"record_offset":15272,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":526,"record_offset":13716,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":18044,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":526,"record_offset":17444,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":16340,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":15576,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":14428,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":12128,"source_line":349,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":13004,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":14836,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":15788,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":17500,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$5::clone_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":15672,"parent_record_offset":12824,"ancestor_record_offset":12824,"inlinee":27573,"source_line":1101,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":12824,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":18216,"parent_record_offset":18192,"ancestor_record_offset":12824,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":18240,"parent_record_offset":18216,"ancestor_record_offset":12824,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":526,"record_offset":18168,"parent_record_offset":18140,"ancestor_record_offset":12824,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":526,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":12824,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":14072,"parent_record_offset":14048,"ancestor_record_offset":12824,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":14048,"parent_record_offset":13804,"ancestor_record_offset":12824,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":17412,"parent_record_offset":17388,"ancestor_record_offset":12824,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":526,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":12824,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":526,"record_offset":15300,"parent_record_offset":15272,"ancestor_record_offset":12824,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":526,"record_offset":13780,"parent_record_offset":13740,"ancestor_record_offset":12824,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":526,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":12824,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":526,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12824,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":526,"record_offset":13472,"parent_record_offset":13268,"ancestor_record_offset":12824,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":526,"record_offset":15748,"parent_record_offset":15720,"ancestor_record_offset":12824,"inlinee":14807,"source_line":245,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":526,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":12824,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":14720,"parent_record_offset":14688,"ancestor_record_offset":12824,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":12824,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":12824,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":17072,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":15000,"parent_record_offset":14968,"ancestor_record_offset":12824,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":526,"record_offset":15368,"parent_record_offset":15340,"ancestor_record_offset":12824,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":526,"record_offset":15340,"parent_record_offset":15300,"ancestor_record_offset":12824,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":526,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":12824,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":526,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":12824,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":526,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":12824,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":526,"record_offset":15036,"parent_record_offset":14900,"ancestor_record_offset":12824,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":526,"record_offset":13436,"parent_record_offset":13412,"ancestor_record_offset":12824,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":16228,"parent_record_offset":16200,"ancestor_record_offset":12824,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":14000,"parent_record_offset":13968,"ancestor_record_offset":12824,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":13848,"parent_record_offset":13804,"ancestor_record_offset":12824,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":526,"record_offset":13596,"parent_record_offset":13544,"ancestor_record_offset":12824,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":16804,"parent_record_offset":16776,"ancestor_record_offset":12824,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":12824,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":526,"record_offset":13180,"parent_record_offset":13144,"ancestor_record_offset":12824,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":12824,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":15424,"parent_record_offset":15340,"ancestor_record_offset":12824,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":13300,"parent_record_offset":13268,"ancestor_record_offset":12824,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":526,"record_offset":16084,"parent_record_offset":15996,"ancestor_record_offset":12824,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":12824,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":12824,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":12824,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":13388,"parent_record_offset":13268,"ancestor_record_offset":12824,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":16288,"parent_record_offset":16156,"ancestor_record_offset":12824,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":526,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":12824,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":17992,"parent_record_offset":17932,"ancestor_record_offset":12824,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":13412,"parent_record_offset":13388,"ancestor_record_offset":12824,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":13904,"parent_record_offset":13804,"ancestor_record_offset":12824,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":14272,"parent_record_offset":13804,"ancestor_record_offset":12824,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":13968,"parent_record_offset":13936,"ancestor_record_offset":12824,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":13936,"parent_record_offset":13904,"ancestor_record_offset":12824,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":526,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":12824,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":526,"record_offset":15500,"parent_record_offset":15448,"ancestor_record_offset":12824,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":526,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":12824,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":526,"record_offset":17352,"parent_record_offset":17328,"ancestor_record_offset":12824,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":16716,"parent_record_offset":16688,"ancestor_record_offset":12824,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":16748,"parent_record_offset":16716,"ancestor_record_offset":12824,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":16688,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":16548,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":14688,"parent_record_offset":14660,"ancestor_record_offset":12824,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":17100,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":14748,"parent_record_offset":14688,"ancestor_record_offset":12824,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":16632,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":526,"record_offset":17224,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":526,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":12824,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":16396,"parent_record_offset":16368,"ancestor_record_offset":12824,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":15632,"parent_record_offset":15604,"ancestor_record_offset":12824,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":14512,"parent_record_offset":14484,"ancestor_record_offset":12824,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":14484,"parent_record_offset":14456,"ancestor_record_offset":12824,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":14940,"parent_record_offset":14900,"ancestor_record_offset":12824,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":16956,"parent_record_offset":16848,"ancestor_record_offset":12824,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":15720,"parent_record_offset":15696,"ancestor_record_offset":12824,"inlinee":27459,"source_line":198,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":15860,"parent_record_offset":15824,"ancestor_record_offset":12824,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":12824,"inlinee":8411,"source_line":3234,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":12824,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":12824,"inlinee":27449,"source_line":3377,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":14900,"parent_record_offset":14868,"ancestor_record_offset":12824,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":17292,"parent_record_offset":16396,"ancestor_record_offset":12824,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17932,"parent_record_offset":17596,"ancestor_record_offset":12824,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16896,"parent_record_offset":16872,"ancestor_record_offset":12824,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17844,"parent_record_offset":17628,"ancestor_record_offset":12824,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17192,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14660,"parent_record_offset":14568,"ancestor_record_offset":12824,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17628,"parent_record_offset":17596,"ancestor_record_offset":12824,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16872,"parent_record_offset":16848,"ancestor_record_offset":12824,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16460,"parent_record_offset":16432,"ancestor_record_offset":12824,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16432,"parent_record_offset":16396,"ancestor_record_offset":12824,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16156,"parent_record_offset":16124,"ancestor_record_offset":12824,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16124,"parent_record_offset":15896,"ancestor_record_offset":12824,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14540,"parent_record_offset":14512,"ancestor_record_offset":12824,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14968,"parent_record_offset":14940,"ancestor_record_offset":12824,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14632,"parent_record_offset":14568,"ancestor_record_offset":12824,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14604,"parent_record_offset":14568,"ancestor_record_offset":12824,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":12824,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":14568,"parent_record_offset":14540,"ancestor_record_offset":12824,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17252,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":526,"record_offset":16200,"parent_record_offset":16156,"ancestor_record_offset":12824,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":15896,"parent_record_offset":15860,"ancestor_record_offset":12824,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":12824,"inlinee":8412,"source_line":3872,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17596,"parent_record_offset":17572,"ancestor_record_offset":12824,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":12824,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":17572,"parent_record_offset":17548,"ancestor_record_offset":12824,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":16848,"parent_record_offset":16488,"ancestor_record_offset":12824,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":15604,"parent_record_offset":15576,"ancestor_record_offset":12824,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":12824,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$0::clone_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":12824,"inlinee":27574,"source_line":125,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":14868,"parent_record_offset":14836,"ancestor_record_offset":12824,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":15824,"parent_record_offset":15788,"ancestor_record_offset":12824,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":12824,"inlinee":27448,"source_line":1806,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":526,"record_offset":17524,"parent_record_offset":17500,"ancestor_record_offset":12824,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":526,"record_offset":16368,"parent_record_offset":16340,"ancestor_record_offset":12824,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":12824,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":526,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":12824,"inlinee":27447,"source_line":1688,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":526,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":12824,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":526,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":12824,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":526,"record_offset":13208,"parent_record_offset":13180,"ancestor_record_offset":12824,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":526,"record_offset":13544,"parent_record_offset":13144,"ancestor_record_offset":12824,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":526,"record_offset":13060,"parent_record_offset":13032,"ancestor_record_offset":12824,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":526,"record_offset":13236,"parent_record_offset":13208,"ancestor_record_offset":12824,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":526,"record_offset":13268,"parent_record_offset":13236,"ancestor_record_offset":12824,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":526,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":12824,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":526,"record_offset":13116,"parent_record_offset":13088,"ancestor_record_offset":12824,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00526.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_nani.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_nani.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_nani.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::coarse_sorted_draw_fn
// 信息 ID: 653acc7cd24f4a10aed636bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::coarse_sorted_draw_fn::closure$0
// 信息 ID: 3fa2f0bd0b1c0fdfa54fa658
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_nani::coarse_sorted_draw_fn", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::coarse_sorted_draw_fn; module=526, inline_record=36596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::create_world_subset_drawable
// 信息 ID: 2c3f157f6445fe9a9eac53a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::nani_on_scene_load; module=526, inline_record=23072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::fine_sorted_draw_fn
// 信息 ID: b78c2f343cec2b6d7f78cb13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::fine_sorted_draw_fn::closure$0
// 信息 ID: 3797e61b420aebfd07e8e60e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_nani::fine_sorted_draw_fn", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::fine_sorted_draw_fn; module=526, inline_record=28964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::nani_on_scene_load
// 信息 ID: 244d68ba70ca2763d2394859
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 435a15dd5d2afd57c5a19680: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> @ 0x116c555 -> 0x9ef860 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> > @ 0x19538f3 -> 0x9ef860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::reload_nani
// 信息 ID: 730be0cb0e28bac9bd5ad9d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eb546082af568fe5a3edd684: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> @ 0x116bffd -> 0x9eee80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> > @ 0x190e75c -> 0x9eee80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::runtime_check_limits
// 信息 ID: 0be678d5a4300d5ce683c693
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=526, inline_record=18720; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::nani_on_scene_load; module=526, inline_record=20904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::startup_nani
// 信息 ID: 9be2990f6841062ad38acb63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6a721ad15de85068ac6429fb: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)> @ 0x116bcff -> 0x9ef350 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)> > @ 0x192724c -> 0x9ef350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::uber_materials
// 信息 ID: 0ab52af58c814854be2e633e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::update_world_subset_material_args; module=526, inline_record=40680; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::update_world_subset_material_args; module=526, inline_record=40872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::update_grass2_subset_material_args
// 信息 ID: fb1f4e841b96a721bf76a654
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::update_world_subset_material_args; module=526, inline_record=41684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::update_grass2_subset_material_args::closure$0
// 信息 ID: 551ec71b7d939c20119d28f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_nani::update_grass2_subset_material_args", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::update_world_subset_material_args; module=526, inline_record=41824; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::update_world_subset_material_args; module=526, inline_record=41968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::update_world_subset_material_args
// 信息 ID: e6d8e6716b43b198359c5e4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_nani::nani_on_scene_load @ 0x9eff0e -> 0x9f0f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,core::slice::iter::Iter<tuple$<bevy_ecs::entity::Entity,country_core::render::nani::gpu_world::GpuWorldSubsetKind,rhapsody::raw::binding_set::BindingSet> >,country_core::render::nani::maintenance::refresh_material_args::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<bevy_ecs::entity::Entity,country_core::render::nani::gpu_world::GpuWorldSubsetKind,rhapsody::raw::binding_set::BindingSet> >,country_core::render::nani::maintenance::refresh_material_args::closure_env$0> > > > @ 0x1811a42 -> 0x9f0f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
