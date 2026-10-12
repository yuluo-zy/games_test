// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/toolbar_systems.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4efa70ce46d11243dec48d59d90f7f6c4232b3ab927b08e52e852b9cdb34549b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2495, 3698
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2495,"record_offset":7064,"rva":23456368,"va":5392165488}
// 编译实例 RVAs: 0x165ea70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9b620
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5490897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e8d1c85300","size":5,"soft_fail":false,"target_va":5397657120,"va":5392166218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e8c0d55300","size":5,"soft_fail":false,"target_va":5397659968,"va":5392165755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e87ad75300","size":5,"soft_fail":false,"target_va":5397660448,"va":5392165793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c3f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e85cd85300","size":5,"soft_fail":false,"target_va":5397660656,"va":5392165775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e8cdd85300","size":5,"soft_fail":false,"target_va":5397660864,"va":5392165870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c590
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e83dd95300","size":5,"soft_fail":false,"target_va":5397661072,"va":5392165966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e881da5300","size":5,"soft_fail":false,"target_va":5397661280,"va":5392165850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e82bdb5300","size":5,"soft_fail":false,"target_va":5397661488,"va":5392165888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e861dc5300","size":5,"soft_fail":false,"target_va":5397661952,"va":5392166042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10813457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e81100a500","size":5,"soft_fail":false,"target_va":5402980128,"va":5392166666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12405839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e8b1b342ff","size":5,"soft_fail":false,"target_va":5379760416,"va":5392166250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-12405359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392165488,"raw_hex":"e891b542ff","size":5,"soft_fail":false,"target_va":5379760832,"va":5392166186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2495,"record_offset":7892,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=20
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":7392,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":7216,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":7932,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":7476,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::roots_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":7448,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":84151,"source_line":70,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=20
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":7612,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":84152,"source_line":66,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=20
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":7064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":7272,"parent_record_offset":7240,"ancestor_record_offset":7064,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2495,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":7064,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":7064,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2495,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":7064,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":7520,"parent_record_offset":7476,"ancestor_record_offset":7064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":7064,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2495,"record_offset":9160,"parent_record_offset":9124,"ancestor_record_offset":7064,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_utils::taffy_mut_style
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_utils.rs
//     evidence={"module":2495,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":7064,"inlinee":84154,"source_line":23,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::toolbar_systems::toolbar_alignment::closure$16
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2495,"record_offset":9344,"parent_record_offset":9280,"ancestor_record_offset":7064,"inlinee":84156,"source_line":260,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::toolbar_systems::toolbar_alignment::closure$17
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2495,"record_offset":9192,"parent_record_offset":9124,"ancestor_record_offset":7064,"inlinee":84158,"source_line":265,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::toolbar_systems::toolbar_alignment::closure$18
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2495,"record_offset":9724,"parent_record_offset":9664,"ancestor_record_offset":7064,"inlinee":84159,"source_line":273,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::toolbar_systems::toolbar_alignment::closure$19
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2495,"record_offset":9548,"parent_record_offset":9488,"ancestor_record_offset":7064,"inlinee":84157,"source_line":278,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2495,"record_offset":9848,"rva":23457616,"va":5392166736}
// 编译实例 RVAs: 0x165ef50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5493148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e89cd15300","size":5,"soft_fail":false,"target_va":5397659968,"va":5392166815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5493692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e8bcd35300","size":5,"soft_fail":false,"target_va":5397660656,"va":5392166959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5493963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e8cbd45300","size":5,"soft_fail":false,"target_va":5397660864,"va":5392166896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e8dad55300","size":5,"soft_fail":false,"target_va":5397661072,"va":5392166833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5494271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e8ffd55300","size":5,"soft_fail":false,"target_va":5397661280,"va":5392167004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392166736,"raw_hex":"e81dd95300","size":5,"soft_fail":false,"target_va":5397661952,"va":5392166878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141b9c320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5493579","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392166736,"raw_hex":"e94bd35300","size":5,"soft_fail":false,"target_va":5397660448,"va":5392166864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141b9c730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5494556","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392166736,"raw_hex":"e91cd75300","size":5,"soft_fail":false,"target_va":5397661488,"va":5392166927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":9984,"parent_record_offset":9848,"ancestor_record_offset":9848,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":10040,"parent_record_offset":9848,"ancestor_record_offset":9848,"inlinee":84152,"source_line":66,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9848,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_auto_hide
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2495,"record_offset":10856,"rva":23457952,"va":5392167072}
// 编译实例 RVAs: 0x165f0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140597150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17596706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e8de7ef3fe","size":5,"soft_fail":false,"target_va":5374570832,"va":5392167533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20860774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e89ab0c1fe","size":5,"soft_fail":false,"target_va":5371306912,"va":5392167681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17768234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e8d6e0f0fe","size":5,"soft_fail":false,"target_va":5374399200,"va":5392167429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16754985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e8d75600ff","size":5,"soft_fail":false,"target_va":5375412208,"va":5392167188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15229757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e8c39c17ff","size":5,"soft_fail":false,"target_va":5376938064,"va":5392167816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7747194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e886c989ff","size":5,"soft_fail":false,"target_va":5384420320,"va":5392167509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7747494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392167072,"raw_hex":"e85ac889ff","size":5,"soft_fail":false,"target_va":5384420000,"va":5392167489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":11004,"parent_record_offset":10856,"ancestor_record_offset":10856,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2495,"record_offset":11408,"parent_record_offset":10856,"ancestor_record_offset":10856,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2495,"record_offset":11376,"parent_record_offset":10856,"ancestor_record_offset":10856,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2495,"record_offset":11064,"parent_record_offset":10856,"ancestor_record_offset":10856,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::root_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":11980,"parent_record_offset":10856,"ancestor_record_offset":10856,"inlinee":84148,"source_line":74,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2495,"record_offset":11592,"parent_record_offset":11560,"ancestor_record_offset":10856,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2495,"record_offset":11560,"parent_record_offset":11524,"ancestor_record_offset":10856,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2495,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":10856,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":10856,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":11432,"parent_record_offset":11408,"ancestor_record_offset":10856,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2495,"record_offset":11280,"parent_record_offset":11216,"ancestor_record_offset":10856,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2495,"record_offset":11092,"parent_record_offset":11064,"ancestor_record_offset":10856,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":10856,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":12072,"parent_record_offset":12036,"ancestor_record_offset":10856,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":11336,"parent_record_offset":11308,"ancestor_record_offset":10856,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":12100,"parent_record_offset":12036,"ancestor_record_offset":10856,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":10856,"inlinee":13747,"source_line":296,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":12036,"parent_record_offset":12004,"ancestor_record_offset":10856,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":2495,"record_offset":12192,"parent_record_offset":12160,"ancestor_record_offset":10856,"inlinee":51336,"source_line":448,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$7::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":2495,"record_offset":12220,"parent_record_offset":12192,"ancestor_record_offset":10856,"inlinee":15054,"source_line":260,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_layout::impl$1::root_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":12160,"parent_record_offset":12004,"ancestor_record_offset":10856,"inlinee":84150,"source_line":75,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_auto_hide
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2495,"record_offset":3656,"rva":23454160,"va":5392163280}
// 编译实例 RVAs: 0x165e1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140597150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17593918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e8c289f3fe","size":5,"soft_fail":false,"target_va":5374570832,"va":5392164745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20857995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e875bbc1fe","size":5,"soft_fail":false,"target_va":5371306912,"va":5392164902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17765529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e867ebf0fe","size":5,"soft_fail":false,"target_va":5374399200,"va":5392164724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16751686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e8ba6300ff","size":5,"soft_fail":false,"target_va":5375412208,"va":5392163889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15227109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e81ba717ff","size":5,"soft_fail":false,"target_va":5376938064,"va":5392165168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7744392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e878d489ff","size":5,"soft_fail":false,"target_va":5384420320,"va":5392164707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7744687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392163280,"raw_hex":"e851d389ff","size":5,"soft_fail":false,"target_va":5384420000,"va":5392164682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":3820,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":6052,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2495,"record_offset":6140,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2495,"record_offset":4348,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2495,"record_offset":4152,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2495,"record_offset":5420,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":3940,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2495,"record_offset":6108,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":5392,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::root_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":6716,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":84148,"source_line":74,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2495,"record_offset":6328,"parent_record_offset":6296,"ancestor_record_offset":3656,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2495,"record_offset":6296,"parent_record_offset":6256,"ancestor_record_offset":3656,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2495,"record_offset":6256,"parent_record_offset":6220,"ancestor_record_offset":3656,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3656,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":3656,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":3656,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2495,"record_offset":6220,"parent_record_offset":6140,"ancestor_record_offset":3656,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":5324,"parent_record_offset":5268,"ancestor_record_offset":3656,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":4764,"parent_record_offset":4732,"ancestor_record_offset":3656,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":3656,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":3656,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":3656,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":4732,"parent_record_offset":4624,"ancestor_record_offset":3656,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2495,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":3656,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2495,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3656,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2495,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3656,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2495,"record_offset":4380,"parent_record_offset":4348,"ancestor_record_offset":3656,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2495,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3656,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2495,"record_offset":5688,"parent_record_offset":5644,"ancestor_record_offset":3656,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2495,"record_offset":5268,"parent_record_offset":4380,"ancestor_record_offset":3656,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2495,"record_offset":4624,"parent_record_offset":4380,"ancestor_record_offset":3656,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2495,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":3656,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2495,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":3656,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2495,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":3656,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2495,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3656,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2495,"record_offset":4952,"parent_record_offset":4888,"ancestor_record_offset":3656,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2495,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":3656,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":5200,"parent_record_offset":5032,"ancestor_record_offset":3656,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":4444,"parent_record_offset":4380,"ancestor_record_offset":3656,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":3656,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":3656,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":3656,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2495,"record_offset":4540,"parent_record_offset":4380,"ancestor_record_offset":3656,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":4032,"parent_record_offset":3968,"ancestor_record_offset":3656,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2495,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":3656,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":3656,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3656,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3656,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":5108,"parent_record_offset":5084,"ancestor_record_offset":3656,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2495,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":3656,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2495,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":3656,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":3656,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":3656,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":3656,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":6360,"parent_record_offset":6256,"ancestor_record_offset":3656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2495,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3656,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2495,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3656,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2495,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3656,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2495,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":3656,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":5872,"parent_record_offset":5808,"ancestor_record_offset":3656,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":6836,"parent_record_offset":6772,"ancestor_record_offset":3656,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":4476,"parent_record_offset":4380,"ancestor_record_offset":3656,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":3656,"inlinee":13747,"source_line":296,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":6772,"parent_record_offset":6740,"ancestor_record_offset":3656,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2495,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":3656,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":2495,"record_offset":6928,"parent_record_offset":6896,"ancestor_record_offset":3656,"inlinee":51336,"source_line":448,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$7::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":2495,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":3656,"inlinee":15054,"source_line":260,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_layout::impl$1::root_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":6896,"parent_record_offset":6740,"ancestor_record_offset":3656,"inlinee":84150,"source_line":75,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB12
// 首条位置: {"module":3698,"inlinee":109330}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB0F
// 首条位置: {"module":3698,"inlinee":109327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB0E
// 首条位置: {"module":3698,"inlinee":109326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB11
// 首条位置: {"module":3698,"inlinee":109329}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB0D
// 首条位置: {"module":3698,"inlinee":109325}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x148BC
// 首条位置: {"module":2495,"inlinee":84156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$17
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x148BE
// 首条位置: {"module":2495,"inlinee":84158}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$18
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x148BF
// 首条位置: {"module":2495,"inlinee":84159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$19
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 278
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x148BD
// 首条位置: {"module":2495,"inlinee":84157}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB10
// 首条位置: {"module":3698,"inlinee":109328}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB0C
// 首条位置: {"module":3698,"inlinee":109324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1AB0A
// 首条位置: {"module":3698,"inlinee":109322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_steal_mouse_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2495,"record_offset":12356,"rva":23458944,"va":5392168064}
// 编译实例 RVAs: 0x165f480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22551813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8fbe2a7fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5392169456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20515344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8f0f5c6fe","size":5,"soft_fail":false,"target_va":5371654272,"va":5392169611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19419833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e847add7fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5392169524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19419938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8deacd7fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5392169645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d92d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16280005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e83b9607ff","size":5,"soft_fail":false,"target_va":5375890128,"va":5392170128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16195988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e86cde08ff","size":5,"soft_fail":false,"target_va":5375974272,"va":5392170255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6fc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12516943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8b10141ff","size":5,"soft_fail":false,"target_va":5379652640,"va":5392169578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11423875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e88350ae00","size":5,"soft_fail":false,"target_va":5403593360,"va":5392169480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10808338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e812eca400","size":5,"soft_fail":false,"target_va":5402978144,"va":5392169801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14639403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8d59e20ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5392169286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10808868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e824eea400","size":5,"soft_fail":false,"target_va":5402978688,"va":5392169815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e866e0ffff","size":5,"soft_fail":false,"target_va":5392161584,"va":5392169669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10811517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e87df8a400","size":5,"soft_fail":false,"target_va":5402980128,"va":5392168606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e807e7ffff","size":5,"soft_fail":false,"target_va":5392161776,"va":5392168164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10812410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8fafba400","size":5,"soft_fail":false,"target_va":5402980656,"va":5392168241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16397852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8e4c905ff","size":5,"soft_fail":false,"target_va":5375771552,"va":5392169399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16398210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e87ec805ff","size":5,"soft_fail":false,"target_va":5375771168,"va":5392169373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16397997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e853c905ff","size":5,"soft_fail":false,"target_va":5375771392,"va":5392169384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16398145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8bfc805ff","size":5,"soft_fail":false,"target_va":5375771280,"va":5392169420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15811871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8e1ba0eff","size":5,"soft_fail":false,"target_va":5376356912,"va":5392168778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15807979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e815ca0eff","size":5,"soft_fail":false,"target_va":5376360560,"va":5392168534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15812005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e85bba0eff","size":5,"soft_fail":false,"target_va":5376356144,"va":5392168144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h6a28a4a69af7475d (.llvm.7250520506814577795)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12546575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e8f18d40ff","size":5,"soft_fail":false,"target_va":5379621984,"va":5392168554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::layout
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12408252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392168064,"raw_hex":"e844aa42ff","size":5,"soft_fail":false,"target_va":5379760912,"va":5392169159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2495,"record_offset":15944,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":15888,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":15196,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2495,"record_offset":15572,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":16768,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2495,"record_offset":15852,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2495,"record_offset":15488,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2495,"record_offset":16252,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":27224,"source_line":87,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2495,"record_offset":16140,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2495,"record_offset":16028,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2495,"record_offset":16196,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2495,"record_offset":16084,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":14148,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":12552,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":13136,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2495,"record_offset":15824,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":84160,"source_line":79,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":12356,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":12356,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16920,"parent_record_offset":16896,"ancestor_record_offset":12356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16396,"parent_record_offset":16360,"ancestor_record_offset":12356,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16428,"parent_record_offset":16396,"ancestor_record_offset":12356,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2495,"record_offset":16896,"parent_record_offset":16840,"ancestor_record_offset":12356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2495,"record_offset":16360,"parent_record_offset":16324,"ancestor_record_offset":12356,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2495,"record_offset":15992,"parent_record_offset":15968,"ancestor_record_offset":12356,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2495,"record_offset":16868,"parent_record_offset":16840,"ancestor_record_offset":12356,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2495,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":12356,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2495,"record_offset":16324,"parent_record_offset":16252,"ancestor_record_offset":12356,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2495,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":12356,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2495,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":12356,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2495,"record_offset":15368,"parent_record_offset":15220,"ancestor_record_offset":12356,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":12356,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":12356,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14576,"parent_record_offset":14548,"ancestor_record_offset":12356,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14488,"parent_record_offset":14460,"ancestor_record_offset":12356,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14520,"parent_record_offset":14488,"ancestor_record_offset":12356,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14692,"parent_record_offset":14520,"ancestor_record_offset":12356,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2495,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":12356,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2495,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":12356,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2495,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":12356,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":2495,"record_offset":15256,"parent_record_offset":15220,"ancestor_record_offset":12356,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2495,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":12356,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2495,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":12356,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2495,"record_offset":12772,"parent_record_offset":12676,"ancestor_record_offset":12356,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2495,"record_offset":16052,"parent_record_offset":16028,"ancestor_record_offset":12356,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2495,"record_offset":16108,"parent_record_offset":16084,"ancestor_record_offset":12356,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2495,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":12356,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":15304,"parent_record_offset":15280,"ancestor_record_offset":12356,"inlinee":6226,"source_line":46,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":15340,"parent_record_offset":15220,"ancestor_record_offset":12356,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":12356,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2495,"record_offset":15280,"parent_record_offset":15256,"ancestor_record_offset":12356,"inlinee":13144,"source_line":160,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":12356,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":14384,"parent_record_offset":14356,"ancestor_record_offset":12356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":13732,"parent_record_offset":13272,"ancestor_record_offset":12356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":12356,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":14432,"parent_record_offset":14256,"ancestor_record_offset":12356,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2495,"record_offset":15652,"parent_record_offset":15572,"ancestor_record_offset":12356,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2495,"record_offset":13084,"parent_record_offset":13056,"ancestor_record_offset":12356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2495,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":12356,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":14756,"parent_record_offset":14460,"ancestor_record_offset":12356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2495,"record_offset":15536,"parent_record_offset":15512,"ancestor_record_offset":12356,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":15776,"parent_record_offset":15752,"ancestor_record_offset":12356,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":12356,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":13484,"parent_record_offset":13460,"ancestor_record_offset":12356,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2495,"record_offset":13692,"parent_record_offset":13660,"ancestor_record_offset":12356,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":12356,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":16504,"parent_record_offset":16476,"ancestor_record_offset":12356,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2495,"record_offset":14648,"parent_record_offset":14624,"ancestor_record_offset":12356,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2495,"record_offset":15156,"parent_record_offset":14220,"ancestor_record_offset":12356,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2495,"record_offset":12708,"parent_record_offset":12676,"ancestor_record_offset":12356,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2495,"record_offset":12736,"parent_record_offset":12708,"ancestor_record_offset":12356,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2495,"record_offset":15512,"parent_record_offset":15488,"ancestor_record_offset":12356,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":15752,"parent_record_offset":15728,"ancestor_record_offset":12356,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":12356,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":12356,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":15728,"parent_record_offset":15676,"ancestor_record_offset":12356,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2495,"record_offset":13436,"parent_record_offset":13328,"ancestor_record_offset":12356,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2495,"record_offset":12676,"parent_record_offset":12648,"ancestor_record_offset":12356,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2495,"record_offset":13168,"parent_record_offset":13136,"ancestor_record_offset":12356,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2495,"record_offset":13300,"parent_record_offset":13272,"ancestor_record_offset":12356,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2495,"record_offset":13272,"parent_record_offset":13240,"ancestor_record_offset":12356,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2495,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":12356,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2495,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":12356,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2495,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":12356,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2495,"record_offset":13764,"parent_record_offset":13240,"ancestor_record_offset":12356,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":12872,"parent_record_offset":12840,"ancestor_record_offset":12356,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":13032,"parent_record_offset":12872,"ancestor_record_offset":12356,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":12356,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":12588,"parent_record_offset":12552,"ancestor_record_offset":12356,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":13056,"parent_record_offset":13032,"ancestor_record_offset":12356,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":12648,"parent_record_offset":12616,"ancestor_record_offset":12356,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2495,"record_offset":12616,"parent_record_offset":12588,"ancestor_record_offset":12356,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2495,"record_offset":13328,"parent_record_offset":13300,"ancestor_record_offset":12356,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2495,"record_offset":13576,"parent_record_offset":13328,"ancestor_record_offset":12356,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2495,"record_offset":13548,"parent_record_offset":13328,"ancestor_record_offset":12356,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2495,"record_offset":13380,"parent_record_offset":13328,"ancestor_record_offset":12356,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2495,"record_offset":16668,"parent_record_offset":16632,"ancestor_record_offset":12356,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2495,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":12356,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2495,"record_offset":16724,"parent_record_offset":16632,"ancestor_record_offset":12356,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$0::to_widget_name_hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":2495,"record_offset":16296,"parent_record_offset":16252,"ancestor_record_offset":12356,"inlinee":25476,"source_line":33,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":14184,"parent_record_offset":14148,"ancestor_record_offset":12356,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2495,"record_offset":14296,"parent_record_offset":14256,"ancestor_record_offset":12356,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":14460,"parent_record_offset":14256,"ancestor_record_offset":12356,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2495,"record_offset":14728,"parent_record_offset":14460,"ancestor_record_offset":12356,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2495,"record_offset":16556,"parent_record_offset":16252,"ancestor_record_offset":12356,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2495,"record_offset":14324,"parent_record_offset":14296,"ancestor_record_offset":12356,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2495,"record_offset":14356,"parent_record_offset":14324,"ancestor_record_offset":12356,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2495,"record_offset":14256,"parent_record_offset":14220,"ancestor_record_offset":12356,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02495.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2495,"record_offset":14220,"parent_record_offset":14184,"ancestor_record_offset":12356,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02495.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment
// 信息 ID: e455c84a9122b78496772571
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 01f77029bb05d937ca08973d: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> @ 0xa8fa9b -> 0x165ea70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> > @ 0x15d8a46 -> 0x165ea70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$0
// 信息 ID: c52e66d8ca534612c6c2f977
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$0 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>; module=3698, inline_record=6100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$11
// 信息 ID: 511d6812739d7c9217ef2afb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$11 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>; module=3698, inline_record=4204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$16
// 信息 ID: 1d630534ec3e3d5753cb43d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 16, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_systems::toolbar_alignment; module=2495, inline_record=9344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$17
// 信息 ID: a7dce6b8650152a6677e4d60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 17, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_systems::toolbar_alignment; module=2495, inline_record=9192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$18
// 信息 ID: 0a4be4f2c23ffebb17781144
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 18, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_systems::toolbar_alignment; module=2495, inline_record=9724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$19
// 信息 ID: 96b4edeb323f1f3957b3b267
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 19, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_systems::toolbar_alignment; module=2495, inline_record=9548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$3
// 信息 ID: dd4519ea6bce09702f7b307a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$3 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$3>; module=3698, inline_record=4828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$5
// 信息 ID: 3c17c40f07873ba423189983
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$5 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$5>; module=3698, inline_record=4516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$6
// 信息 ID: f41188d0e5b7431fc2f0ad15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$6 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$6>; module=3698, inline_record=5452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_auto_hide
// 信息 ID: b61017b7dcbcec3badcb5f5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3efc65e956e1bfaaf24b8485: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0xa8f597 -> 0x165e1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > @ 0x15d9f75 -> 0x165e1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_steal_mouse_input
// 信息 ID: 0e24adc78f8c94e943d71b56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 11a6f58dfe10fc370b96e487: void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0xa8e4f7 -> 0x165f480 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x15e7e19 -> 0x165f480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment
// 信息 ID: 69321e54fef460be7c9d3ad4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a3f62fabb5a5af9af95c6ea1: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)> @ 0xa8e10f -> 0x165ef50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)> > @ 0x15de39a -> 0x165ef50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment::closure$0
// 信息 ID: 7a73fdee98cb96a749834ce5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_systems::undo_redo_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::undo_redo_alignment::closure_env$0 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>; module=3698, inline_record=5140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment::closure$8
// 信息 ID: fe0799a0ba84435af6c5bc87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "toolbar::toolbar_systems::undo_redo_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::undo_redo_alignment::closure_env$8 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$8>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$8>; module=3698, inline_record=3892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment::closure$9
// 信息 ID: 6f663cd1e802a18fd16d19a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::toolbar_systems::undo_redo_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::undo_redo_alignment::closure_env$9 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$9>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$9>; module=3698, inline_record=3232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_auto_hide
// 信息 ID: c80749c58cb36dc3cfbc2344
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86fd301ac998587a217b41d9: void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0xa8eba1 -> 0x165f0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > @ 0x15e5511 -> 0x165f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
