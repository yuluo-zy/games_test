// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/gui/confirm_incoming_panic_repro.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 067a2cc515771ecf4fa20e9de3c92a926baec6483568b93d321f074796a0bbc2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: confirm_incoming_panic_repro.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 70
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"PDB_registry_path_only","version":"0.10.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":70,"record_offset":12216,"rva":900064,"va":5369609184}
// 编译实例 RVAs: 0xdbbe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3140325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e8e5ea2f00","size":5,"soft_fail":false,"target_va":5372749696,"va":5369609366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3139971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e883e92f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369609736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fb90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6045555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e8733f5c00","size":5,"soft_fail":false,"target_va":5375654800,"va":5369609240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: confirm_incoming_panic_repro.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33983956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e8d48d0602","size":5,"soft_fail":false,"target_va":5403593360,"va":5369609399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33368112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e83028fd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369610027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7920541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e89ddb7800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369609342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputStream::frame_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6045000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e8483d5c00","size":5,"soft_fail":false,"target_va":5375654816,"va":5369609811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_buttons
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6387287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e857766100","size":5,"soft_fail":false,"target_va":5375996976,"va":5369609684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_description
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6387068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e87c756100","size":5,"soft_fail":false,"target_va":5375996736,"va":5369609663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::set_title
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6386851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e8a3746100","size":5,"soft_fail":false,"target_va":5375996496,"va":5369609640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rfd::message_dialog::MessageDialog::show
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6387486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609184,"raw_hex":"e81e776100","size":5,"soft_fail":false,"target_va":5375997184,"va":5369609693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":12396,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":70,"record_offset":12476,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":70,"record_offset":13156,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":13212,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":70,"record_offset":13072,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":8360,"source_line":27,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_direct] rfd::message_dialog::MessageDialog::set_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":70,"record_offset":13128,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":8363,"source_line":35,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":12776,"parent_record_offset":12752,"ancestor_record_offset":12216,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":12800,"parent_record_offset":12776,"ancestor_record_offset":12216,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":12216,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":12752,"parent_record_offset":12724,"ancestor_record_offset":12216,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":70,"record_offset":13340,"parent_record_offset":13284,"ancestor_record_offset":12216,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":70,"record_offset":12444,"parent_record_offset":12420,"ancestor_record_offset":12216,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":13312,"parent_record_offset":13284,"ancestor_record_offset":12216,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":12724,"parent_record_offset":12696,"ancestor_record_offset":12216,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":12696,"parent_record_offset":12668,"ancestor_record_offset":12216,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":12872,"parent_record_offset":12696,"ancestor_record_offset":12216,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":70,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12216,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":70,"record_offset":12632,"parent_record_offset":12604,"ancestor_record_offset":12216,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":70,"record_offset":12604,"parent_record_offset":12576,"ancestor_record_offset":12216,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":70,"record_offset":12548,"parent_record_offset":12504,"ancestor_record_offset":12216,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":70,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":12216,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":70,"record_offset":12504,"parent_record_offset":12476,"ancestor_record_offset":12216,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":70,"record_offset":12668,"parent_record_offset":12632,"ancestor_record_offset":12216,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":70,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":12216,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":70,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12216,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":70,"record_offset":12420,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":70,"record_offset":12908,"parent_record_offset":12632,"ancestor_record_offset":12216,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":12216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":70,"record_offset":12828,"parent_record_offset":12800,"ancestor_record_offset":12216,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00070.jsonl.gz"}
//   [pdb_inline_transitive] rfd::message_dialog::impl$5::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/message_dialog.rs
//     evidence={"module":70,"record_offset":13096,"parent_record_offset":13072,"ancestor_record_offset":12216,"inlinee":8362,"source_line":13,"archive":"_pdb/A/modules/00070.jsonl.gz"}
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
// 完整结构及原始类型记录: confirm_incoming_panic_repro.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: confirm_incoming_panic_repro.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: confirm_incoming_panic_repro.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro
// 信息 ID: 645e5bd1be2e74a0f930da93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 287a82fd992ad3fe6973b048: void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)> @ 0x66090 -> 0xdbbe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)> > @ 0xab5f2 -> 0xdbbe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
