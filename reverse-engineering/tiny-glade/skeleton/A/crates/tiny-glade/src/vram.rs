// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/vram.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 04f64078614403c2809d96a0ad05dce47c753aacd655f6c9d884d2104fdae329
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vram.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 28, 153
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_resolution_scale_mem_estimate_text
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":22704,"rva":2823120,"va":5371532240}
// 编译实例 RVAs: 0x2b13d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5996222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371532240,"raw_hex":"e8be7e5b00","size":5,"soft_fail":false,"target_va":5377529888,"va":5371533661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4784162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371532240,"raw_hex":"e822004900","size":5,"soft_fail":false,"target_va":5376316464,"va":5371532297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4680668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371532240,"raw_hex":"e8dc6b4700","size":5,"soft_fail":false,"target_va":5376213328,"va":5371532655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4681296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371532240,"raw_hex":"e8506e4700","size":5,"soft_fail":false,"target_va":5376214000,"va":5371532699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":153,"record_offset":23068,"parent_record_offset":22704,"ancestor_record_offset":22704,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$509::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":153,"record_offset":22860,"parent_record_offset":22704,"ancestor_record_offset":22704,"inlinee":11653,"source_line":610,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram.rs
//     evidence={"module":153,"record_offset":22888,"parent_record_offset":22704,"ancestor_record_offset":22704,"inlinee":6806,"source_line":27,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":153,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":22704,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":153,"record_offset":22932,"parent_record_offset":22888,"ancestor_record_offset":22704,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_world_pixel
// 记录形式: inlinee_declaration; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A7E
// 首条位置: {"module":28,"inlinee":6782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_ui_pixel
// 记录形式: inlinee_declaration; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A7D
// 首条位置: {"module":28,"inlinee":6781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A96
// 首条位置: {"module":153,"inlinee":6806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":153,"record_offset":22932,"parent_record_offset":22888,"ancestor_record_offset":22888,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":153,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":22888,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_max_resolution_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":21900,"rva":2822320,"va":5371531440}
// 编译实例 RVAs: 0x2b10b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4784967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371531440,"raw_hex":"e847034900","size":5,"soft_fail":false,"target_va":5376316464,"va":5371531492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4681814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371531440,"raw_hex":"e856704700","size":5,"soft_fail":false,"target_va":5376213328,"va":5371531509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4681981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371531440,"raw_hex":"e8fd704700","size":5,"soft_fail":false,"target_va":5376213744,"va":5371531758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::enums::impl$509::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":153,"record_offset":22040,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":11653,"source_line":610,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":153,"record_offset":22464,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::vram::calculate_max_resolution_scale_from_model
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram.rs
//     evidence={"module":153,"record_offset":22068,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":11655,"source_line":66,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":153,"record_offset":22124,"parent_record_offset":22068,"ancestor_record_offset":21900,"inlinee":11656,"source_line":935,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":153,"record_offset":22152,"parent_record_offset":22068,"ancestor_record_offset":21900,"inlinee":11657,"source_line":955,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":153,"record_offset":22208,"parent_record_offset":22068,"ancestor_record_offset":21900,"inlinee":11658,"source_line":441,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_max_resolution_scale_from_model
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D87
// 首条位置: {"module":153,"inlinee":11655}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":153,"record_offset":22124,"parent_record_offset":22068,"ancestor_record_offset":22068,"inlinee":11656,"source_line":935,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":153,"record_offset":22152,"parent_record_offset":22068,"ancestor_record_offset":22068,"inlinee":11657,"source_line":955,"archive":"_pdb/A/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] std::f64::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":153,"record_offset":22208,"parent_record_offset":22068,"ancestor_record_offset":22068,"inlinee":11658,"source_line":441,"archive":"_pdb/A/modules/00153.jsonl.gz"}
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
// 完整结构及原始类型记录: vram.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vram.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vram.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<usize,core::num::error::ParseIntError> > (*)(ref$<str$>)
// 候选参数类型: ["ref$<str$>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_max_resolution_scale
// 信息 ID: 99905618412672adb5157149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24a82b -> 0x2b10b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c79f6 -> 0x2b10b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_max_resolution_scale_from_model
// 信息 ID: 04561f49c10a9d9539d7affb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=153, inline_record=22068; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=153, inline_record=22268; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=153, inline_record=22492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_resolution_scale_mem_estimate_text
// 信息 ID: eb6a42c1c5e9c3a45c3a5db2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_ui_pixel
// 信息 ID: e0862f6a2609b9a76e403900
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_world_pixel
// 信息 ID: c7814ddbadb56c8cebbe862c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
// 信息 ID: b6e5972283889876c21406d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=153, inline_record=22888; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=153, inline_record=22992; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=153, inline_record=23028; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=153, inline_record=23152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
