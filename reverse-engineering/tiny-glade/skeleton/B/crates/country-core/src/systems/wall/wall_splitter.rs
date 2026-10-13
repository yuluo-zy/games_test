// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/wall_splitter.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3b015c2d017fa97825df5eb8a945d07bd13a02d5c1f3add953c025e164115fe9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_splitter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 553
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":56916,"rva":10964000,"va":5379673120}
// 编译实例 RVAs: 0xa74c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10655599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e891685dff","size":5,"soft_fail":false,"target_va":5369022192,"va":5379677786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004d980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10650816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8407b5dff","size":5,"soft_fail":false,"target_va":5369026944,"va":5379677755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10078435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e81d3766ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379679230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8044675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e87d3f85ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5379679086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e13b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7945945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e827c186ff","size":5,"soft_fail":false,"target_va":5371728816,"va":5379674756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4997676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8d4bdb3ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5379674007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4869859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e81db1b5ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5379683278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d6640
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4853523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8edf0b5ff","size":5,"soft_fail":false,"target_va":5374830144,"va":5379683662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4664179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e88dd4b8ff","size":5,"soft_fail":false,"target_va":5375019664,"va":5379683838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4592863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e821ebb9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379674346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4595345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e86fe1b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379676844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4356032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e84088bdff","size":5,"soft_fail":false,"target_va":5375326672,"va":5379682699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881b90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2054407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8f9a6e0ff","size":5,"soft_fail":false,"target_va":5377629072,"va":5379683474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409231f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1385382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e85adceaff","size":5,"soft_fail":false,"target_va":5378290160,"va":5379675537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-576382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e88234f7ff","size":5,"soft_fail":false,"target_va":5379102272,"va":5379678649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2259632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8b07a2200","size":5,"soft_fail":false,"target_va":5381934560,"va":5379674923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413e5360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9891405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e84dee9600","size":5,"soft_fail":false,"target_va":5389570912,"va":5379679502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413e7230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9900901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e865139700","size":5,"soft_fail":false,"target_va":5389578800,"va":5379677894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10422862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e84e0a9f00","size":5,"soft_fail":false,"target_va":5390101424,"va":5379678557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10429946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8fa259f00","size":5,"soft_fail":false,"target_va":5390105824,"va":5379675873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10434947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e883399f00","size":5,"soft_fail":false,"target_va":5390113312,"va":5379678360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10438674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e812489f00","size":5,"soft_fail":false,"target_va":5390114304,"va":5379675625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10448365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8ed6d9f00","size":5,"soft_fail":false,"target_va":5390125184,"va":5379676814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141529bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11221155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8a338ab00","size":5,"soft_fail":false,"target_va":5390900144,"va":5379678984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32170424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8b8e1ea01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379676739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31859280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e85022e601","size":5,"soft_fail":false,"target_va":5411540992,"va":5379681707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3164846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8ae4a3000","size":5,"soft_fail":false,"target_va":5382840208,"va":5379675357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t960605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e85da80e00","size":5,"soft_fail":false,"target_va":5380635696,"va":5379675086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31859350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e89622e601","size":5,"soft_fail":false,"target_va":5411540963,"va":5379681608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31860650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8aa27e601","size":5,"soft_fail":false,"target_va":5411541776,"va":5379681121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31860890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e89a28e601","size":5,"soft_fail":false,"target_va":5411542064,"va":5379681169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t31860530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e83227e601","size":5,"soft_fail":false,"target_va":5411542296,"va":5379681761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31860543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e83f27e601","size":5,"soft_fail":false,"target_va":5411541904,"va":5379681356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1213317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e87b7cedff","size":5,"soft_fail":false,"target_va":5378471376,"va":5379684688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t59369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8e9e70000","size":5,"soft_fail":false,"target_va":5379739328,"va":5379679954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t744709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8055d0b00","size":5,"soft_fail":false,"target_va":5380422656,"va":5379677942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-357246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8828cfaff","size":5,"soft_fail":false,"target_va":5379323280,"va":5379680521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2209032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8f84adeff","size":5,"soft_fail":false,"target_va":5377469024,"va":5379678051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1340856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8488aebff","size":5,"soft_fail":false,"target_va":5378339456,"va":5379680307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1340643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e81d8bebff","size":5,"soft_fail":false,"target_va":5378337360,"va":5379677998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1339344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e83090ebff","size":5,"soft_fail":false,"target_va":5378341264,"va":5379680603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1341171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e80d89ebff","size":5,"soft_fail":false,"target_va":5378336480,"va":5379677646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31855558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8c613e601","size":5,"soft_fail":false,"target_va":5411537104,"va":5379681541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8149440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e8c0597c00","size":5,"soft_fail":false,"target_va":5387824496,"va":5379675051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2207362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e882ae2100","size":5,"soft_fail":false,"target_va":5381883088,"va":5379675721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2210096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379673120,"raw_hex":"e830b92100","size":5,"soft_fail":false,"target_va":5381886416,"va":5379676315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":553,"record_offset":61412,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":553,"record_offset":64272,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":61028,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":59736,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":69268,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":553,"record_offset":68164,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":553,"record_offset":75016,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":553,"record_offset":70404,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":58436,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":70920,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":553,"record_offset":61496,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":553,"record_offset":57680,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":553,"record_offset":60308,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":59672,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":59700,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":65360,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":60140,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":70752,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::WallId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":553,"record_offset":69240,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26468,"source_line":88,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":553,"record_offset":71128,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":25006,"source_line":205,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":553,"record_offset":72240,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":31038,"source_line":33,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":553,"record_offset":70640,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":27460,"source_line":202,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":553,"record_offset":71164,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":27451,"source_line":62,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":553,"record_offset":67016,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26461,"source_line":107,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":553,"record_offset":71192,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26460,"source_line":69,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":553,"record_offset":59588,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":553,"record_offset":70948,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":29539,"source_line":146,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":553,"record_offset":58492,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":553,"record_offset":57244,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":29444,"source_line":60,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":553,"record_offset":68072,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26166,"source_line":254,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":553,"record_offset":61552,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26164,"source_line":259,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":553,"record_offset":61524,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26157,"source_line":92,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":553,"record_offset":64488,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":553,"record_offset":65332,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":553,"record_offset":71868,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":31143,"source_line":17,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":553,"record_offset":60448,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4532,"source_line":777,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":60504,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":62588,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":63500,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":61648,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":25767,"source_line":845,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":59616,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":59644,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::max
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":62476,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":28016,"source_line":28,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::min
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":62532,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":28017,"source_line":35,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":62088,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":70344,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":63240,"parent_record_offset":56916,"ancestor_record_offset":56916,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60060,"parent_record_offset":60036,"ancestor_record_offset":56916,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60084,"parent_record_offset":60060,"ancestor_record_offset":56916,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":65536,"parent_record_offset":65512,"ancestor_record_offset":56916,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":64328,"parent_record_offset":64300,"ancestor_record_offset":56916,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60036,"parent_record_offset":59860,"ancestor_record_offset":56916,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":65512,"parent_record_offset":65456,"ancestor_record_offset":56916,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":64300,"parent_record_offset":64272,"ancestor_record_offset":56916,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":553,"record_offset":61460,"parent_record_offset":61436,"ancestor_record_offset":56916,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":61100,"parent_record_offset":61068,"ancestor_record_offset":56916,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":59800,"parent_record_offset":59768,"ancestor_record_offset":56916,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":65484,"parent_record_offset":65456,"ancestor_record_offset":56916,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":65456,"parent_record_offset":65432,"ancestor_record_offset":56916,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":61148,"parent_record_offset":61124,"ancestor_record_offset":56916,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":61124,"parent_record_offset":61100,"ancestor_record_offset":56916,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":59860,"parent_record_offset":59828,"ancestor_record_offset":56916,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":59828,"parent_record_offset":59800,"ancestor_record_offset":56916,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":65432,"parent_record_offset":65408,"ancestor_record_offset":56916,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":59896,"parent_record_offset":59860,"ancestor_record_offset":56916,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":64692,"parent_record_offset":64664,"ancestor_record_offset":56916,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":64664,"parent_record_offset":64636,"ancestor_record_offset":56916,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":64608,"parent_record_offset":64580,"ancestor_record_offset":56916,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":64636,"parent_record_offset":64608,"ancestor_record_offset":56916,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":64580,"parent_record_offset":64552,"ancestor_record_offset":56916,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":553,"record_offset":64552,"parent_record_offset":64524,"ancestor_record_offset":56916,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":553,"record_offset":64524,"parent_record_offset":64488,"ancestor_record_offset":56916,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":553,"record_offset":65200,"parent_record_offset":65176,"ancestor_record_offset":56916,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":61068,"parent_record_offset":61028,"ancestor_record_offset":56916,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":75136,"parent_record_offset":75100,"ancestor_record_offset":56916,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":59768,"parent_record_offset":59736,"ancestor_record_offset":56916,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":61712,"parent_record_offset":61648,"ancestor_record_offset":56916,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":553,"record_offset":68188,"parent_record_offset":68164,"ancestor_record_offset":56916,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":553,"record_offset":75040,"parent_record_offset":75016,"ancestor_record_offset":56916,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":553,"record_offset":75100,"parent_record_offset":75040,"ancestor_record_offset":56916,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":553,"record_offset":59920,"parent_record_offset":59896,"ancestor_record_offset":56916,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":553,"record_offset":59944,"parent_record_offset":59920,"ancestor_record_offset":56916,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":553,"record_offset":70520,"parent_record_offset":70428,"ancestor_record_offset":56916,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":72528,"parent_record_offset":72500,"ancestor_record_offset":56916,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":72580,"parent_record_offset":72556,"ancestor_record_offset":56916,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":72556,"parent_record_offset":72500,"ancestor_record_offset":56916,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":72692,"parent_record_offset":72300,"ancestor_record_offset":56916,"inlinee":8184,"source_line":2098,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$54::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":72300,"parent_record_offset":72240,"ancestor_record_offset":56916,"inlinee":21226,"source_line":2321,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":553,"record_offset":60992,"parent_record_offset":60964,"ancestor_record_offset":56916,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":60404,"parent_record_offset":60380,"ancestor_record_offset":56916,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":71896,"parent_record_offset":71868,"ancestor_record_offset":56916,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":62016,"parent_record_offset":61992,"ancestor_record_offset":56916,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":62040,"parent_record_offset":62016,"ancestor_record_offset":56916,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":71952,"parent_record_offset":71928,"ancestor_record_offset":56916,"inlinee":5882,"source_line":1777,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":59312,"parent_record_offset":59288,"ancestor_record_offset":56916,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":60380,"parent_record_offset":60356,"ancestor_record_offset":56916,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":553,"record_offset":57528,"parent_record_offset":57500,"ancestor_record_offset":56916,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":553,"record_offset":58820,"parent_record_offset":58796,"ancestor_record_offset":56916,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":553,"record_offset":57588,"parent_record_offset":57556,"ancestor_record_offset":56916,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":70368,"parent_record_offset":70344,"ancestor_record_offset":56916,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":62240,"parent_record_offset":62088,"ancestor_record_offset":56916,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":62500,"parent_record_offset":62476,"ancestor_record_offset":56916,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":62556,"parent_record_offset":62532,"ancestor_record_offset":56916,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":553,"record_offset":71032,"parent_record_offset":71008,"ancestor_record_offset":56916,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":553,"record_offset":62268,"parent_record_offset":62088,"ancestor_record_offset":56916,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":58844,"parent_record_offset":58820,"ancestor_record_offset":56916,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":59380,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":64948,"parent_record_offset":64924,"ancestor_record_offset":56916,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":60932,"parent_record_offset":60908,"ancestor_record_offset":56916,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":60332,"parent_record_offset":60308,"ancestor_record_offset":56916,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":60552,"parent_record_offset":60504,"ancestor_record_offset":56916,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":60356,"parent_record_offset":60332,"ancestor_record_offset":56916,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":60580,"parent_record_offset":60552,"ancestor_record_offset":56916,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":553,"record_offset":60908,"parent_record_offset":60504,"ancestor_record_offset":56916,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":553,"record_offset":70544,"parent_record_offset":70520,"ancestor_record_offset":56916,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":553,"record_offset":60760,"parent_record_offset":60736,"ancestor_record_offset":56916,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":68488,"parent_record_offset":68464,"ancestor_record_offset":56916,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":59968,"parent_record_offset":59944,"ancestor_record_offset":56916,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":59992,"parent_record_offset":59968,"ancestor_record_offset":56916,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":70488,"parent_record_offset":70464,"ancestor_record_offset":56916,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":71456,"parent_record_offset":71424,"ancestor_record_offset":56916,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":553,"record_offset":58008,"parent_record_offset":57976,"ancestor_record_offset":56916,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":553,"record_offset":57976,"parent_record_offset":57944,"ancestor_record_offset":56916,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":63164,"parent_record_offset":63132,"ancestor_record_offset":56916,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":63012,"parent_record_offset":62980,"ancestor_record_offset":56916,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":63412,"parent_record_offset":63240,"ancestor_record_offset":56916,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":553,"record_offset":70464,"parent_record_offset":70428,"ancestor_record_offset":56916,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":553,"record_offset":70428,"parent_record_offset":70404,"ancestor_record_offset":56916,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":61992,"parent_record_offset":61968,"ancestor_record_offset":56916,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":61968,"parent_record_offset":61932,"ancestor_record_offset":56916,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":58404,"parent_record_offset":57680,"ancestor_record_offset":56916,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":61436,"parent_record_offset":61412,"ancestor_record_offset":56916,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":70976,"parent_record_offset":70948,"ancestor_record_offset":56916,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":60964,"parent_record_offset":60504,"ancestor_record_offset":56916,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":57624,"parent_record_offset":57468,"ancestor_record_offset":56916,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":72472,"parent_record_offset":72444,"ancestor_record_offset":56916,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":65092,"parent_record_offset":64692,"ancestor_record_offset":56916,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":70568,"parent_record_offset":70544,"ancestor_record_offset":56916,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":65176,"parent_record_offset":64488,"ancestor_record_offset":56916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":553,"record_offset":60188,"parent_record_offset":60164,"ancestor_record_offset":56916,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":553,"record_offset":58268,"parent_record_offset":58244,"ancestor_record_offset":56916,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":70880,"parent_record_offset":70856,"ancestor_record_offset":56916,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":70800,"parent_record_offset":70752,"ancestor_record_offset":56916,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":60788,"parent_record_offset":60760,"ancestor_record_offset":56916,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":64436,"parent_record_offset":64408,"ancestor_record_offset":56916,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":60816,"parent_record_offset":60760,"ancestor_record_offset":56916,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":61740,"parent_record_offset":61712,"ancestor_record_offset":56916,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":61768,"parent_record_offset":61740,"ancestor_record_offset":56916,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":553,"record_offset":60164,"parent_record_offset":60140,"ancestor_record_offset":56916,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":70856,"parent_record_offset":70832,"ancestor_record_offset":56916,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":60616,"parent_record_offset":60580,"ancestor_record_offset":56916,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":70832,"parent_record_offset":70752,"ancestor_record_offset":56916,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":553,"record_offset":65236,"parent_record_offset":65200,"ancestor_record_offset":56916,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":553,"record_offset":65260,"parent_record_offset":65236,"ancestor_record_offset":56916,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":553,"record_offset":65292,"parent_record_offset":65200,"ancestor_record_offset":56916,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":553,"record_offset":58460,"parent_record_offset":58436,"ancestor_record_offset":56916,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":553,"record_offset":71008,"parent_record_offset":70976,"ancestor_record_offset":56916,"inlinee":29547,"source_line":147,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":553,"record_offset":68096,"parent_record_offset":68072,"ancestor_record_offset":56916,"inlinee":26167,"source_line":150,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":553,"record_offset":61580,"parent_record_offset":61552,"ancestor_record_offset":56916,"inlinee":26165,"source_line":168,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":553,"record_offset":61612,"parent_record_offset":61580,"ancestor_record_offset":56916,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":64016,"parent_record_offset":63980,"ancestor_record_offset":56916,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":63980,"parent_record_offset":63944,"ancestor_record_offset":56916,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":63944,"parent_record_offset":63916,"ancestor_record_offset":56916,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":62952,"parent_record_offset":62924,"ancestor_record_offset":56916,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":63916,"parent_record_offset":63888,"ancestor_record_offset":56916,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":62980,"parent_record_offset":62952,"ancestor_record_offset":56916,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":63132,"parent_record_offset":62952,"ancestor_record_offset":56916,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":63796,"parent_record_offset":63500,"ancestor_record_offset":56916,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":553,"record_offset":57904,"parent_record_offset":57864,"ancestor_record_offset":56916,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":553,"record_offset":57944,"parent_record_offset":57904,"ancestor_record_offset":56916,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":553,"record_offset":58052,"parent_record_offset":57864,"ancestor_record_offset":56916,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":553,"record_offset":57864,"parent_record_offset":57812,"ancestor_record_offset":56916,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":58796,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":57500,"parent_record_offset":57468,"ancestor_record_offset":56916,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":59408,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":57556,"parent_record_offset":57500,"ancestor_record_offset":56916,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":58144,"parent_record_offset":57812,"ancestor_record_offset":56916,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":553,"record_offset":58880,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":553,"record_offset":58768,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":553,"record_offset":59288,"parent_record_offset":59264,"ancestor_record_offset":56916,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":58552,"parent_record_offset":58528,"ancestor_record_offset":56916,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":58576,"parent_record_offset":58552,"ancestor_record_offset":56916,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":57340,"parent_record_offset":57308,"ancestor_record_offset":56916,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":73140,"parent_record_offset":72780,"ancestor_record_offset":56916,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":59264,"parent_record_offset":59096,"ancestor_record_offset":56916,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":57752,"parent_record_offset":57728,"ancestor_record_offset":56916,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":72780,"parent_record_offset":72752,"ancestor_record_offset":56916,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":553,"record_offset":58608,"parent_record_offset":58576,"ancestor_record_offset":56916,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":59204,"parent_record_offset":59120,"ancestor_record_offset":56916,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":59148,"parent_record_offset":59120,"ancestor_record_offset":56916,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":58244,"parent_record_offset":57812,"ancestor_record_offset":56916,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":59500,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57468,"parent_record_offset":57404,"ancestor_record_offset":56916,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57812,"parent_record_offset":57776,"ancestor_record_offset":56916,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":59120,"parent_record_offset":59096,"ancestor_record_offset":56916,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":58660,"parent_record_offset":58636,"ancestor_record_offset":56916,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":58636,"parent_record_offset":58576,"ancestor_record_offset":56916,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57372,"parent_record_offset":57340,"ancestor_record_offset":56916,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68432,"parent_record_offset":68400,"ancestor_record_offset":56916,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":73908,"parent_record_offset":73876,"ancestor_record_offset":56916,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57440,"parent_record_offset":57404,"ancestor_record_offset":56916,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68344,"parent_record_offset":68320,"ancestor_record_offset":56916,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":58684,"parent_record_offset":58660,"ancestor_record_offset":56916,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68400,"parent_record_offset":68344,"ancestor_record_offset":56916,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68372,"parent_record_offset":68344,"ancestor_record_offset":56916,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57404,"parent_record_offset":57372,"ancestor_record_offset":56916,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":58740,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68464,"parent_record_offset":68432,"ancestor_record_offset":56916,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":68320,"parent_record_offset":68296,"ancestor_record_offset":56916,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":57776,"parent_record_offset":57752,"ancestor_record_offset":56916,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":553,"record_offset":59096,"parent_record_offset":58684,"ancestor_record_offset":56916,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":553,"record_offset":71832,"parent_record_offset":71808,"ancestor_record_offset":56916,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":553,"record_offset":72752,"parent_record_offset":72724,"ancestor_record_offset":56916,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":60736,"parent_record_offset":60504,"ancestor_record_offset":56916,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":60472,"parent_record_offset":60448,"ancestor_record_offset":56916,"inlinee":4534,"source_line":176,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":58528,"parent_record_offset":58492,"ancestor_record_offset":56916,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":71224,"parent_record_offset":71192,"ancestor_record_offset":56916,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":57308,"parent_record_offset":57276,"ancestor_record_offset":56916,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":57276,"parent_record_offset":57244,"ancestor_record_offset":56916,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":57728,"parent_record_offset":57704,"ancestor_record_offset":56916,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":57704,"parent_record_offset":57680,"ancestor_record_offset":56916,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":553,"record_offset":72328,"parent_record_offset":72300,"ancestor_record_offset":56916,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":553,"record_offset":71808,"parent_record_offset":71192,"ancestor_record_offset":56916,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":553,"record_offset":72724,"parent_record_offset":72240,"ancestor_record_offset":56916,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":64052,"parent_record_offset":64016,"ancestor_record_offset":56916,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":553,"record_offset":72384,"parent_record_offset":72356,"ancestor_record_offset":56916,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":553,"record_offset":72500,"parent_record_offset":72444,"ancestor_record_offset":56916,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":553,"record_offset":72356,"parent_record_offset":72328,"ancestor_record_offset":56916,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":72444,"parent_record_offset":72412,"ancestor_record_offset":56916,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":72412,"parent_record_offset":72384,"ancestor_record_offset":56916,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":63544,"parent_record_offset":63500,"ancestor_record_offset":56916,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":62620,"parent_record_offset":62588,"ancestor_record_offset":56916,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":59556,"parent_record_offset":58492,"ancestor_record_offset":56916,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":61684,"parent_record_offset":61648,"ancestor_record_offset":56916,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":553,"record_offset":61808,"parent_record_offset":61648,"ancestor_record_offset":56916,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":61932,"parent_record_offset":61648,"ancestor_record_offset":56916,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":553,"record_offset":71928,"parent_record_offset":71896,"ancestor_record_offset":56916,"inlinee":27913,"source_line":6,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":553,"record_offset":62924,"parent_record_offset":62620,"ancestor_record_offset":56916,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":553,"record_offset":63888,"parent_record_offset":63500,"ancestor_record_offset":56916,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":78860,"rva":10976000,"va":5379685120}
// 编译实例 RVAs: 0xa77b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4603700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e8ccc0b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379685183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4604268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e894beb9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379685767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1953825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e8df2fe2ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5379685820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31855306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e8ca12e601","size":5,"soft_fail":false,"target_va":5411540992,"va":5379685681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31661374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e83e1de301","size":5,"soft_fail":false,"target_va":5411347040,"va":5379685661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31855238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e88612e601","size":5,"soft_fail":false,"target_va":5411540963,"va":5379685720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1941288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685120,"raw_hex":"e8d860e2ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5379685491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":79032,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80212,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":553,"record_offset":79908,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":80296,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":553,"record_offset":79880,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":26796,"source_line":211,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":553,"record_offset":79328,"parent_record_offset":78860,"ancestor_record_offset":78860,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":79088,"parent_record_offset":79064,"ancestor_record_offset":78860,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":79112,"parent_record_offset":79088,"ancestor_record_offset":78860,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80368,"parent_record_offset":80344,"ancestor_record_offset":78860,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":79064,"parent_record_offset":79032,"ancestor_record_offset":78860,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80344,"parent_record_offset":80320,"ancestor_record_offset":78860,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":80320,"parent_record_offset":80296,"ancestor_record_offset":78860,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":80060,"parent_record_offset":80032,"ancestor_record_offset":78860,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":79604,"parent_record_offset":79576,"ancestor_record_offset":78860,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":80144,"parent_record_offset":80120,"ancestor_record_offset":78860,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":80120,"parent_record_offset":80088,"ancestor_record_offset":78860,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":80088,"parent_record_offset":80060,"ancestor_record_offset":78860,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":79664,"parent_record_offset":79632,"ancestor_record_offset":78860,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":79632,"parent_record_offset":79604,"ancestor_record_offset":78860,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":79540,"parent_record_offset":79512,"ancestor_record_offset":78860,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80236,"parent_record_offset":80212,"ancestor_record_offset":78860,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":79512,"parent_record_offset":79484,"ancestor_record_offset":78860,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":79456,"parent_record_offset":79428,"ancestor_record_offset":78860,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":79484,"parent_record_offset":79456,"ancestor_record_offset":78860,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":79428,"parent_record_offset":79400,"ancestor_record_offset":78860,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":553,"record_offset":79400,"parent_record_offset":79372,"ancestor_record_offset":78860,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":553,"record_offset":79372,"parent_record_offset":79328,"ancestor_record_offset":78860,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":80260,"parent_record_offset":80236,"ancestor_record_offset":78860,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":80032,"parent_record_offset":79996,"ancestor_record_offset":78860,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":79576,"parent_record_offset":79540,"ancestor_record_offset":78860,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":553,"record_offset":79964,"parent_record_offset":79936,"ancestor_record_offset":78860,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":553,"record_offset":79996,"parent_record_offset":79964,"ancestor_record_offset":78860,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":553,"record_offset":79936,"parent_record_offset":79908,"ancestor_record_offset":78860,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":79816,"parent_record_offset":79792,"ancestor_record_offset":78860,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":79792,"parent_record_offset":79540,"ancestor_record_offset":78860,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":79136,"parent_record_offset":79112,"ancestor_record_offset":78860,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":80412,"rva":10976720,"va":5379685840}
// 编译实例 RVAs: 0xa77dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4604420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e8fcbdb9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379685903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4604876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e834bcb9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379686375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1954433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e87f2de2ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5379686428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31854704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e87010e601","size":5,"soft_fail":false,"target_va":5411540992,"va":5379686283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31854653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e83d10e601","size":5,"soft_fail":false,"target_va":5411540963,"va":5379686305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1942016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379685840,"raw_hex":"e8005ee2ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5379686219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80588,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":81944,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":553,"record_offset":80736,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":82028,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":553,"record_offset":81272,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":553,"record_offset":81912,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":27002,"source_line":194,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":81820,"parent_record_offset":80412,"ancestor_record_offset":80412,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80644,"parent_record_offset":80620,"ancestor_record_offset":80412,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80668,"parent_record_offset":80644,"ancestor_record_offset":80412,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":82100,"parent_record_offset":82076,"ancestor_record_offset":80412,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":80620,"parent_record_offset":80588,"ancestor_record_offset":80412,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":82076,"parent_record_offset":82052,"ancestor_record_offset":80412,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":82052,"parent_record_offset":82028,"ancestor_record_offset":80412,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":80996,"parent_record_offset":80968,"ancestor_record_offset":80412,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":81056,"parent_record_offset":81024,"ancestor_record_offset":80412,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":81024,"parent_record_offset":80996,"ancestor_record_offset":80412,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80932,"parent_record_offset":80904,"ancestor_record_offset":80412,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":81968,"parent_record_offset":81944,"ancestor_record_offset":80412,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80904,"parent_record_offset":80876,"ancestor_record_offset":80412,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80848,"parent_record_offset":80820,"ancestor_record_offset":80412,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80876,"parent_record_offset":80848,"ancestor_record_offset":80412,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":80820,"parent_record_offset":80792,"ancestor_record_offset":80412,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":553,"record_offset":80792,"parent_record_offset":80764,"ancestor_record_offset":80412,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":553,"record_offset":80764,"parent_record_offset":80736,"ancestor_record_offset":80412,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":81992,"parent_record_offset":81968,"ancestor_record_offset":80412,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":80968,"parent_record_offset":80932,"ancestor_record_offset":80412,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":81208,"parent_record_offset":81184,"ancestor_record_offset":80412,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":81852,"parent_record_offset":81820,"ancestor_record_offset":80412,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":81184,"parent_record_offset":80932,"ancestor_record_offset":80412,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":80692,"parent_record_offset":80668,"ancestor_record_offset":80412,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00553.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_splitter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_splitter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_splitter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter
// 信息 ID: 0327cfc21551507f1065acf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ddd81e732e2c06de46ead436: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> > > @ 0x47d4c8 -> 0xa74c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> > @ 0x50c61f -> 0xa74c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 信息 ID: ba01572955cd608231aa34ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xa716b2 -> 0xa77b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 信息 ID: 31690b62e157dbeb951017fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xa71722 -> 0xa77dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
