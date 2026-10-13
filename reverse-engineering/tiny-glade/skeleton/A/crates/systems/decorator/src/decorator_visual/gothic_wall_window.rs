// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5d733cb64e2fb7d3c5ee5927d2f18b6ab97642789b6d33d4d0813b78e137681b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_wall_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2764, 3842
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3842,"record_offset":23444,"rva":29625024,"va":5398334144}
// 编译实例 RVAs: 0x1c40ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21394443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e8f58bb9fe","size":5,"soft_fail":false,"target_va":5376939904,"va":5398334342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21391099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e80599b9fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398335094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16078552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e828a90aff","size":5,"soft_fail":false,"target_va":5382255968,"va":5398334515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf2e10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16047857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e80f210bff","size":5,"soft_fail":false,"target_va":5382286864,"va":5398334716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3280
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16046685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e8a3250bff","size":5,"soft_fail":false,"target_va":5382288000,"va":5398334680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4643816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e8e8db4600","size":5,"soft_fail":false,"target_va":5402979152,"va":5398335331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4644032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e8c0dc4600","size":5,"soft_fail":false,"target_va":5402979344,"va":5398335307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16401437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e8e3bb05ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398334488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-16078000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398334144,"raw_hex":"e850ab0aff","size":5,"soft_fail":false,"target_va":5382256592,"va":5398334587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":25228,"parent_record_offset":23444,"ancestor_record_offset":23444,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3842,"record_offset":23620,"parent_record_offset":23444,"ancestor_record_offset":23444,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3842,"record_offset":24016,"parent_record_offset":23444,"ancestor_record_offset":23444,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3842,"record_offset":23956,"parent_record_offset":23444,"ancestor_record_offset":23444,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3842,"record_offset":23900,"parent_record_offset":23444,"ancestor_record_offset":23444,"inlinee":60023,"source_line":205,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":24116,"parent_record_offset":24016,"ancestor_record_offset":23444,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3842,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":23444,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":23672,"parent_record_offset":23620,"ancestor_record_offset":23444,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":23728,"parent_record_offset":23620,"ancestor_record_offset":23444,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":23444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3842,"record_offset":24140,"parent_record_offset":24116,"ancestor_record_offset":23444,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3842,"record_offset":24164,"parent_record_offset":24140,"ancestor_record_offset":23444,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":24420,"parent_record_offset":24384,"ancestor_record_offset":23444,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":24384,"parent_record_offset":24356,"ancestor_record_offset":23444,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":24356,"parent_record_offset":24328,"ancestor_record_offset":23444,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":24328,"parent_record_offset":24300,"ancestor_record_offset":23444,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":24200,"parent_record_offset":24016,"ancestor_record_offset":23444,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3842,"record_offset":24456,"parent_record_offset":24420,"ancestor_record_offset":23444,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3842,"record_offset":24056,"parent_record_offset":24016,"ancestor_record_offset":23444,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3842,"record_offset":24084,"parent_record_offset":24056,"ancestor_record_offset":23444,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3842,"record_offset":24300,"parent_record_offset":24016,"ancestor_record_offset":23444,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3842,"record_offset":5924,"rva":29614528,"va":5398323648}
// 编译实例 RVAs: 0x1c3e1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28707040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e820f749fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5398325067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28512519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8f9ee4cfe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398326066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26681018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e846e168fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398332757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25580494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e832ac79fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398330185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25580993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e83faa79fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398330700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25177916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8c4d07ffe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398327751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24264984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8e8be8dfe","size":5,"soft_fail":false,"target_va":5374061056,"va":5398326035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23770813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8434995fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398325512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23611588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e83cb797fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398324463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23609154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8bec097fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398325949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23115757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e813489ffe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398325528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23111207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8d9599ffe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398324434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21381930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8d6bcb9fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398325893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21386834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8aea9b9fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398330829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16419277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8337605ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398329160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15958552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8e87d0cff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398325555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4646372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8e4e54600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398331767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-4947939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e81d80b4ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398326334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4646868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8d4e74600","size":5,"soft_fail":false,"target_va":5402978688,"va":5398331815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4647340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8ace94600","size":5,"soft_fail":false,"target_va":5402978912,"va":5398331567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4647521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e861ea4600","size":5,"soft_fail":false,"target_va":5402978944,"va":5398331418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e81fddffff","size":5,"soft_fail":false,"target_va":5398322288,"va":5398331212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4655903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e81f0b4700","size":5,"soft_fail":false,"target_va":5402980128,"va":5398324220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8d7faffff","size":5,"soft_fail":false,"target_va":5398322480,"va":5398323796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4656778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e88a0e4700","size":5,"soft_fail":false,"target_va":5402980656,"va":5398323873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-23754647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8698895fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398325634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23112180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e80c569ffe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398324863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::gothic
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23510100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8ac4399fe","size":5,"soft_fail":false,"target_va":5374821040,"va":5398331135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21967483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e885cdb0fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398324390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21963593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8b7dcb0fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398324148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21967637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8ebccb0fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398323776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16034365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8c3550bff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398324168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4839416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e80828b6ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398330083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4834509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8333bb6ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398325992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e8fe200000","size":5,"soft_fail":false,"target_va":5398334144,"va":5398325693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20376988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e86412c9fe","size":5,"soft_fail":false,"target_va":5377952848,"va":5398329831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21381360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e810bfb9fe","size":5,"soft_fail":false,"target_va":5376946416,"va":5398327771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21382541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398323648,"raw_hex":"e873bab9fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398325912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":10044,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3842,"record_offset":19376,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3842,"record_offset":13860,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3842,"record_offset":13916,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3842,"record_offset":8940,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3842,"record_offset":8912,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":19408,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":13260,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":14812,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":16276,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":8856,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3842,"record_offset":14364,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3842,"record_offset":17852,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":13644,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":13832,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":13452,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":17740,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":25315,"source_line":1069,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":18644,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":19660,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":17796,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":7808,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":6220,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":6796,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":9684,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":10184,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":10780,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3842,"record_offset":18732,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_wall_window::get_window_holes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs
//     evidence={"module":3842,"record_offset":18852,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":111755,"source_line":422,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3842,"record_offset":14000,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3842,"record_offset":14448,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3842,"record_offset":13972,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3842,"record_offset":19748,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3842,"record_offset":13424,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":5924,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":5924,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":5924,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18904,"parent_record_offset":18852,"ancestor_record_offset":5924,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18932,"parent_record_offset":18904,"ancestor_record_offset":5924,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":5924,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":10120,"parent_record_offset":10088,"ancestor_record_offset":5924,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":5924,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3842,"record_offset":21136,"parent_record_offset":21100,"ancestor_record_offset":5924,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3842,"record_offset":16096,"parent_record_offset":14852,"ancestor_record_offset":5924,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3842,"record_offset":20948,"parent_record_offset":20924,"ancestor_record_offset":5924,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":10088,"parent_record_offset":10044,"ancestor_record_offset":5924,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3842,"record_offset":19432,"parent_record_offset":19408,"ancestor_record_offset":5924,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3842,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":5924,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3842,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":5924,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":5924,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":5924,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":5924,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8148,"parent_record_offset":8120,"ancestor_record_offset":5924,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8180,"parent_record_offset":8148,"ancestor_record_offset":5924,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8352,"parent_record_offset":8180,"ancestor_record_offset":5924,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3842,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":5924,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3842,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":5924,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3842,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":5924,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3842,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":5924,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3842,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":5924,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3842,"record_offset":9368,"parent_record_offset":9332,"ancestor_record_offset":5924,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3842,"record_offset":18316,"parent_record_offset":17960,"ancestor_record_offset":5924,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3842,"record_offset":18224,"parent_record_offset":18168,"ancestor_record_offset":5924,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3842,"record_offset":18060,"parent_record_offset":17960,"ancestor_record_offset":5924,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3842,"record_offset":18028,"parent_record_offset":18004,"ancestor_record_offset":5924,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3842,"record_offset":18088,"parent_record_offset":17960,"ancestor_record_offset":5924,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3842,"record_offset":18820,"parent_record_offset":18732,"ancestor_record_offset":5924,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3842,"record_offset":13380,"parent_record_offset":13356,"ancestor_record_offset":5924,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3842,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":5924,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3842,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":5924,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3842,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":5924,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3842,"record_offset":11084,"parent_record_offset":10928,"ancestor_record_offset":5924,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11012,"parent_record_offset":10984,"ancestor_record_offset":5924,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11040,"parent_record_offset":11012,"ancestor_record_offset":5924,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":5924,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":5924,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":5924,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":5924,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11380,"parent_record_offset":11332,"ancestor_record_offset":5924,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3842,"record_offset":11772,"parent_record_offset":11332,"ancestor_record_offset":5924,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3842,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":5924,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3842,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":5924,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3842,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":5924,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3842,"record_offset":6432,"parent_record_offset":6336,"ancestor_record_offset":5924,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3842,"record_offset":11480,"parent_record_offset":11380,"ancestor_record_offset":5924,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3842,"record_offset":11448,"parent_record_offset":11380,"ancestor_record_offset":5924,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3842,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":5924,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":13580,"parent_record_offset":13484,"ancestor_record_offset":5924,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":18672,"parent_record_offset":18644,"ancestor_record_offset":5924,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":19688,"parent_record_offset":19660,"ancestor_record_offset":5924,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3842,"record_offset":13524,"parent_record_offset":13484,"ancestor_record_offset":5924,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3842,"record_offset":9648,"parent_record_offset":9616,"ancestor_record_offset":5924,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":5924,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":18764,"parent_record_offset":18732,"ancestor_record_offset":5924,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":18788,"parent_record_offset":18764,"ancestor_record_offset":5924,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":7392,"parent_record_offset":6932,"ancestor_record_offset":5924,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":8092,"parent_record_offset":7916,"ancestor_record_offset":5924,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":13324,"parent_record_offset":13296,"ancestor_record_offset":5924,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3842,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":5924,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":8416,"parent_record_offset":8120,"ancestor_record_offset":5924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3842,"record_offset":9452,"parent_record_offset":9428,"ancestor_record_offset":5924,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3842,"record_offset":9428,"parent_record_offset":9404,"ancestor_record_offset":5924,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":21168,"parent_record_offset":21136,"ancestor_record_offset":5924,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":5924,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":5924,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":7352,"parent_record_offset":7320,"ancestor_record_offset":5924,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":19008,"parent_record_offset":18980,"ancestor_record_offset":5924,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":5924,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3842,"record_offset":8816,"parent_record_offset":7880,"ancestor_record_offset":5924,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3842,"record_offset":6368,"parent_record_offset":6336,"ancestor_record_offset":5924,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3842,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":5924,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3842,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":5924,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3842,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":5924,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3842,"record_offset":7096,"parent_record_offset":6988,"ancestor_record_offset":5924,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3842,"record_offset":6336,"parent_record_offset":6308,"ancestor_record_offset":5924,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3842,"record_offset":6828,"parent_record_offset":6796,"ancestor_record_offset":5924,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3842,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":5924,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3842,"record_offset":6932,"parent_record_offset":6900,"ancestor_record_offset":5924,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3842,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":5924,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3842,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5924,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3842,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":5924,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3842,"record_offset":7424,"parent_record_offset":6900,"ancestor_record_offset":5924,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6532,"parent_record_offset":6500,"ancestor_record_offset":5924,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6692,"parent_record_offset":6532,"ancestor_record_offset":5924,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":5924,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6252,"parent_record_offset":6220,"ancestor_record_offset":5924,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":5924,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6308,"parent_record_offset":6276,"ancestor_record_offset":5924,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3842,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5924,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3842,"record_offset":6988,"parent_record_offset":6960,"ancestor_record_offset":5924,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3842,"record_offset":7236,"parent_record_offset":6988,"ancestor_record_offset":5924,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3842,"record_offset":7208,"parent_record_offset":6988,"ancestor_record_offset":5924,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3842,"record_offset":7040,"parent_record_offset":6988,"ancestor_record_offset":5924,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3842,"record_offset":16124,"parent_record_offset":16096,"ancestor_record_offset":5924,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3842,"record_offset":20984,"parent_record_offset":20948,"ancestor_record_offset":5924,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3842,"record_offset":16148,"parent_record_offset":16124,"ancestor_record_offset":5924,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3842,"record_offset":21008,"parent_record_offset":20984,"ancestor_record_offset":5924,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3842,"record_offset":21040,"parent_record_offset":20948,"ancestor_record_offset":5924,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":14852,"parent_record_offset":14812,"ancestor_record_offset":5924,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":13296,"parent_record_offset":13260,"ancestor_record_offset":5924,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":16316,"parent_record_offset":16276,"ancestor_record_offset":5924,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":15788,"parent_record_offset":15760,"ancestor_record_offset":5924,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":16208,"parent_record_offset":16184,"ancestor_record_offset":5924,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":13356,"parent_record_offset":13324,"ancestor_record_offset":5924,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3842,"record_offset":17672,"parent_record_offset":17648,"ancestor_record_offset":5924,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3842,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":5924,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3842,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":5924,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3842,"record_offset":17960,"parent_record_offset":17852,"ancestor_record_offset":5924,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3842,"record_offset":18344,"parent_record_offset":17960,"ancestor_record_offset":5924,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3842,"record_offset":17924,"parent_record_offset":17900,"ancestor_record_offset":5924,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3842,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":5924,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":13728,"parent_record_offset":13696,"ancestor_record_offset":5924,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":13696,"parent_record_offset":13668,"ancestor_record_offset":5924,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3842,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":5924,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":13484,"parent_record_offset":13452,"ancestor_record_offset":5924,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3842,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":5924,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3842,"record_offset":18004,"parent_record_offset":17960,"ancestor_record_offset":5924,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3842,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":5924,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":10848,"parent_record_offset":10812,"ancestor_record_offset":5924,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":5924,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":5924,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":12992,"parent_record_offset":12768,"ancestor_record_offset":5924,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":8992,"parent_record_offset":8964,"ancestor_record_offset":5924,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3842,"record_offset":10904,"parent_record_offset":10872,"ancestor_record_offset":5924,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":5924,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12880,"parent_record_offset":12768,"ancestor_record_offset":5924,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":5924,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9404,"parent_record_offset":9052,"ancestor_record_offset":5924,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":13200,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9840,"parent_record_offset":9780,"ancestor_record_offset":5924,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9052,"parent_record_offset":9020,"ancestor_record_offset":5924,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":5924,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":5924,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12348,"parent_record_offset":10872,"ancestor_record_offset":5924,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":5924,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":5924,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":5924,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":5924,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3842,"record_offset":9140,"parent_record_offset":9104,"ancestor_record_offset":5924,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3842,"record_offset":9176,"parent_record_offset":9140,"ancestor_record_offset":5924,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3842,"record_offset":9236,"parent_record_offset":9104,"ancestor_record_offset":5924,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3842,"record_offset":9104,"parent_record_offset":9052,"ancestor_record_offset":5924,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12448,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":9020,"parent_record_offset":8992,"ancestor_record_offset":5924,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3842,"record_offset":12768,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3842,"record_offset":12476,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3842,"record_offset":9272,"parent_record_offset":9052,"ancestor_record_offset":5924,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3842,"record_offset":12560,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3842,"record_offset":13108,"parent_record_offset":12396,"ancestor_record_offset":5924,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3842,"record_offset":9332,"parent_record_offset":9272,"ancestor_record_offset":5924,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":7844,"parent_record_offset":7808,"ancestor_record_offset":5924,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3842,"record_offset":7956,"parent_record_offset":7916,"ancestor_record_offset":5924,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":8120,"parent_record_offset":7916,"ancestor_record_offset":5924,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3842,"record_offset":8388,"parent_record_offset":8120,"ancestor_record_offset":5924,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":10812,"parent_record_offset":10780,"ancestor_record_offset":5924,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":5924,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3842,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":5924,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3842,"record_offset":13760,"parent_record_offset":13728,"ancestor_record_offset":5924,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3842,"record_offset":10960,"parent_record_offset":10928,"ancestor_record_offset":5924,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3842,"record_offset":11284,"parent_record_offset":10928,"ancestor_record_offset":5924,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3842,"record_offset":11212,"parent_record_offset":11156,"ancestor_record_offset":5924,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3842,"record_offset":7984,"parent_record_offset":7956,"ancestor_record_offset":5924,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3842,"record_offset":8016,"parent_record_offset":7984,"ancestor_record_offset":5924,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3842,"record_offset":7916,"parent_record_offset":7880,"ancestor_record_offset":5924,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3842,"record_offset":7880,"parent_record_offset":7844,"ancestor_record_offset":5924,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws
// 记录形式: compiled_procedure; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3842,"record_offset":21268,"rva":29624080,"va":5398333200}
// 编译实例 RVAs: 0x1c40710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26633407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e8419b69fe","size":5,"soft_fail":false,"target_va":5371700720,"va":5398334122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25583646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e8e29f79fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398333337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25584089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e8279e79fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398333796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d019a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15986911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e8210f0cff","size":5,"soft_fail":false,"target_va":5382347168,"va":5398334074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4644184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e858dd4600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398333955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e8c6ceffff","size":5,"soft_fail":false,"target_va":5398320880,"va":5398333477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4645423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398333200,"raw_hex":"e82fe24600","size":5,"soft_fail":false,"target_va":5402979344,"va":5398333916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":21460,"parent_record_offset":21268,"ancestor_record_offset":21268,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":21844,"parent_record_offset":21268,"ancestor_record_offset":21268,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3842,"record_offset":21728,"parent_record_offset":21268,"ancestor_record_offset":21268,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3842,"record_offset":21940,"parent_record_offset":21268,"ancestor_record_offset":21268,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":22268,"parent_record_offset":21268,"ancestor_record_offset":21268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":21596,"parent_record_offset":21572,"ancestor_record_offset":21268,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":21620,"parent_record_offset":21596,"ancestor_record_offset":21268,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":22540,"parent_record_offset":22516,"ancestor_record_offset":21268,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":21572,"parent_record_offset":21544,"ancestor_record_offset":21268,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":22516,"parent_record_offset":22416,"ancestor_record_offset":21268,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":22448,"parent_record_offset":22416,"ancestor_record_offset":21268,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":21544,"parent_record_offset":21516,"ancestor_record_offset":21268,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":21516,"parent_record_offset":21488,"ancestor_record_offset":21268,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":21688,"parent_record_offset":21516,"ancestor_record_offset":21268,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":22416,"parent_record_offset":22388,"ancestor_record_offset":21268,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":21488,"parent_record_offset":21460,"ancestor_record_offset":21268,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":21268,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3842,"record_offset":21800,"parent_record_offset":21776,"ancestor_record_offset":21268,"inlinee":41851,"source_line":1169,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":3842,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":21268,"inlinee":41849,"source_line":372,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3842,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":21268,"inlinee":41850,"source_line":1304,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3842,"record_offset":21964,"parent_record_offset":21940,"ancestor_record_offset":21268,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3842,"record_offset":22480,"parent_record_offset":22448,"ancestor_record_offset":21268,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":22316,"parent_record_offset":22292,"ancestor_record_offset":21268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":21992,"parent_record_offset":21964,"ancestor_record_offset":21268,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":21268,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":22020,"parent_record_offset":21964,"ancestor_record_offset":21268,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3842,"record_offset":21872,"parent_record_offset":21844,"ancestor_record_offset":21268,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3842,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":21268,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B489
// 首条位置: {"module":3842,"inlinee":111753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":864,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":1444,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3842,"record_offset":1580,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::enumerate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3842,"record_offset":1388,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":16076,"source_line":976,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3842,"record_offset":976,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":2480,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3842,"record_offset":892,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":3842,"record_offset":1064,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":2632,"parent_record_offset":2532,"ancestor_record_offset":764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":2564,"parent_record_offset":2532,"ancestor_record_offset":764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":1516,"parent_record_offset":1484,"ancestor_record_offset":764,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3842,"record_offset":2532,"parent_record_offset":2504,"ancestor_record_offset":764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3842,"record_offset":1484,"parent_record_offset":1444,"ancestor_record_offset":764,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3842,"record_offset":1984,"parent_record_offset":1612,"ancestor_record_offset":764,"inlinee":6771,"source_line":160,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3842,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":764,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::Enumerate::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3842,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":764,"inlinee":16078,"source_line":23,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3842,"record_offset":1016,"parent_record_offset":976,"ancestor_record_offset":764,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3842,"record_offset":1140,"parent_record_offset":1096,"ancestor_record_offset":764,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3842,"record_offset":1324,"parent_record_offset":1096,"ancestor_record_offset":764,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3842,"record_offset":1860,"parent_record_offset":1828,"ancestor_record_offset":764,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3842,"record_offset":2596,"parent_record_offset":2564,"ancestor_record_offset":764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3842,"record_offset":1292,"parent_record_offset":1140,"ancestor_record_offset":764,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3842,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":764,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":764,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3842,"record_offset":1204,"parent_record_offset":1168,"ancestor_record_offset":764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":1892,"parent_record_offset":1860,"ancestor_record_offset":764,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":1548,"parent_record_offset":1444,"ancestor_record_offset":764,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3842,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":764,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3842,"record_offset":1232,"parent_record_offset":1168,"ancestor_record_offset":764,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3842,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":764,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$45::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3842,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":764,"inlinee":60072,"source_line":18,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3842,"record_offset":1828,"parent_record_offset":1612,"ancestor_record_offset":764,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3842,"record_offset":1612,"parent_record_offset":1580,"ancestor_record_offset":764,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3842,"record_offset":1096,"parent_record_offset":1064,"ancestor_record_offset":764,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F2B
// 首条位置: {"module":2764,"inlinee":89899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2764,"record_offset":18872,"parent_record_offset":18784,"ancestor_record_offset":18784,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2764,"record_offset":18932,"parent_record_offset":18784,"ancestor_record_offset":18784,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2764,"record_offset":18900,"parent_record_offset":18784,"ancestor_record_offset":18784,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2764,"record_offset":19660,"parent_record_offset":18784,"ancestor_record_offset":18784,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2764,"record_offset":18968,"parent_record_offset":18784,"ancestor_record_offset":18784,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2764,"record_offset":19120,"parent_record_offset":18992,"ancestor_record_offset":18784,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2764,"record_offset":19204,"parent_record_offset":18992,"ancestor_record_offset":18784,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2764,"record_offset":20080,"parent_record_offset":19660,"ancestor_record_offset":18784,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2764,"record_offset":19036,"parent_record_offset":18992,"ancestor_record_offset":18784,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2764,"record_offset":19896,"parent_record_offset":19864,"ancestor_record_offset":18784,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2764,"record_offset":19864,"parent_record_offset":19836,"ancestor_record_offset":18784,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2764,"record_offset":19836,"parent_record_offset":19660,"ancestor_record_offset":18784,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2764,"record_offset":19716,"parent_record_offset":19660,"ancestor_record_offset":18784,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2764,"record_offset":19996,"parent_record_offset":19660,"ancestor_record_offset":18784,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2764,"record_offset":18992,"parent_record_offset":18968,"ancestor_record_offset":18784,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2764,"record_offset":19928,"parent_record_offset":19896,"ancestor_record_offset":18784,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02764.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_wall_window::get_window_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B48B
// 首条位置: {"module":3842,"inlinee":111755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18904,"parent_record_offset":18852,"ancestor_record_offset":18852,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18852,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18852,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3842,"record_offset":18932,"parent_record_offset":18904,"ancestor_record_offset":18852,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03842.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3842,"record_offset":19008,"parent_record_offset":18980,"ancestor_record_offset":18852,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03842.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_wall_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_wall_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_wall_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws
// 信息 ID: 1cc2acb9e5947df545aa269f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17aa33a -> 0x1c40710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0
// 信息 ID: 1dd61a64ec79baead1ef82ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0 <- core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env$0> >; module=3842, inline_record=764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure$0
// 信息 ID: 020f26b9c5edd6dbb84b9dab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0>; module=2764, inline_record=18784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows
// 信息 ID: cb44f6021ac11b67b8bc746c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cece2b7d32cab5f8619d1680: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17cd15d -> 0x1c3e1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x17d34b0 -> 0x1c3e1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
// 信息 ID: c5764c4eb76817ffd623f50a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17aa22d -> 0x1c40ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x1c3e9bd -> 0x1c40ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x1c49baa -> 0x1c40ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x1c510e4 -> 0x1c40ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_wall_window::get_window_holes
// 信息 ID: 9f032a93030344630045cc9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=18852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
