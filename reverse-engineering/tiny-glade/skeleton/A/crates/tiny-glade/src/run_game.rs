// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/run_game.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5c970a1e0b024e7a760f3941179bf004809d2a7a8e402f992989cd15fe017b57
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: run_game.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 66, 107, 134, 245, 256
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use bevy_state; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"PDB_registry_path_only","version":"1.1.2"}]
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::select_surface_format
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35E7
// 首条位置: {"module":256,"inlinee":13799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":35916,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":36400,"parent_record_offset":35916,"ancestor_record_offset":35916,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":36564,"parent_record_offset":36540,"ancestor_record_offset":35916,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":36540,"parent_record_offset":36448,"ancestor_record_offset":35916,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":36476,"parent_record_offset":36448,"ancestor_record_offset":35916,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":36448,"parent_record_offset":36424,"ancestor_record_offset":35916,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$7522::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":256,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":35916,"inlinee":8059,"source_line":8940,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":256,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":35916,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35992,"parent_record_offset":35968,"ancestor_record_offset":35916,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":35916,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":36504,"parent_record_offset":36476,"ancestor_record_offset":35916,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":36424,"parent_record_offset":36400,"ancestor_record_offset":35916,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":35916,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36176,"parent_record_offset":36124,"ancestor_record_offset":35916,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":35916,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36276,"parent_record_offset":36088,"ancestor_record_offset":35916,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":35916,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36208,"parent_record_offset":36124,"ancestor_record_offset":35916,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":35916,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36124,"parent_record_offset":36088,"ancestor_record_offset":35916,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":35916,"inlinee":13801,"source_line":152,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::select_surface_format::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35E9
// 首条位置: {"module":256,"inlinee":13801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":36016,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$7522::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":256,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":36016,"inlinee":8059,"source_line":8940,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":256,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":36016,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":36016,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36176,"parent_record_offset":36124,"ancestor_record_offset":36016,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":36016,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36276,"parent_record_offset":36088,"ancestor_record_offset":36016,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36208,"parent_record_offset":36124,"ancestor_record_offset":36016,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":36016,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36124,"parent_record_offset":36088,"ancestor_record_offset":36016,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game
// 记录形式: compiled_procedure; instances=1
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":256,"record_offset":15232,"rva":3953456,"va":5372662576}
// 编译实例 RVAs: 0x3c5330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_app..sub_app..SubApp$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4301799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e7a34100","size":5,"soft_fail":false,"target_va":5376981936,"va":5372680132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..context..Context$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3758985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8895b3900","size":5,"soft_fail":false,"target_va":5376423888,"va":5372664898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t601333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8f52c0900","size":5,"soft_fail":false,"target_va":5373265408,"va":5372664070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3040798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e299d1ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5372673241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3026019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e89dd3d1ff","size":5,"soft_fail":false,"target_va":5369637552,"va":5372663566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=4
//     target_aliases=25 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3009538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8fe13d2ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5372683277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401770c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2418400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e82019dbff","size":5,"soft_fail":false,"target_va":5370245312,"va":5372663707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1013573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8bb88f0ff","size":5,"soft_fail":false,"target_va":5371652896,"va":5372666464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d06f0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1016349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e37df0ff","size":5,"soft_fail":false,"target_va":5371660016,"va":5372676360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-996667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c5caf0ff","size":5,"soft_fail":false,"target_va":5371700800,"va":5372697462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402dcae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-973224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85826f1ff","size":5,"soft_fail":false,"target_va":5371710176,"va":5372683395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f7770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-848545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85f0df3ff","size":5,"soft_fail":false,"target_va":5371819888,"va":5372668428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403466a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-523070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c204f8ff","size":5,"soft_fail":false,"target_va":5372143264,"va":5372666329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035c7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-459052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d4fef8ff","size":5,"soft_fail":false,"target_va":5372233712,"va":5372692759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-447656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8582bf9ff","size":5,"soft_fail":false,"target_va":5372239520,"va":5372687171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-442787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85d3ef9ff","size":5,"soft_fail":false,"target_va":5372242288,"va":5372685070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035f4f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-434041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e88760f9ff","size":5,"soft_fail":false,"target_va":5372245232,"va":5372679268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360190
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-441514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85643f9ff","size":5,"soft_fail":false,"target_va":5372248464,"va":5372689973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360ee0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-424526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8b285f9ff","size":5,"soft_fail":false,"target_va":5372251872,"va":5372676393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-419668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8ac98f9ff","size":5,"soft_fail":false,"target_va":5372267680,"va":5372687343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403656e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-417220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e83ca2f9ff","size":5,"soft_fail":false,"target_va":5372270304,"va":5372687519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-413453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8f3b0f9ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5372687864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140367de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-407869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c3c6f9ff","size":5,"soft_fail":false,"target_va":5372280288,"va":5372688152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368730
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-396535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e809f3f9ff","size":5,"soft_fail":false,"target_va":5372282672,"va":5372679202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-369266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e88e5dfaff","size":5,"soft_fail":false,"target_va":5372309648,"va":5372678909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-370102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e84a5afaff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372684305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403706c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-372372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e86c51faff","size":5,"soft_fail":false,"target_va":5372315328,"va":5372687695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-367128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e865faff","size":5,"soft_fail":false,"target_va":5372321360,"va":5372688483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-333626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c6e8faff","size":5,"soft_fail":false,"target_va":5372345728,"va":5372679349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037b500
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-319018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d621fbff","size":5,"soft_fail":false,"target_va":5372359936,"va":5372678949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037c570
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-308580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e89c4afbff","size":5,"soft_fail":false,"target_va":5372364144,"va":5372672719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037eb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-309720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e82846fbff","size":5,"soft_fail":false,"target_va":5372373808,"va":5372683523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037f2c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-315432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d82ffbff","size":5,"soft_fail":false,"target_va":5372375744,"va":5372691171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-284962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8dea6fbff","size":5,"soft_fail":false,"target_va":5372378176,"va":5372663133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403815d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-306623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e84152fbff","size":5,"soft_fail":false,"target_va":5372384720,"va":5372691338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-286248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d8a1fbff","size":5,"soft_fail":false,"target_va":5372392992,"va":5372679235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403844e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-295144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8187ffbff","size":5,"soft_fail":false,"target_va":5372396768,"va":5372691907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384540
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-280152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a8b9fbff","size":5,"soft_fail":false,"target_va":5372396864,"va":5372677011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140386740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-291666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8ae8cfbff","size":5,"soft_fail":false,"target_va":5372405568,"va":5372697229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-270393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c7dffbff","size":5,"soft_fail":false,"target_va":5372408976,"va":5372679364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038eb90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-239594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e81658fcff","size":5,"soft_fail":false,"target_va":5372439440,"va":5372679029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038fec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-234642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e86e6bfcff","size":5,"soft_fail":false,"target_va":5372444352,"va":5372678989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403901b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-240280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e86855fcff","size":5,"soft_fail":false,"target_va":5372445104,"va":5372685379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-230314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8567cfcff","size":5,"soft_fail":false,"target_va":5372448800,"va":5372679109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391310
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-242526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a24cfcff","size":5,"soft_fail":false,"target_va":5372449552,"va":5372692073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392b60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-223298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8be97fcff","size":5,"soft_fail":false,"target_va":5372455776,"va":5372679069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e854530100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372662823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=44
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8ff500100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372663436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t81165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e80d3d0100","size":5,"soft_fail":false,"target_va":5372749728,"va":5372668558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140513330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1362341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a5c91400","size":5,"soft_fail":false,"target_va":5374030640,"va":5372668294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067b3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2839821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e80d552b00","size":5,"soft_fail":false,"target_va":5375505360,"va":5372665534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067fbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2858103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8779c2b00","size":5,"soft_fail":false,"target_va":5375523824,"va":5372665716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2862420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e854ad2b00","size":5,"soft_fail":false,"target_va":5375527792,"va":5372665367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3534628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e824ef3500","size":5,"soft_fail":false,"target_va":5376212272,"va":5372677639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3565494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8b6673600","size":5,"soft_fail":false,"target_va":5376239920,"va":5372674421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3582569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e869aa3600","size":5,"soft_fail":false,"target_va":5376258656,"va":5372676082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140743de0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3652993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e881bd3700","size":5,"soft_fail":false,"target_va":5376327136,"va":5372674138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30237514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e84a63cd01","size":5,"soft_fail":false,"target_va":5402900112,"va":5372662593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30922359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e877d6d701","size":5,"soft_fail":false,"target_va":5403593344,"va":5372670980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30930499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e843f6d701","size":5,"soft_fail":false,"target_va":5403593360,"va":5372662856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30921738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e80ad4d701","size":5,"soft_fail":false,"target_va":5403593376,"va":5372671633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t30296865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8214bce01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372681274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30243444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8747acd01","size":5,"soft_fail":false,"target_va":5402919952,"va":5372676503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1364277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8cb2eebff","size":5,"soft_fail":false,"target_va":5371309360,"va":5372673632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4845071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e80fee4900","size":5,"soft_fail":false,"target_va":5377527120,"va":5372682044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t-1021834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e87668f0ff","size":5,"soft_fail":false,"target_va":5371657040,"va":5372678869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30296226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a248ce01","size":5,"soft_fail":false,"target_va":5402978432,"va":5372682201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4686106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e81a814700","size":5,"soft_fail":false,"target_va":5377364336,"va":5372678225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4297339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e87b924100","size":5,"soft_fail":false,"target_va":5376962544,"va":5372665200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30297021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8bd4bce01","size":5,"soft_fail":false,"target_va":5402978912,"va":5372681886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30298190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e84e50ce01","size":5,"soft_fail":false,"target_va":5402979344,"va":5372681149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30297685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8554ece01","size":5,"soft_fail":false,"target_va":5402978944,"va":5372681254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-36647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d970ffff","size":5,"soft_fail":false,"target_va":5372657456,"va":5372694098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c897ffff","size":5,"soft_fail":false,"target_va":5372657488,"va":5372684163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e820acffff","size":5,"soft_fail":false,"target_va":5372658448,"va":5372679915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-19561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e897b3ffff","size":5,"soft_fail":false,"target_va":5372657552,"va":5372677108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85884ffff","size":5,"soft_fail":false,"target_va":5372658128,"va":5372689779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30298576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d051ce01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372681547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui_winit::State::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t448388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e884d70600","size":5,"soft_fail":false,"target_va":5373113376,"va":5372664983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=25
//     evidence={"assembly":"\tcallq\t4689495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8578e4700","size":5,"soft_fail":false,"target_va":5377353776,"va":5372664276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3583083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e86bac3600","size":5,"soft_fail":false,"target_va":5376254928,"va":5372671840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3574856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8488c3600","size":5,"soft_fail":false,"target_va":5376254784,"va":5372679923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_supported
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3585115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e85bb43600","size":5,"soft_fail":false,"target_va":5376254608,"va":5372669488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsDevicesForSurface::available_devices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3590423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e817c93600","size":5,"soft_fail":false,"target_va":5376257920,"va":5372667492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsDevicesForSurface::create_device
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3585730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c2b63600","size":5,"soft_fail":false,"target_va":5376257968,"va":5372672233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3588585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e9c13600","size":5,"soft_fail":false,"target_va":5376256256,"va":5372667666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsDevicesForSurface::order_and_filter_devices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3590853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c5ca3600","size":5,"soft_fail":false,"target_va":5376258592,"va":5372667734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3534545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8d1ee3500","size":5,"soft_fail":false,"target_va":5376212240,"va":5372677690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::enumerate_surface_formats
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3603317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e875fb3600","size":5,"soft_fail":false,"target_va":5376277008,"va":5372673686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30308002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a276ce01","size":5,"soft_fail":false,"target_va":5402975328,"va":5372667321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2900727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e809bdd3ff","size":5,"soft_fail":false,"target_va":5369765248,"va":5372665970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::with_capacity
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2901561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c7b9d3ff","size":5,"soft_fail":false,"target_va":5369761216,"va":5372662772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-690206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e277f5ff","size":5,"soft_fail":false,"target_va":5371975728,"va":5372665929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-152128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c0adfdff","size":5,"soft_fail":false,"target_va":5372513888,"va":5372666011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30308330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8ea77ce01","size":5,"soft_fail":false,"target_va":5402975696,"va":5372667361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4729971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8732c4800","size":5,"soft_fail":false,"target_va":5377393008,"va":5372663032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1992563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e88d98e1ff","size":5,"soft_fail":false,"target_va":5370670512,"va":5372663070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2014019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8bd44e1ff","size":5,"soft_fail":false,"target_va":5370666880,"va":5372680894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::run_gpu_selection_gui
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t175434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e84aad0200","size":5,"soft_fail":false,"target_va":5372845552,"va":5372670113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::ClientHandle::send_message
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-86100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8acaffeff","size":5,"soft_fail":false,"target_va":5372593808,"va":5372679903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::event_loop::EguiEventIo::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-751934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8c286f4ff","size":5,"soft_fail":false,"target_va":5371913104,"va":5372665033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::event_loop::run_event_loop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-766354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e86e4ef4ff","size":5,"soft_fail":false,"target_va":5371914496,"va":5372680845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::setup::setup_window
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2988264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e81867d2ff","size":5,"soft_fail":false,"target_va":5369675520,"va":5372663779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_max_resolution_scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1141067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8b596eeff","size":5,"soft_fail":false,"target_va":5371531440,"va":5372672502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-469341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8a3d6f8ff","size":5,"soft_fail":false,"target_va":5372193920,"va":5372663256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-467998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8e2dbf8ff","size":5,"soft_fail":false,"target_va":5372200192,"va":5372668185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t590358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e816020900","size":5,"soft_fail":false,"target_va":5373265824,"va":5372675461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t586161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e8b1f10800","size":5,"soft_fail":false,"target_va":5373253168,"va":5372667002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_fullscreen
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-9428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372662576,"raw_hex":"e82cdbffff","size":5,"soft_fail":false,"target_va":5372658672,"va":5372668095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372662576,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5372663407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":49320,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":47240,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":46872,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":17660,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":19620,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":18012,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":51400,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":19448,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":18568,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":50776,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":20380,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":48024,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":20728,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":17524,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":256,"record_offset":19504,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":6618,"source_line":678,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":18596,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":17928,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":256,"record_offset":17624,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":20624,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":7941,"source_line":848,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":18624,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":7943,"source_line":525,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":51368,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":256,"record_offset":49528,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":12242,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::event_loop::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":256,"record_offset":51004,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":13804,"source_line":65,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::async_load_audio_assets
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":15456,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":13779,"source_line":325,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::create_graphics_device_and_swapchain
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":20932,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":13781,"source_line":1016,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":18364,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":256,"record_offset":16840,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":8182,"source_line":200,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":18040,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15752,"parent_record_offset":15728,"ancestor_record_offset":15232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15776,"parent_record_offset":15752,"ancestor_record_offset":15232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":15232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15696,"parent_record_offset":15656,"ancestor_record_offset":15232,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15728,"parent_record_offset":15696,"ancestor_record_offset":15232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":15232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":27552,"parent_record_offset":27416,"ancestor_record_offset":15232,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":23332,"parent_record_offset":23116,"ancestor_record_offset":15232,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":27576,"parent_record_offset":27552,"ancestor_record_offset":15232,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":15656,"parent_record_offset":15620,"ancestor_record_offset":15232,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23116,"parent_record_offset":23028,"ancestor_record_offset":15232,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23152,"parent_record_offset":23116,"ancestor_record_offset":15232,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23184,"parent_record_offset":23152,"ancestor_record_offset":15232,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":17196,"parent_record_offset":17164,"ancestor_record_offset":15232,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27348,"parent_record_offset":27324,"ancestor_record_offset":15232,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27460,"parent_record_offset":27416,"ancestor_record_offset":15232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":41428,"parent_record_offset":41396,"ancestor_record_offset":15232,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":41396,"parent_record_offset":41368,"ancestor_record_offset":15232,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27416,"parent_record_offset":27380,"ancestor_record_offset":15232,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27380,"parent_record_offset":27284,"ancestor_record_offset":15232,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16204,"parent_record_offset":16168,"ancestor_record_offset":15232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16168,"parent_record_offset":16132,"ancestor_record_offset":15232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16360,"parent_record_offset":16168,"ancestor_record_offset":15232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16720,"parent_record_offset":16696,"ancestor_record_offset":15232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16088,"parent_record_offset":16052,"ancestor_record_offset":15232,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16052,"parent_record_offset":16016,"ancestor_record_offset":15232,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":27088,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":15980,"parent_record_offset":15928,"ancestor_record_offset":15232,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16016,"parent_record_offset":15980,"ancestor_record_offset":15232,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":15928,"parent_record_offset":15892,"ancestor_record_offset":15232,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":27112,"parent_record_offset":27088,"ancestor_record_offset":15232,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":15892,"parent_record_offset":15456,"ancestor_record_offset":15232,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":46924,"parent_record_offset":46872,"ancestor_record_offset":15232,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":51436,"parent_record_offset":51400,"ancestor_record_offset":15232,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":32044,"parent_record_offset":32016,"ancestor_record_offset":15232,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21928,"parent_record_offset":21892,"ancestor_record_offset":15232,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":15232,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":22736,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":15232,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41328,"parent_record_offset":41288,"ancestor_record_offset":15232,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27324,"parent_record_offset":27284,"ancestor_record_offset":15232,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26576,"parent_record_offset":26544,"ancestor_record_offset":15232,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33416,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":8652,"source_line":1535,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27232,"parent_record_offset":27188,"ancestor_record_offset":15232,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24464,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41508,"parent_record_offset":41328,"ancestor_record_offset":15232,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":15232,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33572,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33276,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41368,"parent_record_offset":41328,"ancestor_record_offset":15232,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27284,"parent_record_offset":27232,"ancestor_record_offset":15232,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26188,"parent_record_offset":26132,"ancestor_record_offset":15232,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":16132,"parent_record_offset":16088,"ancestor_record_offset":15232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26052,"parent_record_offset":26012,"ancestor_record_offset":15232,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33516,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41252,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26812,"parent_record_offset":26788,"ancestor_record_offset":15232,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":29020,"parent_record_offset":28992,"ancestor_record_offset":15232,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":15232,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27188,"parent_record_offset":27144,"ancestor_record_offset":15232,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":15232,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":40808,"parent_record_offset":40784,"ancestor_record_offset":15232,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":33684,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":256,"record_offset":32072,"parent_record_offset":32044,"ancestor_record_offset":15232,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":256,"record_offset":31788,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":13790,"source_line":18,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":256,"record_offset":26868,"parent_record_offset":26812,"ancestor_record_offset":15232,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":256,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":15232,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":256,"record_offset":26544,"parent_record_offset":26132,"ancestor_record_offset":15232,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":256,"record_offset":41288,"parent_record_offset":41252,"ancestor_record_offset":15232,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":256,"record_offset":26092,"parent_record_offset":26052,"ancestor_record_offset":15232,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":256,"record_offset":26132,"parent_record_offset":26092,"ancestor_record_offset":15232,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$7522::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":256,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":15232,"inlinee":8059,"source_line":8940,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":256,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":15232,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":26324,"parent_record_offset":26288,"ancestor_record_offset":15232,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":17224,"parent_record_offset":17196,"ancestor_record_offset":15232,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":15232,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28080,"parent_record_offset":28056,"ancestor_record_offset":15232,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":15232,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":256,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":15232,"inlinee":10524,"source_line":219,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":31992,"parent_record_offset":31872,"ancestor_record_offset":15232,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":22652,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":31956,"parent_record_offset":31932,"ancestor_record_offset":15232,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":15232,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":256,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":15232,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":18316,"parent_record_offset":18144,"ancestor_record_offset":15232,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":16556,"parent_record_offset":16524,"ancestor_record_offset":15232,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":33940,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":46952,"parent_record_offset":46872,"ancestor_record_offset":15232,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":15232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":15232,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":256,"record_offset":26640,"parent_record_offset":26608,"ancestor_record_offset":15232,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":256,"record_offset":26708,"parent_record_offset":26672,"ancestor_record_offset":15232,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":33012,"parent_record_offset":32972,"ancestor_record_offset":15232,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":15232,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":31908,"parent_record_offset":31872,"ancestor_record_offset":15232,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26012,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":15232,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35992,"parent_record_offset":35968,"ancestor_record_offset":15232,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":32904,"parent_record_offset":32880,"ancestor_record_offset":15232,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26608,"parent_record_offset":26576,"ancestor_record_offset":15232,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26788,"parent_record_offset":26708,"ancestor_record_offset":15232,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":15232,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":256,"record_offset":18228,"parent_record_offset":18204,"ancestor_record_offset":15232,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":15232,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":17248,"parent_record_offset":17224,"ancestor_record_offset":15232,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":28556,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":18204,"parent_record_offset":18144,"ancestor_record_offset":15232,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":33040,"parent_record_offset":33012,"ancestor_record_offset":15232,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":27484,"parent_record_offset":27460,"ancestor_record_offset":15232,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":256,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":15232,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":256,"record_offset":24180,"parent_record_offset":24148,"ancestor_record_offset":15232,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":23896,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":24120,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":36612,"parent_record_offset":35724,"ancestor_record_offset":15232,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":23856,"parent_record_offset":22844,"ancestor_record_offset":15232,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":24148,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":22944,"parent_record_offset":22884,"ancestor_record_offset":15232,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":16400,"parent_record_offset":16088,"ancestor_record_offset":15232,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=199
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":16596,"parent_record_offset":15456,"ancestor_record_offset":15232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":33316,"parent_record_offset":33276,"ancestor_record_offset":15232,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":15232,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":15232,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":15232,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":21960,"parent_record_offset":21928,"ancestor_record_offset":15232,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":33740,"parent_record_offset":33684,"ancestor_record_offset":15232,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":27996,"parent_record_offset":27952,"ancestor_record_offset":15232,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":28608,"parent_record_offset":28580,"ancestor_record_offset":15232,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":15804,"parent_record_offset":15776,"ancestor_record_offset":15232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":26840,"parent_record_offset":26812,"ancestor_record_offset":15232,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":46980,"parent_record_offset":46872,"ancestor_record_offset":15232,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":33800,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":20688,"parent_record_offset":20648,"ancestor_record_offset":15232,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":28756,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":16480,"parent_record_offset":15456,"ancestor_record_offset":15232,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":17024,"parent_record_offset":16940,"ancestor_record_offset":15232,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":22060,"parent_record_offset":21720,"ancestor_record_offset":15232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::SlicePartialEq::not_equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31844,"parent_record_offset":31816,"ancestor_record_offset":15232,"inlinee":13793,"source_line":48,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":32100,"parent_record_offset":32072,"ancestor_record_offset":15232,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31816,"parent_record_offset":31788,"ancestor_record_offset":15232,"inlinee":13791,"source_line":19,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":15232,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36276,"parent_record_offset":36088,"ancestor_record_offset":15232,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31872,"parent_record_offset":31844,"ancestor_record_offset":15232,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":32128,"parent_record_offset":32100,"ancestor_record_offset":15232,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":15232,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":28332,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36208,"parent_record_offset":36124,"ancestor_record_offset":15232,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":15232,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":26672,"parent_record_offset":26640,"ancestor_record_offset":15232,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":27952,"parent_record_offset":27916,"ancestor_record_offset":15232,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":27916,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23696,"parent_record_offset":23520,"ancestor_record_offset":15232,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":16968,"parent_record_offset":16940,"ancestor_record_offset":15232,"inlinee":8187,"source_line":669,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":15232,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":19556,"parent_record_offset":19528,"ancestor_record_offset":15232,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":48132,"parent_record_offset":48096,"ancestor_record_offset":15232,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":15232,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":19580,"parent_record_offset":19556,"ancestor_record_offset":15232,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":17976,"parent_record_offset":17952,"ancestor_record_offset":15232,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23720,"parent_record_offset":23696,"ancestor_record_offset":15232,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":21988,"parent_record_offset":21928,"ancestor_record_offset":15232,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":16992,"parent_record_offset":16968,"ancestor_record_offset":15232,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":22020,"parent_record_offset":21892,"ancestor_record_offset":15232,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":256,"record_offset":32016,"parent_record_offset":31992,"ancestor_record_offset":15232,"inlinee":12086,"source_line":68,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":256,"record_offset":51044,"parent_record_offset":51004,"ancestor_record_offset":15232,"inlinee":6618,"source_line":678,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":22708,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":22568,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":256,"record_offset":35096,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":13795,"source_line":423,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":256,"record_offset":35164,"parent_record_offset":35096,"ancestor_record_offset":15232,"inlinee":13797,"source_line":42,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":15232,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":15232,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":20648,"parent_record_offset":20624,"ancestor_record_offset":15232,"inlinee":7891,"source_line":304,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":18664,"parent_record_offset":18624,"ancestor_record_offset":15232,"inlinee":7894,"source_line":51,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":256,"record_offset":15620,"parent_record_offset":15584,"ancestor_record_offset":15232,"inlinee":7898,"source_line":21,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::list::Channel::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\list.rs
//     evidence={"module":256,"record_offset":18704,"parent_record_offset":18664,"ancestor_record_offset":15232,"inlinee":7896,"source_line":160,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":15584,"parent_record_offset":15548,"ancestor_record_offset":15232,"inlinee":9960,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":15548,"parent_record_offset":15456,"ancestor_record_offset":15232,"inlinee":7939,"source_line":574,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":256,"record_offset":23488,"parent_record_offset":23460,"ancestor_record_offset":15232,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":256,"record_offset":23520,"parent_record_offset":23488,"ancestor_record_offset":15232,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":256,"record_offset":23756,"parent_record_offset":23488,"ancestor_record_offset":15232,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":15864,"parent_record_offset":15456,"ancestor_record_offset":15232,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":23552,"parent_record_offset":23520,"ancestor_record_offset":15232,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::async_load_audio_assets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":16524,"parent_record_offset":16480,"ancestor_record_offset":15232,"inlinee":13805,"source_line":341,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":15232,"inlinee":13784,"source_line":1035,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":24264,"parent_record_offset":24148,"ancestor_record_offset":15232,"inlinee":13785,"source_line":1048,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":28032,"parent_record_offset":27916,"ancestor_record_offset":15232,"inlinee":13789,"source_line":1103,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_swapchain_with_fallback
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":37196,"parent_record_offset":35724,"ancestor_record_offset":15232,"inlinee":13802,"source_line":1296,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::run_game::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":20408,"parent_record_offset":20380,"ancestor_record_offset":15232,"inlinee":13780,"source_line":228,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":35916,"parent_record_offset":35724,"ancestor_record_offset":15232,"inlinee":13799,"source_line":136,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":15232,"inlinee":13801,"source_line":152,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::try_get_devices_for_surface
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":21720,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":13782,"source_line":1261,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::try_init_swapchain
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":35724,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":13798,"source_line":1270,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":256,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":15232,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":18396,"parent_record_offset":18364,"ancestor_record_offset":15232,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":34496,"parent_record_offset":34468,"ancestor_record_offset":15232,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":256,"record_offset":16940,"parent_record_offset":16840,"ancestor_record_offset":15232,"inlinee":8186,"source_line":120,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":256,"record_offset":16884,"parent_record_offset":16840,"ancestor_record_offset":15232,"inlinee":8184,"source_line":69,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":256,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":15232,"inlinee":8185,"source_line":80,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":25528,"parent_record_offset":25504,"ancestor_record_offset":15232,"inlinee":10684,"source_line":46,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":22988,"parent_record_offset":22944,"ancestor_record_offset":15232,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$10::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":23060,"parent_record_offset":23028,"ancestor_record_offset":15232,"inlinee":10522,"source_line":87,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":23028,"parent_record_offset":22988,"ancestor_record_offset":15232,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":15232,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":22884,"parent_record_offset":22844,"ancestor_record_offset":15232,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":18144,"parent_record_offset":18112,"ancestor_record_offset":15232,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":15232,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":18076,"parent_record_offset":18040,"ancestor_record_offset":15232,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_maximized
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":15232,"inlinee":10563,"source_line":657,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39596,"parent_record_offset":37196,"ancestor_record_offset":15232,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":22764,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":34156,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_maximized
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25804,"parent_record_offset":20932,"ancestor_record_offset":15232,"inlinee":13786,"source_line":1045,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":18112,"parent_record_offset":18076,"ancestor_record_offset":15232,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":22844,"parent_record_offset":22804,"ancestor_record_offset":15232,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25480,"parent_record_offset":25452,"ancestor_record_offset":15232,"inlinee":10682,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_maximized::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":15232,"inlinee":13788,"source_line":1047,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":15232,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1595
// 首条位置: {"module":14,"inlinee":5525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":3752,"parent_record_offset":3644,"ancestor_record_offset":3644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":3688,"parent_record_offset":3644,"ancestor_record_offset":3644,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3644,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3644,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3644,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3644,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":14,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":3644,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":14,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3644,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::run_game::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":14,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3644,"inlinee":5527,"source_line":184,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1597
// 首条位置: {"module":14,"inlinee":5527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D4
// 首条位置: {"module":256,"inlinee":13780}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1598
// 首条位置: {"module":14,"inlinee":5528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":5140,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":5204,"parent_record_offset":5168,"ancestor_record_offset":5060,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":5168,"parent_record_offset":5140,"ancestor_record_offset":5060,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":5060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":5060,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":5452,"parent_record_offset":5416,"ancestor_record_offset":5060,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":5264,"parent_record_offset":5204,"ancestor_record_offset":5060,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":5296,"parent_record_offset":5168,"ancestor_record_offset":5060,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::async_load_audio_assets
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D3
// 首条位置: {"module":256,"inlinee":13779}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":15892,"parent_record_offset":15456,"ancestor_record_offset":15456,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":16596,"parent_record_offset":15456,"ancestor_record_offset":15456,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":16480,"parent_record_offset":15456,"ancestor_record_offset":15456,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":15548,"parent_record_offset":15456,"ancestor_record_offset":15456,"inlinee":7939,"source_line":574,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":15864,"parent_record_offset":15456,"ancestor_record_offset":15456,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15752,"parent_record_offset":15728,"ancestor_record_offset":15456,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15776,"parent_record_offset":15752,"ancestor_record_offset":15456,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":15456,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15696,"parent_record_offset":15656,"ancestor_record_offset":15456,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":15728,"parent_record_offset":15696,"ancestor_record_offset":15456,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":15456,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":15656,"parent_record_offset":15620,"ancestor_record_offset":15456,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16204,"parent_record_offset":16168,"ancestor_record_offset":15456,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16168,"parent_record_offset":16132,"ancestor_record_offset":15456,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16360,"parent_record_offset":16168,"ancestor_record_offset":15456,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":16720,"parent_record_offset":16696,"ancestor_record_offset":15456,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16088,"parent_record_offset":16052,"ancestor_record_offset":15456,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16052,"parent_record_offset":16016,"ancestor_record_offset":15456,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":15980,"parent_record_offset":15928,"ancestor_record_offset":15456,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":16016,"parent_record_offset":15980,"ancestor_record_offset":15456,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":15928,"parent_record_offset":15892,"ancestor_record_offset":15456,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":16132,"parent_record_offset":16088,"ancestor_record_offset":15456,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":16556,"parent_record_offset":16524,"ancestor_record_offset":15456,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":15456,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":16400,"parent_record_offset":16088,"ancestor_record_offset":15456,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":16620,"parent_record_offset":16596,"ancestor_record_offset":15456,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":15804,"parent_record_offset":15776,"ancestor_record_offset":15456,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":256,"record_offset":15620,"parent_record_offset":15584,"ancestor_record_offset":15456,"inlinee":7898,"source_line":21,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":15584,"parent_record_offset":15548,"ancestor_record_offset":15456,"inlinee":9960,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::async_load_audio_assets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":16524,"parent_record_offset":16480,"ancestor_record_offset":15456,"inlinee":13805,"source_line":341,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::async_load_audio_assets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x158D
// 首条位置: {"module":14,"inlinee":5517}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":14,"record_offset":2808,"parent_record_offset":2704,"ancestor_record_offset":2704,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":2784,"parent_record_offset":2704,"ancestor_record_offset":2704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::SyncSender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":14,"record_offset":2756,"parent_record_offset":2704,"ancestor_record_offset":2704,"inlinee":5519,"source_line":678,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":2704,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":2920,"parent_record_offset":2836,"ancestor_record_offset":2704,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":14,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":2704,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":14,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2704,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":14,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2704,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::async_load_audio_assets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35ED
// 首条位置: {"module":256,"inlinee":13805}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":16556,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread
// 记录形式: compiled_procedure; instances=1
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":256,"record_offset":54700,"rva":3988352,"va":5372697472}
// 编译实例 RVAs: 0x3cdb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$LP$S0$C$S1$C$S2$C$S3$C$S4$C$S5$C$S6$C$S7$C$S8$C$S9$C$S10$C$S11$RP$$u20$as$u20$bevy_app..plugin..sealed..Plugins$LT$$LP$bevy_app..plugin..sealed..PluginsTupleMarker$C$P0$C$P1$C$P2$C$P3$C$P4$C$P5$C$P6$C$P7$C$P8$C$P9$C$P10$C$P11$RP$$GT$$GT$::add_to_app
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-475079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e839c0f8ff","size":5,"soft_fail":false,"target_va":5372224672,"va":5372699746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$LP$S0$C$S1$C$S2$C$S3$C$S4$RP$$u20$as$u20$bevy_app..plugin..sealed..Plugins$LT$$LP$bevy_app..plugin..sealed..PluginsTupleMarker$C$P0$C$P1$C$P2$C$P3$C$P4$RP$$GT$$GT$::add_to_app
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2898253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b3c6d3ff","size":5,"soft_fail":false,"target_va":5369801696,"va":5372699944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$LP$S0$C$S1$C$S2$C$S3$RP$$u20$as$u20$bevy_app..plugin..sealed..Plugins$LT$$LP$bevy_app..plugin..sealed..PluginsTupleMarker$C$P0$C$P1$C$P2$C$P3$RP$$GT$$GT$::add_to_app
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3883654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e87abdc4ff","size":5,"soft_fail":false,"target_va":5368816000,"va":5372699649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_app..sub_app..SubApp$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4255642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89aef4000","size":5,"soft_fail":false,"target_va":5376981936,"va":5372726289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..NodeConfigs$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoSystemConfigs$LT$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3769714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88e7ac6ff","size":5,"soft_fail":false,"target_va":5368935744,"va":5372705453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t567424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e880a80800","size":5,"soft_fail":false,"target_va":5373265408,"va":5372697979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140017f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3918277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83b36c4ff","size":5,"soft_fail":false,"target_va":5368807280,"va":5372725552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140018f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3910759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89953c4ff","size":5,"soft_fail":false,"target_va":5368811280,"va":5372722034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=12
//     target_aliases=84 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3797107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88d0fc6ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5372734014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3781373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8034dc6ff","size":5,"soft_fail":false,"target_va":5368937728,"va":5372719096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3767649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89f82c6ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5372705516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400508e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3660780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81424c8ff","size":5,"soft_fail":false,"target_va":5369039072,"va":5372699847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400665f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3590619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e82536c9ff","size":5,"soft_fail":false,"target_va":5369128432,"va":5372719046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140066770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3576954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8866bc9ff","size":5,"soft_fail":false,"target_va":5369128816,"va":5372705765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140069d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e870a1c9ff","size":5,"soft_fail":false,"target_va":5369142576,"va":5372705723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dedd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3095862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8cac2d0ff","size":5,"soft_fail":false,"target_va":5369621968,"va":5372717825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2910913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83f95d3ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5372724460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140131c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2755745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e85ff3d5ff","size":5,"soft_fail":false,"target_va":5369961568,"va":5372717308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401324f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2765769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e837ccd5ff","size":5,"soft_fail":false,"target_va":5369963760,"va":5372729524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14013fce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2688656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e870f9d6ff","size":5,"soft_fail":false,"target_va":5370019040,"va":5372707691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140176690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2479284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84c2bdaff","size":5,"soft_fail":false,"target_va":5370242704,"va":5372721983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a4780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2275973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e87b45ddff","size":5,"soft_fail":false,"target_va":5370431360,"va":5372707328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140251290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1565779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ad1be8ff","size":5,"soft_fail":false,"target_va":5371138704,"va":5372704478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140251a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1563707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c523e8ff","size":5,"soft_fail":false,"target_va":5371140736,"va":5372704438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1499703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c91de9ff","size":5,"soft_fail":false,"target_va":5371215200,"va":5372714898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140264230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1501744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d015e9ff","size":5,"soft_fail":false,"target_va":5371216432,"va":5372718171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140268bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1482312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b861e9ff","size":5,"soft_fail":false,"target_va":5371235248,"va":5372717555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140286960
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1367017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81724ebff","size":5,"soft_fail":false,"target_va":5371357536,"va":5372724548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140289740
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1344861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a37aebff","size":5,"soft_fail":false,"target_va":5371369280,"va":5372714136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14028d140
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1336876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d499ebff","size":5,"soft_fail":false,"target_va":5371384128,"va":5372720999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14028fbc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1329520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e890b6ebff","size":5,"soft_fail":false,"target_va":5371395008,"va":5372724523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1150454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e80a72eeff","size":5,"soft_fail":false,"target_va":5371581872,"va":5372732321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c10a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1135078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81aaeeeff","size":5,"soft_fail":false,"target_va":5371596960,"va":5372732033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1085507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8bd6fefff","size":5,"soft_fail":false,"target_va":5371630992,"va":5372716494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1075808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a095efff","size":5,"soft_fail":false,"target_va":5371651264,"va":5372727067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cea10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1063758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b2c4efff","size":5,"soft_fail":false,"target_va":5371652624,"va":5372716377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1084682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f672efff","size":5,"soft_fail":false,"target_va":5371659056,"va":5372743733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d3cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1033513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d73af0ff","size":5,"soft_fail":false,"target_va":5371673808,"va":5372707316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d7460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1026967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86954f0ff","size":5,"soft_fail":false,"target_va":5371688032,"va":5372714994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d7d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1028010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e85650f0ff","size":5,"soft_fail":false,"target_va":5371690272,"va":5372718277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da640
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1029337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8274bf0ff","size":5,"soft_fail":false,"target_va":5371700800,"va":5372730132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402db650
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1024816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d05cf0ff","size":5,"soft_fail":false,"target_va":5371704912,"va":5372729723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403316b0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-655753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e877fef5ff","size":5,"soft_fail":false,"target_va":5372057264,"va":5372713012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035c7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-513168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8702bf8ff","size":5,"soft_fail":false,"target_va":5372233712,"va":5372746875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035ee70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-472296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e818cbf8ff","size":5,"soft_fail":false,"target_va":5372243568,"va":5372715859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035f4f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-482134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8aaa4f8ff","size":5,"soft_fail":false,"target_va":5372245232,"va":5372727361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-486970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c691f8ff","size":5,"soft_fail":false,"target_va":5372249472,"va":5372736437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-471662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e892cdf8ff","size":5,"soft_fail":false,"target_va":5372262784,"va":5372734441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f090
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-417706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e856a0f9ff","size":5,"soft_fail":false,"target_va":5372309648,"va":5372727349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=5
//     target_aliases=89 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-412500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8acb4f9ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372726703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-416208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e830a6f9ff","size":5,"soft_fail":false,"target_va":5372321360,"va":5372737563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-381462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ea2dfaff","size":5,"soft_fail":false,"target_va":5372345728,"va":5372727185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140378770
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-382938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e82628faff","size":5,"soft_fail":false,"target_va":5372348272,"va":5372731205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037b500
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-367294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84265faff","size":5,"soft_fail":false,"target_va":5372359936,"va":5372727225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037c570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-372572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a450faff","size":5,"soft_fail":false,"target_va":5372364144,"va":5372736711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037ccc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-365151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a16dfaff","size":5,"soft_fail":false,"target_va":5372366016,"va":5372731162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037d2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-376941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8933ffaff","size":5,"soft_fail":false,"target_va":5372367600,"va":5372744536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-348497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8afaefaff","size":5,"soft_fail":false,"target_va":5372378176,"va":5372726668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-364152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88871faff","size":5,"soft_fail":false,"target_va":5372378688,"va":5372742835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140380a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-349387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e835abfaff","size":5,"soft_fail":false,"target_va":5372381792,"va":5372731174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-349476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8dcaafaff","size":5,"soft_fail":false,"target_va":5372394560,"va":5372744031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385b80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-328595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86dfcfaff","size":5,"soft_fail":false,"target_va":5372402560,"va":5372731150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-309995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81545fbff","size":5,"soft_fail":false,"target_va":5372408976,"va":5372718966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b5d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-295919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8117cfbff","size":5,"soft_fail":false,"target_va":5372425680,"va":5372721594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038c0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-313804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83436fbff","size":5,"soft_fail":false,"target_va":5372428448,"va":5372742247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038eb90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-287738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8069cfbff","size":5,"soft_fail":false,"target_va":5372439440,"va":5372727173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140390b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-295648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8207dfbff","size":5,"soft_fail":false,"target_va":5372447584,"va":5372743227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-291388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c48dfbff","size":5,"soft_fail":false,"target_va":5372455776,"va":5372747159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b5290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-103110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83a6dfeff","size":5,"soft_fail":false,"target_va":5372596880,"va":5372699985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b69d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-123411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ed1dfeff","size":5,"soft_fail":false,"target_va":5372602832,"va":5372726238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b6eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88150feff","size":5,"soft_fail":false,"target_va":5372604080,"va":5372714538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b74b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8e970feff","size":5,"soft_fail":false,"target_va":5372605616,"va":5372707778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7630
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-101017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86775feff","size":5,"soft_fail":false,"target_va":5372606000,"va":5372707012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b17cfeff","size":5,"soft_fail":false,"target_va":5372607360,"va":5372706506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89d9bfeff","size":5,"soft_fail":false,"target_va":5372608112,"va":5372699342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=14
//     target_aliases=16 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-73154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83ee2feff","size":5,"soft_fail":false,"target_va":5372625024,"va":5372698173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8cfca0000","size":5,"soft_fail":false,"target_va":5372749696,"va":5372697772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e850870000","size":5,"soft_fail":false,"target_va":5372749712,"va":5372715067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403f0980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t135277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86d100200","size":5,"soft_fail":false,"target_va":5372840320,"va":5372705038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403f1010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t135086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ae0f0200","size":5,"soft_fail":false,"target_va":5372842000,"va":5372706909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403f19a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t130501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c5fd0100","size":5,"soft_fail":false,"target_va":5372844448,"va":5372713942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040f2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t251549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89dd60300","size":5,"soft_fail":false,"target_va":5372965536,"va":5372713982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140410460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t256173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ade80300","size":5,"soft_fail":false,"target_va":5372970080,"va":5372713902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t446097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e891ce0600","size":5,"soft_fail":false,"target_va":5373149840,"va":5372703738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1252565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d51c1300","size":5,"soft_fail":false,"target_va":5373966432,"va":5372713862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1695575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e857df1900","size":5,"soft_fail":false,"target_va":5374399408,"va":5372703828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=12
//     target_aliases=28 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2456253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8bd7a2500","size":5,"soft_fail":false,"target_va":5375171056,"va":5372714798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140648e30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2595011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c3982700","size":5,"soft_fail":false,"target_va":5375299120,"va":5372704104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2612772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e824de2700","size":5,"soft_fail":false,"target_va":5375337344,"va":5372724567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067a920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2799565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8cdb72a00","size":5,"soft_fail":false,"target_va":5375502624,"va":5372703054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2790424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e818942a00","size":5,"soft_fail":false,"target_va":5375504464,"va":5372714035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2800621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8edbb2a00","size":5,"soft_fail":false,"target_va":5375505504,"va":5372704878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067b710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2801269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e875be2a00","size":5,"soft_fail":false,"target_va":5375506192,"va":5372704918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067c630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2805421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8adce2a00","size":5,"soft_fail":false,"target_va":5375510064,"va":5372704638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2806780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8fcd32a00","size":5,"soft_fail":false,"target_va":5375510768,"va":5372703983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067ce80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2807509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d5d62a00","size":5,"soft_fail":false,"target_va":5375512192,"va":5372704678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067e640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2813549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86dee2a00","size":5,"soft_fail":false,"target_va":5375518272,"va":5372704718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067ea50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2816304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e830f92a00","size":5,"soft_fail":false,"target_va":5375519312,"va":5372703003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067eb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2814821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e865f32a00","size":5,"soft_fail":false,"target_va":5375519584,"va":5372704758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067ee50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2815533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e82df62a00","size":5,"soft_fail":false,"target_va":5375520336,"va":5372704798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067eea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2816733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ddfa2a00","size":5,"soft_fail":false,"target_va":5375520416,"va":5372703678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067f060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2816021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e815f82a00","size":5,"soft_fail":false,"target_va":5375520864,"va":5372704838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067f350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2816653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88dfa2a00","size":5,"soft_fail":false,"target_va":5375521616,"va":5372704958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067f780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2817685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e895fe2a00","size":5,"soft_fail":false,"target_va":5375522688,"va":5372704998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067ff50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2820085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f5072b00","size":5,"soft_fail":false,"target_va":5375524688,"va":5372704598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2822184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e828102b00","size":5,"soft_fail":false,"target_va":5375525152,"va":5372702963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2820893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81d0b2b00","size":5,"soft_fail":false,"target_va":5375525456,"va":5372704558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2821845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d50e2b00","size":5,"soft_fail":false,"target_va":5375526928,"va":5372705078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2806117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e865d12a00","size":5,"soft_fail":false,"target_va":5375527792,"va":5372721670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406829c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2831029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b5322b00","size":5,"soft_fail":false,"target_va":5375535552,"va":5372704518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140685a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2843765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e875642b00","size":5,"soft_fail":false,"target_va":5375547968,"va":5372704198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c0a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3076818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d2f22e00","size":5,"soft_fail":false,"target_va":5375789664,"va":5372712841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140729760
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3520867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e863b93500","size":5,"soft_fail":false,"target_va":5376218976,"va":5372698104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3493019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89b4c3500","size":5,"soft_fail":false,"target_va":5376239920,"va":5372746896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30202618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8fadacc01","size":5,"soft_fail":false,"target_va":5402900112,"va":5372697489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a6510
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30217037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84d13cd01","size":5,"soft_fail":false,"target_va":5402944784,"va":5372727742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30895605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f56dd701","size":5,"soft_fail":false,"target_va":5403593360,"va":5372697750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30249972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f493cd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372728167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1410961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86f78eaff","size":5,"soft_fail":false,"target_va":5371305232,"va":5372716188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-1074770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ae99efff","size":5,"soft_fail":false,"target_va":5371652368,"va":5372727133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t30234045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8bd55cd01","size":5,"soft_fail":false,"target_va":5402950048,"va":5372715998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4645526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e896e24600","size":5,"soft_fail":false,"target_va":5377364336,"va":5372718805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::add_plugins
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1365211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8252bebff","size":5,"soft_fail":false,"target_va":5371349376,"va":5372714582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=58
//     evidence={"assembly":"\tcallq\t-1330048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e880b4ebff","size":5,"soft_fail":false,"target_va":5371372880,"va":5372702923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4240885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f5b54000","size":5,"soft_fail":false,"target_va":5376962544,"va":5372721654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::plugins_state::h534f4a5ca483a05a (.llvm.2603154732207373802)
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-1348439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a96cebff","size":5,"soft_fail":false,"target_va":5371351152,"va":5372699586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=37
//     evidence={"assembly":"\tcallq\t-2093351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d90ee0ff","size":5,"soft_fail":false,"target_va":5370606944,"va":5372700290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3688399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e831b8c7ff","size":5,"soft_fail":false,"target_va":5369017104,"va":5372705498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-653878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ca05f6ff","size":5,"soft_fail":false,"target_va":5372072240,"va":5372726113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-927162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e846daf1ff","size":5,"soft_fail":false,"target_va":5371777968,"va":5372705125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-3768207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e87180c6ff","size":5,"soft_fail":false,"target_va":5368937200,"va":5372705402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::run_if
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3571253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8cb81c9ff","size":5,"soft_fail":false,"target_va":5369135808,"va":5372707056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30251230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8de98cd01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372727645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30249752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81893cd01","size":5,"soft_fail":false,"target_va":5402979152,"va":5372729395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t30251062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e83698cd01","size":5,"soft_fail":false,"target_va":5402978944,"va":5372727877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$country_core..meta_commander..MetaCommand$GT$::hce342b9cc2adbabd (.llvm.2064724076916773793)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1099879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e89937efff","size":5,"soft_fail":false,"target_va":5371630064,"va":5372729938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-57746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86e1effff","size":5,"soft_fail":false,"target_va":5372657952,"va":5372715693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t30251330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84299cd01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372728793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1839831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d7121c00","size":5,"soft_fail":false,"target_va":5374565312,"va":5372725476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1482886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e886a01600","size":5,"soft_fail":false,"target_va":5374208864,"va":5372725973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2963233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e821372d00","size":5,"soft_fail":false,"target_va":5375688624,"va":5372725386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2704689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e831452900","size":5,"soft_fail":false,"target_va":5375430208,"va":5372725514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1625581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8edcd1800","size":5,"soft_fail":false,"target_va":5374351024,"va":5372725438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::vao_library::AssetVAOLibrary::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1931652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e884791d00","size":5,"soft_fail":false,"target_va":5374657424,"va":5372725767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_config::impl$9::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-52798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c231ffff","size":5,"soft_fail":false,"target_va":5372661296,"va":5372714089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2598946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e822a82700","size":5,"soft_fail":false,"target_va":5375315632,"va":5372716681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::AvailableGraphicsDevices::from_preference_order
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2669239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b7ba2800","size":5,"soft_fail":false,"target_va":5375372592,"va":5372703348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2438828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ac362500","size":5,"soft_fail":false,"target_va":5375143184,"va":5372704351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorCapturedPosition::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2850642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8527f2b00","size":5,"soft_fail":false,"target_va":5375563024,"va":5372712377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::NextSession::ignore_snapshot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1540937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e849831700","size":5,"soft_fail":false,"target_va":5374257072,"va":5372716130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::NextSession::load
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1540838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8e6821700","size":5,"soft_fail":false,"target_va":5374256944,"va":5372716101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1538284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ec781700","size":5,"soft_fail":false,"target_va":5374255136,"va":5372716847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1538285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ed781700","size":5,"soft_fail":false,"target_va":5374255184,"va":5372716894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::toolbar::Toolbar::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2255936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8406c2200","size":5,"soft_fail":false,"target_va":5374968688,"va":5372712747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::aspect_ratio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2713125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e825662900","size":5,"soft_fail":false,"target_va":5375412224,"va":5372699094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2714114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8026a2900","size":5,"soft_fail":false,"target_va":5375412176,"va":5372698057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1344875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86b851400","size":5,"soft_fail":false,"target_va":5374044032,"va":5372699152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4654731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e88b064700","size":5,"soft_fail":false,"target_va":5377353776,"va":5372699040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3558819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8a34d3600","size":5,"soft_fail":false,"target_va":5376261936,"va":5372703112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3572202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ea813600","size":5,"soft_fail":false,"target_va":5376270592,"va":5372698385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4691816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e868974700","size":5,"soft_fail":false,"target_va":5377408752,"va":5372716931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4690767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84f934700","size":5,"soft_fail":false,"target_va":5377405616,"va":5372714844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4688327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8c7894700","size":5,"soft_fail":false,"target_va":5377406304,"va":5372717972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t4668110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8ce3a4700","size":5,"soft_fail":false,"target_va":5377366224,"va":5372698109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t4722656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8e00f4800","size":5,"soft_fail":false,"target_va":5377439776,"va":5372717115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4723389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8bd124800","size":5,"soft_fail":false,"target_va":5377441120,"va":5372717726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2947659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8b505d3ff","size":5,"soft_fail":false,"target_va":5369766608,"va":5372714262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-736711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e839c2f4ff","size":5,"soft_fail":false,"target_va":5371977504,"va":5372714210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-195775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e84103fdff","size":5,"soft_fail":false,"target_va":5372518544,"va":5372714314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4672858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e85a4d4700","size":5,"soft_fail":false,"target_va":5377393008,"va":5372720145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2039084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8d4e2e0ff","size":5,"soft_fail":false,"target_va":5370681120,"va":5372720199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::CliArgs::to_input_recording_opts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-695966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e86261f5ff","size":5,"soft_fail":false,"target_va":5372007968,"va":5372703929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::run
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1629166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e81224e7ff","size":5,"soft_fail":false,"target_va":5371092400,"va":5372721561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-48652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372697472,"raw_hex":"e8f441ffff","size":5,"soft_fail":false,"target_va":5372649200,"va":5372697847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*38587758(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372697472,"raw_hex":"ff156ecd4c02","size":6,"soft_fail":false,"va":5372697908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107700,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":108744,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7709,"source_line":1658,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":146548,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":55076,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":75384,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":55340,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":59052,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":59848,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_plugins
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":59468,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":11452,"source_line":543,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":75748,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::edit_schedule
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":59412,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":11453,"source_line":699,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":256,"record_offset":122176,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":256,"record_offset":122240,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::after
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":256,"record_offset":114908,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":6396,"source_line":472,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":256,"record_offset":75676,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":120220,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":118004,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10621,"source_line":1365,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::app::impl$1::init_state
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_state/src/app.rs
//     evidence={"module":256,"record_offset":59820,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":11451,"source_line":185,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":58968,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":55692,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":58996,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":146384,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":108256,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":148100,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":109024,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":108524,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":106944,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":117588,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":106456,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":256,"record_offset":108024,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":12633,"source_line":968,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":256,"record_offset":107996,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13831,"source_line":79,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":256,"record_offset":102364,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13812,"source_line":119,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":256,"record_offset":75472,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13808,"source_line":65,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":256,"record_offset":106492,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13828,"source_line":72,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$59::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":256,"record_offset":103876,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13818,"source_line":374,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::NextSession::append_on_load
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":256,"record_offset":108320,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":9508,"source_line":293,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::arches::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/arches.rs
//     evidence={"module":256,"record_offset":103904,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13820,"source_line":12,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::arches::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/arches.rs
//     evidence={"module":256,"record_offset":104392,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13822,"source_line":16,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::arches::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/arches.rs
//     evidence={"module":256,"record_offset":104880,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13824,"source_line":24,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":256,"record_offset":103388,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13816,"source_line":175,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":256,"record_offset":102900,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13814,"source_line":9,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/mod.rs
//     evidence={"module":256,"record_offset":115744,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5557,"source_line":43,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":256,"record_offset":75532,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":6618,"source_line":678,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":59024,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":58884,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_pool::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_pool.rs
//     evidence={"module":256,"record_offset":55928,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5533,"source_line":7,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":256,"record_offset":57016,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":5555,"source_line":79,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":256,"record_offset":105856,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":9059,"source_line":232,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":256,"record_offset":109056,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":256,"record_offset":106688,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":256,"record_offset":112108,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":256,"record_offset":110196,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":256,"record_offset":110036,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":106340,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7941,"source_line":848,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":117632,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7943,"source_line":525,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":256,"record_offset":116264,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7952,"source_line":624,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":116868,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs
//     evidence={"module":256,"record_offset":105368,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13826,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":256,"record_offset":58400,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":7354,"source_line":35,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::event_loop::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":256,"record_offset":58076,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13804,"source_line":65,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":256,"record_offset":122456,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":10721,"source_line":83,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::build_loading_screen_app
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":122600,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13833,"source_line":917,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::capped_task_pool_options
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":59440,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":13806,"source_line":1388,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":55368,"parent_record_offset":54700,"ancestor_record_offset":54700,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":55208,"parent_record_offset":55184,"ancestor_record_offset":54700,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":55232,"parent_record_offset":55208,"ancestor_record_offset":54700,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":107320,"parent_record_offset":107292,"ancestor_record_offset":54700,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":55152,"parent_record_offset":55112,"ancestor_record_offset":54700,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":55184,"parent_record_offset":55152,"ancestor_record_offset":54700,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":107292,"parent_record_offset":107220,"ancestor_record_offset":54700,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":55112,"parent_record_offset":55076,"ancestor_record_offset":54700,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107776,"parent_record_offset":107700,"ancestor_record_offset":54700,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":108828,"parent_record_offset":108744,"ancestor_record_offset":54700,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107744,"parent_record_offset":107700,"ancestor_record_offset":54700,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":108796,"parent_record_offset":108744,"ancestor_record_offset":54700,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":102428,"parent_record_offset":102404,"ancestor_record_offset":54700,"inlinee":12812,"source_line":542,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107808,"parent_record_offset":107776,"ancestor_record_offset":54700,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107884,"parent_record_offset":107700,"ancestor_record_offset":54700,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":107912,"parent_record_offset":107884,"ancestor_record_offset":54700,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":108944,"parent_record_offset":108744,"ancestor_record_offset":54700,"inlinee":7713,"source_line":207,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":256,"record_offset":102404,"parent_record_offset":102364,"ancestor_record_offset":54700,"inlinee":12811,"source_line":151,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":107256,"parent_record_offset":107220,"ancestor_record_offset":54700,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":107844,"parent_record_offset":107808,"ancestor_record_offset":54700,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":117284,"parent_record_offset":117248,"ancestor_record_offset":54700,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":117248,"parent_record_offset":117212,"ancestor_record_offset":54700,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":117452,"parent_record_offset":117248,"ancestor_record_offset":54700,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":107220,"parent_record_offset":107192,"ancestor_record_offset":54700,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":117168,"parent_record_offset":117132,"ancestor_record_offset":54700,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":117132,"parent_record_offset":117096,"ancestor_record_offset":54700,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":117060,"parent_record_offset":117008,"ancestor_record_offset":54700,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":117096,"parent_record_offset":117060,"ancestor_record_offset":54700,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":117008,"parent_record_offset":116972,"ancestor_record_offset":54700,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":116972,"parent_record_offset":116936,"ancestor_record_offset":54700,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":116936,"parent_record_offset":116900,"ancestor_record_offset":54700,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":147508,"parent_record_offset":147472,"ancestor_record_offset":54700,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":59144,"parent_record_offset":59052,"ancestor_record_offset":54700,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":58128,"parent_record_offset":58104,"ancestor_record_offset":54700,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":146088,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":108596,"parent_record_offset":108572,"ancestor_record_offset":54700,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":108412,"parent_record_offset":108364,"ancestor_record_offset":54700,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":108364,"parent_record_offset":108320,"ancestor_record_offset":54700,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":117212,"parent_record_offset":117168,"ancestor_record_offset":54700,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":124876,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::app::App::add_plugins
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":124700,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":11452,"source_line":543,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":146184,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=37
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":256,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":54700,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":256,"record_offset":122204,"parent_record_offset":122176,"ancestor_record_offset":54700,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=42
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":256,"record_offset":59932,"parent_record_offset":59900,"ancestor_record_offset":54700,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":256,"record_offset":122268,"parent_record_offset":122240,"ancestor_record_offset":54700,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":256,"record_offset":75716,"parent_record_offset":75676,"ancestor_record_offset":54700,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":121988,"parent_record_offset":121960,"ancestor_record_offset":54700,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":121960,"parent_record_offset":121932,"ancestor_record_offset":54700,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=42
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":61344,"parent_record_offset":61320,"ancestor_record_offset":54700,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":119684,"parent_record_offset":119580,"ancestor_record_offset":54700,"inlinee":10627,"source_line":294,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":124520,"parent_record_offset":124084,"ancestor_record_offset":54700,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":121840,"parent_record_offset":121400,"ancestor_record_offset":54700,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=42
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":61320,"parent_record_offset":61296,"ancestor_record_offset":54700,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":119580,"parent_record_offset":118004,"ancestor_record_offset":54700,"inlinee":10626,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=41
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":60968,"parent_record_offset":60944,"ancestor_record_offset":54700,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":119168,"parent_record_offset":118004,"ancestor_record_offset":54700,"inlinee":10622,"source_line":297,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":121932,"parent_record_offset":121868,"ancestor_record_offset":54700,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=42
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":61016,"parent_record_offset":60992,"ancestor_record_offset":54700,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=41
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":60992,"parent_record_offset":60968,"ancestor_record_offset":54700,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":119204,"parent_record_offset":119168,"ancestor_record_offset":54700,"inlinee":10623,"source_line":379,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=42
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":61108,"parent_record_offset":61044,"ancestor_record_offset":54700,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=41
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":61196,"parent_record_offset":61172,"ancestor_record_offset":54700,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":119432,"parent_record_offset":119404,"ancestor_record_offset":54700,"inlinee":10625,"source_line":381,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=37
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":59900,"parent_record_offset":59872,"ancestor_record_offset":54700,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":122904,"parent_record_offset":122848,"ancestor_record_offset":54700,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":122064,"parent_record_offset":122036,"ancestor_record_offset":54700,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":122848,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":13622,"source_line":1512,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=37
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":60944,"parent_record_offset":60920,"ancestor_record_offset":54700,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=37
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":61296,"parent_record_offset":61272,"ancestor_record_offset":54700,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":122036,"parent_record_offset":121400,"ancestor_record_offset":54700,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":122932,"parent_record_offset":122904,"ancestor_record_offset":54700,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":124084,"parent_record_offset":122932,"ancestor_record_offset":54700,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":120248,"parent_record_offset":120220,"ancestor_record_offset":54700,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":121400,"parent_record_offset":120248,"ancestor_record_offset":54700,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":256,"record_offset":120120,"parent_record_offset":118004,"ancestor_record_offset":54700,"inlinee":10628,"source_line":428,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":256,"record_offset":60652,"parent_record_offset":60628,"ancestor_record_offset":54700,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":256,"record_offset":124632,"parent_record_offset":122848,"ancestor_record_offset":54700,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":56240,"parent_record_offset":56212,"ancestor_record_offset":54700,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":56292,"parent_record_offset":56268,"ancestor_record_offset":54700,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":56268,"parent_record_offset":56212,"ancestor_record_offset":54700,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":60700,"parent_record_offset":60676,"ancestor_record_offset":54700,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":60628,"parent_record_offset":60604,"ancestor_record_offset":54700,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":106716,"parent_record_offset":106688,"ancestor_record_offset":54700,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":55776,"parent_record_offset":55752,"ancestor_record_offset":54700,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$46::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":256,"record_offset":108184,"parent_record_offset":108060,"ancestor_record_offset":54700,"inlinee":6006,"source_line":76,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":256,"record_offset":55800,"parent_record_offset":55776,"ancestor_record_offset":54700,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":60120,"parent_record_offset":60096,"ancestor_record_offset":54700,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":60204,"parent_record_offset":60180,"ancestor_record_offset":54700,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":256,"record_offset":108120,"parent_record_offset":108060,"ancestor_record_offset":54700,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":256,"record_offset":108148,"parent_record_offset":108120,"ancestor_record_offset":54700,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":55644,"parent_record_offset":55472,"ancestor_record_offset":54700,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":124664,"parent_record_offset":122848,"ancestor_record_offset":54700,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":60144,"parent_record_offset":60120,"ancestor_record_offset":54700,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":55604,"parent_record_offset":55580,"ancestor_record_offset":54700,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":256,"record_offset":55556,"parent_record_offset":55532,"ancestor_record_offset":54700,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":56316,"parent_record_offset":56292,"ancestor_record_offset":54700,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":119832,"parent_record_offset":119760,"ancestor_record_offset":54700,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":55532,"parent_record_offset":55472,"ancestor_record_offset":54700,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":107940,"parent_record_offset":107912,"ancestor_record_offset":54700,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":108972,"parent_record_offset":108944,"ancestor_record_offset":54700,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":256,"record_offset":60316,"parent_record_offset":60292,"ancestor_record_offset":54700,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":60920,"parent_record_offset":59900,"ancestor_record_offset":54700,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":56168,"parent_record_offset":56144,"ancestor_record_offset":54700,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":60888,"parent_record_offset":59932,"ancestor_record_offset":54700,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":108284,"parent_record_offset":108256,"ancestor_record_offset":54700,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=79
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":61172,"parent_record_offset":60992,"ancestor_record_offset":54700,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":106520,"parent_record_offset":106492,"ancestor_record_offset":54700,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":121784,"parent_record_offset":121756,"ancestor_record_offset":54700,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":117492,"parent_record_offset":117168,"ancestor_record_offset":54700,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=135
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":55304,"parent_record_offset":55112,"ancestor_record_offset":54700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":119524,"parent_record_offset":119496,"ancestor_record_offset":54700,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":60540,"parent_record_offset":60516,"ancestor_record_offset":54700,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":120148,"parent_record_offset":120120,"ancestor_record_offset":54700,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":60460,"parent_record_offset":60436,"ancestor_record_offset":54700,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":55580,"parent_record_offset":55556,"ancestor_record_offset":54700,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":147544,"parent_record_offset":147508,"ancestor_record_offset":54700,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":119908,"parent_record_offset":119832,"ancestor_record_offset":54700,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":55260,"parent_record_offset":55232,"ancestor_record_offset":54700,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":119728,"parent_record_offset":119684,"ancestor_record_offset":54700,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":119760,"parent_record_offset":119728,"ancestor_record_offset":54700,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":56340,"parent_record_offset":56316,"ancestor_record_offset":54700,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":106412,"parent_record_offset":106368,"ancestor_record_offset":54700,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":107096,"parent_record_offset":106988,"ancestor_record_offset":54700,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":107372,"parent_record_offset":106988,"ancestor_record_offset":54700,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":61044,"parent_record_offset":61016,"ancestor_record_offset":54700,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":61080,"parent_record_offset":61044,"ancestor_record_offset":54700,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":121756,"parent_record_offset":121720,"ancestor_record_offset":54700,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":119496,"parent_record_offset":119460,"ancestor_record_offset":54700,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":122092,"parent_record_offset":122064,"ancestor_record_offset":54700,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":58156,"parent_record_offset":58128,"ancestor_record_offset":54700,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":108632,"parent_record_offset":108596,"ancestor_record_offset":54700,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":58908,"parent_record_offset":58884,"ancestor_record_offset":54700,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":58180,"parent_record_offset":58156,"ancestor_record_offset":54700,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":58932,"parent_record_offset":58908,"ancestor_record_offset":54700,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":108660,"parent_record_offset":108632,"ancestor_record_offset":54700,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":108696,"parent_record_offset":108596,"ancestor_record_offset":54700,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":256,"record_offset":108060,"parent_record_offset":108024,"ancestor_record_offset":54700,"inlinee":12634,"source_line":1602,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":256,"record_offset":60676,"parent_record_offset":60652,"ancestor_record_offset":54700,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::frame_constants::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":256,"record_offset":146120,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":13835,"source_line":9,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":256,"record_offset":146024,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":13808,"source_line":65,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":256,"record_offset":145860,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":13828,"source_line":72,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$36::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":256,"record_offset":146156,"parent_record_offset":122600,"ancestor_record_offset":54700,"inlinee":13837,"source_line":333,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":256,"record_offset":58104,"parent_record_offset":58076,"ancestor_record_offset":54700,"inlinee":6618,"source_line":678,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":256,"record_offset":60604,"parent_record_offset":60580,"ancestor_record_offset":54700,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":59956,"parent_record_offset":59932,"ancestor_record_offset":54700,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":59980,"parent_record_offset":59956,"ancestor_record_offset":54700,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":60580,"parent_record_offset":60384,"ancestor_record_offset":54700,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":256,"record_offset":56940,"parent_record_offset":56916,"ancestor_record_offset":54700,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60492,"parent_record_offset":60408,"ancestor_record_offset":54700,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60436,"parent_record_offset":60408,"ancestor_record_offset":54700,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60836,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60408,"parent_record_offset":60384,"ancestor_record_offset":54700,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60032,"parent_record_offset":60008,"ancestor_record_offset":54700,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60008,"parent_record_offset":59980,"ancestor_record_offset":54700,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60056,"parent_record_offset":60032,"ancestor_record_offset":54700,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":60264,"parent_record_offset":60236,"ancestor_record_offset":54700,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":60292,"parent_record_offset":60264,"ancestor_record_offset":54700,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":60352,"parent_record_offset":60236,"ancestor_record_offset":54700,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":60236,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":60384,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":60096,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":60180,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":60752,"parent_record_offset":60056,"ancestor_record_offset":54700,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":256,"record_offset":61220,"parent_record_offset":61196,"ancestor_record_offset":54700,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":256,"record_offset":107028,"parent_record_offset":106988,"ancestor_record_offset":54700,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":256,"record_offset":107056,"parent_record_offset":107028,"ancestor_record_offset":54700,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":256,"record_offset":56400,"parent_record_offset":55968,"ancestor_record_offset":54700,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":256,"record_offset":55968,"parent_record_offset":55928,"ancestor_record_offset":54700,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":256,"record_offset":106828,"parent_record_offset":106800,"ancestor_record_offset":54700,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":256,"record_offset":106884,"parent_record_offset":106856,"ancestor_record_offset":54700,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":256,"record_offset":106856,"parent_record_offset":106828,"ancestor_record_offset":54700,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":256,"record_offset":109320,"parent_record_offset":109056,"ancestor_record_offset":54700,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":256,"record_offset":109096,"parent_record_offset":109056,"ancestor_record_offset":54700,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":56016,"parent_record_offset":55992,"ancestor_record_offset":54700,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":256,"record_offset":56092,"parent_record_offset":56064,"ancestor_record_offset":54700,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":56212,"parent_record_offset":56064,"ancestor_record_offset":54700,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":55992,"parent_record_offset":55968,"ancestor_record_offset":54700,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":256,"record_offset":106800,"parent_record_offset":106772,"ancestor_record_offset":54700,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":256,"record_offset":106772,"parent_record_offset":106744,"ancestor_record_offset":54700,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":256,"record_offset":106744,"parent_record_offset":106716,"ancestor_record_offset":54700,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":256,"record_offset":116444,"parent_record_offset":116340,"ancestor_record_offset":54700,"inlinee":7931,"source_line":2357,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":106368,"parent_record_offset":106340,"ancestor_record_offset":54700,"inlinee":7891,"source_line":304,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":117672,"parent_record_offset":117632,"ancestor_record_offset":54700,"inlinee":7894,"source_line":51,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::Sender::acquire
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":256,"record_offset":116340,"parent_record_offset":116296,"ancestor_record_offset":54700,"inlinee":7928,"source_line":45,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::Sender::counter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":256,"record_offset":116372,"parent_record_offset":116340,"ancestor_record_offset":54700,"inlinee":7929,"source_line":40,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":256,"record_offset":117744,"parent_record_offset":117672,"ancestor_record_offset":54700,"inlinee":7898,"source_line":21,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":256,"record_offset":116296,"parent_record_offset":116264,"ancestor_record_offset":54700,"inlinee":7926,"source_line":239,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::list::Channel::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\list.rs
//     evidence={"module":256,"record_offset":117712,"parent_record_offset":117672,"ancestor_record_offset":54700,"inlinee":7896,"source_line":160,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLock::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":256,"record_offset":59112,"parent_record_offset":59088,"ancestor_record_offset":54700,"inlinee":6258,"source_line":223,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":256,"record_offset":59088,"parent_record_offset":59052,"ancestor_record_offset":54700,"inlinee":6256,"source_line":560,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":256,"record_offset":116472,"parent_record_offset":116444,"ancestor_record_offset":54700,"inlinee":7933,"source_line":327,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":256,"record_offset":56116,"parent_record_offset":56092,"ancestor_record_offset":54700,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":256,"record_offset":56144,"parent_record_offset":56116,"ancestor_record_offset":54700,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":256,"record_offset":56064,"parent_record_offset":56040,"ancestor_record_offset":54700,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":256,"record_offset":56040,"parent_record_offset":56016,"ancestor_record_offset":54700,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::run_game_thread::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":106988,"parent_record_offset":106944,"ancestor_record_offset":54700,"inlinee":13829,"source_line":652,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::run_game_thread::closure$5
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":113816,"parent_record_offset":113780,"ancestor_record_offset":54700,"inlinee":13832,"source_line":729,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] utils::camera::impl$12::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/camera.rs
//     evidence={"module":256,"record_offset":75500,"parent_record_offset":75472,"ancestor_record_offset":54700,"inlinee":13810,"source_line":8,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":55752,"parent_record_offset":55720,"ancestor_record_offset":54700,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":55720,"parent_record_offset":55692,"ancestor_record_offset":54700,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":55472,"parent_record_offset":55440,"ancestor_record_offset":54700,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":55404,"parent_record_offset":55368,"ancestor_record_offset":54700,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":55440,"parent_record_offset":55404,"ancestor_record_offset":54700,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x353B
// 首条位置: {"module":245,"inlinee":13627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 631
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA0
// 首条位置: {"module":66,"inlinee":8096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":66,"record_offset":1717736,"parent_record_offset":1717712,"ancestor_record_offset":1717712,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_state/src/state/resources.rs
//     evidence={"module":66,"record_offset":1717792,"parent_record_offset":1717712,"ancestor_record_offset":1717712,"inlinee":8074,"source_line":130,"archive":"_pdb/A/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":66,"record_offset":1717760,"parent_record_offset":1717736,"ancestor_record_offset":1717712,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 651
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADB
// 首条位置: {"module":134,"inlinee":10971}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":12344,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":12324,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":12324,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 652
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3605
// 首条位置: {"module":256,"inlinee":13829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":107136,"parent_record_offset":106988,"ancestor_record_offset":106988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":107096,"parent_record_offset":106988,"ancestor_record_offset":106988,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":107372,"parent_record_offset":106988,"ancestor_record_offset":106988,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":256,"record_offset":107028,"parent_record_offset":106988,"ancestor_record_offset":106988,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":107320,"parent_record_offset":107292,"ancestor_record_offset":106988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":107292,"parent_record_offset":107220,"ancestor_record_offset":106988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":107256,"parent_record_offset":107220,"ancestor_record_offset":106988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":107220,"parent_record_offset":107192,"ancestor_record_offset":106988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":107164,"parent_record_offset":107136,"ancestor_record_offset":106988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":256,"record_offset":107056,"parent_record_offset":107028,"ancestor_record_offset":106988,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 728
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADF
// 首条位置: {"module":134,"inlinee":10975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":14156,"parent_record_offset":14136,"ancestor_record_offset":14136,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":14136,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":14136,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 729
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3608
// 首条位置: {"module":256,"inlinee":13832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":113996,"parent_record_offset":113816,"ancestor_record_offset":113816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":113916,"parent_record_offset":113816,"ancestor_record_offset":113816,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":113964,"parent_record_offset":113816,"ancestor_record_offset":113816,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":256,"record_offset":113848,"parent_record_offset":113816,"ancestor_record_offset":113816,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":114176,"parent_record_offset":114148,"ancestor_record_offset":113816,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":114148,"parent_record_offset":114080,"ancestor_record_offset":113816,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":114112,"parent_record_offset":114080,"ancestor_record_offset":113816,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":114080,"parent_record_offset":114052,"ancestor_record_offset":113816,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":114024,"parent_record_offset":113996,"ancestor_record_offset":113816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":256,"record_offset":113876,"parent_record_offset":113848,"ancestor_record_offset":113816,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 806
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x159A
// 首条位置: {"module":14,"inlinee":5530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::run_game_thread::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 848
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x159B
// 首条位置: {"module":14,"inlinee":5531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":7236,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":14,"record_offset":10348,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5568,"source_line":337,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":14,"record_offset":8804,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":14,"record_offset":10460,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":14,"record_offset":10488,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":10984,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":10740,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/mod.rs
//     evidence={"module":14,"record_offset":8320,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5557,"source_line":43,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":14,"record_offset":10516,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":14,"record_offset":10376,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_pool::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_pool.rs
//     evidence={"module":14,"record_offset":6192,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5533,"source_line":7,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":14,"record_offset":7264,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5555,"source_line":79,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":14,"record_offset":10712,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":10856,"parent_record_offset":10820,"ancestor_record_offset":6056,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":6056,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":14,"record_offset":8856,"parent_record_offset":8828,"ancestor_record_offset":6056,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":14,"record_offset":10300,"parent_record_offset":10268,"ancestor_record_offset":6056,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":14,"record_offset":10244,"parent_record_offset":9856,"ancestor_record_offset":6056,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":14,"record_offset":10268,"parent_record_offset":10244,"ancestor_record_offset":6056,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":14,"record_offset":9884,"parent_record_offset":9856,"ancestor_record_offset":6056,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":14,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":6056,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":14,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":6056,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":14,"record_offset":10024,"parent_record_offset":9960,"ancestor_record_offset":6056,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":14,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":6056,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":14,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":6056,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":14,"record_offset":9856,"parent_record_offset":8828,"ancestor_record_offset":6056,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":14,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":6056,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":6056,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":6056,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6520,"parent_record_offset":6464,"ancestor_record_offset":6056,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":14,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":6056,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":14,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":6056,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":14,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":6056,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":14,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":6056,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":14,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":6056,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":14,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":6056,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":14,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":6056,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":6056,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":9824,"parent_record_offset":8856,"ancestor_record_offset":6056,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":10088,"parent_record_offset":9908,"ancestor_record_offset":6056,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":14,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":6056,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":7120,"parent_record_offset":6192,"ancestor_record_offset":6056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":14,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":6056,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":14,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":6056,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":6056,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":6056,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":14,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":6056,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":14,"record_offset":9996,"parent_record_offset":9960,"ancestor_record_offset":6056,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":14,"record_offset":10168,"parent_record_offset":10136,"ancestor_record_offset":6056,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":6056,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":10400,"parent_record_offset":10376,"ancestor_record_offset":6056,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":6056,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":10916,"parent_record_offset":10856,"ancestor_record_offset":6056,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":10948,"parent_record_offset":10820,"ancestor_record_offset":6056,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":14,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":6056,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":14,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":6056,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":14,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":6056,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":14,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":6056,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":14,"record_offset":9516,"parent_record_offset":9316,"ancestor_record_offset":6056,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":14,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6056,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":9428,"parent_record_offset":9344,"ancestor_record_offset":6056,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":9372,"parent_record_offset":9344,"ancestor_record_offset":6056,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":9772,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":6056,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":6056,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":6056,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":8980,"parent_record_offset":8956,"ancestor_record_offset":6056,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":14,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":6056,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":14,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":6056,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":14,"record_offset":9284,"parent_record_offset":9168,"ancestor_record_offset":6056,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":14,"record_offset":9168,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":14,"record_offset":9316,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":14,"record_offset":9028,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":14,"record_offset":9112,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":14,"record_offset":9688,"parent_record_offset":8980,"ancestor_record_offset":6056,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":14,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":6056,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":6652,"parent_record_offset":6228,"ancestor_record_offset":6056,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":6228,"parent_record_offset":6192,"ancestor_record_offset":6056,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":6056,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":6352,"parent_record_offset":6324,"ancestor_record_offset":6056,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6464,"parent_record_offset":6324,"ancestor_record_offset":6056,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":6056,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":6056,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":14,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":6056,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":6056,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":6056,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::build_loading_screen_app
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 917
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3609
// 首条位置: {"module":256,"inlinee":13833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":146088,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":124876,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_plugins
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":124700,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":11452,"source_line":543,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":146184,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":122848,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":13622,"source_line":1512,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":146216,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":256,"record_offset":146120,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":13835,"source_line":9,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":256,"record_offset":146024,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":13808,"source_line":65,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":256,"record_offset":145860,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":13828,"source_line":72,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$36::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":256,"record_offset":146156,"parent_record_offset":122600,"ancestor_record_offset":122600,"inlinee":13837,"source_line":333,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":145916,"parent_record_offset":145888,"ancestor_record_offset":122600,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":146240,"parent_record_offset":146216,"ancestor_record_offset":122600,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":256,"record_offset":124904,"parent_record_offset":124876,"ancestor_record_offset":122600,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":256,"record_offset":122964,"parent_record_offset":122932,"ancestor_record_offset":122600,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":124580,"parent_record_offset":124548,"ancestor_record_offset":122600,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":124520,"parent_record_offset":124084,"ancestor_record_offset":122600,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":124548,"parent_record_offset":124520,"ancestor_record_offset":122600,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":124116,"parent_record_offset":124084,"ancestor_record_offset":122600,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":124172,"parent_record_offset":124144,"ancestor_record_offset":122600,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":124144,"parent_record_offset":124116,"ancestor_record_offset":122600,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":124276,"parent_record_offset":124204,"ancestor_record_offset":122600,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":124372,"parent_record_offset":124344,"ancestor_record_offset":122600,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":124936,"parent_record_offset":124904,"ancestor_record_offset":122600,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":122904,"parent_record_offset":122848,"ancestor_record_offset":122600,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":126104,"parent_record_offset":126076,"ancestor_record_offset":122600,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":126496,"parent_record_offset":126468,"ancestor_record_offset":122600,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":122932,"parent_record_offset":122904,"ancestor_record_offset":122600,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":256,"record_offset":124084,"parent_record_offset":122932,"ancestor_record_offset":122600,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":256,"record_offset":123788,"parent_record_offset":123760,"ancestor_record_offset":122600,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":256,"record_offset":124632,"parent_record_offset":122848,"ancestor_record_offset":122600,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":123844,"parent_record_offset":123816,"ancestor_record_offset":122600,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":123760,"parent_record_offset":123732,"ancestor_record_offset":122600,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":123188,"parent_record_offset":123160,"ancestor_record_offset":122600,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":123284,"parent_record_offset":123256,"ancestor_record_offset":122600,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":124752,"parent_record_offset":124700,"ancestor_record_offset":122600,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":124664,"parent_record_offset":122848,"ancestor_record_offset":122600,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":123216,"parent_record_offset":123188,"ancestor_record_offset":122600,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":256,"record_offset":123408,"parent_record_offset":123380,"ancestor_record_offset":122600,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":126076,"parent_record_offset":124936,"ancestor_record_offset":122600,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":124300,"parent_record_offset":124276,"ancestor_record_offset":122600,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":124048,"parent_record_offset":122964,"ancestor_record_offset":122600,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":124344,"parent_record_offset":124144,"ancestor_record_offset":122600,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":145888,"parent_record_offset":145860,"ancestor_record_offset":122600,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":124464,"parent_record_offset":124436,"ancestor_record_offset":122600,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":123664,"parent_record_offset":123636,"ancestor_record_offset":122600,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":123572,"parent_record_offset":123544,"ancestor_record_offset":122600,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":124204,"parent_record_offset":124172,"ancestor_record_offset":122600,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":124244,"parent_record_offset":124204,"ancestor_record_offset":122600,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":124436,"parent_record_offset":124400,"ancestor_record_offset":122600,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":145948,"parent_record_offset":145916,"ancestor_record_offset":122600,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":146276,"parent_record_offset":146240,"ancestor_record_offset":122600,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":145976,"parent_record_offset":145948,"ancestor_record_offset":122600,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":146304,"parent_record_offset":146276,"ancestor_record_offset":122600,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":146340,"parent_record_offset":146240,"ancestor_record_offset":122600,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":256,"record_offset":123816,"parent_record_offset":123788,"ancestor_record_offset":122600,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":256,"record_offset":123732,"parent_record_offset":123708,"ancestor_record_offset":122600,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":122992,"parent_record_offset":122964,"ancestor_record_offset":122600,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":123020,"parent_record_offset":122992,"ancestor_record_offset":122600,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":123708,"parent_record_offset":123484,"ancestor_record_offset":122600,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123608,"parent_record_offset":123512,"ancestor_record_offset":122600,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123544,"parent_record_offset":123512,"ancestor_record_offset":122600,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123996,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123512,"parent_record_offset":123484,"ancestor_record_offset":122600,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123080,"parent_record_offset":123052,"ancestor_record_offset":122600,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123052,"parent_record_offset":123020,"ancestor_record_offset":122600,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123108,"parent_record_offset":123080,"ancestor_record_offset":122600,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":123352,"parent_record_offset":123320,"ancestor_record_offset":122600,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":123380,"parent_record_offset":123352,"ancestor_record_offset":122600,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":123448,"parent_record_offset":123320,"ancestor_record_offset":122600,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":123320,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":123484,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":123160,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":123256,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":123900,"parent_record_offset":123108,"ancestor_record_offset":122600,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":256,"record_offset":124400,"parent_record_offset":124372,"ancestor_record_offset":122600,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] utils::camera::impl$12::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/camera.rs
//     evidence={"module":256,"record_offset":146056,"parent_record_offset":146024,"ancestor_record_offset":122600,"inlinee":13810,"source_line":8,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::create_graphics_device_and_swapchain
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1016
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D5
// 首条位置: {"module":256,"inlinee":13781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":27088,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":28672,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":22736,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33416,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":8652,"source_line":1535,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24464,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":32332,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33572,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33276,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":33516,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41252,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":33684,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":256,"record_offset":31788,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":13790,"source_line":18,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":22652,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":27144,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":24408,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":22680,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":33940,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26012,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":28556,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":23896,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":24120,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":24148,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":25580,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":33800,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":28756,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":28332,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":27916,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":22708,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":22568,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":256,"record_offset":35096,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":13795,"source_line":423,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::try_get_devices_for_surface
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":21720,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":13782,"source_line":1261,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::try_init_swapchain
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":35724,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":13798,"source_line":1270,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":22764,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":34156,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_maximized
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25804,"parent_record_offset":20932,"ancestor_record_offset":20932,"inlinee":13786,"source_line":1045,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":23232,"parent_record_offset":23208,"ancestor_record_offset":20932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":23256,"parent_record_offset":23232,"ancestor_record_offset":20932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":20932,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":23208,"parent_record_offset":23184,"ancestor_record_offset":20932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":20932,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":27552,"parent_record_offset":27416,"ancestor_record_offset":20932,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":23332,"parent_record_offset":23116,"ancestor_record_offset":20932,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":27576,"parent_record_offset":27552,"ancestor_record_offset":20932,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23116,"parent_record_offset":23028,"ancestor_record_offset":20932,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23152,"parent_record_offset":23116,"ancestor_record_offset":20932,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":23184,"parent_record_offset":23152,"ancestor_record_offset":20932,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":20932,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27348,"parent_record_offset":27324,"ancestor_record_offset":20932,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27460,"parent_record_offset":27416,"ancestor_record_offset":20932,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":41428,"parent_record_offset":41396,"ancestor_record_offset":20932,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":41396,"parent_record_offset":41368,"ancestor_record_offset":20932,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27416,"parent_record_offset":27380,"ancestor_record_offset":20932,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":27380,"parent_record_offset":27284,"ancestor_record_offset":20932,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":26288,"parent_record_offset":26260,"ancestor_record_offset":20932,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":26260,"parent_record_offset":26224,"ancestor_record_offset":20932,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":26500,"parent_record_offset":26260,"ancestor_record_offset":20932,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":29192,"parent_record_offset":29160,"ancestor_record_offset":20932,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":27112,"parent_record_offset":27088,"ancestor_record_offset":20932,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":256,"record_offset":32044,"parent_record_offset":32016,"ancestor_record_offset":20932,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21928,"parent_record_offset":21892,"ancestor_record_offset":20932,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":25908,"parent_record_offset":25876,"ancestor_record_offset":20932,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":20932,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41328,"parent_record_offset":41288,"ancestor_record_offset":20932,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27324,"parent_record_offset":27284,"ancestor_record_offset":20932,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26576,"parent_record_offset":26544,"ancestor_record_offset":20932,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27232,"parent_record_offset":27188,"ancestor_record_offset":20932,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41508,"parent_record_offset":41328,"ancestor_record_offset":20932,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":20932,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41368,"parent_record_offset":41328,"ancestor_record_offset":20932,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27284,"parent_record_offset":27232,"ancestor_record_offset":20932,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26188,"parent_record_offset":26132,"ancestor_record_offset":20932,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26224,"parent_record_offset":26188,"ancestor_record_offset":20932,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26052,"parent_record_offset":26012,"ancestor_record_offset":20932,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":41872,"parent_record_offset":41252,"ancestor_record_offset":20932,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":26812,"parent_record_offset":26788,"ancestor_record_offset":20932,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":29020,"parent_record_offset":28992,"ancestor_record_offset":20932,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":20932,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":27188,"parent_record_offset":27144,"ancestor_record_offset":20932,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":20932,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":256,"record_offset":40808,"parent_record_offset":40784,"ancestor_record_offset":20932,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":256,"record_offset":32072,"parent_record_offset":32044,"ancestor_record_offset":20932,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":256,"record_offset":26868,"parent_record_offset":26812,"ancestor_record_offset":20932,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":256,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":20932,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":256,"record_offset":26544,"parent_record_offset":26132,"ancestor_record_offset":20932,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":256,"record_offset":41288,"parent_record_offset":41252,"ancestor_record_offset":20932,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":256,"record_offset":26092,"parent_record_offset":26052,"ancestor_record_offset":20932,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":256,"record_offset":26132,"parent_record_offset":26092,"ancestor_record_offset":20932,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$7522::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":256,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":20932,"inlinee":8059,"source_line":8940,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":256,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":20932,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":26324,"parent_record_offset":26288,"ancestor_record_offset":20932,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":30668,"parent_record_offset":30640,"ancestor_record_offset":20932,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":256,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":20932,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28080,"parent_record_offset":28056,"ancestor_record_offset":20932,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":20932,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":256,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":20932,"inlinee":10524,"source_line":219,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":31992,"parent_record_offset":31872,"ancestor_record_offset":20932,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":37440,"parent_record_offset":37196,"ancestor_record_offset":20932,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":31956,"parent_record_offset":31932,"ancestor_record_offset":20932,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":25504,"parent_record_offset":25480,"ancestor_record_offset":20932,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":256,"record_offset":34576,"parent_record_offset":34552,"ancestor_record_offset":20932,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":34420,"parent_record_offset":34252,"ancestor_record_offset":20932,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":20932,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":34380,"parent_record_offset":34356,"ancestor_record_offset":20932,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":256,"record_offset":26640,"parent_record_offset":26608,"ancestor_record_offset":20932,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":256,"record_offset":26708,"parent_record_offset":26672,"ancestor_record_offset":20932,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":33012,"parent_record_offset":32972,"ancestor_record_offset":20932,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":20932,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":256,"record_offset":31908,"parent_record_offset":31872,"ancestor_record_offset":20932,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":20932,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35992,"parent_record_offset":35968,"ancestor_record_offset":20932,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":32904,"parent_record_offset":32880,"ancestor_record_offset":20932,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26608,"parent_record_offset":26576,"ancestor_record_offset":20932,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":26788,"parent_record_offset":26708,"ancestor_record_offset":20932,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":20932,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":256,"record_offset":34332,"parent_record_offset":34308,"ancestor_record_offset":20932,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":20932,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":30692,"parent_record_offset":30668,"ancestor_record_offset":20932,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":34308,"parent_record_offset":34252,"ancestor_record_offset":20932,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":33040,"parent_record_offset":33012,"ancestor_record_offset":20932,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":27484,"parent_record_offset":27460,"ancestor_record_offset":20932,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":256,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":20932,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":256,"record_offset":24180,"parent_record_offset":24148,"ancestor_record_offset":20932,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":36612,"parent_record_offset":35724,"ancestor_record_offset":20932,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":23856,"parent_record_offset":22844,"ancestor_record_offset":20932,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":22944,"parent_record_offset":22884,"ancestor_record_offset":20932,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=132
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":21844,"parent_record_offset":21720,"ancestor_record_offset":20932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":33316,"parent_record_offset":33276,"ancestor_record_offset":20932,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":20932,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":34356,"parent_record_offset":34332,"ancestor_record_offset":20932,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":20932,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":21960,"parent_record_offset":21928,"ancestor_record_offset":20932,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":33740,"parent_record_offset":33684,"ancestor_record_offset":20932,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":27996,"parent_record_offset":27952,"ancestor_record_offset":20932,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":28608,"parent_record_offset":28580,"ancestor_record_offset":20932,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":23284,"parent_record_offset":23256,"ancestor_record_offset":20932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":26840,"parent_record_offset":26812,"ancestor_record_offset":20932,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":27608,"parent_record_offset":27416,"ancestor_record_offset":20932,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":21780,"parent_record_offset":21720,"ancestor_record_offset":20932,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":22060,"parent_record_offset":21720,"ancestor_record_offset":20932,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::SlicePartialEq::not_equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31844,"parent_record_offset":31816,"ancestor_record_offset":20932,"inlinee":13793,"source_line":48,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":32100,"parent_record_offset":32072,"ancestor_record_offset":20932,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31816,"parent_record_offset":31788,"ancestor_record_offset":20932,"inlinee":13791,"source_line":19,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":20932,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36276,"parent_record_offset":36088,"ancestor_record_offset":20932,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":31872,"parent_record_offset":31844,"ancestor_record_offset":20932,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":32128,"parent_record_offset":32100,"ancestor_record_offset":20932,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":20932,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36208,"parent_record_offset":36124,"ancestor_record_offset":20932,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":20932,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":26672,"parent_record_offset":26640,"ancestor_record_offset":20932,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":27952,"parent_record_offset":27916,"ancestor_record_offset":20932,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23696,"parent_record_offset":23520,"ancestor_record_offset":20932,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":20932,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":25936,"parent_record_offset":25908,"ancestor_record_offset":20932,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":22592,"parent_record_offset":22568,"ancestor_record_offset":20932,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":25960,"parent_record_offset":25936,"ancestor_record_offset":20932,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":22616,"parent_record_offset":22592,"ancestor_record_offset":20932,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23720,"parent_record_offset":23696,"ancestor_record_offset":20932,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":21988,"parent_record_offset":21928,"ancestor_record_offset":20932,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":20932,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":22020,"parent_record_offset":21892,"ancestor_record_offset":20932,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":256,"record_offset":32016,"parent_record_offset":31992,"ancestor_record_offset":20932,"inlinee":12086,"source_line":68,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":37496,"parent_record_offset":37196,"ancestor_record_offset":20932,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":37356,"parent_record_offset":37196,"ancestor_record_offset":20932,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":256,"record_offset":35164,"parent_record_offset":35096,"ancestor_record_offset":20932,"inlinee":13797,"source_line":42,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":20932,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":20932,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":256,"record_offset":23488,"parent_record_offset":23460,"ancestor_record_offset":20932,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":256,"record_offset":23520,"parent_record_offset":23488,"ancestor_record_offset":20932,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":256,"record_offset":23756,"parent_record_offset":23488,"ancestor_record_offset":20932,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":23552,"parent_record_offset":23520,"ancestor_record_offset":20932,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":20932,"inlinee":13784,"source_line":1035,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":24264,"parent_record_offset":24148,"ancestor_record_offset":20932,"inlinee":13785,"source_line":1048,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_graphics_device_and_swapchain::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":28032,"parent_record_offset":27916,"ancestor_record_offset":20932,"inlinee":13789,"source_line":1103,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::create_swapchain_with_fallback
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":37196,"parent_record_offset":35724,"ancestor_record_offset":20932,"inlinee":13802,"source_line":1296,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":35916,"parent_record_offset":35724,"ancestor_record_offset":20932,"inlinee":13799,"source_line":136,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":20932,"inlinee":13801,"source_line":152,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":256,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":20932,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":34528,"parent_record_offset":34496,"ancestor_record_offset":20932,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":34496,"parent_record_offset":34468,"ancestor_record_offset":20932,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":25528,"parent_record_offset":25504,"ancestor_record_offset":20932,"inlinee":10684,"source_line":46,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":22988,"parent_record_offset":22944,"ancestor_record_offset":20932,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$10::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":23060,"parent_record_offset":23028,"ancestor_record_offset":20932,"inlinee":10522,"source_line":87,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":23028,"parent_record_offset":22988,"ancestor_record_offset":20932,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":20932,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":22884,"parent_record_offset":22844,"ancestor_record_offset":20932,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":34252,"parent_record_offset":34220,"ancestor_record_offset":20932,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":20932,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":22804,"parent_record_offset":22764,"ancestor_record_offset":20932,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_maximized
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":20932,"inlinee":10563,"source_line":657,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39596,"parent_record_offset":37196,"ancestor_record_offset":20932,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":38204,"parent_record_offset":37196,"ancestor_record_offset":20932,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":36648,"parent_record_offset":35724,"ancestor_record_offset":20932,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":34220,"parent_record_offset":34188,"ancestor_record_offset":20932,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":22844,"parent_record_offset":22804,"ancestor_record_offset":20932,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25480,"parent_record_offset":25452,"ancestor_record_offset":20932,"inlinee":10682,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_maximized::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":20932,"inlinee":13788,"source_line":1047,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":20932,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1035
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D8
// 首条位置: {"module":256,"inlinee":13784}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":23920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":23920,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":23920,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23920,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":23920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1048
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D9
// 首条位置: {"module":256,"inlinee":13785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35DD
// 首条位置: {"module":256,"inlinee":13789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":256,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":28032,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28080,"parent_record_offset":28056,"ancestor_record_offset":28032,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":256,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":28032,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::try_get_devices_for_surface
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35D6
// 首条位置: {"module":256,"inlinee":13782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":21844,"parent_record_offset":21720,"ancestor_record_offset":21720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":21780,"parent_record_offset":21720,"ancestor_record_offset":21720,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":22060,"parent_record_offset":21720,"ancestor_record_offset":21720,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21928,"parent_record_offset":21892,"ancestor_record_offset":21720,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":21720,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":21720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":21960,"parent_record_offset":21928,"ancestor_record_offset":21720,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":21988,"parent_record_offset":21928,"ancestor_record_offset":21720,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":22020,"parent_record_offset":21892,"ancestor_record_offset":21720,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::try_init_swapchain
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35E6
// 首条位置: {"module":256,"inlinee":13798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":40508,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":36960,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":40536,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":36612,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":40648,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":35848,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":40376,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":35888,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":40564,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":40424,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::create_swapchain_with_fallback
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":37196,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":13802,"source_line":1296,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::select_surface_format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":35916,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":13799,"source_line":136,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":36648,"parent_record_offset":35724,"ancestor_record_offset":35724,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38576,"parent_record_offset":38552,"ancestor_record_offset":35724,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38600,"parent_record_offset":38576,"ancestor_record_offset":35724,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":36564,"parent_record_offset":36540,"ancestor_record_offset":35724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38552,"parent_record_offset":38528,"ancestor_record_offset":35724,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":36540,"parent_record_offset":36448,"ancestor_record_offset":35724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38676,"parent_record_offset":38460,"ancestor_record_offset":35724,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38460,"parent_record_offset":38432,"ancestor_record_offset":35724,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38496,"parent_record_offset":38460,"ancestor_record_offset":35724,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38528,"parent_record_offset":38496,"ancestor_record_offset":35724,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":39324,"parent_record_offset":39300,"ancestor_record_offset":35724,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":36476,"parent_record_offset":36448,"ancestor_record_offset":35724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":36448,"parent_record_offset":36424,"ancestor_record_offset":35724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$7522::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":256,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":35724,"inlinee":8059,"source_line":8940,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":256,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":35724,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":37440,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":256,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":35724,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":256,"record_offset":37068,"parent_record_offset":37044,"ancestor_record_offset":35724,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":36912,"parent_record_offset":36744,"ancestor_record_offset":35724,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":38724,"parent_record_offset":38700,"ancestor_record_offset":35724,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":36872,"parent_record_offset":36848,"ancestor_record_offset":35724,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":35724,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35992,"parent_record_offset":35968,"ancestor_record_offset":35724,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":256,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":35724,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":256,"record_offset":36824,"parent_record_offset":36800,"ancestor_record_offset":35724,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":36800,"parent_record_offset":36744,"ancestor_record_offset":35724,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":256,"record_offset":36504,"parent_record_offset":36476,"ancestor_record_offset":35724,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":39164,"parent_record_offset":38276,"ancestor_record_offset":35724,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":38360,"parent_record_offset":38312,"ancestor_record_offset":35724,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":36400,"parent_record_offset":35916,"ancestor_record_offset":35724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":38700,"parent_record_offset":38676,"ancestor_record_offset":35724,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":36848,"parent_record_offset":36824,"ancestor_record_offset":35724,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":35724,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":256,"record_offset":36176,"parent_record_offset":36124,"ancestor_record_offset":35724,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":38628,"parent_record_offset":38600,"ancestor_record_offset":35724,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":39708,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":39940,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":35724,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":256,"record_offset":36276,"parent_record_offset":36088,"ancestor_record_offset":35724,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":35724,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36208,"parent_record_offset":36124,"ancestor_record_offset":35724,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":35724,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":256,"record_offset":36124,"parent_record_offset":36088,"ancestor_record_offset":35724,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39004,"parent_record_offset":38828,"ancestor_record_offset":35724,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39092,"parent_record_offset":39064,"ancestor_record_offset":35724,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":37380,"parent_record_offset":37356,"ancestor_record_offset":35724,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":35724,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39028,"parent_record_offset":39004,"ancestor_record_offset":35724,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39116,"parent_record_offset":39092,"ancestor_record_offset":35724,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":37496,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":37356,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":38908,"parent_record_offset":38884,"ancestor_record_offset":35724,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":35724,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":256,"record_offset":38800,"parent_record_offset":38776,"ancestor_record_offset":35724,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":256,"record_offset":38828,"parent_record_offset":38800,"ancestor_record_offset":35724,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":256,"record_offset":39064,"parent_record_offset":38800,"ancestor_record_offset":35724,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":38860,"parent_record_offset":38828,"ancestor_record_offset":35724,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::select_surface_format::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":256,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":35724,"inlinee":13801,"source_line":152,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":37020,"parent_record_offset":36988,"ancestor_record_offset":35724,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":256,"record_offset":36988,"parent_record_offset":36960,"ancestor_record_offset":35724,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":38400,"parent_record_offset":38360,"ancestor_record_offset":35724,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":38432,"parent_record_offset":38400,"ancestor_record_offset":35724,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":35724,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":38312,"parent_record_offset":38276,"ancestor_record_offset":35724,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":36744,"parent_record_offset":36712,"ancestor_record_offset":35724,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":36680,"parent_record_offset":36648,"ancestor_record_offset":35724,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39596,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":38204,"parent_record_offset":37196,"ancestor_record_offset":35724,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":36712,"parent_record_offset":36680,"ancestor_record_offset":35724,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":38276,"parent_record_offset":38240,"ancestor_record_offset":35724,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":35724,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::create_swapchain_with_fallback
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35EA
// 首条位置: {"module":256,"inlinee":13802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":37440,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":256,"record_offset":37468,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":37692,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":39708,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":256,"record_offset":39940,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":256,"record_offset":37496,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":256,"record_offset":37356,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39596,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":38204,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38576,"parent_record_offset":38552,"ancestor_record_offset":37196,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38600,"parent_record_offset":38576,"ancestor_record_offset":37196,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":37980,"parent_record_offset":37956,"ancestor_record_offset":37196,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38552,"parent_record_offset":38528,"ancestor_record_offset":37196,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":37956,"parent_record_offset":37900,"ancestor_record_offset":37196,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":38676,"parent_record_offset":38460,"ancestor_record_offset":37196,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38460,"parent_record_offset":38432,"ancestor_record_offset":37196,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38496,"parent_record_offset":38460,"ancestor_record_offset":37196,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":38528,"parent_record_offset":38496,"ancestor_record_offset":37196,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":256,"record_offset":39324,"parent_record_offset":39300,"ancestor_record_offset":37196,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":37928,"parent_record_offset":37900,"ancestor_record_offset":37196,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":37900,"parent_record_offset":37872,"ancestor_record_offset":37196,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":38724,"parent_record_offset":38700,"ancestor_record_offset":37196,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":39164,"parent_record_offset":38276,"ancestor_record_offset":37196,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":38360,"parent_record_offset":38312,"ancestor_record_offset":37196,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":37716,"parent_record_offset":37692,"ancestor_record_offset":37196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":38700,"parent_record_offset":38676,"ancestor_record_offset":37196,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":38628,"parent_record_offset":38600,"ancestor_record_offset":37196,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39004,"parent_record_offset":38828,"ancestor_record_offset":37196,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39092,"parent_record_offset":39064,"ancestor_record_offset":37196,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":37380,"parent_record_offset":37356,"ancestor_record_offset":37196,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":37196,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39028,"parent_record_offset":39004,"ancestor_record_offset":37196,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":39116,"parent_record_offset":39092,"ancestor_record_offset":37196,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":38908,"parent_record_offset":38884,"ancestor_record_offset":37196,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":256,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":37196,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":256,"record_offset":38800,"parent_record_offset":38776,"ancestor_record_offset":37196,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":256,"record_offset":38828,"parent_record_offset":38800,"ancestor_record_offset":37196,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":256,"record_offset":39064,"parent_record_offset":38800,"ancestor_record_offset":37196,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":256,"record_offset":38860,"parent_record_offset":38828,"ancestor_record_offset":37196,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":256,"record_offset":38400,"parent_record_offset":38360,"ancestor_record_offset":37196,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":256,"record_offset":38432,"parent_record_offset":38400,"ancestor_record_offset":37196,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":37196,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":38312,"parent_record_offset":38276,"ancestor_record_offset":37196,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":256,"record_offset":38240,"parent_record_offset":38204,"ancestor_record_offset":37196,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":38276,"parent_record_offset":38240,"ancestor_record_offset":37196,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":256,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":37196,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":107,"record_offset":15876,"rva":1536192,"va":5370245312}
// 编译实例 RVAs: 0x1770c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140368730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2036285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e83d121f00","size":5,"soft_fail":false,"target_va":5372282672,"va":5370246382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2037612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e86c171f00","size":5,"soft_fail":false,"target_va":5372284016,"va":5370246399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036c5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2052593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8f1511f00","size":5,"soft_fail":false,"target_va":5372298704,"va":5370246106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2068029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e83d8e1f00","size":5,"soft_fail":false,"target_va":5372314208,"va":5370246174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2132341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e875892000","size":5,"soft_fail":false,"target_va":5372378176,"va":5370245830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387c30
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2164431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8cf062100","size":5,"soft_fail":false,"target_va":5372410928,"va":5370246492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2504237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e82d362600","size":5,"soft_fail":false,"target_va":5372749696,"va":5370245454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2503408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8f0322600","size":5,"soft_fail":false,"target_va":5372749712,"va":5370246299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33347868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e81cd9fc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5370245487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32732273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e87174f301","size":5,"soft_fail":false,"target_va":5402978144,"va":5370245866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7281090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8c2196f00","size":5,"soft_fail":false,"target_va":5377527120,"va":5370246025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32732925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8fd76f301","size":5,"soft_fail":false,"target_va":5402978944,"va":5370246014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::with_capacity
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-483856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8f09df8ff","size":5,"soft_fail":false,"target_va":5369761552,"va":5370245403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7147336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8480f6d00","size":5,"soft_fail":false,"target_va":5377393008,"va":5370245667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t433103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370245312,"raw_hex":"e8cf9b0600","size":5,"soft_fail":false,"target_va":5370678880,"va":5370245772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":107,"record_offset":16516,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":17144,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":107,"record_offset":17040,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":107,"record_offset":16192,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":7939,"source_line":574,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":107,"record_offset":16488,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":15876,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":15876,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":17316,"parent_record_offset":17292,"ancestor_record_offset":15876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":16324,"parent_record_offset":16288,"ancestor_record_offset":15876,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":15876,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":17292,"parent_record_offset":17268,"ancestor_record_offset":15876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":107,"record_offset":16288,"parent_record_offset":16256,"ancestor_record_offset":15876,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":107,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":15876,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":107,"record_offset":16736,"parent_record_offset":16708,"ancestor_record_offset":15876,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":107,"record_offset":16920,"parent_record_offset":16736,"ancestor_record_offset":15876,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":107,"record_offset":17268,"parent_record_offset":17244,"ancestor_record_offset":15876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":107,"record_offset":16672,"parent_record_offset":16644,"ancestor_record_offset":15876,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":107,"record_offset":16644,"parent_record_offset":16616,"ancestor_record_offset":15876,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":107,"record_offset":16588,"parent_record_offset":16544,"ancestor_record_offset":15876,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":107,"record_offset":16616,"parent_record_offset":16588,"ancestor_record_offset":15876,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":107,"record_offset":16544,"parent_record_offset":16516,"ancestor_record_offset":15876,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":107,"record_offset":16708,"parent_record_offset":16672,"ancestor_record_offset":15876,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":107,"record_offset":17108,"parent_record_offset":17080,"ancestor_record_offset":15876,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":107,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":15876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":107,"record_offset":16960,"parent_record_offset":16672,"ancestor_record_offset":15876,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":15876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":15876,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":107,"record_offset":16256,"parent_record_offset":16224,"ancestor_record_offset":15876,"inlinee":7898,"source_line":21,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::sync_channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":107,"record_offset":16224,"parent_record_offset":16192,"ancestor_record_offset":15876,"inlinee":9960,"source_line":64,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::compute_on_dedicated_thread::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":107,"record_offset":17080,"parent_record_offset":17040,"ancestor_record_offset":15876,"inlinee":9961,"source_line":1383,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::compute_on_dedicated_thread::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1593
// 首条位置: {"module":14,"inlinee":5523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":4512,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::SyncSender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":14,"record_offset":4484,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":5519,"source_line":678,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::run_game::run_game::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":14,"record_offset":3644,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":5525,"source_line":182,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3604,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3604,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3604,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3604,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":4676,"parent_record_offset":4640,"ancestor_record_offset":3604,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":3604,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":14,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":3604,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":14,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3604,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":3752,"parent_record_offset":3644,"ancestor_record_offset":3604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":4708,"parent_record_offset":4676,"ancestor_record_offset":3604,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":3688,"parent_record_offset":3644,"ancestor_record_offset":3604,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":4736,"parent_record_offset":4676,"ancestor_record_offset":3604,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":4768,"parent_record_offset":4640,"ancestor_record_offset":3604,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::run_game::run_game::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/run_game.rs
//     evidence={"module":14,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3604,"inlinee":5527,"source_line":184,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::compute_on_dedicated_thread::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26E9
// 首条位置: {"module":107,"inlinee":9961}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":107,"record_offset":17108,"parent_record_offset":17080,"ancestor_record_offset":17080,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::capped_task_pool_options
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35EE
// 首条位置: {"module":256,"inlinee":13806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::run_game::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 1428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":256,"record_offset":152504,"rva":4039280,"va":5372748400}
// 编译实例 RVAs: 0x3da270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d3fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1075080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372748400,"raw_hex":"e87898efff","size":5,"soft_fail":false,"target_va":5371674592,"va":5372749667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b6440
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: run_game.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-147523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372748400,"raw_hex":"e8bdbffdff","size":5,"soft_fail":false,"target_va":5372601408,"va":5372748926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::init_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1398481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372748400,"raw_hex":"e82fa9eaff","size":5,"soft_fail":false,"target_va":5371350416,"va":5372748892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2135049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372748400,"raw_hex":"e8f76bdfff","size":5,"soft_fail":false,"target_va":5370613600,"va":5372748644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2507736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372748400,"raw_hex":"e828bcd9ff","size":5,"soft_fail":false,"target_va":5370241184,"va":5372748915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":152628,"parent_record_offset":152504,"ancestor_record_offset":152504,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":256,"record_offset":155716,"parent_record_offset":152504,"ancestor_record_offset":152504,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":256,"record_offset":152652,"parent_record_offset":152628,"ancestor_record_offset":152504,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":256,"record_offset":152712,"parent_record_offset":152680,"ancestor_record_offset":152504,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":256,"record_offset":154116,"parent_record_offset":154092,"ancestor_record_offset":152504,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":154092,"parent_record_offset":154068,"ancestor_record_offset":152504,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":256,"record_offset":153740,"parent_record_offset":153716,"ancestor_record_offset":152504,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":153788,"parent_record_offset":153764,"ancestor_record_offset":152504,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":153764,"parent_record_offset":153740,"ancestor_record_offset":152504,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":153880,"parent_record_offset":153816,"ancestor_record_offset":152504,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":256,"record_offset":153968,"parent_record_offset":153944,"ancestor_record_offset":152504,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":152680,"parent_record_offset":152652,"ancestor_record_offset":152504,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":153716,"parent_record_offset":153692,"ancestor_record_offset":152504,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":256,"record_offset":154068,"parent_record_offset":154044,"ancestor_record_offset":152504,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":256,"record_offset":153424,"parent_record_offset":153400,"ancestor_record_offset":152504,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":153472,"parent_record_offset":153448,"ancestor_record_offset":152504,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":256,"record_offset":153400,"parent_record_offset":153376,"ancestor_record_offset":152504,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":152900,"parent_record_offset":152876,"ancestor_record_offset":152504,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":256,"record_offset":152984,"parent_record_offset":152960,"ancestor_record_offset":152504,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":152924,"parent_record_offset":152900,"ancestor_record_offset":152504,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":256,"record_offset":153088,"parent_record_offset":153064,"ancestor_record_offset":152504,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":153692,"parent_record_offset":152680,"ancestor_record_offset":152504,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":153904,"parent_record_offset":153880,"ancestor_record_offset":152504,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":153660,"parent_record_offset":152712,"ancestor_record_offset":152504,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":153944,"parent_record_offset":153764,"ancestor_record_offset":152504,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":153312,"parent_record_offset":153288,"ancestor_record_offset":152504,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":256,"record_offset":153232,"parent_record_offset":153208,"ancestor_record_offset":152504,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":256,"record_offset":153816,"parent_record_offset":153788,"ancestor_record_offset":152504,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":256,"record_offset":153852,"parent_record_offset":153816,"ancestor_record_offset":152504,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":256,"record_offset":153448,"parent_record_offset":153424,"ancestor_record_offset":152504,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":256,"record_offset":153376,"parent_record_offset":153352,"ancestor_record_offset":152504,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":152736,"parent_record_offset":152712,"ancestor_record_offset":152504,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":152760,"parent_record_offset":152736,"ancestor_record_offset":152504,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":256,"record_offset":153352,"parent_record_offset":153156,"ancestor_record_offset":152504,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":153264,"parent_record_offset":153180,"ancestor_record_offset":152504,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":153208,"parent_record_offset":153180,"ancestor_record_offset":152504,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":153608,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":153180,"parent_record_offset":153156,"ancestor_record_offset":152504,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":152812,"parent_record_offset":152788,"ancestor_record_offset":152504,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":152788,"parent_record_offset":152760,"ancestor_record_offset":152504,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":152836,"parent_record_offset":152812,"ancestor_record_offset":152504,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":153040,"parent_record_offset":153016,"ancestor_record_offset":152504,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":153064,"parent_record_offset":153040,"ancestor_record_offset":152504,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":153124,"parent_record_offset":153016,"ancestor_record_offset":152504,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":256,"record_offset":153016,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":256,"record_offset":153156,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":152876,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":152960,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":256,"record_offset":153524,"parent_record_offset":152836,"ancestor_record_offset":152504,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":256,"record_offset":153992,"parent_record_offset":153968,"ancestor_record_offset":152504,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00256.jsonl.gz"}
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
// 完整结构及原始类型记录: run_game.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::run_game::FrameUpdateContext
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::run_game::FrameUpdateContext>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::run_game::FrameUpdateContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::run_game::InitializedGraphics
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::run_game::InitializedGraphics>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::run_game::InitializedGraphics"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::run_game::InputLitePlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$6::add_to_app<bevy_app::plugin::sealed::PluginMarker,bevy_core::TaskPoolPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::InputLitePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_time::TimePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_state::app::StatesPlugin>", "constructor": "bevy_app::plugin::sealed::impl$6::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "bevy_core::TaskPoolPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::InputLitePlugin", "bevy_app::plugin::sealed::PluginMarker", "bevy_time::TimePlugin", "bevy_app::plugin::sealed::PluginMarker", "bevy_state::app::StatesPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$5::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::InputLitePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_time::TimePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_state::app::StatesPlugin>", "constructor": "bevy_app::plugin::sealed::impl$5::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::InputLitePlugin", "bevy_app::plugin::sealed::PluginMarker", "bevy_time::TimePlugin", "bevy_app::plugin::sealed::PluginMarker", "bevy_state::app::StatesPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::run_game::InputLitePlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::run_game::InputLitePlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::run_game::InputLitePlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::run_game::InputLitePlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::run_game::InputLitePlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::run_game::InputLitePlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::run_game::InputLitePlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::run_game::InputLitePlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::run_game::InputLitePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: run_game.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/tiny-glade/src/run_game.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: run_game.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":110576,"caller_aliases":{"alias_group_rva":110576,"count":1,"display_examples":["std::sys_common::backtrace::__rust_begin_short_backtrace"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":110969,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3770448,"caller_aliases":{"alias_group_rva":3770448,"count":1,"display_examples":["tiny_glade::main"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3777466,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3953456,"caller_aliases":{"alias_group_rva":3953456,"count":1,"display_examples":["tiny_glade::run_game::run_game"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3954587,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: run_game.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 128; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >,bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>", "bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >", "bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>", "bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>", "bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::async_load_audio_assets
// 信息 ID: 0efebbb58aa981732de43d5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=15456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::async_load_audio_assets::closure$0
// 信息 ID: 91b0c37a915496d5d4e8ba25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::async_load_audio_assets", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::async_load_audio_assets::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::async_load_audio_assets::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::async_load_audio_assets::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::run_game::async_load_audio_assets::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::async_load_audio_assets::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::async_load_audio_assets::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::async_load_audio_assets::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::async_load_audio_assets::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::async_load_audio_assets::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::async_load_audio_assets::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::async_load_audio_assets::closure_env$0,tuple$<> >; module=14, inline_record=2704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::async_load_audio_assets::closure$1
// 信息 ID: 672739a7773b58e21f1c481f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::run_game::async_load_audio_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=16524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::build_loading_screen_app
// 信息 ID: 25ae6e38bbdbbb723cb93f7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=122600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::capped_task_pool_options
// 信息 ID: c057f068a293bac29c4b7ccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=59440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::compute_on_dedicated_thread::closure$0
// 信息 ID: 0baa83e38240dc3d56fe1839
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::compute_on_dedicated_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; module=14, inline_record=3604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::compute_on_dedicated_thread::closure$1
// 信息 ID: b915a8cdc32a66c6f467ea8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::run_game::compute_on_dedicated_thread", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>; module=107, inline_record=17080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>
// 信息 ID: 46c2032462f723a5a63b8b67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::init::GraphicsInitError"]; 字段布局 = unknown。
// [类型关系] core::result::Result -> ["rhapsody::raw::init::GraphicsInstance", "enum2$<rhapsody::raw::init::GraphicsInitError>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >"]; 字段布局 = unknown。
// [类型关系] tiny_glade::run_game::compute_on_dedicated_thread -> ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c579b -> 0x1770c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::create_graphics_device_and_swapchain
// 信息 ID: d72a7b0d0a3fcdeac00b2e07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=20932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$0
// 信息 ID: 5c3a1de824e10742d5c44af9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::create_graphics_device_and_swapchain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=23920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$1
// 信息 ID: f4897883e4ea2178e8d04cda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::run_game::create_graphics_device_and_swapchain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=24264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::create_graphics_device_and_swapchain::closure$2
// 信息 ID: 8a0e2f1761954c3deedaaf9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::run_game::create_graphics_device_and_swapchain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=28032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::create_swapchain_with_fallback
// 信息 ID: 14d2ecf37dace6396eeb61dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=37196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::impl$0::build
// 信息 ID: 4d588978f619f6ba936e135f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game
// 信息 ID: d1ad1ce5fd56cb2c2e3343d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39a3ba -> 0x3c5330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game::closure$0
// 信息 ID: 4f524d4735bbd9658d280597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::run_game", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::run_game::run_game::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; module=14, inline_record=3644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game::closure$0::closure$0
// 信息 ID: 1f9c3ee9bd5b343a68acc446
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::run_game", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >; module=14, inline_record=3720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game::closure$1
// 信息 ID: 6b0edaa9038e1c079d85292b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::run_game::run_game", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=20408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game::closure$2
// 信息 ID: 2a0c8e30b3cc2a308509cc14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::run_game::run_game", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- std::thread::Builder::spawn<tiny_glade::run_game::run_game::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game::closure_env$2,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game::closure_env$2,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- core::ptr::drop_in_place<tiny_glade::run_game::run_game::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game::closure_env$2 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::run_game::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game::closure_env$2,tuple$<> >; module=14, inline_record=5060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread
// 信息 ID: da693d2f7180daf4f69116f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game::closure_env$2,tuple$<> > @ 0x1b179 -> 0x3cdb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$0
// 信息 ID: 716157407f3a701c0653231e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$0 <- bevy_app::sub_app::SubApp::edit_schedule<bevy_app::main_schedule::Update,tiny_glade::run_game::run_game_thread::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] bevy_app::sub_app::SubApp::edit_schedule<bevy_app::main_schedule::Update,tiny_glade::run_game::run_game_thread::closure_env$0>; module=245, inline_record=58580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$1
// 信息 ID: dd8a87782fe7dd9d6329cb7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a0914dd809de2440fd7e1f33: void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::system::function_system::SystemMeta::new<tiny_glade::run_game::run_game_thread::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$1 <- bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 29 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1> >; module=66, inline_record=1717712; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::run_game::run_game_thread::closure_env$1>; module=107, inline_record=2196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$2
// 信息 ID: 7198b1843b056bdcd115c396
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$2 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$2>; module=134, inline_record=12324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$3
// 信息 ID: 8228a6cfed0b1f7bc7f9ebcc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=106988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$4
// 信息 ID: 6b92871f698fee66343218af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$4 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$4>; module=134, inline_record=14136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$5
// 信息 ID: c525e4b43e65f3c9a0ff848b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=113816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$6
// 信息 ID: ae221e9e7310a1f20d302bb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$6 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$6 <- std::thread::Builder::spawn<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$6 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$6 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> >; module=14, inline_record=5796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::run_game_thread::closure$7
// 信息 ID: e79e274e2303a26164824499
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::run_game::run_game_thread", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- std::thread::Builder::spawn<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- core::ptr::drop_in_place<tiny_glade::run_game::run_game_thread::closure_env$7>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::run_game::run_game_thread::closure_env$7 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::run_game_thread::closure_env$7> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; module=14, inline_record=6056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::select_surface_format
// 信息 ID: fbb867e3b7dd0153ad61109a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=35916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::select_surface_format::closure$0
// 信息 ID: 40aebfebfc8aae0878089c34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::run_game::select_surface_format", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=36016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::try_get_devices_for_surface
// 信息 ID: 5d368aa4f69bf06cb8ffa177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=21720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::run_game::try_init_swapchain
// 信息 ID: a6f3684c1ef13b0ba0b4c9e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=35724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
