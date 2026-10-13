// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/winit_bevy_glue.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 303bfe091febb9c241ee1331ceb22ee2317587a5f6cfc256b048abfba55772e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: winit_bevy_glue.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 23
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_bevy_glue::winit_to_bevy_keycode
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":23,"record_offset":536,"rva":1327664,"va":5370036784}
// 编译实例 RVAs: 0x144230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_bevy_glue::winit_to_bevy_logical_key
// 记录形式: compiled_procedure; instances=1
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":23,"record_offset":664,"rva":1327728,"va":5370036848}
// 编译实例 RVAs: 0x144270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::impl$0::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":23,"record_offset":1108,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":6961,"source_line":77,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_direct] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":23,"record_offset":1136,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_bevy_glue::convert_native_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_bevy_glue.rs
//     evidence={"module":23,"record_offset":844,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":6959,"source_line":567,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_bevy_glue::named_key_to_bevy
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_bevy_glue.rs
//     evidence={"module":23,"record_offset":816,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":6958,"source_line":247,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":23,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":664,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":23,"record_offset":964,"parent_record_offset":924,"ancestor_record_offset":664,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":23,"record_offset":1032,"parent_record_offset":1000,"ancestor_record_offset":664,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":23,"record_offset":1060,"parent_record_offset":1000,"ancestor_record_offset":664,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":23,"record_offset":892,"parent_record_offset":844,"ancestor_record_offset":664,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$38::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":23,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":664,"inlinee":6700,"source_line":410,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_bevy_glue::named_key_to_bevy
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B2E
// 首条位置: {"module":23,"inlinee":6958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_bevy_glue::convert_native_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 567
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B2F
// 首条位置: {"module":23,"inlinee":6959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":23,"record_offset":892,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":23,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":844,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":23,"record_offset":964,"parent_record_offset":924,"ancestor_record_offset":844,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":23,"record_offset":1032,"parent_record_offset":1000,"ancestor_record_offset":844,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":23,"record_offset":1060,"parent_record_offset":1000,"ancestor_record_offset":844,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00023.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$38::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":23,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":844,"inlinee":6700,"source_line":410,"archive":"_pdb/C/modules/00023.jsonl.gz"}
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
// 完整结构及原始类型记录: winit_bevy_glue.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: winit_bevy_glue.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: winit_bevy_glue.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_writer::MessageWriter<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<system_wall_constructor::construct_arches::wooden_arch_frame::WoodenArchCornerpieceDebugParams>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::message::message_writer::MessageWriter<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<system_wall_constructor::construct_arches::wooden_arch_frame::WoodenArchCornerpieceDebugParams>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::construct_arches::wooden_arch_frame::WoodenArchCornerpieceDebugParams>

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_bevy_glue::convert_native_key
// 信息 ID: 549a501d9171f6308e642f3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_bevy_glue::winit_to_bevy_logical_key; module=23, inline_record=844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_bevy_glue::named_key_to_bevy
// 信息 ID: f43b600ac7dbe86fb7df71bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_bevy_glue::winit_to_bevy_logical_key; module=23, inline_record=816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_bevy_glue::winit_to_bevy_keycode
// 信息 ID: 557ab7905f204c018f35c2c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_bevy_glue::winit_to_bevy_logical_key
// 信息 ID: 91d7698a8ccaf8742c371a43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
