// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/eraser/wall_eraser.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: de2a562598365d9c7cf3986cf1c25c7468412b4416d20cd9fa6b937452295122
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_eraser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 566, 1585, 1610
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":566,"record_offset":57604,"rva":11324080,"va":5380033200}
// 编译实例 RVAs: 0xaccab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11011910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8baf857ff","size":5,"soft_fail":false,"target_va":5369022192,"va":5380034097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004d980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11007123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e86d0b58ff","size":5,"soft_fail":false,"target_va":5369026944,"va":5380034062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10442832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8b0a760ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5380043627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8409071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e811b07fff","size":5,"soft_fail":false,"target_va":5371634416,"va":5380043482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=6
//     target_aliases=7 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5359548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e84438aeff","size":5,"soft_fail":false,"target_va":5374676336,"va":5380035879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=9
//     target_aliases=12 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5230912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8c02eb0ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5380044331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d6640
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5219684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e89c5ab0ff","size":5,"soft_fail":false,"target_va":5374830144,"va":5380049823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5029716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8ac40b3ff","size":5,"soft_fail":false,"target_va":5375019664,"va":5380049375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4955367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e81963b4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380036850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4952531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e82d6eb4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380034030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2440874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e856c1daff","size":5,"soft_fail":false,"target_va":5377610304,"va":5380051173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881b90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2420984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8080fdbff","size":5,"soft_fail":false,"target_va":5377629072,"va":5380050051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140886d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2400612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e89c5edbff","size":5,"soft_fail":false,"target_va":5377649984,"va":5380050591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-940444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e864a6f1ff","size":5,"soft_fail":false,"target_va":5379102272,"va":5380042711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-150597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8bbb3fdff","size":5,"soft_fail":false,"target_va":5379888656,"va":5380039248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e88faefdff","size":5,"soft_fail":false,"target_va":5379890112,"va":5380042028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-147859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e86dbefdff","size":5,"soft_fail":false,"target_va":5379891344,"va":5380039198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1841402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8fa181c00","size":5,"soft_fail":false,"target_va":5381883040,"va":5380041633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1889026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e802d31c00","size":5,"soft_fail":false,"target_va":5381934560,"va":5380045529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141302160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8600568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8f83b8300","size":5,"soft_fail":false,"target_va":5388640608,"va":5380040035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130b130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8642065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e811de8300","size":5,"soft_fail":false,"target_va":5388677424,"va":5380035354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413e5360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9527003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8db5e9100","size":5,"soft_fail":false,"target_va":5389570912,"va":5380043904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413e7230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9540372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e814939100","size":5,"soft_fail":false,"target_va":5389578800,"va":5380038423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10060135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e867819900","size":5,"soft_fail":false,"target_va":5390101424,"va":5380041284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10078515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e833c99900","size":5,"soft_fail":false,"target_va":5390114912,"va":5380036392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141529bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10856749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e82da9a500","size":5,"soft_fail":false,"target_va":5390900144,"va":5380043390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31623525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e86589e201","size":5,"soft_fail":false,"target_va":5411669968,"va":5380046438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31813041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8b16de501","size":5,"soft_fail":false,"target_va":5411847168,"va":5380034122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t-5678762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e85659a9ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5380034325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31493808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8b08ee001","size":5,"soft_fail":false,"target_va":5411540963,"va":5380047150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31494983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e84793e001","size":5,"soft_fail":false,"target_va":5411541584,"va":5380046596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31496397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8cd98e001","size":5,"soft_fail":false,"target_va":5411541776,"va":5380045374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t31495516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e85c95e001","size":5,"soft_fail":false,"target_va":5411542064,"va":5380046543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t31495423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8ff94e001","size":5,"soft_fail":false,"target_va":5411542296,"va":5380046868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31494876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8dc92e001","size":5,"soft_fail":false,"target_va":5411541904,"va":5380047023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8a8c4ffff","size":5,"soft_fail":false,"target_va":5380018976,"va":5380034163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31509103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e86fcae001","size":5,"soft_fail":false,"target_va":5411543360,"va":5380034252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t384191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8bfdc0500","size":5,"soft_fail":false,"target_va":5380422656,"va":5380038460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-721851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e845fcf4ff","size":5,"soft_fail":false,"target_va":5379323280,"va":5380045126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2570788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8dcc5d8ff","size":5,"soft_fail":false,"target_va":5377469024,"va":5380039807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_component
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-246731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8353cfcff","size":5,"soft_fail":false,"target_va":5379793824,"va":5380040550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1699260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e84412e6ff","size":5,"soft_fail":false,"target_va":5378334544,"va":5380033799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_fully_erased_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1699375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8d111e6ff","size":5,"soft_fail":false,"target_va":5378340400,"va":5380039770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1705236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8ecfae5ff","size":5,"soft_fail":false,"target_va":5378339456,"va":5380044687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1701142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8ea0ae6ff","size":5,"soft_fail":false,"target_va":5378337360,"va":5380038497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1693159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8192ae6ff","size":5,"soft_fail":false,"target_va":5378341264,"va":5380034418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1697484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e83419e6ff","size":5,"soft_fail":false,"target_va":5378336480,"va":5380033959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_rewiring_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1704213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8ebfee5ff","size":5,"soft_fail":false,"target_va":5378336992,"va":5380041200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1702596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e83c05e6ff","size":5,"soft_fail":false,"target_va":5378338800,"va":5380041391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::rewire_enclosed_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-400816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e850e2f9ff","size":5,"soft_fail":false,"target_va":5379635792,"va":5380036603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1350357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8d59a1400","size":5,"soft_fail":false,"target_va":5381385536,"va":5380035174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1355546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e81aaf1400","size":5,"soft_fail":false,"target_va":5381390640,"va":5380035089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1340157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8fd721400","size":5,"soft_fail":false,"target_va":5381386480,"va":5380046318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1350560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8a09b1400","size":5,"soft_fail":false,"target_va":5381384704,"va":5380034139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8771514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8bad78500","size":5,"soft_fail":false,"target_va":5388825296,"va":5380053777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31490587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e81b82e001","size":5,"soft_fail":false,"target_va":5411537104,"va":5380046512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1875828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8749f1c00","size":5,"soft_fail":false,"target_va":5381912960,"va":5380037127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1876458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8eaa11c00","size":5,"soft_fail":false,"target_va":5381911872,"va":5380035409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1846505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380033200,"raw_hex":"e8e92c1c00","size":5,"soft_fail":false,"target_va":5381883088,"va":5380036578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":70580,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":85108,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74196,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":7686,"source_line":2096,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":78092,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":64592,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":566,"record_offset":64648,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":566,"record_offset":58208,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":566,"record_offset":59796,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":82672,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":71060,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":64496,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":81900,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":83796,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":89396,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":58484,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":566,"record_offset":77896,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":82504,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":566,"record_offset":73240,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":566,"record_offset":63968,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":566,"record_offset":81576,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":25006,"source_line":205,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":566,"record_offset":85352,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":31038,"source_line":33,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::StructureType::is_fence
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":566,"record_offset":78064,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":27454,"source_line":156,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":566,"record_offset":79572,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":27458,"source_line":195,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":566,"record_offset":66720,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":27460,"source_line":202,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":566,"record_offset":81612,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":27451,"source_line":62,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":566,"record_offset":68932,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":92208,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26461,"source_line":107,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":65628,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26531,"source_line":116,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":84308,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26460,"source_line":69,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":81640,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26534,"source_line":136,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":74764,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":74848,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26544,"source_line":196,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":566,"record_offset":64024,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32412,"source_line":43,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":566,"record_offset":82700,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":57980,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":28952,"source_line":95,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58328,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26166,"source_line":254,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58064,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":28958,"source_line":264,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":566,"record_offset":64796,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":30440,"source_line":67,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::affected_walls
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":566,"record_offset":60544,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":28971,"source_line":39,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::has_anything_been_affected
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":566,"record_offset":59712,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":28972,"source_line":43,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79600,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32420,"source_line":549,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79312,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32417,"source_line":571,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":71408,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32414,"source_line":204,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":81280,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32423,"source_line":599,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":566,"record_offset":84984,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":31143,"source_line":17,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68456,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68400,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":62984,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":59252,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":62008,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":60952,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":67916,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":68548,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":25767,"source_line":845,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":80008,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":70552,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::segments_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":70608,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":32388,"source_line":146,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":71752,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::max
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":71352,"parent_record_offset":57604,"ancestor_record_offset":57604,"inlinee":28016,"source_line":28,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":69540,"parent_record_offset":69516,"ancestor_record_offset":57604,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":69564,"parent_record_offset":69540,"ancestor_record_offset":57604,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":58844,"parent_record_offset":58816,"ancestor_record_offset":57604,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":69516,"parent_record_offset":69340,"ancestor_record_offset":57604,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":566,"record_offset":58816,"parent_record_offset":58664,"ancestor_record_offset":57604,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":59980,"parent_record_offset":59956,"ancestor_record_offset":57604,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":69280,"parent_record_offset":69252,"ancestor_record_offset":57604,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":65308,"parent_record_offset":65196,"ancestor_record_offset":57604,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":65196,"parent_record_offset":65160,"ancestor_record_offset":57604,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":60028,"parent_record_offset":60004,"ancestor_record_offset":57604,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":60004,"parent_record_offset":59980,"ancestor_record_offset":57604,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":69340,"parent_record_offset":69308,"ancestor_record_offset":57604,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":69308,"parent_record_offset":69280,"ancestor_record_offset":57604,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":65160,"parent_record_offset":65128,"ancestor_record_offset":57604,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":69376,"parent_record_offset":69340,"ancestor_record_offset":57604,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":566,"record_offset":69212,"parent_record_offset":69176,"ancestor_record_offset":57604,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":566,"record_offset":69176,"parent_record_offset":69104,"ancestor_record_offset":57604,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":59956,"parent_record_offset":59916,"ancestor_record_offset":57604,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":69148,"parent_record_offset":69104,"ancestor_record_offset":57604,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74272,"parent_record_offset":74228,"ancestor_record_offset":57604,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":59916,"parent_record_offset":59880,"ancestor_record_offset":57604,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74228,"parent_record_offset":74196,"ancestor_record_offset":57604,"inlinee":7687,"source_line":2122,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74660,"parent_record_offset":74636,"ancestor_record_offset":57604,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":69252,"parent_record_offset":69212,"ancestor_record_offset":57604,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":69068,"parent_record_offset":69012,"ancestor_record_offset":57604,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":67972,"parent_record_offset":67916,"ancestor_record_offset":57604,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":64524,"parent_record_offset":64496,"ancestor_record_offset":57604,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74332,"parent_record_offset":74300,"ancestor_record_offset":57604,"inlinee":7691,"source_line":2100,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74636,"parent_record_offset":74612,"ancestor_record_offset":57604,"inlinee":7696,"source_line":2156,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":74300,"parent_record_offset":74228,"ancestor_record_offset":57604,"inlinee":7689,"source_line":2176,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":69104,"parent_record_offset":69068,"ancestor_record_offset":57604,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":566,"record_offset":88488,"parent_record_offset":88464,"ancestor_record_offset":57604,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":566,"record_offset":89092,"parent_record_offset":89068,"ancestor_record_offset":57604,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":566,"record_offset":82056,"parent_record_offset":82016,"ancestor_record_offset":57604,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":566,"record_offset":58236,"parent_record_offset":58208,"ancestor_record_offset":57604,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":566,"record_offset":59820,"parent_record_offset":59796,"ancestor_record_offset":57604,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":566,"record_offset":61684,"parent_record_offset":61656,"ancestor_record_offset":57604,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":566,"record_offset":59880,"parent_record_offset":59820,"ancestor_record_offset":57604,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":566,"record_offset":69400,"parent_record_offset":69376,"ancestor_record_offset":57604,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":566,"record_offset":69424,"parent_record_offset":69400,"ancestor_record_offset":57604,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63320,"parent_record_offset":63292,"ancestor_record_offset":57604,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":85640,"parent_record_offset":85612,"ancestor_record_offset":57604,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63372,"parent_record_offset":63348,"ancestor_record_offset":57604,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63348,"parent_record_offset":63320,"ancestor_record_offset":57604,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63268,"parent_record_offset":63240,"ancestor_record_offset":57604,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63292,"parent_record_offset":63268,"ancestor_record_offset":57604,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63464,"parent_record_offset":63292,"ancestor_record_offset":57604,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":85804,"parent_record_offset":85412,"ancestor_record_offset":57604,"inlinee":8184,"source_line":2098,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":63576,"parent_record_offset":63552,"ancestor_record_offset":57604,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$54::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":85412,"parent_record_offset":85352,"ancestor_record_offset":57604,"inlinee":21226,"source_line":2321,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":70772,"parent_record_offset":70748,"ancestor_record_offset":57604,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":71476,"parent_record_offset":71452,"ancestor_record_offset":57604,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":85012,"parent_record_offset":84984,"ancestor_record_offset":57604,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":68860,"parent_record_offset":68836,"ancestor_record_offset":57604,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":68884,"parent_record_offset":68860,"ancestor_record_offset":57604,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":85068,"parent_record_offset":85044,"ancestor_record_offset":57604,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":66444,"parent_record_offset":66420,"ancestor_record_offset":57604,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":70748,"parent_record_offset":70724,"ancestor_record_offset":57604,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":60828,"parent_record_offset":60800,"ancestor_record_offset":57604,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":65952,"parent_record_offset":65928,"ancestor_record_offset":57604,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":60888,"parent_record_offset":60856,"ancestor_record_offset":57604,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":81308,"parent_record_offset":81280,"ancestor_record_offset":57604,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":71376,"parent_record_offset":71352,"ancestor_record_offset":57604,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":84192,"parent_record_offset":83984,"ancestor_record_offset":57604,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":566,"record_offset":83936,"parent_record_offset":83912,"ancestor_record_offset":57604,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":566,"record_offset":65976,"parent_record_offset":65952,"ancestor_record_offset":57604,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":566,"record_offset":66512,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":566,"record_offset":70956,"parent_record_offset":70932,"ancestor_record_offset":57604,"inlinee":11851,"source_line":91,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":566,"record_offset":71112,"parent_record_offset":71084,"ancestor_record_offset":57604,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":70932,"parent_record_offset":70904,"ancestor_record_offset":57604,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":70700,"parent_record_offset":70676,"ancestor_record_offset":57604,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":71140,"parent_record_offset":71112,"ancestor_record_offset":57604,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":81948,"parent_record_offset":81924,"ancestor_record_offset":57604,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":71084,"parent_record_offset":71060,"ancestor_record_offset":57604,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":70724,"parent_record_offset":70700,"ancestor_record_offset":57604,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":566,"record_offset":71164,"parent_record_offset":71140,"ancestor_record_offset":57604,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":566,"record_offset":64552,"parent_record_offset":64524,"ancestor_record_offset":57604,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":70676,"parent_record_offset":70644,"ancestor_record_offset":57604,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":566,"record_offset":74588,"parent_record_offset":74228,"ancestor_record_offset":57604,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":566,"record_offset":63396,"parent_record_offset":63372,"ancestor_record_offset":57604,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":58752,"parent_record_offset":58728,"ancestor_record_offset":57604,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":69448,"parent_record_offset":69424,"ancestor_record_offset":57604,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":69472,"parent_record_offset":69448,"ancestor_record_offset":57604,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":70820,"parent_record_offset":70608,"ancestor_record_offset":57604,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":65340,"parent_record_offset":65308,"ancestor_record_offset":57604,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":566,"record_offset":59412,"parent_record_offset":59380,"ancestor_record_offset":57604,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":61260,"parent_record_offset":61228,"ancestor_record_offset":57604,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":61228,"parent_record_offset":61196,"ancestor_record_offset":57604,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":68356,"parent_record_offset":68332,"ancestor_record_offset":57604,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":68272,"parent_record_offset":68248,"ancestor_record_offset":57604,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":68424,"parent_record_offset":68400,"ancestor_record_offset":57604,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":566,"record_offset":63136,"parent_record_offset":63092,"ancestor_record_offset":57604,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":566,"record_offset":68836,"parent_record_offset":68812,"ancestor_record_offset":57604,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":566,"record_offset":68812,"parent_record_offset":68776,"ancestor_record_offset":57604,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":58172,"parent_record_offset":58088,"ancestor_record_offset":57604,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":79344,"parent_record_offset":79312,"ancestor_record_offset":57604,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":66688,"parent_record_offset":65628,"ancestor_record_offset":57604,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":63992,"parent_record_offset":63968,"ancestor_record_offset":57604,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":83880,"parent_record_offset":83852,"ancestor_record_offset":57604,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":82204,"parent_record_offset":82016,"ancestor_record_offset":57604,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":59660,"parent_record_offset":59632,"ancestor_record_offset":57604,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":63924,"parent_record_offset":63092,"ancestor_record_offset":57604,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":69616,"parent_record_offset":69212,"ancestor_record_offset":57604,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":58512,"parent_record_offset":58484,"ancestor_record_offset":57604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":64616,"parent_record_offset":64592,"ancestor_record_offset":57604,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":58784,"parent_record_offset":58696,"ancestor_record_offset":57604,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":566,"record_offset":64764,"parent_record_offset":64648,"ancestor_record_offset":57604,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":566,"record_offset":62420,"parent_record_offset":62396,"ancestor_record_offset":57604,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":566,"record_offset":64708,"parent_record_offset":64648,"ancestor_record_offset":57604,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":566,"record_offset":62396,"parent_record_offset":62372,"ancestor_record_offset":57604,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":566,"record_offset":62604,"parent_record_offset":62576,"ancestor_record_offset":57604,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":64732,"parent_record_offset":64708,"ancestor_record_offset":57604,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":60068,"parent_record_offset":59916,"ancestor_record_offset":57604,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":566,"record_offset":71536,"parent_record_offset":71408,"ancestor_record_offset":57604,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":566,"record_offset":68000,"parent_record_offset":67972,"ancestor_record_offset":57604,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":566,"record_offset":68028,"parent_record_offset":68000,"ancestor_record_offset":57604,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":566,"record_offset":77920,"parent_record_offset":77896,"ancestor_record_offset":57604,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":62348,"parent_record_offset":62324,"ancestor_record_offset":57604,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":64936,"parent_record_offset":64912,"ancestor_record_offset":57604,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":62372,"parent_record_offset":62348,"ancestor_record_offset":57604,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":62676,"parent_record_offset":62648,"ancestor_record_offset":57604,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":64848,"parent_record_offset":64820,"ancestor_record_offset":57604,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":62324,"parent_record_offset":62208,"ancestor_record_offset":57604,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":566,"record_offset":64912,"parent_record_offset":64848,"ancestor_record_offset":57604,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":566,"record_offset":59380,"parent_record_offset":59352,"ancestor_record_offset":57604,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":566,"record_offset":62036,"parent_record_offset":62008,"ancestor_record_offset":57604,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":566,"record_offset":62176,"parent_record_offset":62144,"ancestor_record_offset":57604,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":566,"record_offset":62144,"parent_record_offset":62108,"ancestor_record_offset":57604,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":566,"record_offset":62084,"parent_record_offset":62060,"ancestor_record_offset":57604,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":566,"record_offset":62108,"parent_record_offset":62084,"ancestor_record_offset":57604,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":566,"record_offset":62060,"parent_record_offset":62036,"ancestor_record_offset":57604,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":566,"record_offset":62648,"parent_record_offset":62108,"ancestor_record_offset":57604,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59352,"parent_record_offset":59316,"ancestor_record_offset":57604,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59316,"parent_record_offset":59288,"ancestor_record_offset":57604,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59512,"parent_record_offset":59480,"ancestor_record_offset":57604,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59608,"parent_record_offset":59512,"ancestor_record_offset":57604,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59480,"parent_record_offset":59456,"ancestor_record_offset":57604,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59288,"parent_record_offset":59252,"ancestor_record_offset":57604,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":566,"record_offset":59632,"parent_record_offset":59608,"ancestor_record_offset":57604,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":566,"record_offset":62208,"parent_record_offset":62176,"ancestor_record_offset":57604,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":566,"record_offset":62492,"parent_record_offset":62208,"ancestor_record_offset":57604,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":566,"record_offset":62464,"parent_record_offset":62208,"ancestor_record_offset":57604,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":566,"record_offset":62268,"parent_record_offset":62208,"ancestor_record_offset":57604,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":566,"record_offset":71452,"parent_record_offset":71408,"ancestor_record_offset":57604,"inlinee":4992,"source_line":34,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79396,"parent_record_offset":79372,"ancestor_record_offset":57604,"inlinee":26475,"source_line":200,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79420,"parent_record_offset":79396,"ancestor_record_offset":57604,"inlinee":26476,"source_line":177,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":566,"record_offset":65024,"parent_record_offset":65000,"ancestor_record_offset":57604,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":566,"record_offset":83852,"parent_record_offset":83824,"ancestor_record_offset":57604,"inlinee":29539,"source_line":146,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":566,"record_offset":83912,"parent_record_offset":83880,"ancestor_record_offset":57604,"inlinee":29547,"source_line":147,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58004,"parent_record_offset":57980,"ancestor_record_offset":57604,"inlinee":28899,"source_line":86,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58356,"parent_record_offset":58328,"ancestor_record_offset":57604,"inlinee":26167,"source_line":150,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58112,"parent_record_offset":58088,"ancestor_record_offset":57604,"inlinee":26165,"source_line":168,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::is_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58088,"parent_record_offset":58064,"ancestor_record_offset":57604,"inlinee":28903,"source_line":183,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":566,"record_offset":58140,"parent_record_offset":58112,"ancestor_record_offset":57604,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":566,"record_offset":58028,"parent_record_offset":58004,"ancestor_record_offset":57604,"inlinee":28898,"source_line":175,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::animate_wall_bulldoze::impl$2::contains::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":566,"record_offset":65000,"parent_record_offset":64820,"ancestor_record_offset":57604,"inlinee":30442,"source_line":68,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79656,"parent_record_offset":79628,"ancestor_record_offset":57604,"inlinee":32422,"source_line":558,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79372,"parent_record_offset":79344,"ancestor_record_offset":57604,"inlinee":32419,"source_line":582,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":74356,"parent_record_offset":74332,"ancestor_record_offset":57604,"inlinee":32415,"source_line":296,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":83824,"parent_record_offset":83796,"ancestor_record_offset":57604,"inlinee":32424,"source_line":445,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":566,"record_offset":70520,"parent_record_offset":68932,"ancestor_record_offset":57604,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68480,"parent_record_offset":68456,"ancestor_record_offset":57604,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68220,"parent_record_offset":68192,"ancestor_record_offset":57604,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68248,"parent_record_offset":68220,"ancestor_record_offset":57604,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":566,"record_offset":68332,"parent_record_offset":68220,"ancestor_record_offset":57604,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":61156,"parent_record_offset":61116,"ancestor_record_offset":57604,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":61196,"parent_record_offset":61156,"ancestor_record_offset":57604,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":61304,"parent_record_offset":61116,"ancestor_record_offset":57604,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":566,"record_offset":61116,"parent_record_offset":61068,"ancestor_record_offset":57604,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":65928,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":60800,"parent_record_offset":60744,"ancestor_record_offset":57604,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":66540,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":60856,"parent_record_offset":60800,"ancestor_record_offset":57604,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":61396,"parent_record_offset":61068,"ancestor_record_offset":57604,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":566,"record_offset":66012,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":566,"record_offset":65900,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":566,"record_offset":66420,"parent_record_offset":66396,"ancestor_record_offset":57604,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":65684,"parent_record_offset":65660,"ancestor_record_offset":57604,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":65708,"parent_record_offset":65684,"ancestor_record_offset":57604,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":59760,"parent_record_offset":59736,"ancestor_record_offset":57604,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":60628,"parent_record_offset":60600,"ancestor_record_offset":57604,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":86252,"parent_record_offset":85892,"ancestor_record_offset":57604,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":66396,"parent_record_offset":66228,"ancestor_record_offset":57604,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":61008,"parent_record_offset":60980,"ancestor_record_offset":57604,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":85892,"parent_record_offset":85864,"ancestor_record_offset":57604,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":566,"record_offset":65740,"parent_record_offset":65708,"ancestor_record_offset":57604,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":66336,"parent_record_offset":66252,"ancestor_record_offset":57604,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":66280,"parent_record_offset":66252,"ancestor_record_offset":57604,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":61496,"parent_record_offset":61068,"ancestor_record_offset":57604,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":66632,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":60744,"parent_record_offset":60684,"ancestor_record_offset":57604,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":61068,"parent_record_offset":61036,"ancestor_record_offset":57604,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":66252,"parent_record_offset":66228,"ancestor_record_offset":57604,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":65792,"parent_record_offset":65768,"ancestor_record_offset":57604,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":65768,"parent_record_offset":65708,"ancestor_record_offset":57604,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":60656,"parent_record_offset":60628,"ancestor_record_offset":57604,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58696,"parent_record_offset":58664,"ancestor_record_offset":57604,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":64228,"parent_record_offset":64164,"ancestor_record_offset":57604,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":60716,"parent_record_offset":60684,"ancestor_record_offset":57604,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58608,"parent_record_offset":58584,"ancestor_record_offset":57604,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":65816,"parent_record_offset":65792,"ancestor_record_offset":57604,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58664,"parent_record_offset":58608,"ancestor_record_offset":57604,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58636,"parent_record_offset":58608,"ancestor_record_offset":57604,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":60684,"parent_record_offset":60656,"ancestor_record_offset":57604,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":65872,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58728,"parent_record_offset":58696,"ancestor_record_offset":57604,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":58584,"parent_record_offset":58560,"ancestor_record_offset":57604,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":61036,"parent_record_offset":61008,"ancestor_record_offset":57604,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":566,"record_offset":66228,"parent_record_offset":65816,"ancestor_record_offset":57604,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":566,"record_offset":84948,"parent_record_offset":84924,"ancestor_record_offset":57604,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":566,"record_offset":85864,"parent_record_offset":85836,"ancestor_record_offset":57604,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":566,"record_offset":81340,"parent_record_offset":81280,"ancestor_record_offset":57604,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":63020,"parent_record_offset":62984,"ancestor_record_offset":57604,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":94388,"parent_record_offset":94356,"ancestor_record_offset":57604,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":94904,"parent_record_offset":94644,"ancestor_record_offset":57604,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":566,"record_offset":63164,"parent_record_offset":63136,"ancestor_record_offset":57604,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":63240,"parent_record_offset":63092,"ancestor_record_offset":57604,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":566,"record_offset":63500,"parent_record_offset":63240,"ancestor_record_offset":57604,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":65660,"parent_record_offset":65628,"ancestor_record_offset":57604,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":84340,"parent_record_offset":84308,"ancestor_record_offset":57604,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":59736,"parent_record_offset":59712,"ancestor_record_offset":57604,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":60600,"parent_record_offset":60572,"ancestor_record_offset":57604,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":60572,"parent_record_offset":60544,"ancestor_record_offset":57604,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":60980,"parent_record_offset":60952,"ancestor_record_offset":57604,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":85440,"parent_record_offset":85412,"ancestor_record_offset":57604,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":566,"record_offset":84924,"parent_record_offset":84308,"ancestor_record_offset":57604,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":566,"record_offset":85836,"parent_record_offset":85352,"ancestor_record_offset":57604,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":566,"record_offset":68512,"parent_record_offset":68480,"ancestor_record_offset":57604,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":85496,"parent_record_offset":85468,"ancestor_record_offset":57604,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":85612,"parent_record_offset":85556,"ancestor_record_offset":57604,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":85468,"parent_record_offset":85440,"ancestor_record_offset":57604,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":566,"record_offset":63192,"parent_record_offset":63164,"ancestor_record_offset":57604,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":566,"record_offset":63092,"parent_record_offset":63056,"ancestor_record_offset":57604,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":566,"record_offset":63056,"parent_record_offset":63020,"ancestor_record_offset":57604,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":83764,"parent_record_offset":82700,"ancestor_record_offset":57604,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":70644,"parent_record_offset":70608,"ancestor_record_offset":57604,"inlinee":4633,"source_line":132,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":67944,"parent_record_offset":67916,"ancestor_record_offset":57604,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":68068,"parent_record_offset":67916,"ancestor_record_offset":57604,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":68776,"parent_record_offset":68548,"ancestor_record_offset":57604,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":74384,"parent_record_offset":74356,"ancestor_record_offset":57604,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":71500,"parent_record_offset":71476,"ancestor_record_offset":57604,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":85044,"parent_record_offset":85012,"ancestor_record_offset":57604,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":566,"record_offset":69012,"parent_record_offset":68932,"ancestor_record_offset":57604,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":566,"record_offset":68192,"parent_record_offset":67916,"ancestor_record_offset":57604,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E9E
// 首条位置: {"module":566,"inlinee":32414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":71564,"parent_record_offset":71408,"ancestor_record_offset":71408,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":566,"record_offset":71536,"parent_record_offset":71408,"ancestor_record_offset":71408,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":566,"record_offset":71452,"parent_record_offset":71408,"ancestor_record_offset":71408,"inlinee":4992,"source_line":34,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":71628,"parent_record_offset":71604,"ancestor_record_offset":71408,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":71676,"parent_record_offset":71652,"ancestor_record_offset":71408,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":71652,"parent_record_offset":71628,"ancestor_record_offset":71408,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":71604,"parent_record_offset":71564,"ancestor_record_offset":71408,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":566,"record_offset":71476,"parent_record_offset":71452,"ancestor_record_offset":71408,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":71716,"parent_record_offset":71564,"ancestor_record_offset":71408,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":71500,"parent_record_offset":71476,"ancestor_record_offset":71408,"inlinee":24624,"source_line":6,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E9F
// 首条位置: {"module":566,"inlinee":32415}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":566,"record_offset":74384,"parent_record_offset":74356,"ancestor_record_offset":74356,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 396
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AB9
// 首条位置: {"module":1585,"inlinee":72377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":287640,"parent_record_offset":287604,"ancestor_record_offset":287604,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":287664,"parent_record_offset":287640,"ancestor_record_offset":287604,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":287720,"parent_record_offset":287696,"ancestor_record_offset":287604,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1585,"record_offset":287696,"parent_record_offset":287664,"ancestor_record_offset":287604,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 445
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA8
// 首条位置: {"module":566,"inlinee":32424}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":566,"record_offset":83852,"parent_record_offset":83824,"ancestor_record_offset":83824,"inlinee":29539,"source_line":146,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":566,"record_offset":83936,"parent_record_offset":83912,"ancestor_record_offset":83824,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":83880,"parent_record_offset":83852,"ancestor_record_offset":83824,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":566,"record_offset":83912,"parent_record_offset":83880,"ancestor_record_offset":83824,"inlinee":29547,"source_line":147,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 549
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA4
// 首条位置: {"module":566,"inlinee":32420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":79628,"parent_record_offset":79600,"ancestor_record_offset":79600,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79680,"parent_record_offset":79656,"ancestor_record_offset":79600,"inlinee":26475,"source_line":200,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79656,"parent_record_offset":79628,"ancestor_record_offset":79600,"inlinee":32422,"source_line":558,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 558
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA6
// 首条位置: {"module":566,"inlinee":32422}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79680,"parent_record_offset":79656,"ancestor_record_offset":79656,"inlinee":26475,"source_line":200,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 571
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA1
// 首条位置: {"module":566,"inlinee":32417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":566,"record_offset":79344,"parent_record_offset":79312,"ancestor_record_offset":79312,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79396,"parent_record_offset":79372,"ancestor_record_offset":79312,"inlinee":26475,"source_line":200,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79420,"parent_record_offset":79396,"ancestor_record_offset":79312,"inlinee":26476,"source_line":177,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":566,"record_offset":79372,"parent_record_offset":79344,"ancestor_record_offset":79312,"inlinee":32419,"source_line":582,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 582
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA3
// 首条位置: {"module":566,"inlinee":32419}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79396,"parent_record_offset":79372,"ancestor_record_offset":79372,"inlinee":26475,"source_line":200,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":566,"record_offset":79420,"parent_record_offset":79396,"ancestor_record_offset":79372,"inlinee":26476,"source_line":177,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 599
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EA7
// 首条位置: {"module":566,"inlinee":32423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":81308,"parent_record_offset":81280,"ancestor_record_offset":81280,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":566,"record_offset":84192,"parent_record_offset":83984,"ancestor_record_offset":83984,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":566,"record_offset":81340,"parent_record_offset":81280,"ancestor_record_offset":81280,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":84160,"parent_record_offset":84052,"ancestor_record_offset":83984,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":81392,"parent_record_offset":81340,"ancestor_record_offset":81280,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":566,"record_offset":81364,"parent_record_offset":81340,"ancestor_record_offset":81280,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00566.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_eraser.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_eraser.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_eraser.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
// 信息 ID: a2d57039ffc8eef7eecbcc78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=81456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
// 信息 ID: be4b2504a138ee11c0308758
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=81512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
// 信息 ID: 00204f7d95a588e78ccd021e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
// 信息 ID: 1809df8678d30f2eca53d7e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79372; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 信息 ID: e8740948e0c9c52658f00fb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ee17323cdf4a5e18d06f8425: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> > > @ 0x43525a -> 0xaccab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> > @ 0x5358ef -> 0xaccab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 信息 ID: 279f290de96b5e59f57e865d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=71408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=71904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=72248; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=72892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 信息 ID: 5975bb238f16b11b4c14de71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=74356; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=74492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 信息 ID: 115054fc3d5ac0aceaceb979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; module=1585, inline_record=287604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
// 信息 ID: 3209d7794235c6c1de94a8cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=83824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 信息 ID: 1882efb6936a54683d29ab6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=81280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=83984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
