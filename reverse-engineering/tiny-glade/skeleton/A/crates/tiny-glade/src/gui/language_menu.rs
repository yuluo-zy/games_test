// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/gui/language_menu.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 832eeb9ba5234f7457c2345df6d835468ba02a070af0726600136528b7fd85ca
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 45, 58
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::language_menu::do_gui_language_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":45,"record_offset":16576,"rva":433840,"va":5369142960}
// 编译实例 RVAs: 0x69eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f2e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8ff4f0100","size":5,"soft_fail":false,"target_va":5369230048,"va":5369144028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t472177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e871340700","size":5,"soft_fail":false,"target_va":5369616080,"va":5369143898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t472557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8ed350700","size":5,"soft_fail":false,"target_va":5369616480,"va":5369143918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: language_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t669694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8fe370a00","size":5,"soft_fail":false,"target_va":5369813552,"va":5369143853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4016930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8224b3d00","size":5,"soft_fail":false,"target_va":5373160864,"va":5369143929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4017715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8334e3d00","size":5,"soft_fail":false,"target_va":5373161712,"va":5369143992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::gap
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4037489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e8719b3d00","size":5,"soft_fail":false,"target_va":5373181472,"va":5369143978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4037471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369142960,"raw_hex":"e85f9b3d00","size":5,"soft_fail":false,"target_va":5373181424,"va":5369143948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":45,"record_offset":16712,"parent_record_offset":16576,"ancestor_record_offset":16576,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00045.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":45,"record_offset":16760,"parent_record_offset":16736,"ancestor_record_offset":16576,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00045.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":45,"record_offset":16792,"parent_record_offset":16736,"ancestor_record_offset":16576,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00045.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":45,"record_offset":16736,"parent_record_offset":16712,"ancestor_record_offset":16576,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F55
// 首条位置: {"module":58,"inlinee":8021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":58,"record_offset":6196,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":6736,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":58,"record_offset":6124,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":58,"record_offset":6048,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":5988,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":5988,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":5988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":6492,"parent_record_offset":6456,"ancestor_record_offset":5988,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":6892,"parent_record_offset":6836,"ancestor_record_offset":5988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":5988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":6456,"parent_record_offset":6428,"ancestor_record_offset":5988,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":5988,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":6612,"parent_record_offset":6428,"ancestor_record_offset":5988,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":5988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":5988,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":6336,"parent_record_offset":6308,"ancestor_record_offset":5988,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":5988,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":6308,"parent_record_offset":6280,"ancestor_record_offset":5988,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":5988,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":58,"record_offset":6224,"parent_record_offset":6196,"ancestor_record_offset":5988,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":58,"record_offset":6400,"parent_record_offset":6364,"ancestor_record_offset":5988,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":58,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":5988,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":58,"record_offset":6652,"parent_record_offset":6364,"ancestor_record_offset":5988,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":6164,"parent_record_offset":6124,"ancestor_record_offset":5988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":58,"record_offset":6088,"parent_record_offset":6048,"ancestor_record_offset":5988,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":6568,"parent_record_offset":6540,"ancestor_record_offset":5988,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
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
// 完整结构及原始类型记录: language_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: language_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: language_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu
// 信息 ID: 7c0b14a14342992cd64b1b91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::language_menu::do_gui_language_menu::closure$0
// 信息 ID: 94b3294dcaed4445a8753066
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::language_menu::do_gui_language_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0 <- toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>; module=58, inline_record=5988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
