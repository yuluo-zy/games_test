// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/clear_canvas.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 80265764512d418d92b4b4b5675af5b8b652f79b0eff21f785e940a2c8684e1c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clear_canvas.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 554
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::clear_canvas::clear_canvas
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":25252,"rva":10988496,"va":5379697616}
// 编译实例 RVAs: 0xa7abd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2946182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e886f42c00","size":5,"soft_fail":false,"target_va":5382644528,"va":5379698341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ff40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10666053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8bb3f5dff","size":5,"soft_fail":false,"target_va":5369036608,"va":5379702656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10663375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8314a5dff","size":5,"soft_fail":false,"target_va":5369039104,"va":5379702474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10097652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e80cec65ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379698447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d6640
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4872418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e81ea7b5ff","size":5,"soft_fail":false,"target_va":5374830144,"va":5379702557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060aae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4654521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e847fab8ff","size":5,"soft_fail":false,"target_va":5375044320,"va":5379698836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060cfb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4645734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e89a1cb9ff","size":5,"soft_fail":false,"target_va":5375053744,"va":5379699473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4618320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8b087b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379699803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4618443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e83587b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379699942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4182144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8802fc0ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5379698171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2927746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e87e53d3ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379703949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088acf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2033316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e85cf9e0ff","size":5,"soft_fail":false,"target_va":5377666288,"va":5379699599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408950d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1991502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8b29ce1ff","size":5,"soft_fail":false,"target_va":5377708240,"va":5379699737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1121778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e80ee2eeff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379699981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15da0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-414765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8d3abf9ff","size":5,"soft_fail":false,"target_va":5379284384,"va":5379699144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaae80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t195489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8a1fb0200","size":5,"soft_fail":false,"target_va":5379894912,"va":5379699418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b79700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1039546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8badc0f00","size":5,"soft_fail":false,"target_va":5380740864,"va":5379701313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1522051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e883391700","size":5,"soft_fail":false,"target_va":5381222304,"va":5379700248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f3e20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8882559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e87f898700","size":5,"soft_fail":false,"target_va":5388582432,"va":5379699868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32147096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e89886ea01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379700051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32148285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e83d8bea01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379698878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: clear_canvas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32148201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8e98aea01","size":5,"soft_fail":false,"target_va":5411847200,"va":5379698994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31839270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e826d4e501","size":5,"soft_fail":false,"target_va":5411540963,"va":5379701688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31840146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e892d7e501","size":5,"soft_fail":false,"target_va":5411541776,"va":5379701625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31780866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e802f0e401","size":5,"soft_fail":false,"target_va":5411482433,"va":5379701562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31840968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8c8dae501","size":5,"soft_fail":false,"target_va":5411542880,"va":5379701907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::clear_preset_texture
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1307352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8280decff","size":5,"soft_fail":false,"target_va":5378391776,"va":5379699123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2230874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8a6f5ddff","size":5,"soft_fail":false,"target_va":5377468208,"va":5379699077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t171609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e8599e0200","size":5,"soft_fail":false,"target_va":5379871056,"va":5379699442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2959500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e88c282d00","size":5,"soft_fail":false,"target_va":5382657856,"va":5379698351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3055406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e82e9f2e00","size":5,"soft_fail":false,"target_va":5382753792,"va":5379698381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31835438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379697616,"raw_hex":"e82ec5e501","size":5,"soft_fail":false,"target_va":5411537104,"va":5379701661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379697616,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379700071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":33292,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":37924,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":30052,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":554,"record_offset":26584,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":554,"record_offset":25484,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":26948,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":33228,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":32736,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":38088,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":38008,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":554,"record_offset":33000,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":554,"record_offset":30816,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":554,"record_offset":33636,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::GateColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":554,"record_offset":33920,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":31646,"source_line":147,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::HalftimberColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":554,"record_offset":35740,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":31648,"source_line":216,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorIds::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":554,"record_offset":32096,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":31644,"source_line":79,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":554,"record_offset":31920,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":29237,"source_line":987,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":554,"record_offset":37632,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":28870,"source_line":120,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/curved_halftimber.rs
//     evidence={"module":554,"record_offset":31716,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":31642,"source_line":14,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":554,"record_offset":27156,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":12906,"source_line":44,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::reset_max_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":554,"record_offset":30788,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":26527,"source_line":94,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::clear_canvas::clear_terrain_gpu
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/clear_canvas.rs
//     evidence={"module":554,"record_offset":37832,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":31650,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":554,"record_offset":26980,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":26864,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":30232,"parent_record_offset":25252,"ancestor_record_offset":25252,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":32356,"parent_record_offset":32332,"ancestor_record_offset":25252,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":32380,"parent_record_offset":32356,"ancestor_record_offset":25252,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":32620,"parent_record_offset":32596,"ancestor_record_offset":25252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":32332,"parent_record_offset":32304,"ancestor_record_offset":25252,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":32596,"parent_record_offset":32504,"ancestor_record_offset":25252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":25864,"parent_record_offset":25840,"ancestor_record_offset":25252,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32244,"parent_record_offset":32216,"ancestor_record_offset":25252,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32532,"parent_record_offset":32504,"ancestor_record_offset":25252,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32504,"parent_record_offset":32480,"ancestor_record_offset":25252,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":25912,"parent_record_offset":25888,"ancestor_record_offset":25252,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":25888,"parent_record_offset":25864,"ancestor_record_offset":25252,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32304,"parent_record_offset":32272,"ancestor_record_offset":25252,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32272,"parent_record_offset":32244,"ancestor_record_offset":25252,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":32480,"parent_record_offset":32456,"ancestor_record_offset":25252,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":30840,"parent_record_offset":30816,"ancestor_record_offset":25252,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":37972,"parent_record_offset":37948,"ancestor_record_offset":25252,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":25840,"parent_record_offset":25812,"ancestor_record_offset":25252,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":25812,"parent_record_offset":25784,"ancestor_record_offset":25252,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":33572,"parent_record_offset":33320,"ancestor_record_offset":25252,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":32216,"parent_record_offset":32168,"ancestor_record_offset":25252,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":32132,"parent_record_offset":32096,"ancestor_record_offset":25252,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":25784,"parent_record_offset":25644,"ancestor_record_offset":25252,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":37948,"parent_record_offset":37924,"ancestor_record_offset":25252,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":554,"record_offset":32168,"parent_record_offset":32132,"ancestor_record_offset":25252,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":30080,"parent_record_offset":30052,"ancestor_record_offset":25252,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":554,"record_offset":25596,"parent_record_offset":25532,"ancestor_record_offset":25252,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":554,"record_offset":25644,"parent_record_offset":25596,"ancestor_record_offset":25252,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":554,"record_offset":26628,"parent_record_offset":26584,"ancestor_record_offset":25252,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":554,"record_offset":25532,"parent_record_offset":25484,"ancestor_record_offset":25252,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":27740,"parent_record_offset":27708,"ancestor_record_offset":25252,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":27800,"parent_record_offset":27768,"ancestor_record_offset":25252,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":28224,"parent_record_offset":28168,"ancestor_record_offset":25252,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":33056,"parent_record_offset":33024,"ancestor_record_offset":25252,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":30572,"parent_record_offset":30548,"ancestor_record_offset":25252,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":554,"record_offset":26676,"parent_record_offset":26628,"ancestor_record_offset":25252,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":554,"record_offset":27596,"parent_record_offset":27572,"ancestor_record_offset":25252,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":26232,"parent_record_offset":26200,"ancestor_record_offset":25252,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":25728,"parent_record_offset":25644,"ancestor_record_offset":25252,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":32564,"parent_record_offset":32532,"ancestor_record_offset":25252,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":27768,"parent_record_offset":27708,"ancestor_record_offset":25252,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":554,"record_offset":27344,"parent_record_offset":27308,"ancestor_record_offset":25252,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":26724,"parent_record_offset":26676,"ancestor_record_offset":25252,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":26796,"parent_record_offset":26676,"ancestor_record_offset":25252,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":27460,"parent_record_offset":27432,"ancestor_record_offset":25252,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":26828,"parent_record_offset":26628,"ancestor_record_offset":25252,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":554,"record_offset":27036,"parent_record_offset":27004,"ancestor_record_offset":25252,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":554,"record_offset":27064,"parent_record_offset":27004,"ancestor_record_offset":25252,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":28140,"parent_record_offset":28112,"ancestor_record_offset":25252,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":29824,"parent_record_offset":27156,"ancestor_record_offset":25252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":26476,"parent_record_offset":26452,"ancestor_record_offset":25252,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":27548,"parent_record_offset":27432,"ancestor_record_offset":25252,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":30548,"parent_record_offset":30524,"ancestor_record_offset":25252,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":38052,"parent_record_offset":38008,"ancestor_record_offset":25252,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":27624,"parent_record_offset":27596,"ancestor_record_offset":25252,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":25252,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":27652,"parent_record_offset":27596,"ancestor_record_offset":25252,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":33708,"parent_record_offset":33668,"ancestor_record_offset":25252,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":554,"record_offset":33140,"parent_record_offset":33112,"ancestor_record_offset":25252,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":554,"record_offset":33168,"parent_record_offset":33140,"ancestor_record_offset":25252,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":26136,"parent_record_offset":25644,"ancestor_record_offset":25252,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":26424,"parent_record_offset":25644,"ancestor_record_offset":25252,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":554,"record_offset":26200,"parent_record_offset":26168,"ancestor_record_offset":25252,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":554,"record_offset":26168,"parent_record_offset":26136,"ancestor_record_offset":25252,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":554,"record_offset":26452,"parent_record_offset":26424,"ancestor_record_offset":25252,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":554,"record_offset":33084,"parent_record_offset":33056,"ancestor_record_offset":25252,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":26888,"parent_record_offset":26864,"ancestor_record_offset":25252,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":25252,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":554,"record_offset":33024,"parent_record_offset":33000,"ancestor_record_offset":25252,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":554,"record_offset":33668,"parent_record_offset":33636,"ancestor_record_offset":25252,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":554,"record_offset":31944,"parent_record_offset":31920,"ancestor_record_offset":25252,"inlinee":29239,"source_line":43,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":554,"record_offset":30524,"parent_record_offset":30464,"ancestor_record_offset":25252,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":554,"record_offset":30256,"parent_record_offset":30232,"ancestor_record_offset":25252,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":554,"record_offset":28356,"parent_record_offset":27996,"ancestor_record_offset":25252,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":554,"record_offset":27996,"parent_record_offset":27968,"ancestor_record_offset":25252,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30280,"parent_record_offset":30256,"ancestor_record_offset":25252,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30380,"parent_record_offset":30356,"ancestor_record_offset":25252,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30404,"parent_record_offset":30380,"ancestor_record_offset":25252,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30464,"parent_record_offset":30404,"ancestor_record_offset":25252,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30436,"parent_record_offset":30404,"ancestor_record_offset":25252,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30496,"parent_record_offset":30464,"ancestor_record_offset":25252,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30612,"parent_record_offset":30404,"ancestor_record_offset":25252,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":25252,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":30356,"parent_record_offset":30332,"ancestor_record_offset":25252,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":554,"record_offset":30332,"parent_record_offset":30308,"ancestor_record_offset":25252,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":554,"record_offset":27968,"parent_record_offset":27940,"ancestor_record_offset":25252,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":27092,"parent_record_offset":27004,"ancestor_record_offset":25252,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":554,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":25252,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":27116,"parent_record_offset":27092,"ancestor_record_offset":25252,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":31744,"parent_record_offset":31716,"ancestor_record_offset":25252,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":27908,"parent_record_offset":27212,"ancestor_record_offset":25252,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":27212,"parent_record_offset":27156,"ancestor_record_offset":25252,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":554,"record_offset":27940,"parent_record_offset":27156,"ancestor_record_offset":25252,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":27260,"parent_record_offset":27236,"ancestor_record_offset":25252,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":554,"record_offset":27508,"parent_record_offset":27484,"ancestor_record_offset":25252,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":554,"record_offset":27372,"parent_record_offset":27344,"ancestor_record_offset":25252,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":27708,"parent_record_offset":27308,"ancestor_record_offset":25252,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":25252,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":554,"record_offset":27400,"parent_record_offset":27372,"ancestor_record_offset":25252,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":554,"record_offset":27432,"parent_record_offset":27400,"ancestor_record_offset":25252,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":554,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":25252,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":554,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":25252,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::clear_canvas::clear_terrain_gpu
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BA2
// 首条位置: {"module":554,"inlinee":31650}
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
// 完整结构及原始类型记录: clear_canvas.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clear_canvas.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clear_canvas.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>

// ------------------------------------------------------------------
// 函数: country_core::systems::clear_canvas::clear_canvas
// 信息 ID: 2e2f49ff7719689c0614cf44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9d7340f4641af7902ac9445e: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>)> > @ 0x3761a8 -> 0xa7abd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>)> > > @ 0x45f8c0 -> 0xa7abd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::clear_canvas::clear_terrain_gpu
// 信息 ID: d0f9e757d4d8bc9019e54186
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::clear_canvas::clear_canvas; module=554, inline_record=37832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
