// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/easing_functions.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e8769747b4d9fbcb62527545332294ce63f794fa420a39f91840d8362eeb8f26
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: easing_functions.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 614
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::EasingFunction::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":614,"record_offset":244,"rva":8228944,"va":5376938064}
// 编译实例 RVAs: 0x7d9050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26655689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938064,"raw_hex":"e8c9bb9601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376938306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26655773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938064,"raw_hex":"e81dbc9601","size":5,"soft_fail":false,"target_va":5403594048,"va":5376938270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26656098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938064,"raw_hex":"e862bd9601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376938233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26040376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938064,"raw_hex":"e838588d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5376938771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::easing_functions::circular_in_out
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":400,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36042,"source_line":69,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_in_out
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":736,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36046,"source_line":78,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_out
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":900,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36048,"source_line":37,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_in_out_elastic
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":496,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36043,"source_line":54,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_out_back
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":780,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36047,"source_line":49,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::shoulder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":684,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36045,"source_line":88,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::sine_in_out
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":614,"record_offset":628,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":36044,"source_line":41,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":614,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":244,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":244,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":568,"parent_record_offset":496,"ancestor_record_offset":244,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":840,"parent_record_offset":780,"ancestor_record_offset":244,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":540,"parent_record_offset":496,"ancestor_record_offset":244,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":436,"parent_record_offset":400,"ancestor_record_offset":244,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CD0
// 首条位置: {"module":614,"inlinee":36048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::sine_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCC
// 首条位置: {"module":614,"inlinee":36044}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_out_back
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCF
// 首条位置: {"module":614,"inlinee":36047}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":614,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":840,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":868,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_in_out_elastic
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCB
// 首条位置: {"module":614,"inlinee":36043}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":568,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":540,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::circular_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCA
// 首条位置: {"module":614,"inlinee":36042}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":614,"record_offset":436,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_in_out
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCE
// 首条位置: {"module":614,"inlinee":36046}
// 编译实例 RVAs: 0x7d9340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::shoulder
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CCD
// 首条位置: {"module":614,"inlinee":36045}
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
// 完整结构及原始类型记录: easing_functions.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::easing_functions::EasingFunction
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: easing_functions.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: easing_functions.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>

// ------------------------------------------------------------------
// 函数: utils::easing_functions::EasingFunction::apply
// 信息 ID: 05ffc116ecfb8e0a4c75fd03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 10e73e9a1cfddff55a46e15d: void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340125 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340416 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x3407cb -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::animation::signifier_animation_t @ 0x5beb23 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x5c46c2 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x69ed58 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x6b9fbc -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x6b9e59 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x6b9ec2 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x6b9d1c -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape @ 0x6bae02 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape @ 0x6bae41 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>),void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)> > @ 0xe3c18a -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>),void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)> @ 0x10539f8 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0x112dad7 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_animation_system @ 0x147b815 -> 0x7d9050 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::circular_in_out
// 信息 ID: 4b784167b77ca911dd50d151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_in_out
// 信息 ID: 06f544627d4eb9c1a7de74dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze @ 0x56c561 -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0x664c69 -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7eef -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7f49 -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca584 -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca5de -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xeccc2f -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xeccc89 -> 0x7d9340 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=736; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_out
// 信息 ID: b99d6ff0eb4ba30dd8b79bfc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_in_out_elastic
// 信息 ID: 732c519bc5e51ca5d9f36ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_out_back
// 信息 ID: 9e23f72e0ab273b6e7fc882b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::shoulder
// 信息 ID: 5d95668521096a0e00f4763c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::sine_in_out
// 信息 ID: 2df4c852061ac96e337581a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=614, inline_record=628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
