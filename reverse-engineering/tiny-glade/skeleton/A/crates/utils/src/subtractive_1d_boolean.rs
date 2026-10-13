// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/subtractive_1d_boolean.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b62edd6f31a1dcf3cf95d39f781064528b89662cafb972159949d8b0523771bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: subtractive_1d_boolean.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2723
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2723,"record_offset":540,"rva":24640752,"va":5393349872}
// 编译实例 RVAs: 0x177fcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20600884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393349872,"raw_hex":"e8cca7c5fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393350591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-22044114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393349872,"raw_hex":"e82ea2affe","size":5,"soft_fail":false,"target_va":5371306160,"va":5393350269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2723,"record_offset":708,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2723,"record_offset":1064,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":2723,"record_offset":812,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$55::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":2723,"record_offset":1996,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":17482,"source_line":343,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2723,"record_offset":2520,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2723,"record_offset":736,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":540,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":2660,"parent_record_offset":2568,"ancestor_record_offset":540,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2723,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":540,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2723,"record_offset":1144,"parent_record_offset":1112,"ancestor_record_offset":540,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2723,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":540,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2723,"record_offset":1112,"parent_record_offset":1064,"ancestor_record_offset":540,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":2723,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":540,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":2723,"record_offset":2052,"parent_record_offset":1996,"ancestor_record_offset":540,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2723,"record_offset":888,"parent_record_offset":844,"ancestor_record_offset":540,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2723,"record_offset":944,"parent_record_offset":844,"ancestor_record_offset":540,"inlinee":5479,"source_line":1672,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2723,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":540,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2723,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":540,"inlinee":5481,"source_line":1515,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2723,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":540,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":2723,"record_offset":844,"parent_record_offset":812,"ancestor_record_offset":540,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2723,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2723,"record_offset":776,"parent_record_offset":736,"ancestor_record_offset":540,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2723,"record_offset":1176,"parent_record_offset":1064,"ancestor_record_offset":540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2723,"record_offset":244,"rva":24640624,"va":5393349744}
// 编译实例 RVAs: 0x177fc70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20600094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393349744,"raw_hex":"e8e2aac5fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5393349785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9628283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393349744,"raw_hex":"e87bea9200","size":5,"soft_fail":false,"target_va":5402978144,"va":5393349856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":392,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2723,"record_offset":420,"parent_record_offset":392,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2723,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02723.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2723,"record_offset":2952,"rva":6461936,"va":5375171056}
// 编译实例 RVAs: 0x6299f0
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
// 完整结构及原始类型记录: subtractive_1d_boolean.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::subtractive_1d_boolean::SubtractiveBoolean1D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: subtractive_1d_boolean.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: subtractive_1d_boolean.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<usize,core::num::error::ParseIntError> > (*)(ref$<str$>)
// 候选参数类型: ["ref$<str$>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<tempfile::dir::TempDir,std::io::error::Error> > (*)(std::path::PathBuf)
// 候选参数类型: ["std::path::PathBuf"]

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
// 信息 ID: 5538157bf030592615200280
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2011 -> 0x177fc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3ba5 -> 0x177fc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0xcd998b -> 0x177fc70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
// 信息 ID: b96159cda28415e235e0d71e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3db8a82f64813284db61e7ad: enum2$<core::result::Result<tempfile::dir::TempDir,std::io::error::Error> > (*)(std::path::PathBuf)
// [泛型宿主/闭包/直接引用方参数候选] 50195701fe9904aff6758f86: bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// [泛型宿主/闭包/直接引用方参数候选] 738bb63638a82c50e104f256: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 8b8a159fb78dfc8408019c67: enum2$<core::result::Result<usize,core::num::error::ParseIntError> > (*)(ref$<str$>)
// 引用方: 原程序直接转移 691 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::path::impl$57::eq | std::path::impl$57::eq @ 0x40af -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::path::impl$57::eq | std::path::impl$57::eq @ 0x40c7 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x713a -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6ebd -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6ef9 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6f86 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6fbc -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6fd1 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x70b9 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x790b -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6187 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6372 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x63b7 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x63c6 -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x63ff -> 0x6299f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x640e -> 0x6299f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
// 信息 ID: 9b45f97ec9b018989aff38a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd262a -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd26f0 -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd27bb -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2844 -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3c70 -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0xcd99ce -> 0x177fcf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
